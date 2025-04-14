use borsh::{BorshDeserialize, BorshSerialize};

use crate::{internal_utils::AnchorAccount, UnstakeQuote};

use super::{Fee, ProtocolFee};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Pool {
    /// The authority authorized to set fees
    pub fee_authority: [u8; 32],

    /// The pool's lp token mint
    pub lp_mint: [u8; 32],

    /// The last known value of total number of lamports in stake accounts
    /// owned by the pool that have not been reclaimed yet.
    /// The total SOL owned by a pool accounted for can be calculated by taking
    /// incoming_stake + pool_sol_reserves.lamports
    pub incoming_stake: u64,
}

impl Pool {
    pub fn quote_unstake(
        &self,
        fee_account: &Fee,
        protocol_fee: &ProtocolFee,
        pool_sol_reserves: u64,
        stake_account_lamports: u64,
        with_referrer: bool,
    ) -> Option<UnstakeQuote> {
        let fee_lamports = fee_account.fee.apply(
            self.incoming_stake,
            pool_sol_reserves,
            stake_account_lamports,
        )?;

        let lamports_to_unstaker = stake_account_lamports.checked_sub(fee_lamports)?;
        let protocol_fee_lamports = protocol_fee.fee_ratio.apply(fee_lamports)?.fee();

        match with_referrer {
            true => {
                let referrer_fee_lamports = protocol_fee
                    .referrer_fee_ratio
                    .apply(protocol_fee_lamports)?;

                Some(UnstakeQuote {
                    stake_account_lamports,
                    lamports_to_unstaker,
                    fee: fee_lamports,
                    protocol_fee: referrer_fee_lamports.rem(),
                    referrer_fee: referrer_fee_lamports.fee(),
                })
            }
            false => Some(UnstakeQuote {
                stake_account_lamports,
                lamports_to_unstaker,
                fee: fee_lamports,
                protocol_fee: protocol_fee_lamports,
                referrer_fee: 0,
            }),
        }
    }
}

impl AnchorAccount for Pool {
    const DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
}

impl Pool {
    inherent_borsh_serde!();
    inherent_anchor_serde!();
}
