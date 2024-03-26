use std::str::FromStr;

use arrayvec::{ArrayString, CapacityError};

pub const MAX_STRING_LEN: usize = 128;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayString128(ArrayString<MAX_STRING_LEN>);

impl FromStr for ArrayString128 {
    type Err = CapacityError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(ArrayString::<MAX_STRING_LEN>::from_str(s)?))
    }
}
