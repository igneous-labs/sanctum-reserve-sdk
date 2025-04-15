use crate::internal_utils::AnchorAccount;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct FlashAccount {
    pub lamports_borrowed: u64,
}

impl AnchorAccount for FlashAccount {
    const DISCM: [u8; 8] = [20, 88, 157, 223, 92, 187, 5, 111];
}

impl FlashAccount {
    inherent_borsh_serde!();
    inherent_anchor_serde!();
}
