//! A simple program that implements the user tries in minesweeper

// These two lines are necessary for the program to properly compile.

// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]

sp1_zkvm::entrypoint!(main);

use alloy_sol_types::{sol, SolValue};

sol! {
    /// Commitment to the game state
    struct GameCommitment {
        tuple(uint8, uint8)[] user_guesses;
        /// Whether the game is won or not
        bool result;
    }
}

pub fn main() {
    // Read inputs to the program.
    let size = sp1_zkvm::io::read::<u8>();
    let stored_bombs = sp1_zkvm::io::read::<Vec<(u8, u8)>>();
    let user_guesses = sp1_zkvm::io::read::<Vec<(u8, u8)>>();

    // Check if the number of bombs is within the bounds
    let max_bombs = (size as f32).sqrt() as u8;
    if stored_bombs.len() > max_bombs as usize {
        panic!("Too many bombs");
    }

    // Verify bounds
    for ((b_x, b_y), (u_x, u_y)) in stored_bombs.iter().zip(user_guesses.iter()) {
        if *b_x > size - 1 || *b_y > size - 1 || *u_x > size - 1 || *u_y > size - 1 {
            panic!("Out of bounds");
        }
    }

    // Iterate over the tries and check if the user has hit a bomb
    let mut hit = false;
    for g in user_guesses.iter() {
        if stored_bombs.contains(g) {
            hit = true;
            break;
        }
    }

    let bytes = GameCommitment {
        result: !hit,
        user_guesses,
    }
    .abi_encode();

    sp1_zkvm::io::commit(&bytes);
}
