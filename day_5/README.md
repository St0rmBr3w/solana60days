# Overview

Unlike Ethereum, Solana does not have constructors. They are simply deployed using tools like Anchor.

Deployment is done using `anchor deploy`, and this action is observed in the Solana logs.

The deployment process in Solana is different than the EVM. The typical expectation of an async function call for deployment doesn't hold true for Solana, as anchor handles deployment in the background.

It lacks some of the Ethereum-specific features like constructors and immutable variables, instead offering simpler deployment and upgrade processes.

The mutability of Solana programs provides flexibility but also necessitates careful management to ensure program integrity and trust.

## Mutable Nature of Solana Programs

Solana programs are mutable by default, meaning they can be upgraded or overwritten. This is a contrast to the general immutability assumption in Ethereum.

To make a Solana program immutable, it has to be redeployed as such. This is in general cleaner than the Ethereum proxy contracts where ownership can be renounced.

## Absence of a Constructor

Since there are no constructors, Solana doesn't have to initialize variables the way Ethereum does.

Solana doesn't have immutable variables as seen in Solidity.

There's no need for `delegatecall` in Solana. In Ethereum, `delegatecall` is used by upgrading contracts functionalities, but in Solana, the program bytecode can be directly upgraded.

## Testing Without Redeployment

The provided exercise demonstrates how to observe changes in the program without changing its ID. By altering the `msg!` values in the `lib.rs` file and deploying the program, developers can see the changes in the logs while the program ID remains constant.