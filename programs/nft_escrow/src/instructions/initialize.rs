use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::state::Escrow;
use crate::Errors;
use mpl_core::accounts::BaseAssetV1;
use mpl_core::instructions::TransferV1CpiBuilder;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, //q Shouldn't this be inside a box too?
    #[account(mut)]
    pub mint_sol: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    /// CHECKED : No validation needed here
    pub asset: UncheckedAccount<'info>,
    // pub asset: Box<dyn AccountDeserialize<'info,BaseAssetV1>>,
    #[account(
        init_if_needed,
        space = 8 + Escrow::INIT_SPACE,
        payer = maker,
        seeds = [b"escrow",seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
        init_if_needed,
        payer = maker,
        associated_token::mint = mint_sol,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      init_if_needed,
      payer = maker,
      associated_token::mint = mint_sol,
      associated_token::authority = maker,
      associated_token::token_program = token_program,
    )]
    pub maker_ata_sol: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    ///CHECK: SAFE
    pub mpl_core_program: AccountInfo<'info>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_escrow(
        &mut self,
        seed: u64,
        bumps: &InitializeBumps,
        amount: u64,
    ) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed: seed,
            maker: self.maker.key(),
            maker_mint: self.maker_ata_sol.key(),
            bump: bumps.escrow,
            fee: 5,
            state: crate::state::List {
                maker: self.maker.key(),
                maker_mint: self.mint_sol.key(),
                bump: bumps.escrow,
                vault: self.vault.key(),
                price: amount,
                listed: true,
                owner: self.maker.key(),
            },
        });
        // self.escrow.bump = bumps.escrow;
        Ok(())
    }
}
