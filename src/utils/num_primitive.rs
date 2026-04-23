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

/// A trait for all primitive float-point types.
///
/// This is modeled after [`num-primitive`], but only includes functionality
/// needed for [`Vector<N, T, A>`], [`Matrix<N, T, A>`] and
/// [`Quaternion<T, A>`].
///
/// Ideally, there would be [`PrimitiveSigned`] and [`PrimitiveUnsigned`] traits
/// as well, but the type system cannot currently express mutually exclusive
/// traits, so multiple implementations that are generic over `T` are
/// impossible.
///
/// [`num-primitive`]: https://crates.io/crates/num-primitive
/// [`Vector<N, T, A>`]: crate::Vector
/// [`Matrix<N, T, A>`]: crate::Matrix
/// [`Quaternion<T, A>`]: crate::Quaternion
/// [`PrimitiveSigned`]: https://docs.rs/num-primitive/latest/num_primitive/trait.PrimitiveSigned.html
/// [`PrimitiveUnsigned`]: https://docs.rs/num-primitive/latest/num_primitive/trait.PrimitiveUnsigned.html
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
    const EPSILON: Self;

    fn is_nan(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn recip(self) -> Self;

    fn clamp(self, min: Self, max: Self) -> Self;

    fn abs(self) -> Self;

    fn signum(self) -> Self;

    fn copysign(self, sign: Self) -> Self;

    fn floor(self) -> Self;

    fn ceil(self) -> Self;

    fn round(self) -> Self;

    fn trunc(self) -> Self;

    fn mul_add(self, a: Self, b: Self) -> Self;

    fn div_euclid(self, rhs: Self) -> Self;

    fn rem_euclid(self, rhs: Self) -> Self;

    fn powf(self, n: Self) -> Self;

    fn sqrt(self) -> Self;

    fn exp(self) -> Self;

    fn exp2(self) -> Self;

    fn ln(self) -> Self;

    fn log2(self) -> Self;

    fn sin(self) -> Self;

    fn cos(self) -> Self;

    fn tan(self) -> Self;

    fn asin(self) -> Self;

    fn acos(self) -> Self;

    fn atan(self) -> Self;

    fn atan2(self, other: Self) -> Self;

    fn sin_cos(self) -> (Self, Self);

    fn as_from(value: f64) -> Self;
}

macro_rules! impl_primitive_float {
    ($T:ident) => {
        impl PrimitiveFloat for $T {
            const EPSILON: Self = Self::EPSILON;

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
            fn clamp(self, min: Self, max: Self) -> Self {
                self.clamp(min, max)
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

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn floor(self) -> Self {
                self.floor()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn floor(self) -> Self {
                Libm::<$T>::floor(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn ceil(self) -> Self {
                self.ceil()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn ceil(self) -> Self {
                Libm::<$T>::ceil(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn round(self) -> Self {
                self.round()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn round(self) -> Self {
                Libm::<$T>::round(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn trunc(self) -> Self {
                self.trunc()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn trunc(self) -> Self {
                Libm::<$T>::trunc(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                self.mul_add(a, b)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn mul_add(self, a: Self, b: Self) -> Self {
                Libm::<$T>::fma(self, a, b)
            }

            #[cfg(not(feature = "libm"))]
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

            #[cfg(not(feature = "libm"))]
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

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                self.powf(n)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn powf(self, n: Self) -> Self {
                Libm::<$T>::pow(self, n)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn sqrt(self) -> Self {
                self.sqrt()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn sqrt(self) -> Self {
                Libm::<$T>::sqrt(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn exp(self) -> Self {
                self.exp()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn exp(self) -> Self {
                Libm::<$T>::exp(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn exp2(self) -> Self {
                self.exp2()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn exp2(self) -> Self {
                Libm::<$T>::exp2(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn ln(self) -> Self {
                self.ln()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn ln(self) -> Self {
                Libm::<$T>::log(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn log2(self) -> Self {
                self.log2()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn log2(self) -> Self {
                Libm::<$T>::log2(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn sin(self) -> Self {
                self.sin()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn sin(self) -> Self {
                Libm::<$T>::sin(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn cos(self) -> Self {
                self.cos()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn cos(self) -> Self {
                Libm::<$T>::cos(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn tan(self) -> Self {
                self.tan()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn tan(self) -> Self {
                Libm::<$T>::tan(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn asin(self) -> Self {
                self.asin()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn asin(self) -> Self {
                Libm::<$T>::asin(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn acos(self) -> Self {
                self.acos()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn acos(self) -> Self {
                Libm::<$T>::acos(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn atan(self) -> Self {
                self.atan()
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn atan(self) -> Self {
                Libm::<$T>::atan(self)
            }

            #[cfg(not(feature = "libm"))]
            #[inline(always)]
            fn atan2(self, other: Self) -> Self {
                self.atan2(other)
            }

            #[cfg(feature = "libm")]
            #[inline(always)]
            fn atan2(self, other: Self) -> Self {
                Libm::<$T>::atan2(self, other)
            }

            #[cfg(not(feature = "libm"))]
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
