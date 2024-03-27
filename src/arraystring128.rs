use std::str::FromStr;

use arrayvec::{ArrayString, CapacityError};

pub const MAX_STRING_LEN: usize = 128;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ArrayString128(ArrayString<MAX_STRING_LEN>);

impl FromStr for ArrayString128 {
    type Err = CapacityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(ArrayString::<MAX_STRING_LEN>::from_str(s)?))
    }
}

impl std::fmt::Display for ArrayString128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
