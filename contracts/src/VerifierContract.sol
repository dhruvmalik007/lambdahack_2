// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract VerifierContract {
    uint256 private _nextGameId;

    address public alignedServiceManager;

    /// The keccak 256 hash of the ELF bytecode.
    bytes32 public elfCommitment = 0xe318613034c9e128c045beaeef47ce789528628d281eec1d1fa54988fa4f06f6;

    struct GamePublicInputs {
        uint8[2][] user_guesses;
        bool won;
    }

    /// Map the gameId to the Bet
    struct Bet {
        address user;
        uint256 amount;
        uint8 guessesCount;
        bool settled;
    }
    mapping(uint256 => Bet) public bets;

    /// Map the gameId to the GameCommitment
    struct GameCommits {
        bytes32 Won;
        bytes32 Lost;
    }
    mapping(uint256 => GameCommits) public gameCommits;

    constructor(address _alignedServiceManager) {
        alignedServiceManager = _alignedServiceManager;
    }

    /// Start a new game with the provided guesses
    function startGame(uint8[2][] memory guesses) payable external returns(uint256) {
        // Store the bet in the contract
        bets[_nextGameId] = Bet(msg.sender, msg.value, uint8(guesses.length), false);

        // Store the commitment in the contract by
        // hashing the guesses in order to keep a commitment to them
        GamePublicInputs memory commitWon = GamePublicInputs(guesses, true);
        GamePublicInputs memory commitLost = GamePublicInputs(guesses, false);
        bytes32 guessesWinHash = keccak256(abi.encode(commitWon));
        bytes32 guessesLostHash = keccak256(abi.encode(commitLost));

        gameCommits[_nextGameId] = GameCommits(guessesWinHash, guessesLostHash);


        // Return and increment the gameId
        _nextGameId++;
        return (_nextGameId - 1);
    }

    /// Settle the bet for the provided gameId
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

        // Get the bet
        Bet memory bet = bets[gameId];
        require(!bet.settled, "Bet already settled");

        // Get the commitments
        GameCommits memory commits = gameCommits[gameId];
        require(commits.Won != 0 && commits.Lost != 0, "Game not found");

        // Get the commitment based on the result
        bytes32 commitment;
        if (result) {
            commitment = commits.Won;
        } else {
            commitment = commits.Lost;
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
            // Transfer the funds to the proof generator
            payable(address(proofGeneratorAddr)).transfer(bet.amount);
        }

        // Clean storage and mark the bet as settled
        delete bets[gameId];
        bets[gameId].settled = true;
    }

    /// Verify the inclusion of a proof in a batch
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

    /// Default payable function
    receive() external payable{}
}
