use borsh::{BorshDeserialize, BorshSerialize};

use crate::{internal_utils::AnchorAccount, ProtocolFeeRatios, Rational};

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
    #[inline]
    pub const fn fee_ratios(&self) -> ProtocolFeeRatios {
        ProtocolFeeRatios {
            fee_ratio: self.fee_ratio,
            referrer_fee_ratio: self.referrer_fee_ratio,
        }
    }
}

impl AnchorAccount for ProtocolFee {
    const DISCM: [u8; 8] = [121, 127, 98, 139, 72, 110, 44, 118];
}

impl ProtocolFee {
    inherent_borsh_serde!();
    inherent_anchor_serde!();
}
