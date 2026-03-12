use core::{
    fmt::Debug,
    iter::{Product, Sum},
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

#[cfg(feature = "libm")]
use libm::Libm;

use crate::{
    Aligned, PrimitiveFloatBackend, Unaligned,
    constants::{Infinity, Max, Min, Nan, NegInfinity, NegOne, One, Zero},
};

pub(crate) trait PrimitiveFloat:
    Debug
    + Clone
    + Copy
    + PartialEq
    + PartialOrd
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Rem<Output = Self>
    + Sum
    + Product
    + Zero
    + One
    + NegOne
    + Min
    + Max
    + Nan
    + Infinity
    + NegInfinity
    + PrimitiveFloatBackend<2, Aligned>
    + PrimitiveFloatBackend<3, Aligned>
    + PrimitiveFloatBackend<4, Aligned>
    + PrimitiveFloatBackend<2, Unaligned>
    + PrimitiveFloatBackend<3, Unaligned>
    + PrimitiveFloatBackend<4, Unaligned>
{
    fn is_nan(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn recip(self) -> Self;

    fn abs(self) -> Self;

    fn signum(self) -> Self;

    fn copysign(self, sign: Self) -> Self;

    #[cfg(backend)]
    fn floor(self) -> Self;

    #[cfg(backend)]
    fn ceil(self) -> Self;

    #[cfg(backend)]
    fn round(self) -> Self;

    #[cfg(backend)]
    fn trunc(self) -> Self;

    #[cfg(backend)]
    fn mul_add(self, a: Self, b: Self) -> Self;

    #[cfg(backend)]
    fn div_euclid(self, rhs: Self) -> Self;

    #[cfg(backend)]
    fn rem_euclid(self, rhs: Self) -> Self;

    #[cfg(backend)]
    fn powf(self, n: Self) -> Self;

    #[cfg(backend)]
    fn sqrt(self) -> Self;

    #[cfg(backend)]
    fn exp(self) -> Self;

    #[cfg(backend)]
    fn exp2(self) -> Self;

    #[cfg(backend)]
    fn ln(self) -> Self;

    #[cfg(backend)]
    fn log2(self) -> Self;

    #[cfg(backend)]
    fn sin(self) -> Self;

    #[cfg(backend)]
    fn cos(self) -> Self;

    #[cfg(backend)]
    fn tan(self) -> Self;

    #[cfg(backend)]
    fn asin(self) -> Self;

    #[cfg(backend)]
    fn acos(self) -> Self;

    #[cfg(backend)]
    fn atan(self) -> Self;

    #[cfg(backend)]
    fn sin_cos(self) -> (Self, Self);

    fn as_from(value: f64) -> Self;
}

macro_rules! impl_primitive_float {
    ($T:ident) => {
        impl PrimitiveFloat for $T {
            #[inline(always)]
            fn is_nan(self) -> bool {
                self.is_nan()
            }

            #[inline(always)]
            fn is_finite(self) -> bool {
                self.is_finite()
            }

            #[inline(always)]
            fn is_sign_positive(self) -> bool {
                self.is_sign_positive()
            }

            #[inline(always)]
            fn is_sign_negative(self) -> bool {
                self.is_sign_negative()
            }

            #[inline(always)]
            fn recip(self) -> Self {
                self.recip()
            }

            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline(always)]
            fn signum(self) -> Self {
                self.signum()
            }

            #[inline(always)]
            fn copysign(self, sign: Self) -> Self {
                self.copysign(sign)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn floor(self) -> Self {
                self.floor()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn floor(self) -> Self {
                Libm::<$T>::floor(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn ceil(self) -> Self {
                Libm::<$T>::ceil(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn round(self) -> Self {
                self.round()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn round(self) -> Self {
                Libm::<$T>::round(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn trunc(self) -> Self {
                self.trunc()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn trunc(self) -> Self {
                Libm::<$T>::trunc(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                Libm::<$T>::fma(self, a, b)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn div_euclid(self, rhs: Self) -> Self {
                self.div_euclid(rhs)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn div_euclid(self, rhs: Self) -> Self {
                // Based on https://doc.rust-lang.org/src/std/f32.rs.html#293
                let q = (self / rhs).trunc();
                if self % rhs < 0.0 {
                    return if rhs > 0.0 { q - 1.0 } else { q + 1.0 };
                }

                q
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn rem_euclid(self, rhs: Self) -> Self {
                self.rem_euclid(rhs)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn rem_euclid(self, rhs: Self) -> Self {
                let r = self % rhs;

                if r < 0.0 { r + rhs.abs() } else { r }
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                Libm::<$T>::pow(self, n)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn sqrt(self) -> Self {
                self.sqrt()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn sqrt(self) -> Self {
                Libm::<$T>::sqrt(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn exp(self) -> Self {
                self.exp()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn exp(self) -> Self {
                Libm::<$T>::exp(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn exp2(self) -> Self {
                self.exp2()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn exp2(self) -> Self {
                Libm::<$T>::exp2(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn ln(self) -> Self {
                self.ln()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn ln(self) -> Self {
                Libm::<$T>::log(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn log2(self) -> Self {
                self.log2()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn log2(self) -> Self {
                Libm::<$T>::log2(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn sin(self) -> Self {
                self.sin()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn sin(self) -> Self {
                Libm::<$T>::sin(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn cos(self) -> Self {
                self.cos()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn cos(self) -> Self {
                Libm::<$T>::cos(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn tan(self) -> Self {
                self.tan()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn tan(self) -> Self {
                Libm::<$T>::tan(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn asin(self) -> Self {
                self.asin()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn asin(self) -> Self {
                Libm::<$T>::asin(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn acos(self) -> Self {
                self.acos()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn acos(self) -> Self {
                Libm::<$T>::acos(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn atan(self) -> Self {
                self.atan()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn atan(self) -> Self {
                Libm::<$T>::atan(self)
            }

            #[cfg(all(feature = "std", not(feature = "libm")))]
            #[inline(always)]
            fn sin_cos(self) -> (Self, Self) {
                self.sin_cos()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn sin_cos(self) -> (Self, Self) {
                Libm::<$T>::sincos(self)
            }

            #[inline(always)]
            fn as_from(value: f64) -> Self {
                value as Self
            }
        }
    };
}
impl_primitive_float!(f32);
impl_primitive_float!(f64);
