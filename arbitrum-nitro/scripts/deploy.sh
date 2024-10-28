#!/bin/bash

# Load variables from .env file
set -o allexport
source scripts/.env
set +o allexport

cargo stylus deploy -e $RPC_URL --private-key $PRIVATE_KEY
