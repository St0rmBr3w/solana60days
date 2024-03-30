# solana60days

Learn Solana in 60 days using the [RareSkills Solana Tutorial](https://www.rareskills.io/solana-tutorial).

## Prerequisites

Before you begin, ensure you have the following prerequisites installed. If you already have them, you can skip the respective steps.

### Install Rust

`curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`

### Install yarn

`corepack enable`

## Installation Steps

Follow these steps to set up your environment:

### Install the Solana CLI

Use the stable version for consistency:

`sh -c "$(curl -sSfL https://release.solana.com/stable/install)"`

### Install Anchor

Anchor is a framework for Solana development, similar to Hardhat.

```bash
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest
```

### Initialize and Build an Anchor Program

For Mac users, name your program day_(x) instead of day(x).

```bash
anchor init day1 # or day_1 for Mac
cd day1
anchor build
```
Note: Building may take some time depending on your machine and internet connection.

### Configure Solana for Localhost

In a new shell (not in the Anchor project directory):

`solana config set --url localhost`

### Run the Test Validator Node

Open a new shell:

`solana-test-validator`

Keep the shell running the validator node open.

### Sync the Program ID with the Anchor Key

Return to the shell with the Anchor project and run:

`anchor keys sync`

### Run the Tests

Within the Anchor project:

`anchor test --skip-local-validator`

Note: Follow the instructions from Anchor for setting up a test wallet if needed.

## Troubleshooting

If you encounter issues during installation, please refer to this section.

### Documented Versions

This tutorial is based on:

Anchor: 0.29.0
Solana: 1.16.25
Rustc: 1.77.0-nightly

### Changing Versions

For Anchor:

```
avm install 0.29.0
avm use 0.29.0
```

For Solana:

`sh -c "$(curl -sSfL https://release.solana.com/1.16.25/install)"`

### Common Errors

If you encounter errors like:

`error: package `solana-program v1.18.0` cannot be built`

This might be due to a Rust version mismatch. Ensure you are using Rustc version 1.72.0 or newer, or adjust your Solana program version accordingly.

Check your Solana version:

`solana --version`

Then update the Solana program version:

`cargo update -p solana-program@1.18.0 --precise [version]`

Replace [version] with the appropriate version number.

Note: Your `solana-program@x.x.x` version may be higher, make sure youse use the program version you're using, plus the returned Solana version from the above step.







