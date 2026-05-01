use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
};

use crate::{
    Alignment, DefaultBackend, Scalar,
    constants::{Max, Min, Zero},
};

macro_rules! impl_fixed {
    ($Fixed:ident) => {
        impl<Frac> Scalar for $Fixed<Frac> {}

        impl<const N: usize, Frac, A: Alignment> DefaultBackend<N, A> for $Fixed<Frac> {}

        impl<Frac> Zero for $Fixed<Frac> {
            const ZERO: Self = Self::ZERO;
        }

        impl<Frac> Min for $Fixed<Frac> {
            const MIN: Self = Self::MIN;
        }

        impl<Frac> Max for $Fixed<Frac> {
            const MAX: Self = Self::MAX;
        }
    };
}
impl_fixed!(FixedI8);
impl_fixed!(FixedI16);
impl_fixed!(FixedI32);
impl_fixed!(FixedI64);
impl_fixed!(FixedI128);
impl_fixed!(FixedU8);
impl_fixed!(FixedU16);
impl_fixed!(FixedU32);
impl_fixed!(FixedU64);
impl_fixed!(FixedU128);
