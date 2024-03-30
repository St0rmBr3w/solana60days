use anchor_lang::prelude::*;

declare_id!("4nzt9yPneQZkd2jUAGowHHk3tvbVwqBheunBvkA9FCii");

#[program]
pub mod day_4 {
    use super::*;

    pub fn limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        if a < 10 {
            return err!(MyError::AisTooSmall);
        }
        if a > 100 {
            return err!(MyError::AisTooBig);
        }
        msg!("Result = {}", a);
        Ok(())
    }

    // NEW FUNCTION
    pub fn func(ctx: Context<LimitRange>) -> Result<()> {
        msg!("Will this print?");
        return err!(MyError::AlwaysErrors);
    }

    // Better limit_range with require! macro
    pub fn better_limit_range(ctx: Context<LimitRange>, a: u64) -> Result<()> {
        require!(a >= 10, MyError::AisTooSmall);
        require!(a <= 100, MyError::AisTooBig);
        msg!("Result = {}", a);
        Ok(())
    }

    // What happens if you put a msg! before the return error statements in a Solana program?
    pub fn func_msg(ctx: Context<ReturnError>) -> Result<()> {
        msg!("Will this print?");
        return err!(Day4Error::AlwaysErrors);
    }
}

#[derive(Accounts)]
pub struct ReturnError {}

#[error_code]
pub enum Day4Error {
    #[msg("AlwaysErrors")]
    AlwaysErrors,
}

#[derive(Accounts)]
pub struct LimitRange {}

#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisTooSmall,
    #[msg("a is too big")]
    AisTooBig,
    #[msg("Always errors")] // NEW ERROR, what do you think the error code will be?
    AlwaysErrors,
}
