use crate::{
    assertion,
    vector::{
        Alignment, Length, Scalar, ScalarNegOne, ScalarOne, ScalarZero, SupportedLength, Vector,
        specialize,
    },
};

impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Returns a vector with each element of `self` rounded down.
    ///
    /// This function always produces the same result as `T::floor`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn floor(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_floor(self))
    }

    /// Returns a vector with each element of `self` rounded up.
    ///
    /// This function always produces the same result as `T::ceil`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn ceil(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_ceil(self))
    }

    /// Returns a vector with each element of `self` rounded to the nearest
    /// integer.
    ///
    /// This function always produces the same result as `T::round`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn round(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_round(self))
    }

    /// Returns a vector with each element of `self` rounded towards zero.
    ///
    /// This function always produces the same result as `T::trunc`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn trunc(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_trunc(self))
    }

    /// Returns a vector with the fractional part of each element of `self`.
    ///
    /// This function always produces the same result as `T::fract`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn fract(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_fract(self))
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error instead of two.
    ///
    /// This function always produces the same result as `T::mul_add`.
    ///
    /// Using this function is slower than `(self * a) + b` on most platforms.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_mul_add(self, a, b))
    }

    /// Returns a vector containing the euclidean division of numbers for each
    /// element of `self` and `rhs`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_div_euclid(self, rhs))
    }

    /// Returns a vector containing the euclidean remainder of numbers for each
    /// element of `self` and `rhs`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_rem_euclid(self, rhs))
    }

    /// Returns a vector containing the square root of a number for each element
    /// of `self`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sqrt(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_sqrt(self))
    }

    /// Returns a vector containing the sine of a number for each element of
    /// `self`.
    ///
    /// This function may not produce the same result as `T::sin`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn sin(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_sin(self))
    }

    /// Returns a vector containing the cosine of a number for each element of
    /// `self`.
    ///
    /// This function may not produce the same result as `T::cos`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn cos(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_cos(self))
    }

    /// Returns a vector containing the tangent of a number for each element of
    /// `self`.
    ///
    /// This function may not produce the same result as `T::tan`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn tan(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_tan(self))
    }

    /// Returns a vector containing the arcsine of a number for each element of
    /// `self`.
    ///
    /// This function may not produce the same result as `T::asin`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn asin(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_asin(self))
    }

    /// Returns a vector containing the arccosine of a number for each element
    /// of `self`.
    ///
    /// This function may not produce the same result as `T::acos`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn acos(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_acos(self))
    }

    /// Returns a vector containing the arctangent of a number for each element
    /// of `self`.
    ///
    /// This function may not produce the same result as `T::atan`.
    #[cfg(feature = "std")]
    #[inline(always)]
    pub fn atan(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_atan(self))
    }

    /// Computes `1.0 / vec`.
    #[inline(always)]
    pub fn recip(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_recip(self))
    }

    /// Returns a vector with the larger elements between `self` and `other`.
    ///
    /// This function may produce a different result from `T::max` in regards to
    /// `-0.0` and NaN propagation.
    ///
    /// # Panics
    ///
    /// If `assertions` are enabled, this function will panic when any input is
    /// NaN.
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

        specialize!(<T as FloatBackend<N, A>>::vec_max(self, other))
    }

    /// Returns a vector with the smaller elements between `self` and `other`.
    ///
    /// This function may produce a different result from `T::min` in regards to
    /// `-0.0` and NaN propagation.
    ///
    /// # Panics
    ///
    /// If `assertions` are enabled, this function will panic when any input is
    /// NaN.
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

        specialize!(<T as FloatBackend<N, A>>::vec_min(self, other))
    }

    /// Returns a vector with the elements of `self` clamped between `min` and
    /// `max`.
    ///
    /// This function may produce a different result from `T::clamp` in regards
    /// to `-0.0` and NaN propagation.
    ///
    /// # Panics
    ///
    /// If `assertions` are enabled, this function will panic when:
    /// - Any input is NaN.
    /// - `min` is greater than `max`.
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

        specialize!(<T as FloatBackend<N, A>>::vec_clamp(self, min, max))
    }

    /// Returns a vector with the absolute value the elements of `self`.
    #[inline(always)]
    pub fn abs(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_abs(self))
    }

    /// Returns a vector with the sign of each element of `self`.
    ///
    /// - `1.0` if the element is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the element is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the element is `NAN`
    #[inline(always)]
    pub fn signum(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_signum(self))
    }

    /// Returns a vector with the magnitudes of `self` and the signs of `sign`.
    #[inline(always)]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_copysign(self, sign))
    }

    /// Returns the sum of the elements of `self`.
    #[inline(always)]
    pub fn element_sum(self) -> T {
        specialize!(<T as FloatBackend<N, A>>::vec_element_sum(self))
    }

    /// Returns the product of the elements of `self`.
    #[inline(always)]
    pub fn element_product(self) -> T {
        specialize!(<T as FloatBackend<N, A>>::vec_element_product(self))
    }

    /// Returns the maximum element of `self`.
    ///
    /// This function may produce a different result from `T::max` in regards to
    /// `-0.0` and NaN propagation.
    ///
    /// # Panics
    ///
    /// If `assertions` are enabled, this function will panic when any element
    /// is NaN.
    #[inline(always)]
    pub fn max_element(self) -> T {
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.max_element()"
        );

        specialize!(<T as FloatBackend<N, A>>::vec_max_element(self))
    }

    /// Returns the minimum element of `self`.
    ///
    /// This function may produce a different result from `T::min` in regards to
    /// `-0.0` and NaN propagation.
    ///
    /// # Panics
    ///
    /// If `assertions` are enabled, this function will panic when any element
    /// is NaN.
    #[inline(always)]
    pub fn min_element(self) -> T {
        assertion!(
            !self.iter().any(T::is_nan),
            "vector contains NaN: {self:?}.min_element()"
        );

        specialize!(<T as FloatBackend<N, A>>::vec_min_element(self))
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
    /// This is faster than `vec.length()` as it avoids a square root.
    #[inline(always)]
    pub fn length_squared(self) -> T {
        self.dot(self)
    }

    /// Returns a vector with the direction of `self` and a length of `1.0`.
    ///
    /// # Panics
    ///
    /// If `assertions` are enabled, this function will panic if `self` is a
    /// zero vector or is non finite.
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

    /// Returns a vector with the direction of `self` and a length of `1.0`, or
    /// `None` if the operation fails.
    ///
    /// The operation fails if `self` is a zero vector or is non finite.
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

    /// Returns a vector with the direction of `self` and a length of `1.0`, or
    /// `fallback` if the operation fails.
    ///
    /// The operation fails if `self` is a zero vector or is non finite.
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
        vec.as_array_ref().iter().copied().sum()
    }

    #[inline(always)]
    fn vec_element_product(vec: Vector<N, T, A>) -> T {
        vec.as_array_ref().iter().copied().product()
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
