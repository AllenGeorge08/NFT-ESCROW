use anchor_lang::prelude::*;

#[event]
pub struct List {
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub listed: bool,
    pub price: u64,
}

#[event]
pub struct Unlist {
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub listed: bool,
    pub price: u64,
}

#[event]
pub struct Buy {
    pub taker: Pubkey,
    pub taker_mint: Pubkey,
}
