#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate rocket;

use std::str::FromStr;

use aligned_sdk::types::{AlignedVerificationData, ProvingSystemId};
use alloy::{
    network::{Ethereum, EthereumWallet},
    primitives::{Address, Bytes, B256, U256},
    providers::{
        fillers::{ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller, WalletFiller},
        Identity, ProviderBuilder, RootProvider,
    },
    signers::local::PrivateKeySigner,
    transports::http::Http,
};
use alloy_sol_types::sol;
use mining::{pay_costs_and_submit_proof, proof::sp1, wait_for_proof_confirmation};
use mongodb::{
    options::{DatabaseOptions, ReadConcern, WriteConcern},
    Database,
};
use reqwest::{Client, Url};
use rocket::{fairing::AdHoc, serde::json::Json};
use rocket_cors::{AllowedHeaders, AllowedOrigins, CorsOptions};
use serde::{Deserialize, Serialize};

sol!(
    #[sol(rpc)]
    contract Escrow {
        struct SettlementInput {
            uint256 gameId;
            bool result;
            bytes32[] proofCommitment;
            bytes32 provingSystemAuxDataCommitment;
            bytes20 proofGeneratorAddr;
            bytes32[] batchMerkleRoot;
            bytes[] merkleProof;
            uint256[] verificationDataBatchIndex;
        }
        function settleBet(SettlementInput calldata input) external;
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

#[derive(Deserialize, Serialize, Clone)]
pub struct Guesses {
    guesses: Vec<(u8, u8)>,
    game_id: u128,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameVerificationData {
    data: AlignedVerificationData,
    game: Guesses,
}

type AlloyProvider = FillProvider<
    JoinFill<
        JoinFill<JoinFill<JoinFill<Identity, GasFiller>, NonceFiller>, ChainIdFiller>,
        WalletFiller<EthereumWallet>,
    >,
    RootProvider<Http<Client>>,
    Http<Client>,
    Ethereum,
>;
lazy_static::lazy_static! {
    static ref PROVIDER: AlloyProvider = {
        let rpc_url = Url::parse(&std::env::var("RPC_URL").unwrap()).unwrap();
        let signer = PrivateKeySigner::from_str(&std::env::var("PRIVATE_KEY").unwrap()).unwrap();
        let ethereum_wallet = EthereumWallet::from(signer);
        ProviderBuilder::new().with_recommended_fillers().wallet(ethereum_wallet).on_http(rpc_url)
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

#[post("/start_game", format = "json", data = "<guesses>")]
async fn start_game(guesses: Json<Guesses>) -> Json<bool> {
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

    Json(true)
}

fn settlement_input(
    id: u128,
    outcome: bool,
    verification_data: GameVerificationData,
) -> Escrow::SettlementInput {
    let proof_commitment = B256::from(
        verification_data
            .data
            .verification_data_commitment
            .proof_commitment,
    );
    let proof_aux_data_commitment = B256::from(
        verification_data
            .data
            .verification_data_commitment
            .proving_system_aux_data_commitment,
    );
    let proof_generator_addr = Address::from(
        verification_data
            .data
            .verification_data_commitment
            .proof_generator_addr,
    );
    let proof_merkle_root = B256::from(verification_data.data.batch_merkle_root);
    let proof_merkle_path = Bytes::from(
        verification_data
            .data
            .batch_inclusion_proof
            .merkle_path
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
    );

    // Settle the bet with the outcome
    Escrow::SettlementInput {
        gameId: U256::from(id),
        result: outcome,
        proofCommitment: vec![proof_commitment],
        provingSystemAuxDataCommitment: proof_aux_data_commitment,
        proofGeneratorAddr: *proof_generator_addr,
        batchMerkleRoot: vec![proof_merkle_root],
        merkleProof: vec![proof_merkle_path.clone()],
        verificationDataBatchIndex: vec![U256::from(verification_data.data.index_in_batch)],
    }
}

async fn send_settle_bet(
    id: u128,
    outcome: bool,
    verification_data: GameVerificationData,
) -> anyhow::Result<()> {
    let batch_inclusion_data = settlement_input(id, outcome, verification_data.clone());
    let escrow = Escrow::new(*ESCROW_ADDRESS, PROVIDER.clone());
    let call = escrow.settleBet(batch_inclusion_data.clone());
    let _ = call
        .send()
        .await?
        .with_required_confirmations(2)
        .with_timeout(std::time::Duration::from_secs(60).into())
        .get_receipt()
        .await?;
    Ok(())
}

#[get("/settle_bet/<id>")]
async fn settle_bet(id: u128) {
    // Fetch the verification data from the MongoDB
    let mongo = get_mongo().await.expect("Failed to get MongoDB");
    let maybe_verification_data = mongo
        .collection::<GameVerificationData>("verification_data")
        .find_one(mongodb::bson::doc! {"game_id": id as u32})
        .await
        .expect("Failed to find verification data in MongoDB");
    if maybe_verification_data.is_none() {
        return;
    }
    let verification_data = maybe_verification_data.unwrap();

    // Settle the bet with the winning outcome
    let _ = send_settle_bet(id, true, verification_data.clone()).await;
    let _ = send_settle_bet(id, false, verification_data).await;
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
        .mount("/", routes![start_game, settle_bet])
        .attach(cors)
}
