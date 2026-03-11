use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
};

use crate::{
    Scalar,
    constants::{Max, Min, Zero},
};

// SAFETY: `FixedI8` is a transparent wrapper around `i8`.
unsafe impl<Frac> Scalar for FixedI8<Frac> {
    type Repr = i8;
}

// SAFETY: `FixedI16` is a transparent wrapper around `i16`.
unsafe impl<Frac> Scalar for FixedI16<Frac> {
    type Repr = i16;
}

// SAFETY: `FixedI32` is a transparent wrapper around `i32`.
unsafe impl<Frac> Scalar for FixedI32<Frac> {
    type Repr = i32;
}

// SAFETY: `FixedI64` is a transparent wrapper around `i64`.
unsafe impl<Frac> Scalar for FixedI64<Frac> {
    type Repr = i64;
}

// SAFETY: `FixedI128` is a transparent wrapper around `i128`.
unsafe impl<Frac> Scalar for FixedI128<Frac> {
    type Repr = i128;
}

// SAFETY: `FixedU8` is a transparent wrapper around `u8`, which has the size of
// `i8`.
unsafe impl<Frac> Scalar for FixedU8<Frac> {
    type Repr = i8;
}

// SAFETY: `FixedU16` is a transparent wrapper around `u16`, which has the size
// of `i16`.
unsafe impl<Frac> Scalar for FixedU16<Frac> {
    type Repr = i16;
}

// SAFETY: `FixedU32` is a transparent wrapper around `u32`, which has the size
// of `i32`.
unsafe impl<Frac> Scalar for FixedU32<Frac> {
    type Repr = i32;
}

// SAFETY: `FixedU64` is a transparent wrapper around `u64`, which has the size
// of `i64`.
unsafe impl<Frac> Scalar for FixedU64<Frac> {
    type Repr = i64;
}

// SAFETY: `FixedU128` is a transparent wrapper around `u128`, which has the
// size of `i128`.
unsafe impl<Frac> Scalar for FixedU128<Frac> {
    type Repr = i128;
}

macro_rules! impl_constants {
    ($Fixed:ident) => {
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
impl_constants!(FixedI8);
impl_constants!(FixedI16);
impl_constants!(FixedI32);
impl_constants!(FixedI64);
impl_constants!(FixedI128);
impl_constants!(FixedU8);
impl_constants!(FixedU16);
impl_constants!(FixedU32);
impl_constants!(FixedU64);
impl_constants!(FixedU128);
