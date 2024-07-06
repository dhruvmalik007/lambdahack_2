//! A simple program that implements the user tries in minesweeper

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_sol_types::{sol, SolType};
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::io;

/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type ZkState = sol! {
stored_bombs: Vec<(u32,u32)>,
users_guess: Vec<(u32, u32, bool)>,
};

pub fn main() {
    // Read an input to the program.
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // set initial program state
    let max_tries = 10;
    println!("enter the bounty that you want to commit with the max_difference");
    let max_difference: u32 = io::stdin()
        .read_line()
        .parse()
        .expect("Failed to read max difference");
    let user_amount_committed: u32 = io::stdin()
        .read_line()
        .parse()
        .expect("Failed to read user amount committed");
    let dimensions: u8 = io::stdin()
        .read_line()
        .parse()
        .expect("Failed to read dimensions");

    let results: Vec<(u32, u32, bool)> = vec![];

    let user_amount_committed: u32 = io::stdin().read_line();
    let dimensions: u8 = io::stdin().read_line();
    let mut random = rand::thread_rng();
    let mut users_guess_state = vec![];
    let user_commit_amount = sp1_zkvm::io::read::<u8>;

    let program_state = ZkState::new((0, 0), (0, 0, false), vec![]);
    if n < 0 {
        panic!("This program doesn't support negative numbers.");
    }
    let mut stored_bombs = vec![];

    for i in 0..dimensions.sqrt() {
        stored_bombs.push((
            random.gen_range(0..dimensions),
            random.gen_range(0..dimensions),
        ));
    }

    // do the tries and try to check the distance of the user guess n from the currently initialized bomb

    let mut a = 0;
    while a < max_tries {
        let n = sp1_zkvm::io::read::<(u32, u32)>();
        let temp_distance = 0;

        for i in 0..dimensions.sqrt() {
            let stored_bomb = stored_bombs[i];
            let distance =
                ((stored_bomb.0 - n.0).pow(2) + (stored_bomb.1 - n.1).pow(2) as u32).sqrt();
            temp_distance += distance;

            if distance <= max_difference {
                let users_guess = (n.0, n.1, false);
                let bytes = ZkState::abi_encode(&(users_guess, stored_bomb));
                results.push(users_guess);
                sp1_zkvm::io::commit(&bytes);
            } else {
                let users_guess = (n.0, n.1, false);
                let bytes = ZkState::abi_encode(&(users_guess, stored_bomb));
                results.push(users_guess);
                sp1_zkvm::io::commit(&bytes);
            }
        }

        a += 1;
    }

    print("THE GAME IS OVER");
}

#[cfg(test)]
mod tests {}
