use borsh::{BorshDeserialize, BorshSerialize};
use sanctum_fee_ratio::AftFee;
use sanctum_u64_ratio::{Floor, Ratio};

// TODO: derivation of Eq might be wrong since fraction equality is not necessarily bit equality,
// but this is how upstream does it
#[derive(Debug, Clone, Copy, BorshSerialize, BorshDeserialize, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(
    feature = "wasm",
    derive(tsify_next::Tsify),
    tsify(into_wasm_abi, from_wasm_abi, large_number_types_as_bigints)
)]
pub struct Rational {
    pub num: u64,
    pub denom: u64,
}

impl Rational {
    pub const ZERO: Self = Self { denom: 0, num: 0 };

    /// Applies the Fee's rates to a given amount, `amt`
    /// returning the amount to be subtracted from it as fees
    /// (0 if denominator is 0 or amt is 0),
    /// or None if overflow occurs
    #[inline]
    pub const fn apply(&self, amt: u64) -> Option<AftFee> {
        type F = sanctum_fee_ratio::Fee<Floor<Ratio<u64, u64>>>;

        let f = match F::new(Ratio {
            n: self.num,
            d: self.denom,
        }) {
            None => return None,
            Some(f) => f,
        };
        f.apply(amt)
    }
}

impl Default for Rational {
    #[inline]
    fn default() -> Self {
        Self::ZERO
    }
}

impl Rational {
    inherent_borsh_serde!();
}
