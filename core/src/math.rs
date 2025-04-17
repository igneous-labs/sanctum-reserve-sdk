// Clippy complains about `(x + (y - 1)) / y` found in the uint crate
#![allow(clippy::manual_div_ceil)]

/// Copy-pasted from spl-math (only necessary for fee calculation)
use uint::construct_uint;

construct_uint! {
    pub(crate) struct U256(4);
}

pub(crate) type InnerUint = U256;

/// The representation of the number one as a precise number as 10^12
pub(crate) const ONE: u128 = 1_000_000_000_000;

/// Struct encapsulating a fixed-point number that allows for decimal calculations
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct PreciseNumber {
    /// Wrapper over the inner value, which is multiplied by ONE
    pub value: InnerUint,
}

/// The precise-number 1 as a InnerUint
fn one() -> InnerUint {
    InnerUint::from(ONE)
}

/// The number 0 as a PreciseNumber, used for easier calculations.
fn zero() -> InnerUint {
    InnerUint::from(0)
}

impl PreciseNumber {
    /// Correction to apply to avoid truncation errors on division.  Since
    /// integer operations will always floor the result, we artifically bump it
    /// up by one half to get the expect result.
    fn rounding_correction() -> InnerUint {
        InnerUint::from(ONE / 2)
    }

    fn zero() -> Self {
        Self { value: zero() }
    }

    /// Create a precise number from an imprecise u128, should always succeed
    pub(crate) fn new(value: u128) -> Option<Self> {
        let value = InnerUint::from(value).checked_mul(one())?;
        Some(Self { value })
    }

    /// Convert a precise number back to u128
    pub(crate) fn to_imprecise(&self) -> Option<u128> {
        self.value
            .checked_add(Self::rounding_correction())?
            .checked_div(one())
            .map(|v| v.as_u128())
    }

    /// Ceiling a precise value to a precision of ONE
    pub(crate) fn ceiling(&self) -> Option<Self> {
        let value = self
            .value
            .checked_add(one().checked_sub(InnerUint::from(1))?)?
            .checked_div(one())?
            .checked_mul(one())?;
        Some(Self { value })
    }

    /// Performs a checked division on two precise numbers
    pub(crate) fn checked_div(&self, rhs: &Self) -> Option<Self> {
        if *rhs == Self::zero() {
            return None;
        }
        match self.value.checked_mul(one()) {
            Some(v) => {
                let value = v
                    .checked_add(Self::rounding_correction())?
                    .checked_div(rhs.value)?;
                Some(Self { value })
            }
            None => {
                let value = self
                    .value
                    .checked_add(Self::rounding_correction())?
                    .checked_div(rhs.value)?
                    .checked_mul(one())?;
                Some(Self { value })
            }
        }
    }

    /// Performs a multiplication on two precise numbers
    pub(crate) fn checked_mul(&self, rhs: &Self) -> Option<Self> {
        match self.value.checked_mul(rhs.value) {
            Some(v) => {
                let value = v
                    .checked_add(Self::rounding_correction())?
                    .checked_div(one())?;
                Some(Self { value })
            }
            None => {
                let value = if self.value >= rhs.value {
                    self.value.checked_div(one())?.checked_mul(rhs.value)?
                } else {
                    rhs.value.checked_div(one())?.checked_mul(self.value)?
                };
                Some(Self { value })
            }
        }
    }

    /// Performs addition of two precise numbers
    pub(crate) fn checked_add(&self, rhs: &Self) -> Option<Self> {
        let value = self.value.checked_add(rhs.value)?;
        Some(Self { value })
    }

    /// Subtracts the argument from self
    pub(crate) fn checked_sub(&self, rhs: &Self) -> Option<Self> {
        let value = self.value.checked_sub(rhs.value)?;
        Some(Self { value })
    }
}
