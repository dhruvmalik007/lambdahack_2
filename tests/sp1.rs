//! End 2 end testing for the sp1 proving system along with verifying in Aligned and
//! finally settlement in the smart contract.
#![allow(clippy::too_many_arguments)]

use aligned_sdk::types::ProvingSystemId;
use alloy::{
    network::EthereumWallet,
    primitives::{Address, Bytes, B256, U256},
    providers::ProviderBuilder,
    signers::local::PrivateKeySigner,
    sol,
};
use mining::{pay_costs_and_submit_proof, proof::sp1, wait_for_proof_confirmation};
use reqwest::Url;
use std::str::FromStr;

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
        function startGame(uint8[2][] calldata guesses) payable external returns(uint256);
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
        function gameCommits(uint256) public view returns (bytes memory);
        /// Commitment to the game state
        struct GameCommitment {
            tuple(uint8, uint8)[] user_guesses;
            /// Whether the game is won or not
            bool result;
        }
    }
);

#[tokio::test]
async fn test_start_game() {
    // Setup
    mining::utils::setup();

    // Given
    let guesses = vec![(6, 6), (7, 4), (8, 9), (9, 9)];

    let signer = PrivateKeySigner::from_str(&std::env::var("PRIVATE_KEY").unwrap()).unwrap();
    let ethereum_wallet = EthereumWallet::from(signer);

    let rpc_url = Url::parse(&std::env::var("RPC_URL").unwrap()).unwrap();
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(ethereum_wallet)
        .on_http(rpc_url);

    // When
    let escrow = Escrow::new(
        Address::from_str("0x23057256D67C09F3f6E289AD045cD410D03c4680").unwrap(),
        provider,
    );
    let guesses = guesses.into_iter().map(|(x, y)| [x, y]).collect::<Vec<_>>();

    // Then
    escrow
        .startGame(guesses)
        .value(U256::from(1_000))
        .send()
        .await
        .unwrap()
        .with_required_confirmations(2)
        .with_timeout(Some(std::time::Duration::from_secs(60)))
        .get_receipt()
        .await
        .expect("failed to get receipt");
}

#[tokio::test]
async fn test_sp1_end_to_end() {
    // Setup
    mining::utils::setup();

    // Given
    let guesses = vec![(6, 6), (7, 4), (8, 9), (9, 9), (1, 1), (2, 1)];

    let signer = PrivateKeySigner::from_str(&std::env::var("PRIVATE_KEY").unwrap()).unwrap();
    let ethereum_wallet = EthereumWallet::from(signer);

    let rpc_url = Url::parse(&std::env::var("RPC_URL").unwrap()).unwrap();
    let provider = ProviderBuilder::new()
        .with_recommended_fillers()
        .wallet(ethereum_wallet)
        .on_http(rpc_url);

    // When
    let serialized_proof = sp1::prove_mine_game(guesses.clone()).unwrap();
    let proof: sp1_sdk::SP1CompressedProof = bincode::deserialize(&serialized_proof).unwrap();

    // Then
    assert!(!serialized_proof.is_empty());

    tracing::info!("public inputs {:?}", proof.public_values.to_vec());

    // When
    let verification_data = pay_costs_and_submit_proof(
        serialized_proof,
        proof.public_values.to_vec().into(),
        sp1::ELF.to_vec(),
        ProvingSystemId::SP1,
    )
    .await
    .unwrap();
    wait_for_proof_confirmation(verification_data.clone())
        .await
        .unwrap();

    let escrow = Escrow::new(
        Address::from_str("0x23057256D67C09F3f6E289AD045cD410D03c4680").unwrap(),
        provider,
    );
    let guesses = guesses.into_iter().map(|(x, y)| [x, y]).collect::<Vec<_>>();

    let receipt = escrow
        .startGame(guesses)
        .value(U256::from(1_000))
        .send()
        .await
        .unwrap()
        .with_required_confirmations(2)
        .with_timeout(std::time::Duration::from_secs(60).into())
        .get_receipt()
        .await
        .unwrap();
    tracing::info!("startGame receipt {:?}", receipt);

    let proof_commitment = B256::from(
        verification_data
            .verification_data_commitment
            .proof_commitment,
    );
    let proof_aux_data_commitment = B256::from(
        verification_data
            .verification_data_commitment
            .proving_system_aux_data_commitment,
    );
    let proof_generator_addr = Address::from(
        verification_data
            .verification_data_commitment
            .proof_generator_addr,
    );
    let proof_merkle_root = B256::from(verification_data.batch_merkle_root);
    let proof_merkle_path = Bytes::from(
        verification_data
            .batch_inclusion_proof
            .merkle_path
            .clone()
            .into_iter()
            .flatten()
            .collect::<Vec<_>>(),
    );

    let mut input = Escrow::SettlementInput {
        gameId: U256::ZERO,
        result: true,
        proofCommitment: vec![proof_commitment],
        provingSystemAuxDataCommitment: proof_aux_data_commitment,
        proofGeneratorAddr: *proof_generator_addr,
        batchMerkleRoot: vec![proof_merkle_root],
        merkleProof: vec![proof_merkle_path.clone()],
        verificationDataBatchIndex: vec![U256::from(verification_data.index_in_batch)],
    };
    let receipt = escrow
        .settleBet(input.clone())
        .send()
        .await
        .unwrap()
        .with_required_confirmations(2)
        .with_timeout(std::time::Duration::from_secs(60).into())
        .get_receipt()
        .await
        .unwrap();
    tracing::info!("settleBet receipt true {:?}", receipt);

    input.result = false;
    let settle_bet_call = escrow.settleBet(input.clone());
    let receipt = settle_bet_call.send().await;
    assert!(receipt.is_err());
}
