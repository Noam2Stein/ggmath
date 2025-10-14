use genco::quote;

use crate::{iter::Float, util::TokensExt};

pub fn generate(t: Float) {
    quote!(
        use crate::{Scalar, NonSimd, Simdness, Vector, primitive_api_trick};

        impl<const N: usize, S: Simdness> Vector<N, $t, S>
        where
            $t: Scalar<N>,
        {
            $("/// Returns a vector with the largest integer less than or equal to each element of `self`.")
            #[inline(always)]
            pub fn floor(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_floor(self))
            }

            $("/// Returns a vector with the smallest integer greater than or equal to each element of `self`.")
            #[inline(always)]
            pub fn ceil(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_ceil(self))
            }

            $("/// Returns a vector with the nearest integer to each element of `self`.")
            $("/// If an element is half-way between two integers, round away from `0.0`.")
            #[inline(always)]
            pub fn round(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_round(self))
            }

            $("/// Returns a vector with the integer part of each element of `self`.")
            $("/// This means that each element is rounded towards zero.")
            #[inline(always)]
            pub fn trunc(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_trunc(self))
            }

            $("/// Returns a vector with the fractional part of each element of `self`.")
            $("/// This is equivalent to `self - self.trunc()`.")
            #[inline(always)]
            pub fn fract(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_fract(self))
            }

            $("/// Fused multiply-add. Computes `(self * a) + b` with only one rounding error,")
            $("/// yielding a more accurate result than an unfused multiply-add.")
            $("///")
            $("/// Using `mul_add` *may* be more performant than an unfused multiply-add")
            $("/// if the target architecture has a dedicated `fma` CPU instruction.")
            $("/// However, this is not always true, and will be heavily dependent on designing")
            $("/// algorithms with specific target hardware in mind.")
            #[inline(always)]
            pub fn mul_add(self, a: Self, b: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_mul_add(self, a, b))
            }

            $("/// Calculates Euclidean division, the matching method for `rem_euclid`.")
            #[inline(always)]
            pub fn div_euclid(self, rhs: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_div_euclid(self, rhs))
            }

            $("/// Calculates the least nonnegative remainder of `self (mod rhs)`.")
            #[inline(always)]
            pub fn rem_euclid(self, rhs: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_rem_euclid(self, rhs))
            }

            $("/// Returns a vector with the square root of each element of `self`.")
            $("///")
            $("/// Returns `NaN` for elements that are negative numbers other than `-0.0`,")
            $("/// but only for those elements.")
            #[inline(always)]
            pub fn sqrt(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_sqrt(self))
            }

            $("/// Returns a vector with the sine of each element of `self`.")
            #[inline(always)]
            pub fn sin(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_sin(self))
            }

            $("/// Returns a vector with the cosine of each element of `self`.")
            #[inline(always)]
            pub fn cos(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_cos(self))
            }

            $("/// Returns a vector with the tangent of each element of `self`.")
            #[inline(always)]
            pub fn tan(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_tan(self))
            }

            $("/// Returns a vector with the arcsine of each element of `self`.")
            #[inline(always)]
            pub fn asin(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_asin(self))
            }

            $("/// Returns a vector with the arccosine of each element of `self`.")
            #[inline(always)]
            pub fn acos(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_acos(self))
            }

            $("/// Returns a vector with the arctangent of each element of `self`.")
            #[inline(always)]
            pub fn atan(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_atan(self))
            }

            $("/// Returns a vector with the reciprocal of each element of `self`, `1.0 / self`.")
            #[inline(always)]
            pub fn recip(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_recip(self))
            }

            $("/// Returns a vector with the maximum of each element of `self` and `other`.")
            #[inline(always)]
            pub fn max(self, other: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_max(self, other))
            }

            $("/// Returns a vector with the minimum of each element of `self` and `other`.")
            #[inline(always)]
            pub fn min(self, other: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_min(self, other))
            }

            $("/// Returns a vector with the midpoint of each element of `self` and `other`.")
            #[inline(always)]
            pub fn midpoint(self, other: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_midpoint(self, other))
            }

            $("/// Returns a vector with the clamp of each element of `self` between each element of `min` and `max`.")
            $("///")
            $("/// In release mode, semantics may differ if `min` is greater than `max` in any element,")
            $("/// or if `min` or `max` is `NaN`.")
            $("///")
            $("/// # Panics")
            $("///")
            $("/// In debug mode, this will panic if `min` is greater than `max` in any element,")
            $("/// or if `min` or `max` is `NaN`.")
            #[inline(always)]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_clamp(self, min, max))
            }

            $("/// Returns a vector with the absolute value of each element of `self`.")
            #[inline(always)]
            pub fn abs(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_abs(self))
            }
            
            $("/// Returns a vector with elements representing the sign of each element of `self`.")
            $("///")
            $("/// - `1.0` if an element is positive, `+0.0` or `INFINITY`")
            $("/// - `-1.0` if an element is negative, `-0.0` or `NEG_INFINITY`")
            $("/// - `NaN` if the element is `NaN`.")
            #[inline(always)]
            pub fn signum(self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_signum(self))
            }
            
            $("/// Returns a vector with the signs of `sign` and the magnitudes of `self`.")
            $("///")
            $("/// Sign of `0.0` is considered positive.")
            #[inline(always)]
            pub fn copysign(self, sign: Self) -> Self {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_copysign(self, sign))
            }

            $("/// Returns the sum of all elements of `self`.")
            $("/// Equivalent to `self.x + self.y + ..`.")
            #[inline(always)]
            pub fn sum(self) -> $t {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_sum(self))
            }

            $("/// Returns the product of all elements of `self`.")
            $("/// Equivalent to `self.x * self.y * ..`.")
            #[inline(always)]
            pub fn product(self) -> $t {
                primitive_api_trick!(<$t as $(t.camelcase())Api<N, S>>::vec_product(self))
            }
        }

        pub trait $(t.camelcase())Api<const N: usize, S: Simdness>
        where
            $t: Scalar<N>,
        {
            #[inline(always)]
            fn vec_floor(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::floor)
            }

            #[inline(always)]
            fn vec_ceil(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::ceil)
            }

            #[inline(always)]
            fn vec_round(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::round)
            }

            #[inline(always)]
            fn vec_trunc(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::trunc)
            }

            #[inline(always)]
            fn vec_fract(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::fract)
            }

            #[inline(always)]
            fn vec_mul_add(
                vec: Vector<N, $t, S>,
                a: Vector<N, $t, S>,
                b: Vector<N, $t, S>,
            ) -> Vector<N, $t, S> {
                Vector::from_fn(|i| $t::mul_add(vec[i], a[i], b[i]))
            }

            #[inline(always)]
            fn vec_div_euclid(vec: Vector<N, $t, S>, rhs: Vector<N, $t, S>) -> Vector<N, $t, S> {
                Vector::from_fn(|i| $t::div_euclid(vec[i], rhs[i]))
            }

            #[inline(always)]
            fn vec_rem_euclid(vec: Vector<N, $t, S>, rhs: Vector<N, $t, S>) -> Vector<N, $t, S> {
                Vector::from_fn(|i| $t::rem_euclid(vec[i], rhs[i]))
            }

            #[inline(always)]
            fn vec_sqrt(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::sqrt)
            }

            #[inline(always)]
            fn vec_sin(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::sin)
            }

            #[inline(always)]
            fn vec_cos(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::cos)
            }

            #[inline(always)]
            fn vec_tan(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::tan)
            }

            #[inline(always)]
            fn vec_asin(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::asin)
            }

            #[inline(always)]
            fn vec_acos(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::acos)
            }

            #[inline(always)]
            fn vec_atan(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::atan)
            }

            #[inline(always)]
            fn vec_recip(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::recip)
            }

            #[inline(always)]
            fn vec_max(vec: Vector<N, $t, S>, other: Vector<N, $t, S>) -> Vector<N, $t, S> {
                debug_assert!(vec.iter().all(|x| !x.is_nan()));
                debug_assert!(other.iter().all(|x| !x.is_nan()));

                Vector::from_fn(|i| $t::max(vec[i], other[i]))
            }

            #[inline(always)]
            fn vec_min(vec: Vector<N, $t, S>, other: Vector<N, $t, S>) -> Vector<N, $t, S> {
                debug_assert!(vec.iter().all(|x| !x.is_nan()));
                debug_assert!(other.iter().all(|x| !x.is_nan()));

                Vector::from_fn(|i| $t::min(vec[i], other[i]))
            }

            #[inline(always)]
            fn vec_midpoint(vec: Vector<N, $t, S>, other: Vector<N, $t, S>) -> Vector<N, $t, S> {
                Vector::from_fn(|i| $t::midpoint(vec[i], other[i]))
            }

            #[inline(always)]
            fn vec_clamp(
                vec: Vector<N, $t, S>,
                min: Vector<N, $t, S>,
                max: Vector<N, $t, S>,
            ) -> Vector<N, $t, S> {
                debug_assert!(min.iter().zip(max.iter()).all(|(min, max)| min <= max));
                debug_assert!(min.iter().all(|x| !x.is_nan()));
                debug_assert!(max.iter().all(|x| !x.is_nan()));

                Vector::from_fn(|i| $t::clamp(vec[i], min[i], max[i]))
            }

            #[inline(always)]
            fn vec_abs(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::abs)
            }

            #[inline(always)]
            fn vec_signum(vec: Vector<N, $t, S>) -> Vector<N, $t, S> {
                vec.map($t::signum)
            }

            #[inline(always)]
            fn vec_copysign(vec: Vector<N, $t, S>, sign: Vector<N, $t, S>) -> Vector<N, $t, S> {
                Vector::from_fn(|i| $t::copysign(vec[i], sign[i]))
            }

            #[inline(always)]
            fn vec_sum(vec: Vector<N, $t, S>) -> $t {
                vec.iter().sum()
            }

            #[inline(always)]
            fn vec_product(vec: Vector<N, $t, S>) -> $t {
                vec.iter().product()
            }
        }

        impl<const N: usize> $(t.camelcase())Api<N, NonSimd> for $t
        where
            $t: Scalar<N>,
        {
        }
    )
    .write_in_src(format!("vector/primitive_api/{t}.rs"));
}
