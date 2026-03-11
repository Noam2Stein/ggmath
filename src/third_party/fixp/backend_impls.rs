use fixp::FixedPoint;

use crate::{Alignment, Length, ScalarBackend, SupportedLength};

macro_rules! int_impl {
    ($T:ident) => {
        impl<const N: usize, const FRAC_BITS: usize, A: Alignment> ScalarBackend<N, A>
            for FixedPoint<$T, FRAC_BITS>
        where
            Length<N>: SupportedLength,
        {
        }
    };
}
int_impl!(i8);
int_impl!(i16);
int_impl!(i32);
int_impl!(i64);
int_impl!(i128);
int_impl!(isize);
int_impl!(u8);
int_impl!(u16);
int_impl!(u32);
int_impl!(u64);
int_impl!(u128);
int_impl!(usize);
