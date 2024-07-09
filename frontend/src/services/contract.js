export const ContractAddress = "0x23057256D67C09F3f6E289AD045cD410D03c4680";


export const ContractABI = [
    {
        "inputs": [
            {
                "internalType": "struct VerifierContract.SettlementInput",
                "name": "input",
                "type": "tuple",
                "components": [
                    {
                        "internalType": "uint256",
                        "name": "gameId",
                        "type": "uint256"
                    },
                    {
                        "internalType": "bool",
                        "name": "result",
                        "type": "bool"
                    },
                    {
                        "internalType": "bytes32[]",
                        "name": "proofCommitment",
                        "type": "bytes32[]"
                    },
                    {
                        "internalType": "bytes32",
                        "name": "provingSystemAuxDataCommitment",
                        "type": "bytes32"
                    },
                    {
                        "internalType": "bytes20",
                        "name": "proofGeneratorAddr",
                        "type": "bytes20"
                    },
                    {
                        "internalType": "bytes32[]",
                        "name": "batchMerkleRoot",
                        "type": "bytes32[]"
                    },
                    {
                        "internalType": "bytes[]",
                        "name": "merkleProof",
                        "type": "bytes[]"
                    },
                    {
                        "internalType": "uint256[]",
                        "name": "verificationDataBatchIndex",
                        "type": "uint256[]"
                    }
                ]
            }
        ],
        "stateMutability": "nonpayable",
        "type": "function",
        "name": "settleBet"
    },
    {
        "type": "function",
        "name": "startGame",
        "inputs": [{
            "name": "guesses",
            "type": "uint8[2][]",
            "internalType": "uint8[2][]"
        }],
        "outputs": [{
            "name": "",
            "type": "uint256",
            "internalType": "uint256"
        }],
        "stateMutability": "payable"
    }
]


