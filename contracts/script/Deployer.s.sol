// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.12;

import {Script, console} from "forge-std/Script.sol";
import {VerifierContract} from "../src/VerifierContract.sol";

contract CounterScript is Script {
    function setUp() public {}

    function run(address _targetContract) external returns (address) {
        vm.startBroadcast();

        VerifierContract verifyBatchInclusionCaller = new VerifierContract(_targetContract);

        vm.stopBroadcast();

        return address(verifyBatchInclusionCaller);
    }
}
