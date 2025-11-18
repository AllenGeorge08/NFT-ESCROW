#![allow(deprecated)]
use anchor_lang::prelude::*;

declare_id!("2dupZTbJ6jcBDRT3wa1A5JADh2q4hWLgTSf8dCCQSvoA");

pub mod event;
pub mod instructions;
pub mod state;
pub use instructions::*;

pub mod errors;
pub use errors::*;

#[program]
pub mod nft_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.initialize_escrow(seed, &ctx.bumps, amount)?;
        msg!("Program Initialized: {:?}", ctx.program_id);
        Ok(())
    }

    pub fn list_nft(ctx: Context<List>, amount: u64, seed: u64) -> Result<()> {
        ctx.accounts.list_nft(amount, seed)?;
        msg!("Nft Listed");
        Ok(())
    }

    pub fn buy_nft(ctx: Context<Buy>, seed: u64) -> Result<()> {
        ctx.accounts
            .buy_nft(seed, &ctx.bumps, ctx.accounts.escrow.state.price)?;
        Ok(())
    }

    pub fn unlist(ctx: Context<Unlist>, seed: u64) -> Result<()> {
        ctx.accounts.unlist(seed, &ctx.bumps)?;
        Ok(())
    }
}
