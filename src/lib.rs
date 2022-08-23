//! A concentrated liquidity AMM contract powered by Orca.
use anchor_lang::prelude::*;

declare_id!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

#[doc(hidden)]
pub mod constants;
#[doc(hidden)]
pub mod errors;
#[doc(hidden)]
pub mod instructions;
#[doc(hidden)]
pub mod manager;
#[doc(hidden)]
pub mod math;
pub mod state;
#[doc(hidden)]
pub mod util;

use crate::state::{OpenPositionBumps, OpenPositionWithMetadataBumps, WhirlpoolBumps};
use instructions::*;

#[program]
pub mod whirlpool {
    use super::*;
    /// Open a position in a Whirlpool. A unique token will be minted to represent the position
    /// in the users wallet. The position will start off with 0 liquidity.
    ///
    /// ### Parameters
    /// - `tick_lower_index` - The tick specifying the lower end of the position range.
    /// - `tick_upper_index` - The tick specifying the upper end of the position range.
    ///
    /// #### Special Errors
    /// - `InvalidTickIndex` - If a provided tick is out of bounds, out of order or not a multiple of
    ///                        the tick-spacing in this pool.
    pub fn open_position(
        ctx: Context<OpenPosition>,
        bumps: OpenPositionBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> Result<()> {
        Ok(())
    }

    /// Open a position in a Whirlpool. A unique token will be minted to represent the position
    /// in the users wallet. Additional Metaplex metadata is appended to identify the token.
    /// The position will start off with 0 liquidity.
    ///
    /// ### Parameters
    /// - `tick_lower_index` - The tick specifying the lower end of the position range.
    /// - `tick_upper_index` - The tick specifying the upper end of the position range.
    ///
    /// #### Special Errors
    /// - `InvalidTickIndex` - If a provided tick is out of bounds, out of order or not a multiple of
    ///                        the tick-spacing in this pool.
    pub fn open_position_with_metadata(
        ctx: Context<OpenPositionWithMetadata>,
        bumps: OpenPositionWithMetadataBumps,
        tick_lower_index: i32,
        tick_upper_index: i32,
    ) -> Result<()> {
        Ok(())
    }

    /// Add liquidity to a position in the Whirlpool. This call also updates the position's accrued fees and rewards.
    ///
    /// ### Authority
    /// - `position_authority` - authority that owns the token corresponding to this desired position.
    ///
    /// ### Parameters
    /// - `liquidity_amount` - The total amount of Liquidity the user is willing to deposit.
    /// - `token_max_a` - The maximum amount of tokenA the user is willing to deposit.
    /// - `token_max_b` - The maximum amount of tokenB the user is willing to deposit.
    ///
    /// #### Special Errors
    /// - `LiquidityZero` - Provided liquidity amount is zero.
    /// - `LiquidityTooHigh` - Provided liquidity exceeds u128::max.
    /// - `TokenMaxExceeded` - The required token to perform this operation exceeds the user defined amount.
    pub fn increase_liquidity(
        ctx: Context<ModifyLiquidity>,
        liquidity_amount: u128,
        token_max_a: u64,
        token_max_b: u64,
    ) -> Result<()> {
        return instructions::increase_liquidity::handler(
            ctx,
            liquidity_amount,
            token_max_a,
            token_max_b,
        );
    }

    /// Withdraw liquidity from a position in the Whirlpool. This call also updates the position's accrued fees and rewards.
    ///
    /// ### Authority
    /// - `position_authority` - authority that owns the token corresponding to this desired position.
    ///
    /// ### Parameters
    /// - `liquidity_amount` - The total amount of Liquidity the user desires to withdraw.
    /// - `token_min_a` - The minimum amount of tokenA the user is willing to withdraw.
    /// - `token_min_b` - The minimum amount of tokenB the user is willing to withdraw.
    ///
    /// #### Special Errors
    /// - `LiquidityZero` - Provided liquidity amount is zero.
    /// - `LiquidityTooHigh` - Provided liquidity exceeds u128::max.
    /// - `TokenMinSubceeded` - The required token to perform this operation subceeds the user defined amount.
    pub fn decrease_liquidity(
        ctx: Context<ModifyLiquidity>,
        liquidity_amount: u128,
        token_min_a: u64,
        token_min_b: u64,
    ) -> Result<()> {
        return instructions::decrease_liquidity::handler(
            ctx,
            liquidity_amount,
            token_min_a,
            token_min_b,
        );
    }

    /// Collect fees accrued for this position.
    ///
    /// ### Authority
    /// - `position_authority` - authority that owns the token corresponding to this desired position.
    pub fn collect_fees(ctx: Context<CollectFees>) -> Result<()> {
        return instructions::collect_fees::handler(ctx);
    }

    /// Collect rewards accrued for this position.
    ///
    /// ### Authority
    /// - `position_authority` - authority that owns the token corresponding to this desired position.
    pub fn collect_reward(ctx: Context<CollectReward>, reward_index: u8) -> Result<()> {
        return instructions::collect_reward::handler(ctx, reward_index);
    }

    /// Perform a swap in this Whirlpool
    ///
    /// ### Authority
    /// - "token_authority" - The authority to withdraw tokens from the input token account.
    ///
    /// ### Parameters
    /// - `amount` - The amount of input or output token to swap from (depending on exact_input).
    /// - `other_amount_threshold` - The maximum/minimum of input/output token to swap into (depending on exact_input).
    /// - `sqrt_price_limit` - The maximum/minimum price the swap will swap to.
    /// - `exact_input` - Specifies the token the parameter `amount`represents. If true, the amount represents the input token of the swap.
    /// - `a_to_b` - The direction of the swap. True if swapping from A to B. False if swapping from B to A.
    ///
    /// #### Special Errors
    /// - `ZeroTradableAmount` - User provided parameter `amount` is 0.
    /// - `InvalidSqrtPriceLimitDirection` - User provided parameter `sqrt_price_limit` does not match the direction of the trade.
    /// - `SqrtPriceOutOfBounds` - User provided parameter `sqrt_price_limit` is over Whirlppool's max/min bounds for sqrt-price.
    /// - `InvalidTickArraySequence` - User provided tick-arrays are not in sequential order required to proceed in this trade direction.
    /// - `TickArraySequenceInvalidIndex` - The swap loop attempted to access an invalid array index during the query of the next initialized tick.
    /// - `TickArrayIndexOutofBounds` - The swap loop attempted to access an invalid array index during tick crossing.
    /// - `LiquidityOverflow` - Liquidity value overflowed 128bits during tick crossing.
    /// - `InvalidTickSpacing` - The swap pool was initialized with tick-spacing of 0.
    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        other_amount_threshold: u64,
        sqrt_price_limit: u128,
        amount_specified_is_input: bool,
        a_to_b: bool,
    ) -> Result<()> {
        return instructions::swap::handler(
            ctx,
            amount,
            other_amount_threshold,
            sqrt_price_limit,
            amount_specified_is_input,
            a_to_b,
        );
    }

    /// Close a position in a Whirlpool. Burns the position token in the owner's wallet.
    ///
    /// ### Authority
    /// - "position_authority" - The authority that owns the position token.
    ///
    /// #### Special Errors
    /// - `ClosePositionNotEmpty` - The provided position account is not empty.
    pub fn close_position(ctx: Context<ClosePosition>) -> Result<()> {
        return instructions::close_position::handler(ctx);
    }

        /// Update the accrued fees and rewards for a position.
    ///
    /// #### Special Errors
    /// - `TickNotFound` - Provided tick array account does not contain the tick for this position.
    /// - `LiquidityZero` - Position has zero liquidity and therefore already has the most updated fees and reward values.
    pub fn update_fees_and_rewards(ctx: Context<UpdateFeesAndRewards>) -> Result<()> {
        return instructions::update_fees_and_rewards::handler(ctx);
    }

}
