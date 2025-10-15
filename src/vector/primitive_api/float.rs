use crate::{
    NonSimd, ScalarNegOne, ScalarOne, ScalarZero, Simdness, SupportedVecLen, VecLen, Vector,
    vector::specialized_vector_api,
};

macro_rules! declare_float_api {
    ($T:ty: $TVectorApi:ident) => {
        impl<const N: usize, S: Simdness> Vector<N, $T, S>
        where
            VecLen<N>: SupportedVecLen,
        {
            specialized_vector_api! {
                $TVectorApi for <N, $T, S>:

                /// Returns a vector with the largest integer less than or equal to each
                /// element of `self`.
                pub fn floor(self) -> Self;

                /// Returns a vector with the smallest integer greater than or equal to each
                /// element of `self`.
                pub fn ceil(self) -> Self;

                /// Returns a vector with the nearest integer to each element of `self`.
                pub fn round(self) -> Self;

                /// Returns a vector with the integer part of each element of `self`.
                pub fn trunc(self) -> Self;

                /// Returns a vector with the fractional part of each element of `self`.
                /// Is equivalent to `self - self.trunc()`.
                pub fn fract(self) -> Self;

                /// Fused multiply-add. Computes (self * a) + b with only one rounding error,
                /// yielding a more accurate result than an unfused multiply-add.
                ///
                /// Using `mul_add` *may* be more performant than an unfused multiply-add if
                /// the target architecture has a dedicated `fma` CPU instruction. However,
                /// this is not always true and depends on designing algorithms with specific
                /// target hardware in mind.
                ///
                /// ## Precision
                /// The result of this operation is guaranteed to be the rounded
                /// infinite-precision result. It is specified by IEEE 754 as `fusedMultiplyAdd`
                /// and guaranteed not to change.
                pub fn mul_add(self, a: Self, b: Self) -> Self;

                /// Calculates Euclidean division following the standard library's definition.
                pub fn div_euclid(self, rhs: Self) -> Self;

                /// Calculates Euclidean remainder following the standard library's definition.
                pub fn rem_euclid(self, rhs: Self) -> Self;

                /// Calculates the square root of each element of `self`.
                pub fn sqrt(self) -> Self;

                /// Calculates the sine of each element of `self`.
                pub fn sin(self) -> Self;

                /// Calculates the cosine of each element of `self`.
                pub fn cos(self) -> Self;

                /// Calculates the tangent of each element of `self`.
                pub fn tan(self) -> Self;

                /// Calculates the arcsine of each element of `self`.
                pub fn asin(self) -> Self;

                /// Calculates the arccosine of each element of `self`.
                pub fn acos(self) -> Self;

                /// Calculates the arctangent of each element of `self`.
                pub fn atan(self) -> Self;

                /// Calculates the reciprocal (`1.0 / x`) of each element of `self`.
                pub fn recip(self) -> Self;

                /// Returns the largest of each element of `self` and `other`.
                ///
                /// This function may be inconsistent with the standard library's definition
                /// in regards to `-0.0`, and NaN in release mode.
                ///
                /// ## Panics
                ///
                /// Panics *in debug mode* if either `self` or `other` contains NaN.
                pub fn max(self, other: Self) -> Self;

                /// Returns the smallest of each element of `self` and `other`.
                ///
                /// This function may be inconsistent with the standard library's definition
                /// in regards to `-0.0`, and NaN in release mode.
                ///
                /// ## Panics
                ///
                /// Panics *in debug mode* if either `self` or `other` contains NaN.
                pub fn min(self, other: Self) -> Self;

                /// Calculates the midpoint (average) of each element of `self` and `other`.
                ///
                /// This function may be imprecise for very large values.
                pub fn midpoint(self, other: Self) -> Self;

                /// Clamps each element of `self` to the range `[min, max]`.
                ///
                /// This function may be inconsistent with the standard library's definition
                /// in regards to `-0.0`, and NaN in release mode.
                ///
                /// ## Panics
                ///
                /// Panics *in debug mode* if either `self`, `min`, or `max` contains NaN,
                /// or if `min` is greater than `max`.
                pub fn clamp(self, min: Self, max: Self) -> Self;

                /// Returns a vector with the absolute value of each element of `self`.
                pub fn abs(self) -> Self;

                /// Returns a vector with the signum of each element of `self`.
                pub fn signum(self) -> Self;

                /// Returns a vector with the same elements as `self`, but with the sign of each element replaced with the sign of `sign`.
                pub fn copysign(self, sign: Self) -> Self;

                /// Returns the sum of all elements of `self`.
                ///
                /// This function may be imprecise for large values.
                pub fn sum(self) -> $T;

                /// Returns the product of all elements of `self`.
                ///
                /// This function may be imprecise for large values.
                pub fn product(self) -> $T;
            }
        }

        impl ScalarZero for $T {
            const ZERO: Self = 0.0;
        }

        impl ScalarOne for $T {
            const ONE: Self = 1.0;
        }

        impl ScalarNegOne for $T {
            const NEG_ONE: Self = -1.0;
        }

        pub(in crate::vector) trait $TVectorApi<const N: usize, S: Simdness>
        where
            VecLen<N>: SupportedVecLen,
        {
            #[inline(always)]
            fn vec_floor(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::floor)
            }

            #[inline(always)]
            fn vec_ceil(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::ceil)
            }

            #[inline(always)]
            fn vec_round(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::round)
            }

            #[inline(always)]
            fn vec_trunc(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::trunc)
            }

            #[inline(always)]
            fn vec_fract(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::fract)
            }

            #[inline(always)]
            fn vec_mul_add(
                vec: Vector<N, $T, S>,
                a: Vector<N, $T, S>,
                b: Vector<N, $T, S>,
            ) -> Vector<N, $T, S> {
                Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
            }

            #[inline(always)]
            fn vec_div_euclid(vec: Vector<N, $T, S>, rhs: Vector<N, $T, S>) -> Vector<N, $T, S> {
                Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
            }

            #[inline(always)]
            fn vec_rem_euclid(vec: Vector<N, $T, S>, rhs: Vector<N, $T, S>) -> Vector<N, $T, S> {
                Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
            }

            #[inline(always)]
            fn vec_sqrt(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::sqrt)
            }

            #[inline(always)]
            fn vec_sin(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::sin)
            }

            #[inline(always)]
            fn vec_cos(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::cos)
            }

            #[inline(always)]
            fn vec_tan(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::tan)
            }

            #[inline(always)]
            fn vec_asin(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::asin)
            }

            #[inline(always)]
            fn vec_acos(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::acos)
            }

            #[inline(always)]
            fn vec_atan(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::atan)
            }

            #[inline(always)]
            fn vec_recip(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::recip)
            }

            #[inline(always)]
            fn vec_max(vec: Vector<N, $T, S>, other: Vector<N, $T, S>) -> Vector<N, $T, S> {
                debug_assert!(!vec.iter().any(<$T>::is_nan));
                debug_assert!(!other.iter().any(<$T>::is_nan));

                Vector::from_fn(|i| vec[i].max(other[i]))
            }

            #[inline(always)]
            fn vec_min(vec: Vector<N, $T, S>, other: Vector<N, $T, S>) -> Vector<N, $T, S> {
                debug_assert!(!vec.iter().any(<$T>::is_nan));
                debug_assert!(!other.iter().any(<$T>::is_nan));

                Vector::from_fn(|i| vec[i].min(other[i]))
            }

            #[inline(always)]
            fn vec_midpoint(vec: Vector<N, $T, S>, other: Vector<N, $T, S>) -> Vector<N, $T, S> {
                Vector::from_fn(|i| vec[i].midpoint(other[i]))
            }

            #[inline(always)]
            fn vec_clamp(
                vec: Vector<N, $T, S>,
                min: Vector<N, $T, S>,
                max: Vector<N, $T, S>,
            ) -> Vector<N, $T, S> {
                debug_assert!(!vec.iter().any(<$T>::is_nan));
                debug_assert!(!min.iter().any(<$T>::is_nan));
                debug_assert!(!max.iter().any(<$T>::is_nan));
                debug_assert!(min.iter().zip(max).all(|(min, max)| min <= max));

                Vector::from_fn(|i| vec[i].clamp(min[i], max[i]))
            }

            #[inline(always)]
            fn vec_abs(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::abs)
            }

            #[inline(always)]
            fn vec_signum(vec: Vector<N, $T, S>) -> Vector<N, $T, S> {
                vec.map(<$T>::signum)
            }

            #[inline(always)]
            fn vec_copysign(vec: Vector<N, $T, S>, sign: Vector<N, $T, S>) -> Vector<N, $T, S> {
                Vector::from_fn(|i| vec[i].copysign(sign[i]))
            }

            #[inline(always)]
            fn vec_sum(vec: Vector<N, $T, S>) -> $T {
                vec.as_array().iter().sum()
            }

            #[inline(always)]
            fn vec_product(vec: Vector<N, $T, S>) -> $T {
                vec.as_array().iter().product()
            }
        }

        impl<const N: usize> $TVectorApi<N, NonSimd> for $T where VecLen<N>: SupportedVecLen {}
    };
}

declare_float_api!(f32: F32VectorApi);
declare_float_api!(f64: F64VectorApi);
