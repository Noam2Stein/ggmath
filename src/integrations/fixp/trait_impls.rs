use fixp::FixedPoint;

use crate::{
    Scalar,
    constants::{Max, Min, Zero},
};

macro_rules! int_impl {
    ($T:ident) => {
        // SAFETY: `FixedPoint<T, FRAC_BITS>` is a transparent wrapper around
        // `T`.
        unsafe impl<const FRAC_BITS: usize> Scalar for FixedPoint<$T, FRAC_BITS> {
            type Repr = <$T as Scalar>::Repr;
        }

        impl<const FRAC_BITS: usize> Zero for FixedPoint<$T, FRAC_BITS> {
            const ZERO: Self = Self::ZERO;
        }

        impl<const FRAC_BITS: usize> Min for FixedPoint<$T, FRAC_BITS> {
            const MIN: Self = Self::MIN;
        }

        impl<const FRAC_BITS: usize> Max for FixedPoint<$T, FRAC_BITS> {
            const MAX: Self = Self::MAX;
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
