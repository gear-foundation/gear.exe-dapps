#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

# Check for variables
if [ -z "$CONTRACT_ADDRESS" ] 
then
    echo "CONTRACT_ADDRESS is not set"
    echo "You can run the script by setting the variables at the beginning: CONTRACT_ADDRESS=0x getCount.sh"
    exit 0
fi

# Call
echo "Init game..."
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "initGame()"

echo "Simulating game..."
cast send --rpc-url $RPC_URL --private-key $PRIVATE_KEY $CONTRACT_ADDRESS "simulateGame(uint32)" $STEPS

paddle_hits=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "paddleHits() (uint16)")
echo "Paddle hits: $paddle_hits"

destoryed_blocks=$(cast call --rpc-url $RPC_URL $CONTRACT_ADDRESS "destoryedBlocks() (uint16)")
echo "Destoryed blocks: $destoryed_blocks"
# CONTRACT_ADDRESS= ./scripts/getCount.sh