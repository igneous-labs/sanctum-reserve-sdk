use borsh::{BorshDeserialize, BorshSerialize};

use crate::Rational;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct FlashLoanFee {
    /// The proportion of the flash loan amount that is levied as fees
    pub fee_ratio: Rational,
}

impl FlashLoanFee {
    inherent_borsh_serde!();
}
