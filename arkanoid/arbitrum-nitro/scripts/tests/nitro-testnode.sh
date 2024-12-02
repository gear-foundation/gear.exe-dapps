#!/bin/bash

MYDIR=$(realpath "$(dirname "$0")")
cd "$MYDIR" || exit
TEST_NODE_DIR="$MYDIR/../../nitro-testnode"
cd ../..

# Clone nitro-testnode
git clone --recurse-submodules https://github.com/OffchainLabs/nitro-testnode.git
cd ./nitro-testnode || exit
# `release` branch.
git checkout 2c2a97c2608ab0c83f2322af2a0dc2bf0b15ea31 || exit

# Initialize test node
./test-node.bash --no-run --init || exit

# Start with detached mode
./test-node.bash --detach
