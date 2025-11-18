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
pub struct Buy<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub maker: SystemAccount<'info>, //e Why is this a system account
    #[account(mut)]
    pub mint_sol: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    ///CHECKED: No Validation needed here
    pub asset: UncheckedAccount<'info>,
    #[account(
        mut,
        close = maker,
        seeds = [b"escrow",seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    #[account(
       mut ,
       associated_token::mint = mint_sol,
       associated_token::authority = escrow,
       associated_token::token_program = token_program,
    )]
    pub vault: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        associated_token::mint =mint_sol,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_sol: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::mint = mint_sol,  
        associated_token::authority = buyer, //e correct
        associated_token::token_program = token_program
    )]
    pub buyer_ata_sol: Box<InterfaceAccount<'info, TokenAccount>>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    ///CHECK: Safe
    pub mpl_core_program: AccountInfo<'info>,
}

impl<'info> Buy<'info> {
    pub fn buy_nft(&mut self, seed: u64, bumps: &BuyBumps, price_amount: u64) -> Result<()> {
        let base_asset = BaseAssetV1::try_from(&self.asset.to_account_info())
            .map_err(|_| error!(Errors::InvalidAsset))?;
        let mint_account = &self.mint_sol.to_account_info();
        let maker = &self.maker.to_account_info();
        let escrow = &self.escrow.to_account_info();
        let vault = &self.vault.to_account_info();
        let mpl_program = &self.mpl_core_program.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let buyer = &self.buyer.to_account_info();

        let transfer_accounts = TransferChecked {
            from: self.buyer_ata_sol.to_account_info(),
            to: self.maker_ata_sol.to_account_info(),
            mint: self.mint_sol.to_account_info(),
            authority: self.buyer.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, price_amount, self.mint_sol.decimals)?;

        // let bump = bumps.escrow;
        // let binding = &self.key();
        // let seeds: &[&[u8]] = &[b"escrow", binding.as_ref(), &[bump]];

        let binding = seed.to_le_bytes();
        let seeds: &[&[u8]] = &[
            b"escrow",
            &binding.as_ref(),
            &[self.escrow.bump], // Use the bump stored in escrow state
        ];

        //e ERROR: When the owner is an escrow/pda you have to specify authority...
        TransferV1CpiBuilder::new(&mpl_program)
            .asset(&self.asset.to_account_info())
            .authority(Some(&self.escrow.to_account_info()))
            .payer(&buyer)
            .new_owner(&buyer)
            .invoke_signed(&[seeds])?;

        msg!("NFT Bought succesfully");
        //e Transferring NFT Price to maker_ata_sol
        Ok(())
    }
}
