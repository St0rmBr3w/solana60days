use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("DntexDPByFxpVeBSjd6nLqQQSqZmSaDkP8TUbcJ9jAgt");

#[program]
pub mod example_map {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, key: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(key: u64)]
pub struct Initialize<'info> {

    #[account(init,
              payer = signer,
              space = size_of::<Val>() + 8,
              seeds=[&key.to_le_bytes().as_ref()],
              bump)]
    val: Account<'info, Val>,
    
    #[account(mut)]
    signer: Signer<'info>,
    
    system_program: Program<'info, System>,
}

#[account]
pub struct Val {
    value: u64,
}