use crate::{FeeEnum, PoolUnstakeParams, Rational, ReserveError, STAKE_ACCOUNT_RECORD_RENT};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct UnstakeQuote {
    /// Total lamports of stake account being unstaked
    pub stake_account_lamports: u64,

    /// Output lamports, after subtracting fees
    pub lamports_to_unstaker: u64,

    /// Fees levied. `lamports_to_unstaker + fee.total() = stake_account_lamports`
    pub fee: UnstakeFee,
}

impl UnstakeQuote {
    #[inline]
    pub const fn reserves_lamports_outflow(&self) -> u64 {
        // unchecked-arith: SOL total supply is well below u64::MAX
        self.lamports_to_unstaker
            + self.fee.protocol
            + self.fee.referrer
            + STAKE_ACCOUNT_RECORD_RENT
    }
}

/// All values in terms of lamports
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct UnstakeFee {
    /// Fees that go to the pool
    pub lp: u64,

    /// Fees that go to the protocol
    pub protocol: u64,

    /// Fees that go to the referrer
    pub referrer: u64,
}

impl UnstakeFee {
    #[inline]
    pub const fn total(&self) -> u64 {
        // unchecked-arith: SOL total supply is well below u64::MAX
        self.lp + self.protocol + self.referrer
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct ProtocolFeeRatios {
    pub fee_ratio: Rational,
    pub referrer_fee_ratio: Rational,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QuoteUnstakeOpts {
    pub with_referrer: bool,

    /// Lamports already existing in the corresponding `StakeAccountRecord` account.
    ///
    /// Usually 0. SOL reserves will fund any rent-exemption shortfall **before** calculating fees.
    pub stake_account_record_lamports: u64,
}

impl QuoteUnstakeOpts {
    pub const DEFAULT: Self = Self {
        with_referrer: false,
        stake_account_record_lamports: 0,
    };
}

impl Default for QuoteUnstakeOpts {
    #[inline]
    fn default() -> Self {
        Self::DEFAULT
    }
}

/// # Usage Notes
/// - Assumes that the `StakeAccountRecord` account is unfunded and therefore rent-exemption
///   will be funded from the SOL reserves. `PoolBalance` param should be the current account state, **NOT**
///   the state after this rent-exemption funding procedure; this function deducts the StakeAccountRecord
///   rent-exemption from the balances.
pub fn quote_unstake(
    PoolUnstakeParams {
        pool_incoming_stake,
        sol_reserves_lamports,
    }: &PoolUnstakeParams,
    fee: &FeeEnum,
    ProtocolFeeRatios {
        fee_ratio: protocol_fee_fee_ratio,
        referrer_fee_ratio,
    }: &ProtocolFeeRatios,
    stake_account_lamports: u64,
    QuoteUnstakeOpts {
        with_referrer,
        stake_account_record_lamports,
    }: &QuoteUnstakeOpts,
) -> Result<UnstakeQuote, ReserveError> {
    // Need to subtract stake account rent from sol_reserves_lamports
    // because creating that account happens before fees are calculated
    let stake_account_record_rent_shortfall =
        STAKE_ACCOUNT_RECORD_RENT.saturating_sub(*stake_account_record_lamports);
    let pool_balance = PoolUnstakeParams {
        pool_incoming_stake: *pool_incoming_stake,
        sol_reserves_lamports: sol_reserves_lamports
            .checked_sub(stake_account_record_rent_shortfall)
            .ok_or(ReserveError::NotEnoughLiquidity)?,
    };

    let fee_lamports = fee
        .apply(&pool_balance, stake_account_lamports)
        .ok_or(ReserveError::InternalError)?;

    let lamports_to_unstaker = stake_account_lamports
        .checked_sub(fee_lamports)
        .ok_or(ReserveError::InternalError)?;

    let aft_protocol_fee = protocol_fee_fee_ratio
        .apply(fee_lamports)
        .ok_or(ReserveError::InternalError)?;
    let lp_fee = aft_protocol_fee.rem();
    let protocol_fee = aft_protocol_fee.fee();

    let quote_no_ref = UnstakeQuote {
        stake_account_lamports,
        lamports_to_unstaker,
        fee: UnstakeFee {
            lp: lp_fee,
            protocol: protocol_fee,
            referrer: 0,
        },
    };

    if pool_balance.sol_reserves_lamports < quote_no_ref.reserves_lamports_outflow() {
        return Err(ReserveError::NotEnoughLiquidity);
    }

    match with_referrer {
        true => {
            let referrer_fee = referrer_fee_ratio
                .apply(protocol_fee)
                .ok_or(ReserveError::InternalError)?;

            Ok(UnstakeQuote {
                fee: UnstakeFee {
                    lp: lp_fee,
                    protocol: referrer_fee.rem(),
                    referrer: referrer_fee.fee(),
                },
                ..quote_no_ref
            })
        }
        false => Ok(quote_no_ref),
    }
}
