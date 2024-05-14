use anchor_lang::prelude::*;

declare_id!("Aktnrif6isEN3DiQ18de7KZbTPM1WwkjwcMtced7uSWX");

#[program]
pub mod day_21b {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
