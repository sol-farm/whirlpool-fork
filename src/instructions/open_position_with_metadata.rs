use crate::state::Whirlpool;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{self, Mint, Token, TokenAccount};

use crate::{state::*, util::mint_position_token_with_metadata_and_remove_authority};

use whirlpool_nft_update_auth::ID as WP_NFT_UPDATE_AUTH;
mod whirlpool_nft_update_auth {
    use super::*;
    declare_id!("3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr");
}

#[derive(Accounts)]
#[instruction(bumps: OpenPositionWithMetadataBumps)]
pub struct OpenPositionWithMetadata<'info> {
    #[account(mut)]
    pub funder: Signer<'info>,

    pub owner: UncheckedAccount<'info>,

    #[account(mut,
      seeds = [b"position".as_ref(), position_mint.key().as_ref()],
      bump = bumps.position_bump,
    )]
    pub position: Box<Account<'info, Position>>,

    #[account(mut,
        mint::authority = whirlpool,
        mint::decimals = 0,
    )]
    pub position_mint: Account<'info, Mint>,

    /// CHECK: checked via the Metadata CPI call
    /// https://github.com/metaplex-foundation/metaplex-program-library/blob/master/token-metadata/program/src/utils.rs#L873
    #[account(mut)]
    pub position_metadata_account: UncheckedAccount<'info>,

    #[account(init,
      payer = funder,
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

    /// CHECK: checked via account constraints
    #[account(address = mpl_token_metadata::ID)]
    pub metadata_program: UncheckedAccount<'info>,

    /// CHECK: checked via account constraints
    #[account(address = WP_NFT_UPDATE_AUTH)]
    pub metadata_update_auth: UncheckedAccount<'info>,
}
