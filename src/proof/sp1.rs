use rand::Rng;
use sp1_sdk::{ProverClient, SP1Stdin};

const ELF: &[u8] = include_bytes!("../../mine/elf/riscv32im-succinct-zkvm-elf");
/// The size of the mine field. The amount of bombs is determined by the
/// square root of the size. Here we get 3 bombs.
const FIELD_SIZE: u8 = 10;

/// Returns a proof for the execution of the mine game with the given guesses,
/// in the form of a serialized `Proof` struct.
pub fn prove_mine_game(guesses: Vec<(u8, u8)>) -> anyhow::Result<Vec<u8>> {
    // Create a new prover client
    let client = ProverClient::new();
    let (pk, _) = client.setup(ELF);

    // Create a new SP1Stdin instance and write the guesses to it
    let mut input = SP1Stdin::new();
    // Write the field size to the input
    input.write(&FIELD_SIZE);

    // Write the bomb locations to the input
    let mut bombs_location_x = [0u8; 3];
    let mut bombs_location_y = [0u8; 3];
    rand::thread_rng().fill(&mut bombs_location_x);
    rand::thread_rng().fill(&mut bombs_location_y);
    let bombs_location = bombs_location_x
        .iter()
        .zip(bombs_location_y.iter())
        .map(|(x, y)| (*x % FIELD_SIZE, *y % FIELD_SIZE))
        .collect::<Vec<_>>();
    input.write(&bombs_location);

    // Write the guesses to the input
    input.write(&guesses);

    tracing::info!("Proving mine game with guesses: {:?}", guesses);

    let proof = client.prove_compressed(&pk, input)?;

    Ok(bincode::serialize(&proof)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_prove_mine_game() {
        let guesses = vec![
            (0, 0),
            (1, 1),
            (2, 2),
            (3, 3),
            (2, 4),
            (5, 4),
            (6, 6),
            (7, 4),
            (8, 9),
            (9, 9),
        ];

        let proof = prove_mine_game(guesses).unwrap();

        assert!(!proof.is_empty());
    }
}
