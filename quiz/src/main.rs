//! A simple program that implements the user tries in minesweeper

// These two lines are necessary for the program to properly compile.

// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use alloy_sol_types::{sol, SolValue};
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

    let size = sp1_zkvm::io::read::<u32>();

    let mut results: Vec<(u32, u32, bool)> = vec![];
    let mut bomb_locations: Vec<(u32, u32)> = vec![];

    // let program_state;
    let mut stored_bombs = vec![];

    let num_of_bomb: i32 = (size as f32).sqrt() as i32;
    for _ in 0..num_of_bomb {
        stored_bombs.push((1, 2));
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
}
