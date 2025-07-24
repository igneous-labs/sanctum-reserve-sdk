use borsh::{BorshDeserialize, BorshSerialize};

use crate::internal_utils::AnchorAccount;

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
    /// `incoming_stake + stake_account_record_rent_lamports + pool_sol_reserves.lamports`
    pub incoming_stake: u64,

    /// The total amount of rent in lamports locked in associated
    /// StakeAccountRecord accounts as rent-exemption. Reclaimed to reserves when the stake
    /// account is reclaimed.
    ///
    /// The total SOL owned by a pool accounted for can be calculated by taking
    /// `incoming_stake + stake_account_record_rent_lamports + pool_sol_reserves.lamports`
    pub stake_account_record_rent_lamports: u64,
}

impl AnchorAccount for Pool {
    const DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];
}

impl Pool {
    inherent_borsh_serde!();
    inherent_anchor_serde!();
}
