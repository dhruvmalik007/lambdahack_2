// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {ERC721} from "@openzeppelin/contracts/token/ERC721/ERC721.sol";
import {ERC721URIStorage} from "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";

contract VerifierContract is ERC721URIStorage {
    uint256 private _nextTokenId;

    address public alignedServiceManager;

    bytes32 public elfCommitment = 0x35dd40ab04e180712996495caec915b8a7c488433acbb50c4d8d912cb55bf1f1;

    // map to check if proof has already been submitted
    mapping(bytes32 => bool) public mintedProofs;

    constructor(address _alignedServiceManager) ERC721("Aligned Layer ZK Quiz", "AZKQ") {
        alignedServiceManager = _alignedServiceManager;
    }

    function verifyBatchInclusion(
        bytes32 proofCommitment,
        bytes32 pubInputCommitment,
        bytes32 provingSystemAuxDataCommitment,
        bytes20 proofGeneratorAddr,
        bytes32 batchMerkleRoot,
        bytes memory merkleProof,
        uint256 verificationDataBatchIndex
    ) external returns (uint256) {
        require(elfCommitment == provingSystemAuxDataCommitment, "ELF does not match");
        require(address(proofGeneratorAddr) == msg.sender, "proofGeneratorAddr does not match");

        bytes32 fullHash = keccak256(abi.encodePacked(proofCommitment,
            pubInputCommitment, provingSystemAuxDataCommitment, proofGeneratorAddr));
        require(!mintedProofs[fullHash], "proof already minted");

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

        require(callWasSuccessfull, "static_call failed");

        bool proofIsIncludedBool = abi.decode(proofIsIncluded, (bool));
        require(proofIsIncludedBool, "proof not included in batch");

        mintedProofs[fullHash] = true;

        uint256 tokenId = _nextTokenId++;
        _mint(msg.sender, tokenId);
        _setTokenURI(tokenId, "ipfs://QmUKviny9x2oQUegyJFFBAUU2q5rvu5CsPzrUaBSDukpHQ");

        return tokenId;
    }

    function tokenURI(uint256 tokenId) public override view virtual returns (string memory) {
        _requireOwned(tokenId);

        return "ipfs://QmUKviny9x2oQUegyJFFBAUU2q5rvu5CsPzrUaBSDukpHQ";
    }

}
