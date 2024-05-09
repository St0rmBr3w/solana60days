use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("E4QFEcHGagrBCwJzQsTXn8i5wMuVqMGGuw8PS23o2ZBq");

#[program]
pub mod day_17 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    // ****************************
    // *** THIS FUNCTION IS NEW ***
    // ****************************
    pub fn set(ctx: Context<Set>, new_x: u64) -> Result<()> {
        let my_storage = &mut ctx.accounts.my_storage;
        my_storage.x = new_x;

        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let my_storage = &mut ctx.accounts.my_storage;
        my_storage.x += 1;

        Ok(())
    }

    pub fn print_x(ctx: Context<PrintX>) -> Result<()> {
        let x = ctx.accounts.my_storage.x;
        msg!("The value of x is {}", x);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct PrintX<'info> {
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

// **************************
// *** THIS STRUCT IS NEW ***
// **************************
#[derive(Accounts)]
pub struct Set<'info> {
    #[account(mut, seeds = [], bump)]
    pub my_storage: Account<'info, MyStorage>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init,
              payer = signer,
              space=size_of::<MyStorage>() + 8,
              seeds = [],
              bump)]
    pub my_storage: Account<'info, MyStorage>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct MyStorage {
    x: u64,
}