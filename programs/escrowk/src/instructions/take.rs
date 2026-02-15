#![allow(unused_imports)]

use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account,
        transfer_checked,
        CloseAccount,
        Mint,
        TokenAccount,
        TokenInterface,
        TransferChecked,
    },
};

use crate::Escrow;

#[derive(Accounts)]
pub struct Take<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,

    /// maker receives mint_b from taker
    /// CHECK: validated via escrow data
    #[account(mut)]
    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        has_one = maker,
        has_one = mint_a,
        has_one = mint_b,
        seeds = [b"escrow", maker.key().as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump = escrow.bump,
        close = maker
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,

    /// vault holding maker's tokens
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    /// taker sends mint_b to maker
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    /// taker receives mint_a
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> Take<'info> {
    pub fn deposit_to_maker(&mut self) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer_checked(cpi_ctx, self.escrow.receive, self.mint_b.decimals)
    }

    pub fn withdraw_from_vault(&mut self) -> Result<()> {
    let maker_key = self.maker.key();
    let seed_bytes = self.escrow.seed.to_le_bytes();
    let bump = self.escrow.bump;

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        maker_key.as_ref(),
        &seed_bytes,
        &[bump],
    ]];

    let cpi_accounts = TransferChecked {
        from: self.vault.to_account_info(),
        mint: self.mint_a.to_account_info(),
        to: self.taker_ata_a.to_account_info(),
        authority: self.escrow.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    transfer_checked(
        cpi_ctx,
        self.vault.amount,
        self.mint_a.decimals,
    )
}

    pub fn close_vault(&mut self) -> Result<()> {
    let maker_key = self.maker.key();
    let seed_bytes = self.escrow.seed.to_le_bytes();
    let bump = self.escrow.bump;

    let signer_seeds: &[&[&[u8]]] = &[&[
        b"escrow",
        maker_key.as_ref(),
        &seed_bytes,
        &[bump],
    ]];

    let cpi_accounts = CloseAccount {
        account: self.vault.to_account_info(),
        destination: self.maker.to_account_info(),
        authority: self.escrow.to_account_info(),
    };

    let cpi_ctx = CpiContext::new_with_signer(
        self.token_program.to_account_info(),
        cpi_accounts,
        signer_seeds,
    );

    close_account(cpi_ctx)
}
}