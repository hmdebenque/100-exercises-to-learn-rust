// TODO: Define a new trait, `Power`, that has a method `power` that raises `self`
//  to the power of `n`.
//  The trait definition and its implementations should be enough to get
//  the tests to compile and pass.
//
// Recommendation: you may be tempted to write a generic implementation to handle
// all cases at once. However, this is fairly complicated and requires the use of
// additional crates (i.e. `num-traits`).
// Even then, it might be preferable to use a simple macro instead to avoid
// the complexity of a highly generic implementation. Check out the
// "Little book of Rust macros" () if you're
// interested in learning more about it.
// You don't have to though: it's perfectly okay to write three separate
// implementations manually. Venture further only if you're curious.

trait Power<POW> {
    type Output;
    fn power(&self, pow: POW) -> Self::Output;
}

macro_rules! power_impl {
    ($($t:ty)*) => ($(
        impl Power<$t> for u32 {
            type Output = u32;
            fn power(&self, pow: $t) -> u32 { self.pow(pow as u32) }
        }
    )*)
}

power_impl! {  usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! power_deref_impl {
    ($($t:ty)*) => ($(
        impl Power<$t> for u32 {
            type Output = u32;
            fn power(&self, pow: $t) -> u32 { self.pow(*pow as u32) }
        }
    )*)
}

power_deref_impl! {  &usize &u8 &u16 &u32 &u64 &u128 &isize &i8 &i16 &i32 &i64 &i128 }


#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
