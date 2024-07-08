// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

contract EscrowContract {
    uint256 private _nextGameId;

    address public alignedServiceManager;

    /// The keccak 256 hash of the ELF bytecode.
    bytes32 public elfCommitment = 0x59ccdc82ba96b96d1ebabcf00b867b3695767206e51361b70793ae225851c978;

    struct GameOutcome {
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
    function startGame(uint8[2][] calldata guesses) payable external returns(uint256) {
        // Store the bet in the contract
        bets[_nextGameId] = Bet(msg.sender, msg.value, uint8(guesses.length), false);

        // Store the commitment in the contract by
        // hashing the guesses in order to keep a commitment to them
        GameOutcome memory outcomeWin = GameOutcome(guesses, true);
        GameOutcome memory outcomeLost = GameOutcome(guesses, false);

        bytes32 publicInputWon = keccak256(abi.encode(outcomeWin));
        bytes32 publicInputLost = keccak256(abi.encode(outcomeLost));

        bytes32 publicInputWonCommit = keccak256(abi.encode(publicInputWon));
        bytes32 publicInputLostCommit = keccak256(abi.encode(publicInputLost));

        gameCommits[_nextGameId] = GameCommits(publicInputWonCommit, publicInputLostCommit);


        // Return and increment the gameId
        _nextGameId++;
        return (_nextGameId - 1);
    }

    struct SettlementInput {
        uint256 gameId;
        bool result;
        bytes32[] proofCommitment;
        bytes32 provingSystemAuxDataCommitment;
        bytes20 proofGeneratorAddr;
        bytes32[] batchMerkleRoot;
        bytes[] merkleProof;
        uint256[] verificationDataBatchIndex;
    }

    /// Settle the bet for the provided gameId
    function settleBet(SettlementInput calldata input) external {
        require(elfCommitment == input.provingSystemAuxDataCommitment, "ELF does not match");

        // Get the bet
        Bet memory bet = bets[input.gameId];
        require(!bet.settled, "Bet already settled");

        // Get the commitments
        GameCommits memory commits = gameCommits[input.gameId];
        require(commits.Won != 0 && commits.Lost != 0, "Game not found");

        // Get the commitment based on the result
        bytes32 commitment;
        if (input.result) {
            commitment = commits.Won;
        } else {
            commitment = commits.Lost;
        }

        // Check all proofs
        for (uint256 i = 0; i < input.proofCommitment.length; i++) {
            // if verification fails, return early
            if (!verifyBatchInclusion(
                    input.proofCommitment[i],
                    commitment,
                    input.provingSystemAuxDataCommitment,
                    input.proofGeneratorAddr,
                    input.batchMerkleRoot[i],
                    input.merkleProof[i],
                    input.verificationDataBatchIndex[i]
            )) {
                return;
            }
        }

        // Payout the bet
        _payout(input.result, input.gameId, address(input.proofGeneratorAddr));

        // Clean storage and mark the bet as settled
        delete bets[input.gameId];
        bets[input.gameId].settled = true;
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
    ) public view returns (bool) {
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

    function _payout(bool result, uint256 gameId, address proofGeneratorAddr) internal {
        Bet memory bet = bets[gameId];

        // Based on the result, pay out or take the funds
        if (result) {
            // Return winnings
            uint256 winnings = bet.amount + bet.amount * 30/100 * bet.guessesCount;
            // Transfer the funds to the user
            payable(bet.user).transfer(winnings);
        } else {
            // Transfer the funds to the proof generator
            payable(proofGeneratorAddr).transfer(bet.amount);
        }
    }

    /// Default payable function
    receive() external payable{}
}
