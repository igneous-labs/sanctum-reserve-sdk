use borsh::{BorshDeserialize, BorshSerialize};

use crate::{internal_utils::AnchorAccount, math::PreciseNumber, PoolBalance, Rational};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Fee(pub FeeEnum);

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
    fn linear_params(
        &self,
        PoolBalance {
            pool_incoming_stake,
            sol_reserves_lamports,
        }: &PoolBalance,
    ) -> Option<[PreciseNumber; 3]> {
        let zero_liq_fee = self.zero_liq_remaining.into_precise_number()?;
        let max_liq_fee = self.max_liq_remaining.into_precise_number()?;
        let owned_lamports =
            (*pool_incoming_stake as u128).checked_add(*sol_reserves_lamports as u128)?;

        let slope_num = zero_liq_fee.checked_sub(&max_liq_fee)?;
        let slope_denom = PreciseNumber::new(owned_lamports)?;
        Some([max_liq_fee, slope_num, slope_denom])
    }

    #[inline]
    fn fee_ratio(
        &self,
        pool_balance: &PoolBalance,
        stake_account_lamports: u64,
    ) -> Option<PreciseNumber> {
        // Reference:
        // https://github.com/igneous-labs/sanctum-unstake-program/pull/158/files#diff-159d967da4ccc6a80ee8c3959e7afa73dfcb8e29c2d142ec37a39b63977454fcR160-R222
        let [max_liq_fee, slope_num, slope_denom] = self.linear_params(pool_balance)?;
        let incoming_plus_stake = (pool_balance.pool_incoming_stake as u128)
            .checked_add(stake_account_lamports as u128)?;
        let num = slope_denom
            .checked_mul(&max_liq_fee)?
            .checked_div(&slope_num)?
            .checked_add(&PreciseNumber::new(incoming_plus_stake)?)?;
        let denom = slope_denom
            .checked_div(&slope_num)?
            .checked_add(&PreciseNumber::new(stake_account_lamports as u128)?)?;
        num.checked_div(&denom)
    }

    #[inline]
    fn reverse_fee_ratio(
        &self,
        pool_balance: &PoolBalance,
        lamports_after_fee: u64,
    ) -> Option<PreciseNumber> {
        // Reference:
        // https://github.com/igneous-labs/sanctum-unstake-program/pull/158/files#diff-159d967da4ccc6a80ee8c3959e7afa73dfcb8e29c2d142ec37a39b63977454fcR224-R248
        let [max_liq_fee, slope_num, slope_denom] = self.linear_params(pool_balance)?;
        let incoming_plus_after_fee =
            (pool_balance.pool_incoming_stake as u128).checked_add(lamports_after_fee as u128)?;
        let num = slope_num.checked_mul(&PreciseNumber::new(incoming_plus_after_fee)?)?;
        num.checked_div(&slope_denom)
            .and_then(|x| x.checked_add(&max_liq_fee))
    }
}

impl FeeEnum {
    #[inline]
    pub fn apply(&self, pool_balance: &PoolBalance, stake_account_lamports: u64) -> Option<u64> {
        let ratio = match self {
            Self::Flat(ratio) => ratio.into_precise_number(),
            Self::LiquidityLinear(params) => params.fee_ratio(pool_balance, stake_account_lamports),
        }?;
        PreciseNumber::new(stake_account_lamports as u128)?
            .checked_mul(&ratio)?
            .ceiling()?
            .to_imprecise()
            .and_then(|v| u64::try_from(v).ok())
    }

    #[inline]
    pub fn reverse_from_rem(
        &self,
        pool_balance: &PoolBalance,
        lamports_after_fee: u64,
    ) -> Option<u64> {
        let ratio = match self {
            FeeEnum::Flat(ratio) => ratio.into_precise_number()?,
            FeeEnum::LiquidityLinear(params) => {
                params.reverse_fee_ratio(pool_balance, lamports_after_fee)?
            }
        };
        let invert_by = PreciseNumber::new(1)?.checked_sub(&ratio)?;

        PreciseNumber::new(lamports_after_fee as u128)?
            .checked_div(&invert_by)?
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

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;

    prop_compose! {
        fn pool_balances()
            (pool_incoming_stake in any::<u64>())
            (sol_reserves_lamports in 0..=(u64::MAX - pool_incoming_stake), pool_incoming_stake in Just(pool_incoming_stake)) -> PoolBalance {
                PoolBalance { pool_incoming_stake, sol_reserves_lamports }
            }
    }

    prop_compose! {
        fn valid_ratio_lte_one()
            (denom in 1..=u64::MAX)
            (num in 0..=denom, denom in Just(denom)) -> Rational {
                Rational { num, denom }
            }
    }

    prop_compose! {
        fn flat_fees()
            (ratio in valid_ratio_lte_one()) -> FeeEnum {
                FeeEnum::Flat(ratio)
            }
    }

    prop_compose! {
        fn liq_linear_fees()
            (r1 in valid_ratio_lte_one(), r2 in valid_ratio_lte_one()) -> FeeEnum {
                let c1: u128 = r1.num as u128 * r2.denom as u128;
                let c2: u128 = r2.num as u128 * r1.denom as u128;
                if c1 >= c2 {
                    FeeEnum::LiquidityLinear(LiquidityLinearParams { max_liq_remaining: r2, zero_liq_remaining: r1 })
                } else {
                    FeeEnum::LiquidityLinear(LiquidityLinearParams { max_liq_remaining: r1, zero_liq_remaining: r2 })
                }
            }
    }

    proptest! {
        #[test]
        fn fee_apply_reverse_round_trip(
            pool_balance in pool_balances(),
            fee in flat_fees().boxed().prop_union(liq_linear_fees().boxed()),
            stake_account_lamports: u64
        ) {
            let fee_lamports = fee.apply(
                &pool_balance,
                stake_account_lamports,
            ).unwrap();
            let lamports_after_fee = stake_account_lamports - fee_lamports;
            if lamports_after_fee > 0 {
                // reversed might not be = stake_account_lamports due to rounding
                let reversed = fee.reverse_from_rem(&pool_balance, lamports_after_fee).unwrap();

                // however, make sure apply(reverse) gives the same result
                let reversed_fee = fee.apply(&pool_balance, reversed).unwrap();
                prop_assert_eq!(lamports_after_fee, reversed - reversed_fee);
            }
        }
    }
}
