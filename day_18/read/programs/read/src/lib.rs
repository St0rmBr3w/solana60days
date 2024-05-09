use anchor_lang::prelude::*;

declare_id!("8jexXiSQirm2tTJbUA48RLbNFn3YitFAzKEMMScvbpxq");

#[program]
pub mod read {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
