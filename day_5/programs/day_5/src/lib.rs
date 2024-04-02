use anchor_lang::prelude::*;

declare_id!("9hJ65GoGWi8ZTHRDzAA6Fu7wYsiTPnMyX3TNtgxtnCVd");

#[program]
pub mod day_5 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Goodnight world!");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
