use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("CvZJMZSFu66dxPh8B8ZiURpSSgX1AXs49KPkdgckzzGn");

#[program]
pub mod lookup_tables{
    use super::*;

    pub fn create_store (ctx: Context<CreateStore>, seed: String) -> Result<()> {
        create_store::handler(ctx, seed)
    }

    pub fn add_to_store (ctx: Context<AddToStore>, data: u8) -> Result<()> {
        add_to_store::handler(ctx, data)
    }
}
