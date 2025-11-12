use crate::{
    ScalarNegOne, ScalarOne, ScalarZero, SupportedVecLen, VecLen, Vector, vector::specialize,
};

impl<const N: usize> Vector<N, f>
where
    VecLen<N>: SupportedVecLen,
{
    /// Returns the elements of `self` rounded towards `-infinity`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn floor(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_floor(self))
    }

    /// Returns the elements of `self` rounded towards `+infinity`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn ceil(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_ceil(self))
    }

    /// Returns the elements of `self` rounded to the nearest integer.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn round(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_round(self))
    }

    /// Returns the integer part of `self`. Is equivalent to rounding the elements
    /// of `self` towards `0`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn trunc(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_trunc(self))
    }

    /// Returns the fractional part of `self`. Is equivalent to
    /// `self - self.trunc()`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn fract(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_fract(self))
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise.
    ///
    /// This saves 1 rounding error that would occur in `self * a + b`,
    /// but is usually slower than `self * a + b`.
    ///
    /// The precision of the result is guaranteed to follow `std`'s
    /// `mul_add`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_mul_add(self, a, b))
    }

    /// element-wise Euclidean division
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_div_euclid(self, rhs))
    }

    /// element-wise Euclidean remainder
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_rem_euclid(self, rhs))
    }

    /// element-wise square root
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sqrt(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_sqrt(self))
    }

    /// element-wise sine
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sin(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_sin(self))
    }

    /// element-wise cosine
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn cos(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_cos(self))
    }

    /// element-wise tangent
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn tan(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_tan(self))
    }

    /// element-wise arcsine
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn asin(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_asin(self))
    }

    /// element-wise arccosine
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn acos(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_acos(self))
    }

    /// element-wise arctangent
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn atan(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_atan(self))
    }

    /// Returns `1.0 / self`.
    #[inline(always)]
    pub fn recip(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_recip(self))
    }

    /// Returns the maximum of each element of `self` and `other`.
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_max(self, other))
    }

    /// Returns the minimum of each element of `self` and `other`.
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_min(self, other))
    }

    /// Returns the middle between `self` and `other`.
    #[inline(always)]
    pub fn midpoint(self, other: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_midpoint(self, other))
    }

    /// Clamps each element of `self` between `min` and `max`.
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_clamp(self, min, max))
    }

    /// Returns the absolute value of each element of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_abs(self))
    }

    /// Returns the sign of each element of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_signum(self))
    }

    /// Returns the magnitudes of `self` with the signs of `sign`.
    #[inline(always)]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<f as FloatBackend<N>>::vec_copysign(self, sign))
    }

    /// Returns the sum of all elements in `self`.
    #[inline(always)]
    pub fn element_sum(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_element_sum(self))
    }

    /// Returns the product of all elements in `self`.
    #[inline(always)]
    pub fn element_product(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_element_product(self))
    }

    /// Returns the maximum element in `self`.
    #[inline(always)]
    pub fn max_element(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_max_element(self))
    }

    /// Returns the minimum element in `self`.
    #[inline(always)]
    pub fn min_element(self) -> f {
        specialize!(<f as FloatBackend<N>>::vec_min_element(self))
    }

    /// Returns the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> f {
        (self * other).element_sum()
    }

    /// Returns the length/magnitude of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn length(self) -> f {
        self.dot(self).sqrt()
    }

    /// Returns the squared length/magnitude of `self`.
    ///
    /// This is faster than `length` because it avoids a square root.
    #[inline(always)]
    pub fn length_squared(self) -> f {
        self.dot(self)
    }

    /// Returns a vector with the same direction as `self` but length `1.0`.
    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let result = self / self.length();

        debug_assert!(result.iter().all(<f>::is_finite));

        result
    }

    /// Returns `self.normalize()`, or `None` if the result is not finite (for
    /// example if `self` is zero`).
    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn try_normalize(self) -> Option<Self> {
        let rcp = 1.0 / self.length();
        if rcp.is_finite() && rcp > 0.0 {
            Some(self * rcp)
        } else {
            None
        }
    }

    /// Returns `self.normalize()`, or `fallback` if the result is not finite (for
    /// example if `self` is zero`).
    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }
}

impl ScalarZero for f {
    const ZERO: Self = 0.0;
}

impl ScalarOne for f {
    const ONE: Self = 1.0;
}

impl ScalarNegOne for f {
    const NEG_ONE: Self = -1.0;
}

pub(in crate::vector) trait FloatBackend<const N: usize>
where
    VecLen<N>: SupportedVecLen,
{
    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_floor(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::floor)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_ceil(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::ceil)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_round(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::round)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_trunc(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::trunc)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_fract(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::fract)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_mul_add(vec: Vector<N, f>, a: Vector<N, f>, b: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_div_euclid(vec: Vector<N, f>, rhs: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_rem_euclid(vec: Vector<N, f>, rhs: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_sqrt(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::sqrt)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_sin(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::sin)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_cos(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::cos)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_tan(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::tan)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_asin(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::asin)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_acos(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::acos)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_atan(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::atan)
    }

    #[inline(always)]
    fn vec_recip(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::recip)
    }

    #[inline(always)]
    fn vec_max(vec: Vector<N, f>, other: Vector<N, f>) -> Vector<N, f> {
        debug_assert!(!vec.iter().any(f::is_nan));
        debug_assert!(!other.iter().any(f::is_nan));

        Vector::from_fn(|i| vec[i].max(other[i]))
    }

    #[inline(always)]
    fn vec_min(vec: Vector<N, f>, other: Vector<N, f>) -> Vector<N, f> {
        debug_assert!(!vec.iter().any(f::is_nan));
        debug_assert!(!other.iter().any(f::is_nan));

        Vector::from_fn(|i| vec[i].min(other[i]))
    }

    #[inline(always)]
    fn vec_midpoint(vec: Vector<N, f>, other: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].midpoint(other[i]))
    }

    #[inline(always)]
    fn vec_clamp(vec: Vector<N, f>, min: Vector<N, f>, max: Vector<N, f>) -> Vector<N, f> {
        debug_assert!(!vec.iter().any(f::is_nan));
        debug_assert!(!min.iter().any(f::is_nan));
        debug_assert!(!max.iter().any(f::is_nan));
        debug_assert!(min.iter().zip(max).all(|(min, max)| min <= max));

        Vector::from_fn(|i| vec[i].max(min[i]).min(max[i]))
    }

    #[inline(always)]
    fn vec_abs(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::abs)
    }

    #[inline(always)]
    fn vec_signum(vec: Vector<N, f>) -> Vector<N, f> {
        vec.map(f::signum)
    }

    #[inline(always)]
    fn vec_copysign(vec: Vector<N, f>, sign: Vector<N, f>) -> Vector<N, f> {
        Vector::from_fn(|i| vec[i].copysign(sign[i]))
    }

    #[inline(always)]
    fn vec_element_sum(vec: Vector<N, f>) -> f {
        vec.to_array().iter().sum()
    }

    #[inline(always)]
    fn vec_element_product(vec: Vector<N, f>) -> f {
        vec.to_array().iter().product()
    }

    #[inline(always)]
    fn vec_max_element(vec: Vector<N, f>) -> f {
        debug_assert!(!vec.iter().any(|x| x.is_nan()));

        vec.iter().reduce(|a, b| a.max(b)).unwrap()
    }

    #[inline(always)]
    fn vec_min_element(vec: Vector<N, f>) -> f {
        debug_assert!(!vec.iter().any(|x| x.is_nan()));

        vec.iter().reduce(|a, b| a.min(b)).unwrap()
    }
}
