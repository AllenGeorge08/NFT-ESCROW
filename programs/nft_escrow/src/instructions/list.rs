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
pub struct List<'info> {
    #[account(mut)]
    pub maker: Signer<'info>, //q Shouldn't this be inside a box too?
    #[account(mut)]
    pub mint_sol: Box<InterfaceAccount<'info, Mint>>,
    // #[account(mut)]
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

impl<'info> List<'info> {
    pub fn initialize_escrow(&mut self, seed: u64, bumps: &ListBumps,amount: u64) -> Result<()> {
        self.escrow.set_inner(Escrow {
            seed: seed,
            maker: self.maker.key(),
            maker_mint: self.maker_ata_sol.key(),
            bump: bumps.escrow,
            fee: 5,
            state: crate::state::List { maker: self.maker.key(), maker_mint: self.mint_sol.key(), bump: bumps.escrow , vault: self.vault.key(), price: amount, listed: true, owner: self.maker.key()}
        });
        Ok(())
    }

    pub fn list_nft(&mut self, amount: u64, bumps: &ListBumps, seed: u64) -> Result<()> {
        let base_asset = BaseAssetV1::try_from(&self.asset.to_account_info())
            .map_err(|_| error!(Errors::InvalidAsset))?;
        let mint_account = &self.mint_sol.to_account_info();
        let maker = &self.maker.to_account_info();
        let escrow = &self.escrow.to_account_info();
        let vault = &self.vault.to_account_info();
        let mpl_program = &self.mpl_core_program.to_account_info();
        let system_program = &self.system_program.to_account_info();
        let payer = &self.maker.to_account_info();

        // e Transferring listing price(sol) to escrow (Vault)
        let transfer_accounts = TransferChecked {
            from: self.maker_ata_sol.to_account_info(),
            to: self.vault.to_account_info(),
            mint: self.mint_sol.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);
        transfer_checked(cpi_ctx, amount, self.mint_sol.decimals)?;

        //e PDA Signer seeds
        let binding = seed.to_be_bytes();
        let seeds: &[&[u8]] = &[b"escrow", &binding.as_ref(), &[self.escrow.bump]];

        TransferV1CpiBuilder::new(&mpl_program)
            .asset(&self.asset.to_account_info())
            .payer(&payer)
            .new_owner(&escrow)
            .invoke_signed(&[seeds])?;

        msg!("NFT Listed Succesfully");
        Ok(())
    }
}
