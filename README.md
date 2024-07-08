# ZK Miner

## Introduction

The ZK Miner application is based off of the popular [Mines Gambling Game](https://stake.com/casino/games/mines). In this game, you are presented with a grid of tiles, each of which may or may not contain a mine. The goal is to uncover as much tiles as possible, without uncovering any tiles that do contain a mine. Each time you uncover a tile, you are presented with a reward.

The ZK Miner application is a decentralized version of the game, where the game logic is executed and proven in a trustless manner using a SP1. The outcome of the game is committed too and verified on [Aligned](https://alignedlayer.com/), bringing the verification costs down to a fraction of the cost for verification on the L1. This proof can then be pulled from a L1 Escrow contract, and used to settle the game.

## Requirements

- Rust
- [SP1 toolchain](https://docs.succinct.xyz/getting-started/install.html): used to compile the SP1 code to an ELF file.

## Team

## Challenges

- **Stack overflow**: When trying to proof using the SP1 SDK, we encountered a stack overflow error. This was due to the fact that we were compiling without the `--release` flag. This flag is necessary to optimize the code and prevent the stack overflow error.

- **Ether-rs Encoding error**: When trying to call the contract and pass the proof, we encountered an encoding error using Ethers-rs. This was fixed when switching to the `alloy` crate.
