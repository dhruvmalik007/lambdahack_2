use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../mine/elf/riscv32im-succinct-zkvm-elf");

/// Returns a proof for the execution of the mine game with the given guesses,
/// in the form of a serialized `Proof` struct.
pub fn prove_mine_game(guesses: Vec<(u8, u8)>) -> anyhow::Result<Vec<u8>> {
    // Create a new prover client
    let client = ProverClient::new();
    let (pk, _) = client.setup(ELF);

    // Create a new SP1Stdin instance and write the guesses to it
    let mut input = SP1Stdin::new();
    input.write(&guesses);

    tracing::info!("Proving mine game with guesses: {:?}", guesses);

    let proof = client.prove_compressed(&pk, input)?;

    Ok(bincode::serialize(&proof)?)
}
