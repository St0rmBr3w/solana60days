# Overview of Arithmetic and Types in Solana and Rust

- A new project is initiated using Anchor (`anchor init`), built (`anchor build`), and synced with keys (`anchor keys sync`).

- The Solana test validator and logs are set up to monitor program activity.

## Key Learning Points

- When supplying Function Arguments, transition from Ethereum’s `uint256` to Solana’s `u64`.

- Arithmetic in Solana:

Floating Point Operations: Limited support in Solana; generally avoided due to high computational cost.

Arithmetic Overflow:

Method 1: Using overflow-checks = true in Cargo.toml for automatic overflow checks at the compiler level.

Method 2: Employing Rust's checked_* operators for manual overflow checks.

Exercises involve testing arithmetic under/overflow.

## Compute Units in Solana

Solana transactions are limited to a default of 200,000 compute units.

The cost of arithmetic operations (with and without overflow protection) is compared.

Solana’s limitations for computationally intensive tasks are noted.

## Differences from Solidity

Power Operations: Rust uses .pow() and .checked_pow() instead of Solidity's ** operator.

Handling Floating Points: Example provided with cube root calculation; floating-point operations consume more compute units.