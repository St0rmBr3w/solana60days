use anchor_lang::prelude::*;

declare_id!("6whbPcj1481C92fVMEEhCLoQUJmqPvmEY95paVJFBMnd");

#[program]
pub mod day_1 {
    use super::*;

    pub fn initialize2(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello world!"); //NEW LINE HERE
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
