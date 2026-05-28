#[cfg(feature = "libm")]
use libm::Libm;

use crate::PrimitiveUnsigned;

/// An internal trait that wraps primitive-float functions used in generic
/// contexts.
pub(crate) trait PrimitiveFloatFns: Sized {
    type U: PrimitiveUnsigned;

    const EPSILON: Self;
    const PI: Self;
    #[cfg(test)]
    const TAU: Self;

    #[expect(clippy::wrong_self_convention)]
    fn is_nan(self) -> bool;

    #[expect(clippy::wrong_self_convention)]
    fn is_finite(self) -> bool;

    #[expect(clippy::wrong_self_convention)]
    fn is_sign_positive(self) -> bool;

    #[expect(clippy::wrong_self_convention)]
    fn is_sign_negative(self) -> bool;

    fn recip(self) -> Self;

    fn max(self, other: Self) -> Self;

    fn min(self, other: Self) -> Self;

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

/// An internal trait that wraps primitive-integer functions used in generic
/// contexts.
pub(crate) trait PrimitiveIntegerFns: Sized {
    fn checked_add(self, rhs: Self) -> Option<Self>;

    fn checked_sub(self, rhs: Self) -> Option<Self>;

    fn checked_mul(self, rhs: Self) -> Option<Self>;

    fn checked_div(self, rhs: Self) -> Option<Self>;

    fn checked_rem(self, rhs: Self) -> Option<Self>;

    fn saturating_add(self, rhs: Self) -> Self;

    fn saturating_sub(self, rhs: Self) -> Self;

    fn saturating_mul(self, rhs: Self) -> Self;

    fn saturating_div(self, rhs: Self) -> Self;

    fn wrapping_add(self, rhs: Self) -> Self;

    fn wrapping_sub(self, rhs: Self) -> Self;

    fn wrapping_mul(self, rhs: Self) -> Self;

    fn wrapping_div(self, rhs: Self) -> Self;

    fn wrapping_rem(self, rhs: Self) -> Self;
}

/// An internal trait that wraps primitive-signed-integer functions used in
/// generic contexts.
pub(crate) trait PrimitiveSignedFns: Sized {
    fn abs(self) -> Self;

    fn signum(self) -> Self;
}

macro_rules! impl_float {
    ($T:ident, $UnsignedT:ident) => {
        impl PrimitiveFloatFns for $T {
            type U = $UnsignedT;

            const EPSILON: Self = Self::EPSILON;
            const PI: Self = core::$T::consts::PI;
            #[cfg(test)]
            const TAU: Self = core::$T::consts::TAU;

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
            fn max(self, other: Self) -> Self {
                self.max(other)
            }

            #[inline(always)]
            fn min(self, other: Self) -> Self {
                self.min(other)
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
impl_float!(f32, u32);
impl_float!(f64, u64);

macro_rules! impl_integer {
    ($T:ident) => {
        impl PrimitiveIntegerFns for $T {
            #[inline(always)]
            fn checked_add(self, rhs: Self) -> Option<Self> {
                self.checked_add(rhs)
            }

            #[inline(always)]
            fn checked_sub(self, rhs: Self) -> Option<Self> {
                self.checked_sub(rhs)
            }

            #[inline(always)]
            fn checked_mul(self, rhs: Self) -> Option<Self> {
                self.checked_mul(rhs)
            }

            #[inline(always)]
            fn checked_div(self, rhs: Self) -> Option<Self> {
                self.checked_div(rhs)
            }

            #[inline(always)]
            fn checked_rem(self, rhs: Self) -> Option<Self> {
                self.checked_rem(rhs)
            }

            #[inline(always)]
            fn saturating_add(self, rhs: Self) -> Self {
                self.saturating_add(rhs)
            }

            #[inline(always)]
            fn saturating_sub(self, rhs: Self) -> Self {
                self.saturating_sub(rhs)
            }

            #[inline(always)]
            fn saturating_mul(self, rhs: Self) -> Self {
                self.saturating_mul(rhs)
            }

            #[inline(always)]
            fn saturating_div(self, rhs: Self) -> Self {
                self.saturating_div(rhs)
            }

            #[inline(always)]
            fn wrapping_add(self, rhs: Self) -> Self {
                self.wrapping_add(rhs)
            }

            #[inline(always)]
            fn wrapping_sub(self, rhs: Self) -> Self {
                self.wrapping_sub(rhs)
            }

            #[inline(always)]
            fn wrapping_mul(self, rhs: Self) -> Self {
                self.wrapping_mul(rhs)
            }

            #[inline(always)]
            fn wrapping_div(self, rhs: Self) -> Self {
                self.wrapping_div(rhs)
            }

            #[inline(always)]
            fn wrapping_rem(self, rhs: Self) -> Self {
                self.wrapping_rem(rhs)
            }
        }
    };
}
impl_integer!(i8);
impl_integer!(i16);
impl_integer!(i32);
impl_integer!(i64);
impl_integer!(i128);
impl_integer!(isize);
impl_integer!(u8);
impl_integer!(u16);
impl_integer!(u32);
impl_integer!(u64);
impl_integer!(u128);
impl_integer!(usize);

macro_rules! impl_signed {
    ($T:ident) => {
        impl PrimitiveSignedFns for $T {
            #[inline(always)]
            fn abs(self) -> Self {
                self.abs()
            }

            #[inline(always)]
            fn signum(self) -> Self {
                self.signum()
            }
        }
    };
}
impl_signed!(i8);
impl_signed!(i16);
impl_signed!(i32);
impl_signed!(i64);
impl_signed!(i128);
impl_signed!(isize);
