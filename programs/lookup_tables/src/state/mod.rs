use anchor_lang::prelude::*;

#[account]
pub struct Store {
    pub data: u8
}
