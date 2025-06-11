use borsh::{BorshDeserialize, BorshSerialize};

use crate::{internal_utils::AnchorAccount, math::PreciseNumber, PoolBalance, Rational};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Fee {
    pub fee: FeeEnum,
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub enum FeeEnum {
    /// Charges a flat fee based on a set fee ratio
    /// applied to the size of a given swap.
    /// E.g. num: 1, denom: 10_000 => 1bps fee
    ///
    /// Invariants:
    ///  - ratio is a valid Rational
    ///  - ratio <= 1
    Flat(Rational),

    /// Charges a fee based on how much liquidity
    /// a swap leaves in the liquidity pool,
    /// increasing linearly as less liquidity is left.
    /// See diagram in apply() below for details
    ///
    /// Invariants:
    ///  - max_liq_remaining is a valid Rational
    ///  - max_liq_remaining <= 1
    ///  - zero_liq_remaining is a valid Rational
    ///  - zero_liq_remaining <= 1
    ///  - max_liq_remaining <= zero_liq_remaining
    LiquidityLinear(LiquidityLinearParams),
}

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct LiquidityLinearParams {
    /// The fee applied to a swap that leaves
    /// 100% of all liquidity in the SOL reserves account
    pub max_liq_remaining: Rational,

    /// The fee applied to a swap that leaves
    /// no liquidity remaining in the SOL reserves account
    pub zero_liq_remaining: Rational,
}

impl LiquidityLinearParams {
    #[inline]
    fn to_fee_ratio(
        &self,
        PoolBalance {
            pool_incoming_stake,
            sol_reserves_lamports,
        }: &PoolBalance,
        stake_account_lamports: u64,
    ) -> Option<PreciseNumber> {
        let zero_liq_fee = self.zero_liq_remaining.into_precise_number()?;
        let max_liq_fee = self.max_liq_remaining.into_precise_number()?;
        let owned_lamports =
            (*pool_incoming_stake as u128).checked_add(*sol_reserves_lamports as u128)?;

        let slope_num = zero_liq_fee.checked_sub(&max_liq_fee)?;
        let slope_denom = PreciseNumber::new(owned_lamports)?;

        let incoming_plus_stake =
            (*pool_incoming_stake as u128).checked_add(stake_account_lamports as u128)?;
        let num = slope_denom
            .checked_mul(&max_liq_fee)?
            .checked_div(&slope_num)?
            .checked_add(&PreciseNumber::new(incoming_plus_stake)?)?;
        let denom = slope_denom
            .checked_div(&slope_num)?
            .checked_add(&PreciseNumber::new(stake_account_lamports as u128)?)?;
        num.checked_div(&denom)
    }
}

impl FeeEnum {
    pub fn apply(&self, pool_balance: &PoolBalance, stake_account_lamports: u64) -> Option<u64> {
        let ratio = match self {
            Self::Flat(ratio) => ratio.into_precise_number(),
            Self::LiquidityLinear(params) => {
                params.to_fee_ratio(pool_balance, stake_account_lamports)
            }
        }?;

        PreciseNumber::new(stake_account_lamports as u128)?
            .checked_mul(&ratio)?
            .ceiling()?
            .to_imprecise()
            .and_then(|v| u64::try_from(v).ok())
    }
}

impl AnchorAccount for Fee {
    const DISCM: [u8; 8] = [24, 55, 150, 250, 168, 27, 101, 178];
}

impl Fee {
    inherent_borsh_serde!();
    inherent_anchor_serde!();
}

impl FeeEnum {
    inherent_borsh_serde!();
}
