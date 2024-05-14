1. Why does Solana not use the concept of "`payable`" functions or "`msg.value`" as in Ethereum?

Ethereum allows wallets to specify `msg.value` as part of the transaction to **push** the ETH to the destination contract. Solana contracts instead **pull** the Solana from the wallet.

Because Solana always pulls the required SOL, there's no need for `payable` functions or a concept of `msg.value`.

2. What is the purpose of CPI in Solana, and how does it compare to Ethereum's way of transferring ETH?

Cross program invocation is designed for applications on the Solana blockchain to be composable with one another. In Ethereum, you can transfer ETH simply by just passing `msg.value`. Solana, instead, has a built in function (smiliar to a precompile in Ethereum) called the `system program`. The system program can be used to transfer SOL from one account to another. Think of it similarly to a a precompile with a public function called `transfer`.

3. Why is a `Context` needed in Solana when calling a function that involves a transfer of SOL?

The `Context` provides the relationship between the program's execution logic and what accounts will be affected by the execution itself. The Context holds all the accounts that the program will interact with. Calling the system program is no different, so we have to pass the accounts that we will interact with.

4.  Explain how the `CpiContext` is structured in the Rust code provided in the tutorial. What does it signify and why is it necessary?

The CpiContext holds what program we are going to be calling (`system_program`), the from account signing the transaction (`signer`), and the destination address to send the SOL to (`recipient`).

We pass the CpiContext to the `system_program::transfer()` method, along with the amount to transfer.

5. What specific error handling method is employed in the Rust function `send_sol` if the transfer fails?

In the `send_sol` function, we check that the return value from the CPI call is `Ok(())`. Rust makes this straightforward by providing an `is_ok()` method.

6. How does the `remaining_accounts` field enhance the functionality of the payment splitter program described in the tutorial?

Instead of manually defining each `UncheckedAccount` from 1, 2, 3 ..., N, we can instead create a `remaining_accounts` in the Context struct of the function to make passing in multiple addresses more dynamic.

7. The tutorial mentions Rust lifetimes (`'a, 'b, 'c`) in the function declaration. What is the purpose of these lifetimes in the context of the function `split_sol`?

Rust lifetimes are a feature of the Rust programming language used to ensure memory safety without needing a garbage collector. Lifetimes are annotations in the code that specify for how long a reference should be considered valid.

But a high level explanation is that the Rust code needs assurances the resources passed into the loop for `recipient` in `ctx.remaining_accounts` will exist for the entirety of the loop.
