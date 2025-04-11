use borsh::{BorshDeserialize, BorshSerialize};

use crate::Rational;

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct ProtocolFee {
    /// Protocol-owned account to receive the protocol fees to
    pub destination: [u8; 32],

    /// Signer that is authorized to modify this account
    pub authority: [u8; 32],

    /// The proportion of unstake fees that go to the protocol
    pub fee_ratio: Rational,

    /// The proprtion of the protocol fees that go to the referrer
    pub referrer_fee_ratio: Rational,
}

impl ProtocolFee {
    inherent_borsh_serde!();
}
