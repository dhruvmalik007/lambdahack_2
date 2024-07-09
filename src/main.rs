#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate rocket;

use std::str::FromStr;

use aligned_sdk::types::{AlignedVerificationData, ProvingSystemId};
use alloy::{
    network::Ethereum,
    primitives::{Address, Bytes, B256, U256},
    providers::{
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller},
        Identity, ProviderBuilder, RootProvider,
    },
    transports::http::Http,
};
use alloy_sol_types::{sol, SolValue};
use mining::{pay_costs_and_submit_proof, proof::sp1, wait_for_proof_confirmation};
use mongodb::{
    options::{DatabaseOptions, ReadConcern, WriteConcern},
    Database,
};
use reqwest::{Client, Url};
use rocket::{fairing::AdHoc, serde::json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};
use tiny_keccak::Hasher;
use Escrow::GameCommitment;

sol!(
    #[sol(rpc)]
    contract Escrow {
        function verifyBatchInclusion(
            bytes32 proofCommitment,
            bytes32 pubInputCommitment,
            bytes32 provingSystemAuxDataCommitment,
            bytes20 proofGeneratorAddr,
            bytes32 batchMerkleRoot,
            bytes memory merkleProof,
            uint256 verificationDataBatchIndex
        ) public view returns (bool);
        /// Commitment to the game state
        struct GameCommitment {
            tuple(uint8, uint8)[] user_guesses;
            /// Whether the game is won or not
            bool result;
        }
    }
);

#[derive(Deserialize, Serialize)]
pub struct Guesses {
    guesses: Vec<(u8, u8)>,
    game_id: u128,
}

#[derive(Serialize, Deserialize)]
pub struct GameVerificationData {
    data: AlignedVerificationData,
    game: Guesses,
}

type AlloyProvider = FillProvider<
    JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;
lazy_static::lazy_static! {
    static ref PROVIDER: AlloyProvider = {
        let rpc_url = Url::parse(&std::env::var("RPC_URL").unwrap()).unwrap();
        ProviderBuilder::new().with_recommended_fillers().on_http(rpc_url)
    };

    static ref ESCROW_ADDRESS: Address = Address::from_str(&std::env::var("ESCROW_ADDRESS").unwrap()).unwrap();
}

async fn get_mongo() -> anyhow::Result<Database> {
    // Connect to the MongoDB and save the verification data
    let db_client =
        mongodb::Client::with_uri_str(std::env::var("MONGO_CONNECTION_STRING")?).await?;

    Ok(db_client.database_with_options(
        &std::env::var("MONGO_DATABASE_NAME")?,
        DatabaseOptions::builder()
            .read_concern(ReadConcern::majority())
            .write_concern(WriteConcern::majority())
            .build(),
    ))
}

#[post("/submit_proofs", format = "json", data = "<guesses>")]
async fn submit_proofs(guesses: Json<Guesses>) -> Json<AlignedVerificationData> {
    // Proof the game and submit the proof
    let serialized_proof =
        sp1::prove_mine_game(guesses.0.guesses.clone()).expect("failed to prove");
    let proof: sp1_sdk::SP1CompressedProof =
        bincode::deserialize(&serialized_proof).expect("failed to deserialize");

    // Pay the costs and submit the proof
    let aligned_verification_data = pay_costs_and_submit_proof(
        serialized_proof,
        proof.public_values.to_vec().into(),
        sp1::ELF.to_vec(),
        ProvingSystemId::SP1,
    )
    .await
    .expect("failed to pay costs and submit proof");

    // Wait for the proof confirmation
    wait_for_proof_confirmation(aligned_verification_data.clone())
        .await
        .expect("failed to wait for proof confirmation");

    let db = get_mongo().await.expect("Failed to get MongoDB");
    let verification_data = GameVerificationData {
        data: aligned_verification_data.clone(),
        game: guesses.0,
    };
    db.collection("verification_data")
        .insert_one(verification_data)
        .await
        .expect("Failed to insert verification data into MongoDB");

    Json(aligned_verification_data)
}

#[get("/game_result/<id>")]
async fn game_result(id: u128) -> Json<bool> {
    // Fetch the verification data from the MongoDB
    let mongo = get_mongo().await.expect("Failed to get MongoDB");
    let maybe_verification_data = mongo
        .collection::<GameVerificationData>("verification_data")
        .find_one(mongodb::bson::doc! {"game_id": id as u32})
        .await
        .expect("Failed to find verification data in MongoDB");
    if maybe_verification_data.is_none() {
        return Json(false);
    }
    let verification_data = maybe_verification_data.unwrap();

    // Public commit with a true outcome value
    let bytes = GameCommitment {
        user_guesses: verification_data.game.guesses,
        result: true,
    }
    .abi_encode();
    // Hash twice to get the commitment
    let mut hasher = tiny_keccak::Keccak::v256();
    hasher.update(&bytes);
    let mut output = [0; 32];
    hasher.finalize(&mut output);

    let mut hasher = tiny_keccak::Keccak::v256();
    hasher.update(&output);
    let mut pub_commitment = [0; 32];
    hasher.finalize(&mut pub_commitment);

    // Check if the proof is valid
    let data = verification_data.data.clone();
    let escrow = Escrow::new(*ESCROW_ADDRESS, PROVIDER.clone());
    let res = escrow
        .verifyBatchInclusion(
            B256::from_slice(&data.verification_data_commitment.proof_commitment),
            B256::from_slice(&pub_commitment),
            B256::from_slice(
                &data
                    .verification_data_commitment
                    .proving_system_aux_data_commitment,
            ),
            *Address::from_slice(&data.verification_data_commitment.proof_generator_addr),
            B256::from_slice(&data.batch_merkle_root),
            Bytes::from(
                data.batch_inclusion_proof
                    .merkle_path
                    .clone()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>(),
            ),
            U256::from(data.index_in_batch),
        )
        .call()
        .await;
    if res.is_err() {
        return Json(false);
    }
    let outcome = res.unwrap()._0;

    Json(outcome)
}

#[launch]
fn rocket() -> _ {
    let cors = AdHoc::on_ignite("CORS Config", |rocket| async move {
        let cors = CorsOptions {
            allowed_origins: AllowedOrigins::all(),
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("error creating CORS");
        rocket.manage(cors)
    });

    rocket::build()
        .mount("/", routes![submit_proofs, game_result])
        .attach(cors)
}
