use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("GhmS32KJtwbC622Mhv16e94hVVHUWFjsV7khTxmwBx67");

#[program]
pub mod day_16 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // attribute-like-macro for augmenting the struct with additional functionality 
    #[account(init,
        payer = signer,
        // this indiciates how much space the account will take
        // the "+ 8" refers to the space needed for a "discriminator"
        // A program can own multiple accounts, it “discriminates” among the accounts with the “seed”
        space=size_of::<MyStorage>() + 8,
        seeds = [],
        bump)]
    pub my_storage: Account<'info, MyStorage>,

    // when we init an account, we must supply the payer
    // the signer is mutable because their account balance will change
    #[account(mut)]
    pub signer: Signer<'info>,

    //  a program built into the Solana runtime that transfers SOL from one account to another
    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: i64,
    y: i64
}
