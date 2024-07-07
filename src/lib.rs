use aligned_sdk::{
    sdk::submit,
    types::{ProvingSystemId, VerificationData},
};
use ethers::{signers::Signer, types::H160};

pub mod ethereum;
pub mod proof;

const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";
const BATCHER_COST: u64 = 0;

/// Pays the batcher costs and submits the proof to the batcher.
pub async fn pay_costs_and_submit_proof(
    proof: Vec<u8>,
    pub_input: Option<Vec<u8>>,
    program_code: Vec<u8>,
    proving_system: ProvingSystemId,
) -> anyhow::Result<()> {
    // Pay the batcher costs
    ethereum::pay_batcher_costs(BATCHER_COST).await?;

    // Set up the verification data structure
    let verification_data = VerificationData {
        proving_system,
        proof,
        pub_input,
        verification_key: None,
        vm_program_code: program_code.into(),
        proof_generator_addr: ethereum::WALLET.address(),
    };

    // Submit the proof to the batcher
    let res = submit(BATCHER_URL, &verification_data, ethereum::WALLET.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit proof for verification: {:?}", e))?;

    Ok(())
}
