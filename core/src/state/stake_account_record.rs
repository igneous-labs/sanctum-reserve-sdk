use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct StakeAccountRecord {
    /// The stake account's lamports in the associated stake account
    /// at time of Unstake.
    /// Note: this is the account's total lamports not staked lamports
    /// Solana enforces this to be at least rent exempt balance + 1 lamport
    pub lamports_at_creation: u64,
}

impl StakeAccountRecord {
    inherent_borsh_serde!();
}
