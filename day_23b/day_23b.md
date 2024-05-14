1. What is the role of the Token Program in the Solana ecosystem?

The Solana token program provides a simple interface for application developers to produce their own fungible and non-fungible tokens without needing to necessarily write the program logic itself.

2. What is a Token Mint, and what information does it contain? Explain the concept of a Token Account and its relationship to a Token Mint. What are Associated Token Accounts and how are they determined?

To create a new SPL Token, you need to first create a Token Mint: an account that holds data about a specific token. Each Mint Account contains the specific information about the token derived from the parent Token Program. In this sense, you can think of each Token Mint as the specific token, while the Token Program as the factory all tokens are deployed from.

An account that holds the specific token derived from the token-mint is called the `Associated Token Account`. 

An Associated Token Account stores tokens in an address made from:

- The owner's public key
- The token mint

The token account holds tokens of a specific "mint" and has a specified "owner" of the account. Only the owner is authorized to decrease the Token Account balance (transfer, burn, etc.) while anyone can send tokens to the Token Account to increase it's balance.

3. Outline the steps to create a new Token Mint using the `@solana/spl-token` library.

To create a new token a user has to:

- initiate a Token Mint from the SPL Token Program

- create an Associated Token Account from the Token Mint Account

- mint tokens to the specific associated token account

4. How does minting tokens affect the supply of a specific token? Explain the procedure to transfer tokens using the SPL Token library.

When you mint tokens, you increase the supply of the token mint and deposit the newly minted tokens into a token account. To mint tokens using the `spl-token library`, you can use the `mintTo` function.

5. Discuss the significance of rent and rent exemption in the context of Token Accounts and Token Mints. Why is it important to ensure that token minting instructions are added to the same transaction when building them manually?

If you were to do each step in a separate transaction, it's theoretically possible for somebody else to take the account you create and initialize it for their own mint.

6. How would you create an Associated Token Account for a new user? What steps would you take to mint new tokens to a specified token account?

7. Describe the potential risks and considerations when allowing users to create new token mints via a website.

If you were to build a website to allow users to create a new token mint, you would need to do so with the user's secret key without making them expose it to the browser. In that case, you would want to build and submit a transaction with the right instructions.