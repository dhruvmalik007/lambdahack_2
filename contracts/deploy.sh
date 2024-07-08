#!/bin/bash

source .env

# cd to the directory of this script so that this can be run from anywhere
parent_path=$( cd "$(dirname "${BASH_SOURCE[0]}")" || exit 1 ; pwd -P )

# At this point we are in examples/zkquiz
cd "$parent_path" || exit 1

if [ -z "$PRIVATE_KEY" ]; then
    echo "PRIVATE_KEY is not set. Please set it in .env"
    exit 1
fi

forge install

forge script script/Deployer.s.sol \
    0x58F280BeBE9B34c9939C3C39e0890C81f163B623 \
    --rpc-url https://ethereum-holesky-rpc.publicnode.com \
    --private-key "$PRIVATE_KEY" \
    --broadcast \
    --sig "run(address _alignedServiceManager)"
