# ZK game proof aggregator(version for Aligned hackathon)

## Introduction
This is the extension of the project we build during the [zklambdaweek](https://dorahacks.io/buidl/14120) with the aim to provide the developers full stack tools to: 

1.Build their fully onchain games using the [Mud framework]() with indexer and then create composite proofs of certain events (proof of location, ending state etc.), executed across the various ZkVMs.

2. The aim is to provide the developers their custom 


## Architecture

- **Aligned**: Contains the onchain 
- **Mine**: Contains the SP1 code for the Mines game along with the compiled ELF file for it.
- **src**: Contains the Rust code for the ZK Miner application backend and testing.

## Requirements





### 

- [SP1 toolchain](https://docs.succinct.xyz/getting-started/install.html): used to compile the SP1 code to an ELF file.

## Execution

1. Clone the repository and navigate to the root directory.
2. Run `cargo run --release` to start the application.

## Testing

1. Set the `RPC_URL` and `PRIVATE_KEY` environment variables in the `.env` file. The `RPC_URL` should point to a Holesky endpoint, and the `PRIVATE_KEY` should be the private key of the account that will be used to sign transactions during testing.
2. Run `cargo test -r test_sp1_end_to_end` to run the end to end test.

## Team

- `Dhruv Malik`: CTO of extralabs.xyz / backend dev and web3 integration.
- `Pol Montero`: Full Stack Developer from Palma, Spain, transitioning into blockchain development. Co-founder and lead developer of Criptodigital.cat.
- `Anukkrit`: Founding team Karnot.
- `Greg`: Core developer at Kakarot ZK-EVM.

## Challenges

- **Stack overflow**: When trying to proof using the SP1 SDK, we encountered a stack overflow error. This was due to the fact that we were compiling without the `--release` flag. This flag is necessary to optimize the code and prevent the stack overflow error.

- **Ether-rs Encoding error**: When trying to call the contract and pass the proof, we encountered an encoding error using Ethers-rs. This was fixed when switching to the `alloy` crate.
