use anchor_lang::prelude::*;

declare_id!("GBRnGhaM3cU1KHbK15nHNAFoFxJYSX1mubeeD9ds7B8s");

#[program]
pub mod day_21 {
    use super::*;

    pub fn read_balance(ctx: Context<ReadBalance>) -> Result<()> {
        let balance = ctx.accounts.acct.to_account_info().lamports();
        
        msg!("Balance in lamports is {}", balance);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct ReadBalance<'info> {
    /// CHECK: although we read this account's balance, we don't do anything with the information
    pub acct: UncheckedAccount<'info>, // <-- where does this UncheckedAccount come from?
}
