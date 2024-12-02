# Stylus workshop: Counter contract

Simple Counter contract written in Rust.

## Getting started

Follow the instructions in the [Stylus quickstart](https://docs.arbitrum.io/stylus/stylus-quickstart) to configure your development environment.

You'll also need [Foundry](https://github.com/foundry-rs/foundry) to interact with the contract.

## Check and deploy

You can use [cargo stylus](https://github.com/OffchainLabs/cargo-stylus) to check that your contract is compatible with Stylus by running

```shell
cargo stylus check
```

With the following command you can deploy it to an Arbitrum chain

```shell
cargo stylus deploy --endpoint $RPC_URL --private-key $PRIVATE_KEY
```

Alternatively, you can use the bash scripts available to build and deploy the contract:

- [build.sh](/scripts/build.sh)
- [deploy.sh](/scripts/deploy.sh): this script requires the environment variables `$RPC_URL` and `$PRIVATE_KEY`.

## Tests

For unit testing, this example integrates the [motsu](https://github.com/OpenZeppelin/rust-contracts-stylus/tree/main/lib/motsu) library from OpenZeppelin. To run unit tests, you can simply use

```shell
cargo test --locked --lib
```

Alternatively, you can use the bash script available [test-unit.sh](/scripts/tests/test-unit.sh).

For integration tests, this example integrates a fork of the [e2e](https://github.com/OpenZeppelin/rust-contracts-stylus/tree/main/lib/e2e) library from OpenZeppelin available [here](https://github.com/TucksonDev/e2e-lib). To run the tests you need to run a [nitro-testnode](https://github.com/OffchainLabs/nitro-testnode). A script is available to clone and run the nitro-testnode, [nitro-testnode.sh](/scripts/tests/nitro-testnode.sh). Once the nitro-testnode is running, you can run the e2e tests using the script available [test-e2e.sh](/scripts/tests/test-e2e.sh).

## Additional scripts

The `scripts` folder contains several other scripts that make individual calls to perform the most important actions:

1. [./scripts/getCount.sh](./scripts/getCount.sh) to get the current counter
2. [./scripts/increment.sh](./scripts/increment.sh) to increment the current counter
3. [./scripts/setCount.sh](./scripts/setCount.sh) to set the current counter

Remember to set the environment variables in an `.env` file.

## How to run a local dev node

Instructions to setup a local dev node can be found [here](https://docs.arbitrum.io/run-arbitrum-node/run-local-dev-node).

## Useful resources

- [Stylus quickstart](https://docs.arbitrum.io/stylus/stylus-quickstart)
- [Stylus by example](https://stylus-by-example.org/)
- [Counter contract](https://github.com/OffchainLabs/stylus-workshop-counter)
- [Interactions between Rust and Solidity](https://github.com/OffchainLabs/stylus-workshop-rust-solidity/)
- [Telegram group](https://t.me/arbitrum_stylus)
- [Discord channel](https://discord.com/channels/585084330037084172/1146789176939909251)

## Stylus reference links

- [Stylus documentation](https://docs.arbitrum.io/stylus/stylus-gentle-introduction)
- [Stylus SDK](https://github.com/OffchainLabs/stylus-sdk-rs)
- [Cargo Stylus](https://github.com/OffchainLabs/cargo-stylus)