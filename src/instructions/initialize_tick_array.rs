use anchor_lang::prelude::*;

use crate::state::*;

#[derive(Accounts)]
#[instruction(start_tick_index: i32)]
pub struct InitializeTickArray<'info> {
    pub whirlpool: Account<'info, Whirlpool>,

    #[account(mut)]
    pub funder: Signer<'info>,

    #[account(mut)]
    pub tick_array: AccountLoader<'info, TickArray>,

    pub system_program: Program<'info, System>,
}