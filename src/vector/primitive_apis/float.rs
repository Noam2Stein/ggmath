use crate::{
    Alignment, Length, Scalar, ScalarNegOne, ScalarOne, ScalarZero, SupportedLength, Vector,
    assertion, vector::specialize,
};

impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Returns a new [`Vector`] with the elements of `self` rounded down.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn floor(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_floor(self) })
    }

    /// Returns a new [`Vector`] with the elements of `self` rounded up.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn ceil(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_ceil(self) })
    }

    /// Returns a new [`Vector`] with the elements of `self` rounded to the nearest
    /// integer.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn round(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_round(self) })
    }

    /// Returns a new [`Vector`] with the elements of `self` rounded towards zero.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn trunc(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_trunc(self) })
    }

    /// Returns a new [`Vector`] with the fractional part of `self`.
    ///
    /// This is equivalent to `self - self.trunc()`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn fract(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_fract(self) })
    }

    /// Computes `self * a + b` with only a single rounding error instead of two.
    ///
    /// This may be faster on specific target architectures but is usually slower
    /// than `self * a + b`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_mul_add(self, a, b) })
    }

    /// Computes element-wise euclidean division.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_div_euclid(self, rhs) })
    }

    /// Computes element-wise euclidean remainder.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_rem_euclid(self, rhs) })
    }

    /// Returns a new [`Vector`] with the square root of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sqrt(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_sqrt(self) })
    }

    /// Returns a new [`Vector`] with the sine of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sin(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_sin(self) })
    }

    /// Returns a new [`Vector`] with the cosine of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn cos(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_cos(self) })
    }

    /// Returns a new [`Vector`] with the tangent of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn tan(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_tan(self) })
    }

    /// Returns a new [`Vector`] with the arcsine of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn asin(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_asin(self) })
    }

    /// Returns a new [`Vector`] with the arccosine of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn acos(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_acos(self) })
    }

    /// Returns a new [`Vector`] with the arctangent of the elements of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn atan(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_atan(self) })
    }

    /// Computes `1.0 / self`.
    #[inline(always)]
    pub fn recip(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_recip(self) })
    }

    /// Returns a new [`Vector`] with the greater elements between the elements of
    /// `self` and `other`.
    ///
    /// ## Precision
    ///
    /// This may return `-0.0` instead of `0.0` or `0.0` instead of `-0.0`.
    ///
    /// If `assertions` is disabled, NaN propagation is unspecified.
    ///
    /// ## Panics
    ///
    /// Panics if any element of `self` or `other` is NaN.
    ///
    /// Note: this only panics if `assertions` is enabled (is enabled by default in
    /// debug mode).
    #[inline(always)]
    pub fn max(self, other: Self) -> Self {
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.max({other:?})"
        );
        assertion!(
            !other.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.max({other:?})"
        );

        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_max(self, other) })
    }

    /// Returns a new [`Vector`] with the lesser elements between the elements of
    /// `self` and `other`.
    ///
    /// ## Precision
    ///
    /// This may return `-0.0` instead of `0.0` or `0.0` instead of `-0.0`.
    ///
    /// If `assertions` is disabled, NaN propagation is unspecified.
    ///
    /// ## Panics
    ///
    /// Panics if any element of `self` or `other` is NaN.
    ///
    /// Note: this only panics if `assertions` is enabled (is enabled by default in
    /// debug mode).
    #[inline(always)]
    pub fn min(self, other: Self) -> Self {
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.min({other:?})"
        );
        assertion!(
            !other.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.min({other:?})"
        );

        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_min(self, other) })
    }

    /// Returns a new [`Vector`] with the elements of `self` clamped between the
    /// elements of `min` and `max`.
    ///
    /// ## Precision
    ///
    /// This may return `-0.0` instead of `0.0` or `0.0` instead of `-0.0`.
    ///
    /// If `assertions` is disabled, NaN propagation is unspecified.
    ///
    /// ## Panics
    ///
    /// Panics if `min` is greater than `max`, or if any value is NaN.
    ///
    /// Note: this only panics if `assertions` is enabled (is enabled by default in
    /// debug mode).
    #[inline(always)]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        assertion!(
            (0..N).all(|i| min[i] <= max[i]),
            "min is greater than max: {self:?}.clamp({min:?}, {max:?})"
        );
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.clamp({min:?}, {max:?})"
        );
        assertion!(
            !min.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.clamp({min:?}, {max:?})"
        );
        assertion!(
            !max.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.clamp({min:?}, {max:?})"
        );

        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_clamp(self, min, max) })
    }

    /// Returns a new [`Vector`] with the absolute value of the elements of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_abs(self) })
    }

    /// Returns a new [`Vector`] with the sign of the elements of `self`:
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_signum(self) })
    }

    /// Returns a new [`Vector`] with the elements of `self` with the sign of the
    /// elements of `sign`.
    #[inline(always)]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_copysign(self, sign) })
    }

    /// Returns the sum of the elements of `self`.
    #[inline(always)]
    pub fn element_sum(self) -> T {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_element_sum(self) })
    }

    /// Returns the product of the elements of `self`.
    #[inline(always)]
    pub fn element_product(self) -> T {
        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_element_product(self) })
    }

    /// Returns the maximum element of `self`.
    ///
    /// ## Precision
    ///
    /// This may return `-0.0` instead of `0.0` or `0.0` instead of `-0.0`.
    ///
    /// If `assertions` is disabled, NaN propagation is unspecified.
    ///
    /// ## Panics
    ///
    /// Panics if any element of `self` is NaN.
    ///
    /// Note: this only panics if `assertions` is enabled (is enabled by default in
    /// debug mode).
    #[inline(always)]
    pub fn max_element(self) -> T {
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.max_element()"
        );

        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_max_element(self) })
    }

    /// Returns the minimum element of `self`.
    ///
    /// ## Precision
    ///
    /// This may return `-0.0` instead of `0.0` or `0.0` instead of `-0.0`.
    ///
    /// If `assertions` is disabled, NaN propagation is unspecified.
    ///
    /// ## Panics
    ///
    /// Panics if any element of `self` is NaN.
    ///
    /// Note: this only panics if `assertions` is enabled (is enabled by default in
    /// debug mode).
    #[inline(always)]
    pub fn min_element(self) -> T {
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.min_element()"
        );

        specialize!(unsafe { <T as FloatBackend<N, A>>::vec_min_element(self) })
    }

    /// Returns the dot product of `self` and `other`.
    #[inline(always)]
    pub fn dot(self, other: Self) -> T {
        (self * other).element_sum()
    }

    /// Returns the magnitude of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn length(self) -> T {
        self.dot(self).sqrt()
    }

    /// Returns the square of the magnitude of `self`.
    ///
    /// This is faster than [`Self::length`] as it avoids a square root.
    #[inline(always)]
    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    /// Returns a new [`Vector`] with the direction of `self` and a length of `1.0`.
    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn normalize(self) -> Self {
        let result = self / self.length();

        assertion!(self != Self::ZERO, "cannot normalize a zero vector");
        assertion!(
            result.iter().all(T::is_finite),
            "result is not finite: {self:?}.normalize()"
        );

        result
    }

    /// Returns a new [`Vector`] with the direction of `self` and a length of `1.0`,
    /// or `None` if the operation fails.
    ///
    /// The operation fails if `self` is a zero vector or is not finite.
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

    /// Returns a new [`Vector`] with the direction of `self` and a length of `1.0`,
    /// or `fallback` if the operation fails.
    ///
    /// The operation fails if `self` is a zero vector or is not finite.
    #[cfg(feature = "std")]
    #[must_use]
    #[inline(always)]
    pub fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }
}

impl Scalar for T {}

impl ScalarZero for T {
    const ZERO: Self = 0.0;
}

impl ScalarOne for T {
    const ONE: Self = 1.0;
}

impl ScalarNegOne for T {
    const NEG_ONE: Self = -1.0;
}

pub(in crate::vector) trait FloatBackend<const N: usize, A: Alignment>:
    Scalar
where
    Length<N>: SupportedLength,
{
    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_floor(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::floor)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_ceil(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::ceil)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_round(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::round)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_trunc(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::trunc)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_fract(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::fract)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_mul_add(
        vec: Vector<N, T, A>,
        a: Vector<N, T, A>,
        b: Vector<N, T, A>,
    ) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_div_euclid(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_rem_euclid(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_sqrt(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::sqrt)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_sin(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::sin)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_cos(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::cos)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_tan(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::tan)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_asin(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::asin)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_acos(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::acos)
    }

    #[cfg(feature = "std")]
    #[inline(always)]
    fn vec_atan(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::atan)
    }

    #[inline(always)]
    fn vec_recip(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::recip)
    }

    #[inline(always)]
    fn vec_max(vec: Vector<N, T, A>, other: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| if vec[i] > other[i] { vec[i] } else { other[i] })
    }

    #[inline(always)]
    fn vec_min(vec: Vector<N, T, A>, other: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| if vec[i] < other[i] { vec[i] } else { other[i] })
    }

    #[inline(always)]
    fn vec_clamp(
        vec: Vector<N, T, A>,
        min: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Vector<N, T, A> {
        vec.max(min).min(max)
    }

    #[inline(always)]
    fn vec_abs(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::abs)
    }

    #[inline(always)]
    fn vec_signum(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::signum)
    }

    #[inline(always)]
    fn vec_copysign(vec: Vector<N, T, A>, sign: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].copysign(sign[i]))
    }

    #[inline(always)]
    fn vec_element_sum(vec: Vector<N, T, A>) -> T {
        vec.to_array().iter().sum()
    }

    #[inline(always)]
    fn vec_element_product(vec: Vector<N, T, A>) -> T {
        vec.to_array().iter().product()
    }

    #[inline(always)]
    fn vec_max_element(vec: Vector<N, T, A>) -> T {
        vec.iter().reduce(|a, b| if a > b { a } else { b }).unwrap()
    }

    #[inline(always)]
    fn vec_min_element(vec: Vector<N, T, A>) -> T {
        vec.iter().reduce(|a, b| if a < b { a } else { b }).unwrap()
    }
}
