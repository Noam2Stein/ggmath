use core::mem::transmute;

use wide::{
    f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8,
    i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2,
    u64x4, u64x8,
};

use crate::{
    Scalar,
    constants::{Infinity, Max, Min, Nan, NegInfinity, NegOne, One, Zero},
};

macro_rules! float_impl {
    ($T:ident, $F:ident, $N:literal) => {
        // SAFETY: Scalar implementations where `Repr = ()` are safe.
        unsafe impl Scalar for $T {
            type Repr = ();
        }

        impl Zero for $T {
            const ZERO: Self = unsafe { transmute::<[$F; $N], $T>([0.0; $N]) };
        }

        impl One for $T {
            const ONE: Self = unsafe { transmute::<[$F; $N], $T>([1.0; $N]) };
        }

        impl NegOne for $T {
            const NEG_ONE: Self = unsafe { transmute::<[$F; $N], $T>([-1.0; $N]) };
        }

        impl Min for $T {
            const MIN: Self = unsafe { transmute::<[$F; $N], $T>([$F::MIN; $N]) };
        }

        impl Max for $T {
            const MAX: Self = unsafe { transmute::<[$F; $N], $T>([$F::MAX; $N]) };
        }

        impl Nan for $T {
            const NAN: Self = unsafe { transmute::<[$F; $N], $T>([$F::MAX; $N]) };
        }

        impl Infinity for $T {
            const INFINITY: Self = unsafe { transmute::<[$F; $N], $T>([$F::INFINITY; $N]) };
        }

        impl NegInfinity for $T {
            const NEG_INFINITY: Self = unsafe { transmute::<[$F; $N], $T>([$F::NEG_INFINITY; $N]) };
        }
    };
}
float_impl!(f32x4, f32, 4);
float_impl!(f32x8, f32, 8);
float_impl!(f32x16, f32, 16);
float_impl!(f64x2, f64, 2);
float_impl!(f64x4, f64, 4);
float_impl!(f64x8, f64, 8);

macro_rules! int_impl {
    ($T:ident) => {
        // SAFETY: Scalar implementations where `Repr = ()` are safe.
        unsafe impl Scalar for $T {
            type Repr = ();
        }

        impl Zero for $T {
            const ZERO: Self = Self::ZERO;
        }

        impl One for $T {
            const ONE: Self = Self::ONE;
        }

        impl NegOne for $T {
            const NEG_ONE: Self = Self::new([-1; _]);
        }

        impl Min for $T {
            const MIN: Self = Self::MIN;
        }

        impl Max for $T {
            const MAX: Self = Self::MAX;
        }
    };
}
int_impl!(i8x16);
int_impl!(i8x32);
int_impl!(i16x8);
int_impl!(i16x16);
int_impl!(i16x32);
int_impl!(i32x4);
int_impl!(i32x8);
int_impl!(i32x16);
int_impl!(i64x2);
int_impl!(i64x4);
int_impl!(i64x8);

macro_rules! uint_impl {
    ($T:ident) => {
        // SAFETY: Scalar implementations where `Repr = ()` are safe.
        unsafe impl Scalar for $T {
            type Repr = ();
        }

        impl Zero for $T {
            const ZERO: Self = Self::ZERO;
        }

        impl One for $T {
            const ONE: Self = Self::ONE;
        }

        impl Min for $T {
            const MIN: Self = Self::MIN;
        }

        impl Max for $T {
            const MAX: Self = Self::MAX;
        }
    };
}
uint_impl!(u8x16);
uint_impl!(u8x32);
uint_impl!(u16x8);
uint_impl!(u16x16);
uint_impl!(u16x32);
uint_impl!(u32x4);
uint_impl!(u32x8);
uint_impl!(u32x16);
uint_impl!(u64x2);
uint_impl!(u64x4);
uint_impl!(u64x8);
