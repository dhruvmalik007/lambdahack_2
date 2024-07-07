//! A simple program that implements the user tries in minesweeper

// These two lines are necessary for the program to properly compile.

// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_sol_types::{sol, SolValue};

sol! {
    struct GameResult {
        tuple(uint8, uint8)[] stored_bombs;
        tuple(uint8, uint8)[] user_guesses;
        bool hit;
    }

}

pub fn main() {
    // Read an input to the program.
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // set initial program state
    let max_tries = 10;

    let size = sp1_zkvm::io::read::<u8>();
    let stored_bombs = sp1_zkvm::io::read::<Vec<(u8, u8)>>();
    let user_guesses = sp1_zkvm::io::read::<Vec<(u8, u8)>>();

    // Verify bounds
    for ((b_x, b_y), (u_x, u_y)) in stored_bombs.iter().zip(user_guesses.iter()) {
        if *b_x >= size - 1 || *b_y >= size - 1 || *u_x >= size - 1 || *u_y >= size - 1 {
            panic!("Out of bounds");
        }
    }

    // Check if the user has tried too many times
    if user_guesses.len() > max_tries {
        panic!("Too many tries");
    }

    // Iterate over the tries and check if the user has hit a bomb
    let mut hit = false;
    for g in user_guesses.iter() {
        if stored_bombs.contains(g) {
            hit = true;
            break;
        }
    }

    let game_result = GameResult {
        stored_bombs,
        user_guesses,
        hit,
    };
    let bytes = game_result.abi_encode();

    // TODO(greg) check if we need to commit to all this
    sp1_zkvm::io::commit(&bytes);
}
