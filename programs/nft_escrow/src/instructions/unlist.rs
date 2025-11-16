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
pub struct Unlist<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    ///CHECKED: No Validation needed
    pub asset: UncheckedAccount<'info>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow",seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub buyer_ata_sol: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    ///CHECK: Safe
    pub mpl_core_program: AccountInfo<'info>,
}

impl<'info> Unlist<'info> {
    pub fn unlist(&mut self, seed: u64, bumps: &UnlistBumps) -> Result<()> {
        let base_asset = BaseAssetV1::try_from(&self.asset.to_account_info())
            .map_err(|_| error!(Errors::InvalidAsset))?;
        let escrow = &self.escrow.to_account_info();
        let mpl_program = &self.mpl_core_program.to_account_info();
        let maker = &self.maker.to_account_info();

        let binding = seed.to_le_bytes();
        let seeds: &[&[u8]] = &[b"escrow", &binding.as_ref(), &[self.escrow.bump]];

        TransferV1CpiBuilder::new(&mpl_program)
            .asset(&self.asset.to_account_info())
            .payer(&maker)
            .new_owner(&maker)
            .invoke_signed(&[seeds])?;

        msg!("NFT Unlisted Succesfully");

        Ok(())
    }
}
