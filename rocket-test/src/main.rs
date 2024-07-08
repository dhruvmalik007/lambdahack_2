#[macro_use]
extern crate rocket;
use derive_adhoc::{define_derive_adhoc, Adhoc};
use mining::proof::sp1;
use rocket::serde::{json::Json, Serialize};
use rocket::Response;
use rocket_cors::{AllowedHeaders, AllowedMethods, AllowedOrigins, CorsOptions};
use sha2::Digest;
use sha256::{digest_bytes};
use ethers::{
    middleware::SignerMiddleware,
    providers::{Http, Middleware, Provider},
    signers::{LocalWallet, Signer},
    types::{NameOrAddress, TransactionRequest, U256},
};

use rs_merkle::MerkleTree;


struct ResultGame {
    result: bool,
}

// corresponding to the input of the batch proof
#[derive(Serialize)]
struct InputResult {
    proofCommitment: Vec<u8>,
    pubInputCommitment: Vec<u8>,
    provingSystemAuxDataCommitment: Vec<u8>, // elf file hash
    proofGeneratorAddr: [u8; 20],
    batchMerkleRoot: Vec<u8>,
    merkleProof: Vec<u8>,
    verificationDataBatchIndex: U256,
}

#[get("/submit_proofs")]
fn index(guesses: Vec<(u8, u8)>) -> Response<any, Error> {
    sp1::prove_mine_game(guesses)
}
#[get("/get_result")]
fn call_result_game(guesses: Vec<(u8, u8)>) -> Response<Json<ResultGame>, Error> {
    let  mut proof_gen_address: String = "0x61ca19b7717d47Ce63C361ABe54c69E4dE68478f" 
    let public_commitments: Vec<u8> = sp1::prove_mine_game(guesses);
    let bin_elf_file = include_bytes!("../../mine/elf/riscv32im-succinct-zkvm-elf");
    let elf_commitments = sha2::Sha256::digest(bin_elf_file);
    let initial_param: InputResult = InputResult {
        proofCommitment: proof_commitments,
        pubInputCommitment: guesses,
        provingSystemAuxDataCommitment: elf_commitments ,
        proofGeneratorAddr: proof_gen_address,
        // TODO: @greg can you check the implementation
        // batchMerkleRoot: ,
        // merkleProof: vec![],
        // verificationDataBatchIndex: U256::from(0),
    };

    let result_game = ResultGame { result: 0 };
    Ok(Json(result_game))
}

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::all();
    let cors = AdHoc::on_ignite("CORS Config", |rocket| {
        let allowed_origins = AllowedOrigins::all();
        let cors = CorsOptions {
            allowed_origins,
            allowed_headers: AllowedHeaders::all(),
            allow_credentials: true,
            ..Default::default()
        }
        .to_cors()
        .expect("error creating CORS");
        Ok(rocket.manage(cors))
    });

    rocket::build()
        .mount("/submit_proofs", routes![index])
        .attach(cors)
}
