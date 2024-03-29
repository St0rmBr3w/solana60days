use anchor_lang::prelude::*;

declare_id!("48HZL5Sks7pS66J4SxXVox5hC6WunrnWyXwryE2dmomY");

#[program]
pub mod calculator {
    use super::*;

    pub fn add(ctx: Context<Calculate>, a: f64, b: f64) -> Result<()> {
        msg!("The sum of {} and {} is {}", a, b, a + b);
        Ok(())
    }

    pub fn subtract(ctx: Context<Calculate>, a: f64, b: f64) -> Result<()> {
        msg!("The product of {} and {} is {}", a, b, a - b);
        Ok(())
    }

    pub fn multiply(ctx: Context<Calculate>, a: f64, b: f64) -> Result<()> {
        msg!("The product of {} and {} is {}", a, b, a * b);
        Ok(())
    }

    pub fn divide(ctx: Context<Calculate>, a: f64, b: f64) -> Result<()> {
        if b == 0.0 {
            return Err(ProgramError::InvalidArgument.into());
        }
        msg!("The quotient of {} and {} is {}", a, b, a / b);
        Ok(())
    }

    pub fn sqrt(ctx: Context<Calculate>, a: f64) -> Result<()> {
        if a < 0.0 {
            return Err(ProgramError::InvalidArgument.into());
        }
        msg!("The square root of {} is {}", a, a.sqrt());
        Ok(())
    }

    pub fn log10(ctx: Context<Calculate>, a: f64) -> Result<()> {
        if a <= 0.0 {
            return Err(ProgramError::InvalidArgument.into()); // Logarithm for non-positive numbers is undefined
        }
    
        let result = a.log10(); // Calculates the base-10 logarithm
        msg!("The log(10) of {} is {}", a, result);
        Ok(())
    }
    
}

#[derive(Accounts)]
pub struct Calculate {}