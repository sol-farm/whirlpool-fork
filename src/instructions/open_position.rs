use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{state::*, util::mint_position_token_and_remove_authority};

#[derive(Accounts)]
#[instruction(bumps: OpenPositionBumps)]
pub struct OpenPosition<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    pub owner: UncheckedAccount<'info>,

    #[account(mut,
      seeds = [b"position".as_ref(), position_mint.key().as_ref()],
      bump = bumps.position_bump,
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(mut, signer,
        mint::authority = whirlpool,
        mint::decimals = 0,
    )]
    pub position_mint: Account<'info, Mint>,

    #[account(mut,
      associated_token::mint = position_mint,
      associated_token::authority = owner,
    )]
    pub position_token_account: Box<Account<'info, TokenAccount>>,

    pub whirlpool: UncheckedAccount<'info>,

    #[account(address = token::ID)]
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
