// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract VerifierContract {
    uint256 private _nextGameId;

    address public alignedServiceManager;

    // TODO update this with the correct ELF commitment
    bytes32 public elfCommitment = 0x35dd40ab04e180712996495caec915b8a7c488433acbb50c4d8d912cb55bf1f1;

    // Map the gameId to the Bet
    struct Bet {
        address user;
        uint256 amount;
        bytes32 guessesWinCommitment;
        bytes32 guessesLostCommitment;
        uint8 guessesCount;
        bool settled;
    }
    mapping(uint256 => Bet) public bets;

    constructor(address _alignedServiceManager) {
        alignedServiceManager = _alignedServiceManager;
    }

    // Start a new game with the provided guesses
    function startGame(uint8[2][] memory guesses) payable external returns(uint256) {
        // Hash the guesses in order to keep a commitment to them
        // TODO here we actually need to have a commitment to the public
        // inputs of the proof, not the just the guesses
        // Should be something like keccak256(abi.encodePacked(true, guesses))
        // Should be something like keccak256(abi.encodePacked(false, guesses))
        bytes32 guessesWinHash = keccak256(abi.encodePacked(guesses));
        bytes32 guessesLostHash = keccak256(abi.encodePacked(guesses));

        // Store the bet in the contract
        bets[_nextGameId] = Bet(msg.sender, msg.value, guessesWinHash, guessesLostHash, uint8(guesses.length), false);

        // Return and increment the gameId
        _nextGameId++;
        return (_nextGameId - 1);
    }

    // Settle the bet for the provided gameId
    function settleBet(
        uint256 gameId,
        bool result,
        bytes32[] memory proofCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32[] memory batchMerkleRoot,
        bytes[] memory merkleProof,
        uint256[] memory verificationDataBatchIndex
    ) external {
        require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");

        Bet memory bet = bets[gameId];
        require(!bet.settled, "Bet already settled");

        bytes32 commitment;
        if (result) {
            commitment = bet.guessesWinCommitment;
        } else {
            commitment = bet.guessesLostCommitment;
        }

        // Check all proofs
        for (uint256 i = 0; i < proofCommitment.length; i++) {
            bool gameWon =
                verifyBatchInclusion(
                    proofCommitment[i],
                    commitment,
                    provingSystemAuxDataCommitment,
                    proofGeneratorAddr,
                    batchMerkleRoot[i],
                    merkleProof[i],
                    verificationDataBatchIndex[i]
                );
            // if verification fails, return early
            if (!gameWon) {
                return;
            }
        }

        // Based on the result, pay out or take the funds
        if (result) {
            // Return winnings
            uint256 winnings = bet.amount + bet.amount * 30/100 * bet.guessesCount;
            // Transfer the funds to the user
            payable(bet.user).transfer(winnings);
        } else {
            payable(address(proofGeneratorAddr)).transfer(bet.amount);
        }

        // Mark the bet as settled
        delete bets[gameId];
        bets[gameId].settled = true;
    }

    // Verify the inclusion of a proof in a batch
    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) internal view returns (bool) {
        (bool callWasSuccessfull, bytes memory proofIsIncluded) = alignedServiceManager.staticcall(
            abi.encodeWithSignature(
                "verifyBatchInclusion(bytes32,bytes32,bytes32,bytes20,bytes32,bytes,uint256)",
                proofCommitment,
                pubInputCommitment,
                provingSystemAuxDataCommitment,
                proofGeneratorAddr,
                batchMerkleRoot,
                merkleProof,
                verificationDataBatchIndex
            )
        );

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));

        return (callWasSuccessfull && proofIsIncludedBool);
    }
}
