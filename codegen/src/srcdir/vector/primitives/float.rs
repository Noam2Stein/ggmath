use genco::quote;

use crate::{iter::Float, util::TokensExt};

pub fn generate(t: Float) {
    quote!(
        use crate::{FloatElementOfVector, Simdness, Vector};

        impl<const N: usize, S: Simdness> Vector<N, $t, S>
        where
            $t: FloatElementOfVector<N, S>,
        {
            $("/// Returns a vector with the largest integer less than or equal to each element of `self`.")
            #[inline(always)]
            pub fn floor(self) -> Self {
                $t::vec_floor(self)
            }

            $("/// Returns a vector with the smallest integer greater than or equal to each element of `self`.")
            #[inline(always)]
            pub fn ceil(self) -> Self {
                $t::vec_ceil(self)
            }

            $("/// Returns a vector with the nearest integer to each element of `self`.")
            $("/// If an element is half-way between two integers, round away from `0.0`.")
            #[inline(always)]
            pub fn round(self) -> Self {
                $t::vec_round(self)
            }

            $("/// Returns a vector with the integer part of each element of `self`.")
            $("/// This means that each element is rounded towards zero.")
            #[inline(always)]
            pub fn trunc(self) -> Self {
                $t::vec_trunc(self)
            }

            $("/// Returns a vector with the fractional part of each element of `self`.")
            $("/// This is equivalent to `self - self.trunc()`.")
            #[inline(always)]
            pub fn fract(self) -> Self {
                $t::vec_fract(self)
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
                $t::vec_mul_add(self, a, b)
            }

            $("/// Calculates Euclidean division, the matching method for `rem_euclid`.")
            #[inline(always)]
            pub fn div_euclid(self, rhs: Self) -> Self {
                $t::vec_div_euclid(self, rhs)
            }

            $("/// Calculates the least nonnegative remainder of `self (mod rhs)`.")
            #[inline(always)]
            pub fn rem_euclid(self, rhs: Self) -> Self {
                $t::vec_rem_euclid(self, rhs)
            }

            $("/// Returns a vector with the square root of each element of `self`.")
            $("///")
            $("/// Returns `NaN` for elements that are negative numbers other than `-0.0`,")
            $("/// but only for those elements.")
            #[inline(always)]
            pub fn sqrt(self) -> Self {
                $t::vec_sqrt(self)
            }

            $("/// Returns a vector with the sine of each element of `self`.")
            #[inline(always)]
            pub fn sin(self) -> Self {
                $t::vec_sin(self)
            }

            $("/// Returns a vector with the cosine of each element of `self`.")
            #[inline(always)]
            pub fn cos(self) -> Self {
                $t::vec_cos(self)
            }

            $("/// Returns a vector with the tangent of each element of `self`.")
            #[inline(always)]
            pub fn tan(self) -> Self {
                $t::vec_tan(self)
            }

            $("/// Returns a vector with the arcsine of each element of `self`.")
            #[inline(always)]
            pub fn asin(self) -> Self {
                $t::vec_asin(self)
            }

            $("/// Returns a vector with the arccosine of each element of `self`.")
            #[inline(always)]
            pub fn acos(self) -> Self {
                $t::vec_acos(self)
            }

            $("/// Returns a vector with the arctangent of each element of `self`.")
            #[inline(always)]
            pub fn atan(self) -> Self {
                $t::vec_atan(self)
            }

            $("/// Returns a vector with the reciprocal of each element of `self`, `1.0 / self`.")
            #[inline(always)]
            pub fn recip(self) -> Self {
                $t::vec_recip(self)
            }

            $("/// Returns a vector with the maximum of each element of `self` and `other`.")
            #[inline(always)]
            pub fn max(self, other: Self) -> Self {
                $t::vec_max(self, other)
            }

            $("/// Returns a vector with the minimum of each element of `self` and `other`.")
            #[inline(always)]
            pub fn min(self, other: Self) -> Self {
                $t::vec_min(self, other)
            }

            $("/// Returns a vector with the midpoint of each element of `self` and `other`.")
            #[inline(always)]
            pub fn midpoint(self, other: Self) -> Self {
                $t::vec_midpoint(self, other)
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
                $t::vec_clamp(self, min, max)
            }

            $("/// Returns a vector with the absolute value of each element of `self`.")
            #[inline(always)]
            pub fn abs(self) -> Self {
                $t::vec_abs(self)
            }
            
            $("/// Returns a vector with elements representing the sign of each element of `self`.")
            $("///")
            $("/// - `1.0` if an element is positive, `+0.0` or `INFINITY`")
            $("/// - `-1.0` if an element is negative, `-0.0` or `NEG_INFINITY`")
            $("/// - `NaN` if the element is `NaN`.")
            #[inline(always)]
            pub fn signum(self) -> Self {
                $t::vec_signum(self)
            }
            
            $("/// Returns a vector with the signs of `sign` and the magnitudes of `self`.")
            $("///")
            $("/// Sign of `0.0` is considered positive.")
            #[inline(always)]
            pub fn copysign(self, sign: Self) -> Self {
                $t::vec_copysign(self, sign)
            }

            $("/// Returns the sum of all elements of `self`.")
            $("/// Equivalent to `self.x + self.y + ..`.")
            #[inline(always)]
            pub fn sum(self) -> $t {
                $t::vec_sum(self)
            }

            $("/// Returns the product of all elements of `self`.")
            $("/// Equivalent to `self.x * self.y * ..`.")
            #[inline(always)]
            pub fn product(self) -> $t {
                $t::vec_product(self)
            }
        }
    )
    .write_in_src(format!("vector/primitives/{t}.rs"));
}
