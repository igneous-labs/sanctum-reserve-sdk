use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct UnstakeQuote {
    /// Total stake account lamports, before subtracting fees
    pub stake_account_lamports: u64,

    /// Output lamports, after subtracting fees
    pub lamports_to_unstaker: u64,

    /// In terms of lamports, contains protocol and referrer fees
    pub fee: u64,

    /// In terms of lamports, percentage of `fee`
    pub protocol_fee: u64,

    /// In terms of lamports, percentage of `protocol_fee`
    pub referrer_fee: u64,
}
