1. What is the primary reason Solana does not have fallback or receive functions?

Solana programs must specify in advance ALL the accounts that the transaction will interact with. The idea of a `fallback` is if a specific method is NOT available then this method is called, but since Solana requires that all accounts be known prior to the tx, there is no state for a fallback function since the user would need to know ahead of time the accounts the fallback would be interacting with.

2. How does Solana's handling of state changes in functions differ from Solidity's handling of view and pure functions?

View and pure functions in Solidity enforce at the compiler level that no state change opcodes will be invoked by the function marked with the modifiers. Anchor specifically does not implement any of these checks. While an alterantive compiler might one day implement these checks, most developers can rely on the guarantee that **if an Account is not included in the Context struct definition, that function won't access the account.**

3. According to the text, what role do staticcalls play in Solidity, and why is this concept absent in Solana?

4. Explain how a Solana program can access the data of an account owned by another program without needing specific view functions.

Because Solana specifies all the accounts a specific transaction will work with, the solana program can read any account passed to it even if the account is owned by another program.

5. What is the significance of not being able to use reentrancy locks in Solana for defending against read-only reentrancy, and how can this issue be addressed?

6. Why are custom modifiers like `onlyOwner` or `nonReentrant` not available in Rust, as used in Solana programming?

Custom modifiers are a Solidity concept only, and don't have an application within Rust.

7. Discuss the absence of unit aliases like 'ethers' or 'wei' in Solana's programming environment, and what does this imply about the differences between Solidity and Rust/Anchor?

For now, Anchor has no concept of custom units that are handy in Solidity like "1 ether" or "1 day". Instead, everything must be defined in the literal sense without a convenient alias.