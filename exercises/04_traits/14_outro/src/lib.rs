// TODO: Define a new `SaturatingU16` type.
//   It should hold a `u16` value.
//   It should provide conversions from `u16`, `u8`, `&u16` and `&u8`.
//   It should support addition with a right-hand side of type
//   SaturatingU16, u16, &u16, and &SaturatingU16. Addition should saturate at the
//   maximum value for `u16`.
//   It should be possible to compare it with another `SaturatingU16` or a `u16`.
//   It should be possible to print its debug representation.
//
// Tests are located in the `tests` folder—pay attention to the visibility of your types and methods.

use std::ops::Add;

#[derive(PartialOrd, PartialEq, Debug)]
struct SaturatingU16 {
    value: u16,
}

impl SaturatingU16 {
    fn new(value: u16) -> SaturatingU16 {
        SaturatingU16 { value }
    }
}

impl Add for SaturatingU16 {
    type Output = Self;

    fn add(self, addition: SaturatingU16) -> SaturatingU16 {
        SaturatingU16::new(self.value.saturating_add(addition.value))
    }
}



macro_rules! from_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for SaturatingU16 {
            fn from(value: $t) -> Self {
                SaturatingU16::new(value as u16)
            }
        }
    )*)
}

macro_rules! from_deref_impl {
    ($($t:ty)*) => ($(
        impl From<$t> for SaturatingU16 {
            fn from(value: $t) -> Self {
                SaturatingU16::new(*value as u16)
            }
        }
    )*)
}

from_impl! { u16 u8 }
from_deref_impl! { &u16 &u8 }

#[cfg(test)]
mod test {
    use crate::SaturatingU16;

    #[test]
    fn conv() {
        assert_eq!(SaturatingU16::new(10u16), SaturatingU16::from(10u8));
        assert_eq!(SaturatingU16::new(10u16), SaturatingU16::from(10u16));
        assert_eq!(SaturatingU16::new(10u16), SaturatingU16::from(&10u8));
        assert_eq!(SaturatingU16::new(10u16), SaturatingU16::from(&10u16));
    }

    #[test]
    fn add() {
        assert_eq!(SaturatingU16::new(10u16) + SaturatingU16::new(6u16), SaturatingU16::new(16u16));
    }

    #[test]
    fn saturating_add() {
        assert_eq!(SaturatingU16::new(u16::MAX), SaturatingU16::new(u16::MAX) + SaturatingU16::from(100u16));
    }
}

