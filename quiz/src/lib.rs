//! A simple program that implements the user tries in minesweeper

// These two lines are necessary for the program to properly compile.

// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_sol_types::{sol, SolType, SolValue};
use std::io;
//use rand::prelude::*;
use rand::Rng;
sol! {
    struct ZkState {
        tuple(uint32,uint32)[] stored_bombs;
        tuple(uint32, uint32, bool)[] users_guess;
    }

}

pub fn main() {
    // Read an input to the program.
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // set initial program state
    let max_tries = 10;
    println!("enter the bounty that you want to commit with the max_difference");

    let mut max_diff = String::new();
    let mut user_amt_type = String::new();
    let mut size = String::new();

    io::stdin().read_line(&mut max_diff);
    io::stdin().read_line(&mut user_amt_type);
    io::stdin().read_line(&mut size);

    // Parse values with error handling using `parse()`
    let max_difference: u32 = max_diff.trim().parse().expect("add the difference");
    // let user_amount_committed = user_amt_type.trim().parse().expect("add committed amount");
    let dimensions = size.trim().parse().expect("Please type a dimensions");

    let mut results: Vec<(u32, u32, bool)> = vec![];
    let mut bomb_locations: Vec<(u32, u32)> = vec![];

    let mut random = rand::thread_rng();
    // let mut users_guess_state = vec![];
    let user_commit_amount = sp1_zkvm::io::read::<u8>;

    // let program_state;
    let mut stored_bombs = vec![];

    let mut num_of_bomb: i32 = (dimensions as f32).sqrt() as i32;
    for i in 0..num_of_bomb {
        stored_bombs.push((
            random.gen_range(0..dimensions) as u32,
            random.gen_range(0..dimensions) as u32,
        ));
    }
    // do the tries and try to check the distance of the user guess n from the currently initialized bomb

    let mut a = 0;
    while a < max_tries {
        let n = sp1_zkvm::io::read::<(u32, u32)>();
        if n < (0, 0) {
            panic!("This program doesn't support negative numbers.");
        }

        for i in 0..num_of_bomb {
            let (store_bomb_xpt, store_bomb_ypt) = stored_bombs[i as usize];

            if store_bomb_xpt == n.0 && store_bomb_ypt == n.1 {
                let users_guess = (n.0, n.1, false);
                //let bytes = ZkState::abi_encode(&(users_guess, (store_bomb_xpt, store_bomb_ypt)));
                bomb_locations.push((n.0, n.1));

                results.push(users_guess);
                // sp1_zkvm::io::commit(&bytes);
            } else {
                let users_guess = (n.0, n.1, true);
                //let bytes = ZkState::abi_encode(&(users_guess, (store_bomb_xpt, store_bomb_ypt)));
                bomb_locations.push((n.0, n.1));
                results.push(users_guess);
                //sp1_zkvm::io::commit(&bytes);
            }
            let state = ZkState {
                stored_bombs: stored_bombs.clone(),
                users_guess: results.clone(),
            };
            let bytes = state.abi_encode();

            sp1_zkvm::io::commit(&bytes);
        }
        a += 1;
    }

    print!("THE GAME IS OVER");
}
