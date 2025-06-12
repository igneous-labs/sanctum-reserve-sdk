use core::{error::Error, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReserveError {
    NotEnoughLiquidity,
    InternalError,
}

impl Display for ReserveError {
    // Display=Debug, since this is just a simple str enum
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl Error for ReserveError {}
