use borsh::{BorshDeserialize, BorshSerialize};

use crate::Rational;

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

#[repr(C)]
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
    Flat { ratio: Rational },

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
    LiquidityLinear { params: LiquidityLinearParams },
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

impl Fee {
    inherent_borsh_serde!();
}

impl FeeEnum {
    inherent_borsh_serde!();
}
