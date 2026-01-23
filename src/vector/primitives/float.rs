use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::specialize};

impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Returns true if any components of the vector are NaN (Not a Number).
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        specialize!(<T as FloatBackend<N, A>>::vec_is_nan(self))
    }

    /// Returns true if all components of the vector are finite.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        specialize!(<T as FloatBackend<N, A>>::vec_is_finite(self))
    }

    /// Returns one divided by the vector.
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Returns the maximum between the components of `self` and `other`.
    ///
    /// This function is not consistant with IEEE semantics in regards to NaN
    /// propagation and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if any input is NaN.
    #[inline]
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        #[cfg(assertions)]
        assert!(
            !self.is_nan() && !other.is_nan(),
            "NaN: {self:?}.max({other:?})"
        );

        specialize!(<T as FloatBackend<N, A>>::vec_max(self, other))
    }

    /// Returns the minimum between the components of `self` and `other`.
    ///
    /// This function is not consistant with IEEE semantics in regards to NaN
    /// propagation and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if any input is NaN.
    #[inline]
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        #[cfg(assertions)]
        assert!(
            !self.is_nan() && !other.is_nan(),
            "NaN: {self:?}.min({other:?})"
        );

        specialize!(<T as FloatBackend<N, A>>::vec_min(self, other))
    }

    /// Clamps the component of the vector between `min` and `max`.
    ///
    /// This function is not consistant with IEEE semantics in regards to NaN
    /// propagation and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if any input is NaN or
    /// if `min > max`.
    #[inline]
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        #[cfg(assertions)]
        assert!(
            !self.is_nan() && !min.is_nan() && !max.is_nan(),
            "NaN: {self:?}.clamp({min:?}, {max:?})"
        );

        #[cfg(assertions)]
        assert!(
            (0..N).all(|i| min[i] <= max[i]),
            "min > max: {self:?}.clamp({min:?}, {max:?})"
        );

        self.max(min).min(max)
    }

    /// Element-wise absolute value.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_abs(self))
    }

    /// Element-wise signum.
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_signum(self))
    }

    /// Returns the magnitudes of `self` and the signs of `sign`.
    #[inline]
    #[must_use]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_copysign(self, sign))
    }

    /// Returns the sum of the vector's elements.
    ///
    /// The order of additions is unspecified and may differ between targets.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> T {
        specialize!(<T as FloatBackend<N, A>>::vec_element_sum(self))
    }

    /// Returns the product of the vector's elements.
    ///
    /// The order of multiplications is unspecified and may differ between targets.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> T {
        specialize!(<T as FloatBackend<N, A>>::vec_element_product(self))
    }

    /// Returns the maximum component of the vector.
    ///
    /// This function is not consistant with IEEE semantics in regards to NaN
    /// propagation and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if any input is NaN.
    #[inline]
    #[must_use]
    pub fn max_element(self) -> T {
        #[cfg(assertions)]
        assert!(!self.is_nan(), "NaN: {self:?}.max_element()");

        specialize!(<T as FloatBackend<N, A>>::vec_max_element(self))
    }

    /// Returns the minimum component of the vector.
    ///
    /// This function is not consistant with IEEE semantics in regards to NaN
    /// propagation and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if any input is NaN.
    #[inline]
    #[must_use]
    pub fn min_element(self) -> T {
        #[cfg(assertions)]
        assert!(!self.is_nan(), "NaN: {self:?}.min_element()");

        specialize!(<T as FloatBackend<N, A>>::vec_min_element(self))
    }

    /// Returns the dot product of `self` and `other`.
    #[inline]
    #[must_use]
    pub fn dot(self, other: Self) -> T {
        (self * other).element_sum()
    }

    /// Returns the square length of the vector.
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> T {
        (self * self).element_sum()
    }
}

#[cfg(backend)]
impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Rounds the vector's components down.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_floor(self))
    }

    /// Rounds the vector's components up.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_ceil(self))
    }

    /// Rounds the vector's components to the nearest integer.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_round(self))
    }

    /// Rounds the vector's components towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_trunc(self))
    }

    /// Returns the fractional part of the vector.
    ///
    /// Equivalent to `vec - vec.trunc()`.
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    /// Fused Multiply Add. Computes `self * a + b` with only one rounding error
    /// instead of two.
    ///
    /// This function is guaranteed to return the exact same value as the
    /// standard library.
    ///
    /// This is slower than an unfused multiply add for most target
    /// architectures.
    #[inline]
    #[must_use]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_mul_add(self, a, b))
    }

    /// Euclidiean Division.
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_div_euclid(self, rhs))
    }

    /// Euclidiean Remainder.
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_rem_euclid(self, rhs))
    }

    /// Element-wise square root.
    ///
    /// This function is guaranteed to return the exact value.
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_sqrt(self))
    }

    /// Element-wise sine.
    ///
    /// This function may not return the exact same value as the standard library.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_sin(self))
    }

    /// Element-wise cosine.
    ///
    /// This function may not return the exact same value as the standard library.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_cos(self))
    }

    /// Element-wise tangent.
    ///
    /// This function may not return the exact same value as the standard library.
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_tan(self))
    }

    /// Element-wise arc sine.
    ///
    /// This function may not return the exact same value as the standard library.
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_asin(self))
    }

    /// Element-wise arc cosine.
    ///
    /// This function may not return the exact same value as the standard library.
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_acos(self))
    }

    /// Element-wise arc tangent.
    ///
    /// This function may not return the exact same value as the standard library.
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_atan(self))
    }

    /// Returns the magnitude of the vector.
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.dot(self).sqrt()
    }

    /// Returns a vector with the direction of `self` and length `1`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if the input is zero
    /// or if the result is non finite or zero.
    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        #[cfg(assertions)]
        assert!(self != Self::ZERO, "cannot normalize a zero vector");

        let result = self / self.length();

        #[cfg(assertions)]
        assert!(
            result.is_finite() && result != Self::ZERO,
            "non finite result: {self:?}.normalize()"
        );

        result
    }

    /// Returns a vector with the direction of `self` and length `1`, or `None`
    /// if the operation fails.
    ///
    /// This function fails if the input is zero
    /// or if the result is non finite or zero.
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let recip = 1.0 / self.length();
        if recip.is_finite() && recip > 0.0 {
            Some(self * recip)
        } else {
            None
        }
    }

    /// Returns a vector with the direction of `self` and length `1`, or `fallback`
    /// if the operation fails.
    ///
    /// This function fails if the input is zero
    /// or if the result is non finite or zero.
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }

    /// Returns a vector with the direction of `self` and length `1`, or zero
    /// if the operation fails.
    ///
    /// This function fails if the input is zero
    /// or if the result is non finite or zero.
    pub fn normalize_or_zero(self) -> Self {
        self.normalize_or(Self::ZERO)
    }
}

pub(crate) trait FloatBackend<const N: usize, A: Alignment>: Scalar
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vec_is_nan(vec: Vector<N, T, A>) -> bool {
        (0..N).any(|i| vec[i].is_nan())
    }

    #[inline]
    fn vec_is_finite(vec: Vector<N, T, A>) -> bool {
        (0..N).all(|i| vec[i].is_finite())
    }

    #[inline]
    fn vec_max(vec: Vector<N, T, A>, other: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| if vec[i] > other[i] { vec[i] } else { other[i] })
    }

    #[inline]
    fn vec_min(vec: Vector<N, T, A>, other: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| if vec[i] < other[i] { vec[i] } else { other[i] })
    }

    #[inline]
    fn vec_abs(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::abs)
    }

    #[inline]
    fn vec_signum(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::signum)
    }

    #[inline]
    fn vec_copysign(vec: Vector<N, T, A>, sign: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].copysign(sign[i]))
    }

    #[inline]
    fn vec_element_sum(vec: Vector<N, T, A>) -> T {
        vec.as_array_ref().iter().copied().sum()
    }

    #[inline]
    fn vec_element_product(vec: Vector<N, T, A>) -> T {
        vec.as_array_ref().iter().copied().product()
    }

    #[inline]
    fn vec_max_element(vec: Vector<N, T, A>) -> T {
        vec.iter().reduce(|a, b| if a > b { a } else { b }).unwrap()
    }

    #[inline]
    fn vec_min_element(vec: Vector<N, T, A>) -> T {
        vec.iter().reduce(|a, b| if a < b { a } else { b }).unwrap()
    }

    #[cfg(backend)]
    #[inline]
    fn vec_floor(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::floor)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_ceil(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::ceil)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_round(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::round)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_trunc(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::trunc)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_mul_add(
        vec: Vector<N, T, A>,
        a: Vector<N, T, A>,
        b: Vector<N, T, A>,
    ) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_div_euclid(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_rem_euclid(vec: Vector<N, T, A>, rhs: Vector<N, T, A>) -> Vector<N, T, A> {
        Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_sqrt(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::sqrt)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_sin(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::sin)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_cos(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::cos)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_tan(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::tan)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_asin(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::asin)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_acos(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::acos)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_atan(vec: Vector<N, T, A>) -> Vector<N, T, A> {
        vec.map(T::atan)
    }
}
