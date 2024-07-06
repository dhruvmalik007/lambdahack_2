//! A simple program that implements the user tries in minesweeper 

// These two lines are necessary for the program to properly compile.
//
// Under the hood, we wrap your main function with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use rand::Rng;
use alloy_sol_types::{sol, SolType};
use std::io;





/// The public values encoded as a tuple that can be easily deserialized inside Solidity.
type ProgramState = sol! {
stored_gift: u32,
users_guess: Vec<(u32, u32)>,
};

pub fn main() {
    // Read an input to the program.
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    // set initial program state
    let max_tries = 10;

    let boundations = 20;
    println!("enter the bounty that you want to commit with the max_difference");
    let max_difference: u32 = io::stdin().read_line();
    
    let user_amount_committed: u8 = io::stdin().read_line();

    let mut random = rand::thread_rng();
    let mut users_guess_state = vec![];
    let user_commit_amount = sp1_zkvm::io::read::<u8>;
    
    if n < 0  {
        panic!("This program doesn't support negative numbers.");
    }
    let state = n % std::u32::MAX ;

    // do the tries from the user till the time the difference is reached
    let mut a = 0;
    while a < max_tries {
        let n = sp1_zkvm::io::read::<(u32, u32)>().unwrap();
        let mut stored_gift_state: any = [rng.gen_range(0, 20), rng.gen_range(0, 20)];
        // Encocde the public values of the program.
    let bytes = ProgramState::abi_encode(&(stored_gift_state,n));

    // Commit to the public values of the program.
    sp1_zkvm::io::commit_slice(&bytes);

    }


}
