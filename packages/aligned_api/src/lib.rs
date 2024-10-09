pub mod ethereum;
pub mod proof;
pub mod utils;

use aligned_sdk::{
    sdk::{submit, verify_proof_onchain},
    types::{AlignedVerificationData, Chain, ProvingSystemId, VerificationData},
};
use ethers::signers::Signer;

const BATCHER_URL: &str = "wss://batcher.alignedlayer.com";
const BATCHER_COST: u64 = 4e15 as u64;
const RETRY_INTERVAL_SEC: u64 = 10;
const MAX_RETRIES: u64 = 10;

/// Pays the batcher costs and submits the proof to the batcher.
pub async fn pay_costs_and_submit_proof(
    proof: Vec<u8>,
    pub_input: Option<Vec<u8>>,
    program_code: Vec<u8>,
    proving_system: ProvingSystemId,
) -> anyhow::Result<AlignedVerificationData> {
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
    let verification_data = submit(BATCHER_URL, &verification_data, ethereum::WALLET.clone())
        .await
        .map_err(|e| anyhow::anyhow!("Failed to submit proof for verification: {:?}", e))?
        .ok_or(anyhow::anyhow!("Failed to submit proof for verification"))?;

    Ok(verification_data)
}

/// Waits for the proof to be confirmed on-chain.
pub async fn wait_for_proof_confirmation(
    verification_data: AlignedVerificationData,
) -> anyhow::Result<()> {
    for _ in 0..MAX_RETRIES {
        if verify_proof_onchain(
            verification_data.clone(),
            Chain::Holesky,
            &ethereum::RPC_URL,
        )
        .await
        .inspect_err(|e| tracing::error!("Failed to verify proof: {:?}", e))
        .is_ok_and(|r| r)
        {
            tracing::info!("Proof confirmed on chain");
            return Ok(());
        }
        tracing::info!(
            "Proof not yet confirmed, retrying in {} seconds",
            RETRY_INTERVAL_SEC
        );
        tokio::time::sleep(std::time::Duration::from_secs(RETRY_INTERVAL_SEC)).await;
    }

    anyhow::bail!("Proof not confirmed after {} retries", MAX_RETRIES);
}

// #[cfg(test)]
// pub mod tests {
//     use super::*;
//     use proof::sp1;

//     #[tokio::test]
//     async fn test_pay_costs_and_submit_proof() {
//         crate::utils::setup();

//         // Given
//         let serialized_proof = sp1::prove_mine_game(vec![(0, 2), (1, 5)]).expect("failed to prove");
//         let proof = bincode::deserialize::<sp1_sdk::SP1CompressedProof>(&serialized_proof)
//             .expect("failed to deserialize proof");
//         let code = sp1::ELF.to_vec();

//         // When
//         let verification_data = pay_costs_and_submit_proof(
//             serialized_proof.clone(),
//             Some(proof.public_values.to_vec()),
//             code,
//             ProvingSystemId::SP1,
//         )
//         .await
//         .expect("failed to pay costs and submit proof");

//         // Then
//         wait_for_proof_confirmation(verification_data)
//             .await
//             .expect("failed to wait for confirmation");
//     }
// }
