use anchor_lang::prelude::*;

declare_id!("G76eZ46UnTga83BvH1AT2arHsu7M94wkPsVdoHxGrEFb");

#[program]
pub mod day_20 {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
