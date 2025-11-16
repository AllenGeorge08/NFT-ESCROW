use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct List {
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub vault: Pubkey,
    pub price: u64,
    pub bump: u8,
    pub listed: bool,
    pub owner: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Buy {
    pub buyer: Pubkey,
    pub buyer_mint: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct Escrow {
    pub seed: u64,
    pub maker: Pubkey,
    pub maker_mint: Pubkey,
    pub bump: u8,
    pub fee: u8,
    pub state: List 
}
