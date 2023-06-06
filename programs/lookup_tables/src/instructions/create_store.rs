use anchor_lang::prelude::*;

use crate::state::Store;

#[derive(Accounts)]
#[instruction(_seed: String)]
pub struct CreateStore <'info> {
    pub system_program: Program<'info, System>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(
        init, 
        seeds=[_seed.as_bytes().as_ref()],
        bump,
        payer=payer,
        space=8+1
    )]
    pub store: Account<'info, Store>,
}

pub fn handler (_ctx: Context<CreateStore>, _seed: String) -> Result<()> {
        Ok(())
    }