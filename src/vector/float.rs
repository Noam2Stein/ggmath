use crate::{
    Alignment, FloatExt, Length, Mask, PrimitiveFloat, PrimitiveFloatBackend, Quaternion,
    SupportedLength, Vector,
    utils::{specialize, transmute_generic},
};

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveFloat,
{
    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let nan = Vec3::new(1.0, 2.0, f32::NAN);
    /// let f = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert!(nan.is_nan());
    /// assert!(!f.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.nan_mask().any()
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is NaN.
    ///
    /// Equivalent to `(self.x.is_nan(), self.y.is_nan(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vector = Vec3::new(1.0, 2.0, f32::NAN);
    /// let mask = vector.nan_mask();
    ///
    /// assert_eq!(mask, Mask3::new(false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn nan_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_nan_mask(self))
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let f = Vec3::new(1.0, 2.0, 3.0);
    /// let inf = Vec3::new(1.0, f32::INFINITY, 3.0);
    /// let neg_inf = Vec3::new(1.0, f32::NEG_INFINITY, 3.0);
    /// let nan = Vec3::new(1.0, f32::NEG_INFINITY, 3.0);
    ///
    /// assert!(f.is_finite());
    /// assert!(!inf.is_finite());
    /// assert!(!neg_inf.is_finite());
    /// assert!(!nan.is_finite());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.finite_mask().all()
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` is neither infinite nor NaN.
    ///
    /// Equivalent to `(self.x.is_finite(), self.y.is_finite(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask3, Vec3};
    /// #
    /// let vector = Vec3::new(1.0, f32::INFINITY, f32::NAN);
    /// let mask = vector.finite_mask();
    ///
    /// assert_eq!(mask, Mask3::new(true, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn finite_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_finite_mask(self))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` has a positive sign, including `+0.0`, NaNs with
    /// positive sign bit and positive infinity.
    ///
    /// Equivalent to
    /// `(self.x.is_sign_positive(), self.y.is_sign_positive(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask4, Vec4};
    /// #
    /// let vector = Vec4::new(1.0, -2.0, -3.0, f32::INFINITY);
    /// let mask = vector.sign_positive_mask();
    ///
    /// assert_eq!(mask, Mask4::new(true, false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn sign_positive_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_sign_positive_mask(self))
    }

    /// Returns a vector mask where each element is `true` if the corresponding
    /// element of `self` has a negative sign, including `-0.0`, NaNs with
    /// negative sign bit and negative infinity.
    ///
    /// Equivalent to
    /// `(self.x.is_sign_negative(), self.y.is_sign_negative(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mask4, Vec4};
    /// #
    /// let vector = Vec4::new(1.0, -2.0, 3.0, f32::NEG_INFINITY);
    /// let mask = vector.sign_negative_mask();
    ///
    /// assert_eq!(mask, Mask4::new(false, true, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn sign_negative_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_sign_negative_mask(self))
    }

    /// Returns the element-wise reciprocal (inverse) of a vector, `1 / self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(2.0, 3.0, 4.0);
    /// let recip = vector.recip();
    /// let div = Vec3::ONE / vector;
    ///
    /// assert_eq!(recip, div);
    /// ```
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Returns the maximum elements between `self` and `other`.
    ///
    /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
    ///
    /// This is not consistent with IEEE semantics in regards to NaN propagation
    /// and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec4;
    /// #
    /// let a = Vec4::new(1.0, 5.0, 3.0, 0.0);
    /// let b = Vec4::new(3.0, 2.0, 7.0, -1.0);
    /// let max = a.max(b);
    ///
    /// assert_eq!(max, Vec4::new(3.0, 5.0, 7.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn max(self, other: Self) -> Self {
        debug_assert!(
            !self.is_nan() && !other.is_nan(),
            "NaN: {self:?}.max({other:?})"
        );

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_max(self, other))
    }

    /// Returns the minimum elements between `self` and `other`.
    ///
    /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
    ///
    /// This is not consistent with IEEE semantics in regards to NaN propagation
    /// and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec4;
    /// #
    /// let a = Vec4::new(1.0, 5.0, 3.0, 0.0);
    /// let b = Vec4::new(3.0, 2.0, 7.0, -1.0);
    /// let min = a.min(b);
    ///
    /// assert_eq!(min, Vec4::new(1.0, 2.0, 3.0, -1.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn min(self, other: Self) -> Self {
        debug_assert!(
            !self.is_nan() && !other.is_nan(),
            "NaN: {self:?}.min({other:?})"
        );

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_min(self, other))
    }

    /// Clamps the elements of `self` between the elements of `min` and `max`.
    ///
    /// Equivalent to
    /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
    ///
    /// This is not consistent with IEEE semantics in regards to NaN propagation
    /// and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any element is NaN, or if any element of `min` is greater than
    /// the corresponding element of `max`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec4;
    /// #
    /// let vector = Vec4::new(1.0, 2.0, 3.0, 0.0);
    /// let min = Vec4::new(0.0, 5.0, 1.0, -2.0);
    /// let max = Vec4::new(3.0, 6.0, 2.0, -1.0);
    /// let clamp = vector.clamp(min, max);
    ///
    /// assert_eq!(clamp, Vec4::new(1.0, 5.0, 2.0, -1.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        debug_assert!(
            !self.is_nan() && !min.is_nan() && !max.is_nan(),
            "NaN: {self:?}.clamp({min:?}, {max:?})"
        );

        debug_assert!(
            (0..N).all(|i| min[i] <= max[i]),
            "min > max: {self:?}.clamp({min:?}, {max:?})"
        );

        self.max(min).min(max)
    }

    /// Returns the maximum between the elements of `self`.
    ///
    /// Equivalent to `self.x.max(self.y).max(self.z)...`.
    ///
    /// This is not consistent with IEEE semantics in regards to NaN propagation
    /// and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(-1.0, 7.0, 3.0);
    ///
    /// assert_eq!(vector.max_element(), 7.0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn max_element(self) -> T {
        debug_assert!(!self.is_nan(), "NaN: {self:?}.max_element()");

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_max_element(self))
    }

    /// Returns the minimum between the elements of `self`.
    ///
    /// Equivalent to `self.x.min(self.y).min(self.z)...`.
    ///
    /// This is not consistent with IEEE semantics in regards to NaN propagation
    /// and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(7.0, -1.0, 3.0);
    ///
    /// assert_eq!(vector.min_element(), -1.0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn min_element(self) -> T {
        debug_assert!(!self.is_nan(), "NaN: {self:?}.min_element()");

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_min_element(self))
    }

    /// Returns the absolute values of elements of `self`.
    ///
    /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(7.0, -1.0, -3.0);
    ///
    /// assert_eq!(vector.abs(), Vec3::new(7.0, 1.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_abs(self))
    }

    /// Returns the signum of the elements of `self`.
    ///
    /// Equivalent to `(self.x.signum(), self.y.signum(), ...)`.
    ///
    /// For each element:
    ///
    /// - `1.0` if the element is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the element is negative, `-0.0` or `NEG_INFINITY`
    /// - NaN if the element is NaN
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec4;
    /// #
    /// let vector = Vec4::new(7.0, -1.0, -3.0, f32::NAN);
    ///
    /// assert_eq!(vector.signum().x, 1.0);
    /// assert_eq!(vector.signum().y, -1.0);
    /// assert_eq!(vector.signum().z, -1.0);
    /// assert!(vector.signum().w.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_signum(self))
    }

    /// Returns a vector with the element magnitudes of `self` and the element
    /// signs of `sign`.
    ///
    /// Equivalent to `(self.x.copysign(sign.x), self.y.copysign(sign.y), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(7.0, -1.0, -3.0);
    /// let sign = Vec3::new(-5.0, -2.0, 1.0);
    /// let copysign = vector.copysign(sign);
    ///
    /// assert_eq!(copysign, Vec3::new(-7.0, -1.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_copysign(
            self, sign
        ))
    }

    /// Returns the largest integers less than or equal to the elements of
    /// `self`.
    ///
    /// This always returns the precise result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(3.7, 3.0, -3.7);
    ///
    /// assert_eq!(vector.floor(), Vec3::new(3.0, 3.0, -4.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_floor(self))
    }

    /// Returns the smallest integers greater than or equal to the elements of
    /// `self`.
    ///
    /// This always returns the precise result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(3.01, 4.0, -4.99);
    ///
    /// assert_eq!(vector.ceil(), Vec3::new(4.0, 4.0, -4.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_ceil(self))
    }

    /// Returns the nearest integers to the elements of `self`.
    ///
    /// This always returns the precise result. If a value is half-way between
    /// two integers, round away from 0.0.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(3.3, -3.3, 3.5);
    ///
    /// assert_eq!(vector.round(), Vec3::new(3.0, -3.0, 4.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_round(self))
    }

    /// Returns the integer part of the elements of `self`. This means that
    /// non-integer numbers are always truncated towards zero.
    ///
    /// This always returns the precise result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(3.7, 3.0, -3.7);
    ///
    /// assert_eq!(vector.trunc(), Vec3::new(3.0, 3.0, -3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_trunc(self))
    }

    /// Returns the fractional part of `self`. Equivalent to
    /// `self - self.trunc()`.
    ///
    /// This always returns the precise result.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let vector = Vec2::new(3.25, -3.25);
    ///
    /// assert_eq!(vector.fract(), Vec2::new(0.25, -0.25));
    /// ```
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    /// Fused multiply-add. Computes `(self * a) + b` with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` is slower than an unfused multiply-add on most target
    /// architectures.
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded
    /// infinite-precision result. It is specified by IEEE 754 as
    /// `fusedMultiplyAdd` and guaranteed not to change.
    #[inline]
    #[must_use]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_mul_add(
            self, a, b
        ))
    }

    /// Calculates Euclidean division for the elements of `self`.
    ///
    /// Equivalent to
    /// `(self.x.div_euclid(rhs.x), self.y.div_euclid(rhs.y), ...)`.
    ///
    /// See [`f32::div_euclid`].
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded
    /// infinite-precision result.
    ///
    /// [`f32::div_euclid`]: https://doc.rust-lang.org/std/primitive.f32.html#method.div_euclid
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_div_euclid(
            self, rhs
        ))
    }

    /// Calculates Euclidean remainder for the elements of `self`.
    ///
    /// Equivalent to
    /// `(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y), ...)`.
    ///
    /// See [`f32::rem_euclid`].
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded
    /// infinite-precision result.
    ///
    /// [`f32::rem_euclid`]: https://doc.rust-lang.org/std/primitive.f32.html#method.rem_euclid
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_rem_euclid(
            self, rhs
        ))
    }

    /// Computes `x^n` for the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn powf(self, n: T) -> Self {
        self.map(|x| x.powf(n))
    }

    /// Returns the square root of the elements of `self`.
    ///
    /// Equivalent to `(self.x.sqrt(), self.y.sqrt(), ...)`.
    ///
    /// # Precision
    ///
    /// The result of this operation is guaranteed to be the rounded
    /// infinite-precision result. It is specified by IEEE 754 as `squareRoot`
    /// and guaranteed not to change.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::<f32>::new(4.0, 16.0, -4.0);
    ///
    /// assert_eq!(vector.sqrt().x, 2.0);
    /// assert_eq!(vector.sqrt().y, 4.0);
    /// assert!(vector.sqrt().z.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_sqrt(self))
    }

    /// Computes the exponential function `e^x` for the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        self.map(T::exp)
    }

    /// Computes `2^x` for the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        self.map(T::exp2)
    }

    /// Computes the natural logarithm for the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        self.map(T::ln)
    }

    /// Computes the base 2 logarithm for the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(2.0, 4.0, 8.0);
    ///
    /// assert_eq!(vector.log2(), Vec3::new(1.0, 2.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        self.map(T::log2)
    }

    /// Computes the sine of the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_sin(self))
    }

    /// Computes the cosine of the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_cos(self))
    }

    /// Computes the tangent of the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_tan(self))
    }

    /// Computes the arcsine of the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_asin(self))
    }

    /// Computes the arccosine of the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_acos(self))
    }

    /// Computes the arctangent of the elements of `self`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_atan(self))
    }

    /// Simultaneously computes the sine and cosine of the elements of `self`.
    ///
    /// Equivalent to `(self.sin(), self.cos())`, but may be more performant.
    /// This might return a slightly different value.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn sin_cos(self) -> (Self, Self) {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vector_sin_cos(self))
    }

    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`.
    ///
    /// When `t` is `0.0`, the result is `self`.  When `t` is `1.0`, the result
    /// is `rhs`. When `t` is outside of the range `0.0..=1.0`, the result is
    /// linearly extrapolated.
    #[inline]
    #[must_use]
    pub fn lerp(self, other: Self, t: T) -> Self {
        self * (T::as_from(1.0) - t) + other * t
    }

    /// Computes the middle point between `self` and `other`.
    ///
    /// Equivalent to `self.lerp(other, 0.5)`, but is cheaper to compute. This
    /// may return a slightly different value.
    #[inline]
    #[must_use]
    pub fn midpoint(self, other: Self) -> Self {
        (self + other) * T::as_from(0.5)
    }

    /// Moves `self` towards `other` by at most `max_delta`.
    ///
    /// When `max_delta` is `0.0`, the result is `self`. When `max_delta` is
    /// equal to or greater than `self.distance(other)`, the result is `other`.
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(2.0, 0.0, 0.0);
    /// let target = Vec3::new(5.0, 0.0, 0.0);
    /// let max_delta = 1.0;
    /// let move_towards = vector.move_towards(target, max_delta);
    ///
    /// assert_eq!(move_towards, Vec3::new(3.0, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn move_towards(self, target: Self, max_delta: T) -> Self {
        let delta = target - self;
        let delta_length = delta.length();

        if delta_length <= max_delta || delta_length <= T::as_from(1e-4) {
            target
        } else {
            self + delta / delta_length * max_delta
        }
    }

    /// Computes the spherical linear interpolation between `self` and `other`
    /// based on the value `t`.
    ///
    /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result
    /// is `other`. When `t` is outside of the range `0..=1`, the result is
    /// spherically linearly extrapolated.
    ///
    /// The vectors do not need to be unit vectors but they do need to be
    /// non-zero.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are zero vectors.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn slerp(self, other: Self, t: T) -> Self {
        let self_length = self.length();
        let other_length = other.length();

        debug_assert!(self_length >= T::as_from(1e-7) && other_length >= T::as_from(1e-7));

        match N {
            2 => {
                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<2, T, A>>(self) };
                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                let other = unsafe { transmute_generic::<Vector<N, T, A>, Vector<2, T, A>>(other) };

                let self_normalized = self_ / self_length;
                let angle_cos = self_normalized.dot(other) / other_length;
                let angle = angle_cos.acos() * self_normalized.wedge(other).signum();

                let result_length = self_length.lerp(other_length, t);
                let result = self_normalized.rotate(angle * t) * result_length;

                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                unsafe { transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(result) }
            }
            3 => {
                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<3, T, A>>(self) };
                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                let other = unsafe { transmute_generic::<Vector<N, T, A>, Vector<3, T, A>>(other) };

                // Ported from `https://github.com/bitshifter/glam-rs`.

                let angle_cos = self_.dot(other) / (self_length * other_length);

                // If `angle_cos` is close to `1` or `-1` or is NaN the normal
                // calculation breaks down.
                let result = if angle_cos.abs() < T::as_from(1.0 - 3e-7) {
                    let angle = angle_cos.acos();
                    let angle_sin = angle.sin();
                    let self_factor = (angle * (T::ONE - t)).sin();
                    let other_factor = (angle * t).sin();

                    let result_length = self_length.lerp(other_length, t);

                    (self_ * (result_length / self_length) * self_factor
                        + other * (result_length / other_length) * other_factor)
                        / angle_sin
                } else if angle_cos.is_sign_negative() {
                    // Vectors are almost parallel in opposing directions.

                    let axis = self_.any_orthogonal_vector().normalize();
                    let rotation = Quaternion::<T, A>::from_axis_angle(axis, t * T::PI);

                    let result_length = self_length.lerp(other_length, t);
                    self_ * rotation * (result_length / self_length)
                } else {
                    // Vectors are almost parallel in the same direction.
                    self_.lerp(other, t)
                };

                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                unsafe { transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(result) }
            }
            4 => {
                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<4, T, A>>(self) };
                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                let other = unsafe { transmute_generic::<Vector<N, T, A>, Vector<4, T, A>>(other) };

                // Ported from `https://github.com/bitshifter/glam-rs`.

                let angle_cos = self_.dot(other) / (self_length * other_length);

                // If `angle_cos` is close to `1` or `-1` or is NaN the normal
                // calculation breaks down.
                let result = if angle_cos.abs() < T::as_from(1.0 - 3e-7) {
                    let angle = angle_cos.acos();
                    let angle_sin = angle.sin();
                    let t1 = (angle * (T::ONE - t)).sin();
                    let t2 = (angle * t).sin();

                    let result_length = self_length.lerp(other_length, t);

                    (self_ * (result_length / self_length) * t1
                        + other * (result_length / other_length) * t2)
                        / angle_sin
                } else if angle_cos.is_sign_negative() {
                    // Vectors are almost parallel in opposing directions.

                    let axis = self_.any_orthogonal_vector().normalize();
                    let (sin, cos) = (t * T::PI).sin_cos();

                    let result_dir = self_ * cos + axis * sin;
                    let result_length = self_length.lerp(other_length, t);
                    result_dir * (result_length / result_dir.length())
                } else {
                    // Vectors are almost parallel in the same direction.
                    self_.lerp(other, t)
                };

                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                unsafe { transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(result) }
            }
            _ => unreachable!(),
        }
    }

    /// Rotates `self` towards `target` by at most `max_angle` (in radians).
    ///
    /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
    /// to or greater than `self.angle_between(target)`, the result is `target`.
    /// When `max_angle` is negative, this rotates towards `-target`.
    ///
    /// The vectors do not need to be unit vectors but `target` does need to be
    /// non-zero.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `target` is a zero vector.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn rotate_towards(self, target: Self, max_angle: T) -> Self {
        let self_length = self.length();
        let target_length = target.length();

        debug_assert!(target_length >= T::as_from(1e-7));

        if self == Self::ZERO {
            return self;
        }

        match N {
            2 => {
                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<2, T, A>>(self) };
                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                let target =
                    unsafe { transmute_generic::<Vector<N, T, A>, Vector<2, T, A>>(target) };

                let target_angle = (self_.dot(target) / self_length / target_length)
                    .max(T::NEG_ONE)
                    .min(T::ONE)
                    .acos();
                let angle_sign = self_.wedge(target).signum();
                let angle = max_angle.clamp(target_angle - T::PI, target_angle) * angle_sign;

                let result = self_.rotate(angle);

                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                unsafe { transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(result) }
            }
            3 => {
                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<3, T, A>>(self) };
                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                let target =
                    unsafe { transmute_generic::<Vector<N, T, A>, Vector<3, T, A>>(target) };

                // Ported from `https://github.com/bitshifter/glam-rs`.

                let target_angle = (self_.dot(target) / (self_length * target_length))
                    .max(T::NEG_ONE)
                    .min(T::ONE)
                    .acos();
                let angle = max_angle.clamp(target_angle - T::PI, target_angle);
                let axis = self_
                    .cross(target)
                    .try_normalize()
                    .unwrap_or_else(|| self_.any_orthonormal_vector());

                let result = self_ * Quaternion::<T, A>::from_axis_angle(axis, angle);

                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                unsafe { transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(result) }
            }
            4 => {
                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<4, T, A>>(self) };
                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                let target =
                    unsafe { transmute_generic::<Vector<N, T, A>, Vector<4, T, A>>(target) };

                let target_angle_cos = self_.dot(target) / (self_length * target_length);
                let target_angle = target_angle_cos.max(T::NEG_ONE).min(T::ONE).acos();
                let angle = max_angle.clamp(target_angle - T::PI, target_angle);

                if angle == T::ZERO {
                    return self;
                }

                // If `target_angle_cos` is close to `1` or `-1` or is NaN the
                // normal calculation breaks down.
                let result = if target_angle_cos.abs() <= T::as_from(1.0 - 3e-7) {
                    let self_factor = (target_angle - angle).sin();
                    let target_factor = angle.sin();

                    (self_ * self_factor + target * (self_length / target_length) * target_factor)
                        .normalize()
                        * self_length
                } else if target_angle_cos.is_sign_negative() {
                    // Vectors are almost parallel in opposing directions.

                    let axis = self_.any_orthogonal_vector().normalize();
                    let (sin, cos) = angle.sin_cos();

                    let result_dir = self_ * cos + axis * sin;
                    result_dir * (self_length / result_dir.length())
                } else {
                    // Vectors are almost parallel in the same direction.
                    target / target_length * self_length
                };

                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                unsafe { transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(result) }
            }
            _ => unreachable!(),
        }
    }

    /// Returns the length/magnitude of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(2.0, 3.0, 1.0);
    ///
    /// assert_eq!(vector.length(), 14.0_f32.sqrt());
    /// ```
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.dot(self).sqrt()
    }

    /// Computes the Euclidean distance between `self` and `other`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(1.0, 2.0, 3.0);
    /// let b = Vec3::new(4.0, 5.0, 6.0);
    ///
    /// assert_eq!(a.distance(b), (a - b).length());
    /// ```
    #[inline]
    #[must_use]
    pub fn distance(self, other: Self) -> T {
        (self - other).length()
    }

    /// Returns a vector with the direction of `self` and length `1.0`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is a zero vector, or if the result is non finite or
    /// zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(vector.normalize(), vector / vector.length());
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        debug_assert!(self != Self::ZERO, "cannot normalize a zero vector");

        let result = self / self.length();

        debug_assert!(
            result.is_finite() && result != Self::ZERO,
            "non finite result: {self:?}.normalize()"
        );

        result
    }

    /// Returns [`normalize`], or `None` if `self` is zero or if the result is
    /// non finite or zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let non_zero = Vec3::new(1.0, 2.0, 3.0);
    /// let zero = Vec3::new(0.0, 0.0, 0.0);
    ///
    /// assert_eq!(non_zero.try_normalize(), Some(non_zero.normalize()));
    /// assert_eq!(zero.try_normalize(), None);
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        let recip = T::as_from(1.0) / self.length();
        if recip.is_finite() && recip > T::as_from(0.0) {
            Some(self * recip)
        } else {
            None
        }
    }

    /// Returns [`normalize`], or `fallback` if `self` is zero or if the result
    /// is non finite or zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let non_zero = Vec3::new(1.0, 2.0, 3.0);
    /// let zero = Vec3::new(0.0, 0.0, 0.0);
    /// let fallback = Vec3::new(9.0, 10.0, 21.0);
    ///
    /// assert_eq!(non_zero.normalize_or(fallback), non_zero.normalize());
    /// assert_eq!(zero.normalize_or(fallback), fallback);
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }

    /// Returns [`normalize`], or a zero vector if `self` is zero or if the
    /// result is non finite.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let non_zero = Vec3::new(1.0, 2.0, 3.0);
    /// let zero = Vec3::new(0.0, 0.0, 0.0);
    ///
    /// assert_eq!(non_zero.normalize_or_zero(), non_zero.normalize());
    /// assert_eq!(zero.normalize_or_zero(), zero);
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn normalize_or_zero(self) -> Self {
        self.normalize_or(Self::ZERO)
    }

    /// Simultaneously computes [`normalize`] and [`length`].
    ///
    /// If `self` is a zero vector, the result is `(Self::ZERO, 0.0)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vector = Vec3::new(1.0, 2.0, 3.0);
    /// let (normalize, length) = vector.normalize_and_length();
    ///
    /// assert_eq!(normalize, vector.normalize());
    /// assert_eq!(length, vector.length());
    /// ```
    ///
    /// [`normalize`]: Self::normalize
    /// [`length`]: Self::length
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        let length = self.length();
        let recip = T::as_from(1.0) / length;

        if recip.is_finite() && recip > T::as_from(0.0) {
            (self * recip, length)
        } else {
            (Self::ZERO, T::as_from(0.0))
        }
    }

    /// Returns whether the vector has the length `1.0` or not.
    ///
    /// This uses a precision threshold of approximately `1e-4`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let unit = Vec3::splat((1.0_f32 / 3.0).sqrt());
    /// let non_unit = Vec3::splat(2.0);
    ///
    /// assert!(unit.is_normalized());
    /// assert!(!non_unit.is_normalized());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        (self.length_squared() - T::as_from(1.0)).abs() <= T::as_from(2e-4)
    }

    /// Returns `self` with a length of no more than `max`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `max` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(2.0, 0.0, 0.0);
    /// let b = Vec3::new(6.0, 0.0, 0.0);
    /// let max = 4.0;
    ///
    /// assert_eq!(a.with_max_length(max), Vec3::new(2.0, 0.0, 0.0));
    /// assert_eq!(b.with_max_length(max), Vec3::new(4.0, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_max_length(self, max: T) -> Self {
        debug_assert!(max >= T::as_from(0.0), "negative maximum length");

        let length_squared = self.length_squared();
        if length_squared > max * max {
            self / length_squared.sqrt() * max
        } else {
            self
        }
    }

    /// Returns `self` with a length of no less than `min`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `min` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(2.0, 0.0, 0.0);
    /// let b = Vec3::new(6.0, 0.0, 0.0);
    /// let min = 4.0;
    ///
    /// assert_eq!(a.with_min_length(min), Vec3::new(4.0, 0.0, 0.0));
    /// assert_eq!(b.with_min_length(min), Vec3::new(6.0, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn with_min_length(self, min: T) -> Self {
        debug_assert!(min >= T::as_from(0.0), "negative minimum length");

        let length_squared = self.length_squared();
        if length_squared < min * min {
            self / length_squared.sqrt() * min
        } else {
            self
        }
    }

    /// Returns `self` with a length of no less than `min` and no more than
    /// `max`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `min > max`, or if `min` is negative.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let a = Vec3::new(2.0, 0.0, 0.0);
    /// let b = Vec3::new(6.0, 0.0, 0.0);
    /// let c = Vec3::new(10.0, 0.0, 0.0);
    /// let min = 4.0;
    /// let max = 8.0;
    ///
    /// assert_eq!(a.clamp_length(min, max), Vec3::new(4.0, 0.0, 0.0));
    /// assert_eq!(b.clamp_length(min, max), Vec3::new(6.0, 0.0, 0.0));
    /// assert_eq!(c.clamp_length(min, max), Vec3::new(8.0, 0.0, 0.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn clamp_length(self, min: T, max: T) -> Self {
        debug_assert!(min >= T::as_from(0.0), "negative minimum length");
        debug_assert!(min <= max, "minimum length is greater than maximum length");

        let length_squared = self.length_squared();
        if length_squared < min * min {
            self / length_squared.sqrt() * min
        } else if length_squared > max * max {
            self / length_squared.sqrt() * max
        } else {
            self
        }
    }

    /// Returns the angle (in radians) between `self` and `other` in the range
    /// `0..=+π`.
    ///
    /// The vectors do not need to be unit vectors but they do need to be
    /// non-zero.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let x = Vec3::new(2.0, 0.0, 0.0);
    /// let y = Vec3::new(0.0, 3.0, 0.0);
    /// let angle = x.angle_between(y);
    ///
    /// assert!((angle - 90.0_f32.to_radians()).abs() < 1e-5);
    /// ```
    #[inline]
    #[must_use]
    pub fn angle_between(self, other: Self) -> T {
        (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt())
            .max(T::NEG_ONE)
            .min(T::ONE)
            .acos()
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must not be a zero vector.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `other` is a zero vector.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn project_onto(self, other: Self) -> Self {
        let other_length_squared_recip = other.length_squared().recip();

        debug_assert!(other_length_squared_recip.is_finite());

        other * self.dot(other) * other_length_squared_recip
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `other` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        debug_assert!(other.is_normalized());

        other * self.dot(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// Equivalent to `self - self.project_onto(other)`.
    ///
    /// `other` must not be a zero vector.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `other` is a zero vector.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// Equivalent to `self - self.project_onto(other)`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `other` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn reject_from_normalized(self, other: Self) -> Self {
        self - self.project_onto_normalized(other)
    }

    /// Returns the reflection of `self` through `normal`.
    ///
    /// `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `normal` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn reflect(self, normal: Self) -> Self {
        debug_assert!(normal.is_normalized());

        self - normal * (T::as_from(2.0) * self.dot(normal))
    }

    /// Returns the vector refraction of `self` through `normal` and `eta`.
    ///
    /// `eta` is the incident refraction-index divided by the transmitted
    /// refraction-index.
    ///
    /// When total internal reflection occurs, the result is a zero vector.
    ///
    /// `self` and `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `normal` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn refract(self, normal: Self, eta: T) -> Self {
        debug_assert!(self.is_normalized());
        debug_assert!(normal.is_normalized());

        let self_dot_normal = self.dot(normal);
        let k = T::as_from(1.0) - eta * eta * (T::as_from(1.0) - self_dot_normal * self_dot_normal);
        if k >= T::as_from(0.0) {
            self * eta - normal * (eta * self_dot_normal + k.sqrt())
        } else {
            Self::ZERO
        }
    }

    /// Returns some vector that is orthogonal to `self`.
    ///
    /// The result is not necessarily normalized. For that use
    /// [`any_orthonormal_vector`] instead.
    ///
    /// For 2D vectors this is equivalent to [`perp`].
    ///
    /// [`any_orthonormal_vector`]: Self::any_orthonormal_vector
    /// [`perp`]: Vector::perp
    #[inline]
    #[must_use]
    pub fn any_orthogonal_vector(self) -> Self {
        match N {
            2 => {
                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<2, T, A>>(self) };

                let result = self_.perp();

                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                unsafe { transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(result) }
            }
            3 => {
                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<3, T, A>>(self) };

                let result = if self_.x.abs() > self_.y.abs() {
                    Vector::<3, T, A>::new(-self_.z, T::ZERO, self_.x)
                } else {
                    Vector::<3, T, A>::new(T::ZERO, self_.z, -self_.y)
                };

                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                unsafe { transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(result) }
            }
            4 => {
                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<4, T, A>>(self) };

                let self_abs = self_.abs();
                let result = if self_abs.x > self_abs.y {
                    if self_abs.x > self_abs.z {
                        Vector::<4, T, A>::new(-self_.w, T::ZERO, T::ZERO, self_.x)
                    } else {
                        Vector::<4, T, A>::new(T::ZERO, T::ZERO, -self_.w, self_.z)
                    }
                } else if self_abs.y > self_abs.z {
                    Vector::<4, T, A>::new(T::ZERO, -self_.w, T::ZERO, self_.y)
                } else {
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, -self_.w, self_.z)
                };

                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                unsafe { transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(result) }
            }
            _ => unreachable!(),
        }
    }

    /// Returns some unit vector that is orthogonal to `self`.
    ///
    /// `self` must normalized.
    ///
    /// For 2D vectors this is equivalent to [`perp`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    ///
    /// [`perp`]: Vector::perp
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn any_orthonormal_vector(self) -> Self {
        debug_assert!(self.is_normalized());

        match N {
            2 => {
                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<2, T, A>>(self) };

                let result = self_.perp();

                // SAFETY: Because `N = 2`, `Vector<N, T, A> = Vector<2, T, A>`.
                unsafe { transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(result) }
            }
            3 => {
                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<3, T, A>>(self) };

                // Ported from https://github.com/bitshifter/glam-rs.
                let sign = self_.z.signum();
                let a = T::NEG_ONE / (sign + self_.z);
                let b = self_.x * self_.y * a;
                let result = Vector::<3, T, A>::new(b, sign + self_.y * self_.y * a, -self_.y);

                // SAFETY: Because `N = 3`, `Vector<N, T, A> = Vector<3, T, A>`.
                unsafe { transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(result) }
            }
            4 => {
                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                let self_ = unsafe { transmute_generic::<Vector<N, T, A>, Vector<4, T, A>>(self) };

                let result = self_.any_orthogonal_vector().normalize();

                // SAFETY: Because `N = 4`, `Vector<N, T, A> = Vector<4, T, A>`.
                unsafe { transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(result) }
            }
            _ => unreachable!(),
        }
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two vectors that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        (self - other)
            .abs()
            .le_mask(Self::splat(max_abs_diff))
            .all()
    }
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Returns the angle (in radians) that rotates `self` to `other` in the
    /// range `-π..=+π`.
    ///
    /// The vectors do not need to be unit vectors but they do need to be
    /// non-zero.
    ///
    /// Equivalent to `other.angle_from(self)`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let x = Vec2::new(2.0, 0.0);
    /// let y = Vec2::new(0.0, 3.0);
    ///
    /// assert!(x.angle_to(y) > 0.0);
    /// assert!(y.angle_to(x) < 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn angle_to(self, other: Self) -> T {
        let outer_product = (self.x * other.y) - (self.y * other.x);
        self.angle_between(other) * outer_product.signum()
    }

    /// Returns the angle (in radians) that rotates `other` to `self` in the
    /// range `-π..=+π`.
    ///
    /// The vectors do not need to be unit vectors but they do need to be
    /// non-zero.
    ///
    /// Equivalent to `other.angle_to(self)`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let x = Vec2::new(2.0, 0.0);
    /// let y = Vec2::new(0.0, 3.0);
    ///
    /// assert!(x.angle_from(y) < 0.0);
    /// assert!(y.angle_from(x) > 0.0);
    /// ```
    #[inline]
    #[must_use]
    pub fn angle_from(self, other: Self) -> T {
        let outer_product = (other.x * self.y) - (other.y * self.x);
        self.angle_between(other) * outer_product.signum()
    }

    /// Rotates `self` by `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn rotate(self, angle: T) -> Self {
        let (angle_sin, angle_cos) = angle.sin_cos();
        Self::new(
            self.x * angle_cos - self.y * angle_sin,
            self.x * angle_sin + self.y * angle_cos,
        )
    }
}

impl<T, A: Alignment> Vector<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a 3D vector from homogeneous coordinates by performing
    /// perspective divide.
    ///
    /// Equivalent to `homogeneous.xyz / homogeneous.w`.
    pub fn from_homogeneous(homogeneous: Vector<4, T, A>) -> Self {
        homogeneous.xyz() / homogeneous.w
    }

    /// Rotates `self` around the x axis by `angle` (in radians).
    ///
    /// This rotates `+Y` to `+Z`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn rotate_x(self, angle: T) -> Self {
        let (angle_sin, angle_cos) = angle.sin_cos();
        Self::new(
            self.x,
            self.y * angle_cos - self.z * angle_sin,
            self.y * angle_sin + self.z * angle_cos,
        )
    }

    /// Rotates `self` around the y axis by `angle` (in radians).
    ///
    /// This rotates `+Z` to `+X`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn rotate_y(self, angle: T) -> Self {
        let (angle_sin, angle_cos) = angle.sin_cos();
        Self::new(
            self.x * angle_cos + self.z * angle_sin,
            self.y,
            self.x * -angle_sin + self.z * angle_cos,
        )
    }

    /// Rotates `self` around the z axis by `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// # Unspecified precision
    ///
    /// The precision of this function is non-deterministic. This means it
    /// varies by platform, version, and can even differ within the same
    /// execution from one invocation to the next.
    #[inline]
    #[must_use]
    pub fn rotate_z(self, angle: T) -> Self {
        let (angle_sin, angle_cos) = angle.sin_cos();
        Self::new(
            self.x * angle_cos - self.y * angle_sin,
            self.x * angle_sin + self.y * angle_cos,
            self.z,
        )
    }

    /// Returns two unit vectors that are orthogonal to `self` and to each
    /// other.
    ///
    /// Together with `self`, they form an orthonormal basis where the three
    /// vectors are all orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn any_orthonormal_pair(self) -> (Self, Self) {
        debug_assert!(self.is_normalized());

        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = self.z.signum();
        let a = T::as_from(-1.0) / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(
                T::as_from(1.0) + sign * self.x * self.x * a,
                sign * b,
                -sign * self.x,
            ),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        FloatExt, Mask, Vec2, Vector,
        utils::{PrimitiveFloatFns, assert_debug_panic, assert_float_eq, float_eq, for_parameters},
    };

    #[test]
    fn test_is_nan() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).is_nan(),
                x.is_nan() || y.is_nan()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).is_nan(),
                x.is_nan() || y.is_nan() || z.is_nan()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).is_nan(),
                x.is_nan() || y.is_nan() || z.is_nan() || w.is_nan()
            );
        });
    }

    #[test]
    fn test_nan_mask() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).nan_mask(),
                Mask::<2, T, A>::new(x.is_nan(), y.is_nan())
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).nan_mask(),
                Mask::<3, T, A>::new(x.is_nan(), y.is_nan(), z.is_nan())
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).nan_mask(),
                Mask::<4, T, A>::new(x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan())
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).is_finite(),
                x.is_finite() && y.is_finite()
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).is_finite(),
                x.is_finite() && y.is_finite() && z.is_finite()
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).is_finite(),
                x.is_finite() && y.is_finite() && z.is_finite() && w.is_finite()
            );
        });
    }

    #[test]
    fn test_finite_mask() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).finite_mask(),
                Mask::<2, T, A>::new(x.is_finite(), y.is_finite())
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).finite_mask(),
                Mask::<3, T, A>::new(x.is_finite(), y.is_finite(), z.is_finite())
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).finite_mask(),
                Mask::<4, T, A>::new(x.is_finite(), y.is_finite(), z.is_finite(), w.is_finite())
            );
        });
    }

    #[test]
    fn test_sign_positive_mask() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).sign_positive_mask(),
                Mask::<2, T, A>::new(x.is_sign_positive(), y.is_sign_positive())
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).sign_positive_mask(),
                Mask::<3, T, A>::new(
                    x.is_sign_positive(),
                    y.is_sign_positive(),
                    z.is_sign_positive()
                )
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).sign_positive_mask(),
                Mask::<4, T, A>::new(
                    x.is_sign_positive(),
                    y.is_sign_positive(),
                    z.is_sign_positive(),
                    w.is_sign_positive()
                )
            );
        });
    }

    #[test]
    fn test_sign_negative_mask() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).sign_negative_mask(),
                Mask::<2, T, A>::new(x.is_sign_negative(), y.is_sign_negative())
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).sign_negative_mask(),
                Mask::<3, T, A>::new(
                    x.is_sign_negative(),
                    y.is_sign_negative(),
                    z.is_sign_negative()
                )
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).sign_negative_mask(),
                Mask::<4, T, A>::new(
                    x.is_sign_negative(),
                    y.is_sign_negative(),
                    z.is_sign_negative(),
                    w.is_sign_negative()
                )
            );
        });
    }

    #[test]
    fn test_recip() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).recip(),
                Vector::<2, T, A>::new(x.recip(), y.recip())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).recip(),
                Vector::<3, T, A>::new(x.recip(), y.recip(), z.recip())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).recip(),
                Vector::<4, T, A>::new(x.recip(), y.recip(), z.recip(), w.recip())
            );
        });
    }

    #[test]
    fn test_max() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::midpoint(x, y);

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) && !T::is_nan(w) {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).max(Vector::<2, T, A>::new(z, w)),
                    Vector::<2, T, A>::new(x.max(z), y.max(w)),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).max(Vector::<3, T, A>::new(z, w, y)),
                    Vector::<3, T, A>::new(x.max(z), y.max(w), z.max(y)),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).max(Vector::<4, T, A>::new(z, w, y, x)),
                    Vector::<4, T, A>::new(x.max(z), y.max(w), z.max(y), w.max(x)),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).max(Vector::<2, T, A>::new(z, w)));
                assert_debug_panic!(
                    Vector::<3, T, A>::new(x, y, z).max(Vector::<3, T, A>::new(z, w, y))
                );
                assert_debug_panic!(
                    Vector::<4, T, A>::new(x, y, z, w).max(Vector::<4, T, A>::new(z, w, y, x))
                );
            }
        });
    }

    #[test]
    fn test_min() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::midpoint(x, y);

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) && !T::is_nan(w) {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).min(Vector::<2, T, A>::new(z, w)),
                    Vector::<2, T, A>::new(x.min(z), y.min(w)),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).min(Vector::<3, T, A>::new(z, w, y)),
                    Vector::<3, T, A>::new(x.min(z), y.min(w), z.min(y)),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).min(Vector::<4, T, A>::new(z, w, y, x)),
                    Vector::<4, T, A>::new(x.min(z), y.min(w), z.min(y), w.min(x)),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).min(Vector::<2, T, A>::new(z, w)));
                assert_debug_panic!(
                    Vector::<3, T, A>::new(x, y, z).min(Vector::<3, T, A>::new(z, w, y))
                );
                assert_debug_panic!(
                    Vector::<4, T, A>::new(x, y, z, w).min(Vector::<4, T, A>::new(z, w, y, x))
                );
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::midpoint(x, y);

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) && z <= y && w <= z {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y)
                        .clamp(Vector::<2, T, A>::new(z, w), Vector::<2, T, A>::new(y, z)),
                    Vector::<2, T, A>::new(x.clamp(z, y), y.clamp(w, z)),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(
                    Vector::<2, T, A>::new(x, y)
                        .clamp(Vector::<2, T, A>::new(z, w), Vector::<2, T, A>::new(y, z))
                );
            }

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) && z <= y && w <= z && y <= x {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).clamp(
                        Vector::<3, T, A>::new(z, w, y),
                        Vector::<3, T, A>::new(y, z, x)
                    ),
                    Vector::<3, T, A>::new(x.clamp(z, y), y.clamp(w, z), z.clamp(y, x)),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).clamp(
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(y, z, x)
                ));
            }

            if !T::is_nan(x)
                && !T::is_nan(y)
                && !T::is_nan(z)
                && !T::is_nan(w)
                && z <= y
                && w <= z
                && y <= x
                && z <= x
            {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).clamp(
                        Vector::<4, T, A>::new(z, w, y, z),
                        Vector::<4, T, A>::new(y, z, x, x)
                    ),
                    Vector::<4, T, A>::new(
                        x.clamp(z, y),
                        y.clamp(w, z),
                        z.clamp(y, x),
                        w.clamp(z, x)
                    ),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).clamp(
                    Vector::<4, T, A>::new(z, w, y, z),
                    Vector::<4, T, A>::new(y, z, x, x)
                ));
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::midpoint(x, y);

            if !T::is_nan(x) && !T::is_nan(y) {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).max_element(),
                    x.max(y),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).max_element());
            }

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).max_element(),
                    x.max(y).max(z),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).max_element());
            }

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) && !T::is_nan(w) {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).max_element(),
                    x.max(y).max(z),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).max_element());
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::midpoint(x, y);

            if !T::is_nan(x) && !T::is_nan(y) {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).min_element(),
                    x.min(y),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).min_element());
            }

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).min_element(),
                    x.min(y).min(z),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).min_element());
            }

            if !T::is_nan(x) && !T::is_nan(y) && !T::is_nan(z) && !T::is_nan(w) {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).min_element(),
                    x.min(y).min(z),
                    0.0 = -0.0
                );
            } else {
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).min_element());
            }
        });
    }

    #[test]
    fn test_abs() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).abs(),
                Vector::<2, T, A>::new(x.abs(), y.abs())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).abs(),
                Vector::<3, T, A>::new(x.abs(), y.abs(), z.abs())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).abs(),
                Vector::<4, T, A>::new(x.abs(), y.abs(), z.abs(), w.abs())
            );
        });
    }

    #[test]
    fn test_signum() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).signum(),
                Vector::<2, T, A>::new(x.signum(), y.signum())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).signum(),
                Vector::<3, T, A>::new(x.signum(), y.signum(), z.signum())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).signum(),
                Vector::<4, T, A>::new(x.signum(), y.signum(), z.signum(), w.signum())
            );
        });
    }

    #[test]
    fn test_copysign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).copysign(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.copysign(z), y.copysign(w)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).copysign(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.copysign(z), y.copysign(w), z.copysign(y)),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).copysign(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(x.copysign(z), y.copysign(w), z.copysign(y), w.copysign(x)),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_floor() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).floor(),
                Vector::<2, T, A>::new(x.floor(), y.floor())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).floor(),
                Vector::<3, T, A>::new(x.floor(), y.floor(), z.floor())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).floor(),
                Vector::<4, T, A>::new(x.floor(), y.floor(), z.floor(), w.floor())
            );
        });
    }

    #[test]
    fn test_ceil() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).ceil(),
                Vector::<2, T, A>::new(x.ceil(), y.ceil())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).ceil(),
                Vector::<3, T, A>::new(x.ceil(), y.ceil(), z.ceil())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).ceil(),
                Vector::<4, T, A>::new(x.ceil(), y.ceil(), z.ceil(), w.ceil())
            );
        });
    }

    #[test]
    fn test_round() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).round(),
                Vector::<2, T, A>::new(x.round(), y.round())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).round(),
                Vector::<3, T, A>::new(x.round(), y.round(), z.round())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).round(),
                Vector::<4, T, A>::new(x.round(), y.round(), z.round(), w.round())
            );
        });
    }

    #[test]
    fn test_trunc() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).trunc(),
                Vector::<2, T, A>::new(x.trunc(), y.trunc())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).trunc(),
                Vector::<3, T, A>::new(x.trunc(), y.trunc(), z.trunc())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).trunc(),
                Vector::<4, T, A>::new(x.trunc(), y.trunc(), z.trunc(), w.trunc())
            );
        });
    }

    #[test]
    fn test_fract() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).fract(),
                Vector::<2, T, A>::new(x.fract(), y.fract())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).fract(),
                Vector::<3, T, A>::new(x.fract(), y.fract(), z.fract())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).fract(),
                Vector::<4, T, A>::new(x.fract(), y.fract(), z.fract(), w.fract())
            );
        });
    }

    #[test]
    fn test_mul_add() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y)
                    .mul_add(Vector::<2, T, A>::new(z, w), Vector::<2, T, A>::new(y, z)),
                Vector::<2, T, A>::new(x.mul_add(z, y), y.mul_add(w, z))
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).mul_add(
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(y, z, w)
                ),
                Vector::<3, T, A>::new(x.mul_add(z, y), y.mul_add(w, z), z.mul_add(y, w))
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).mul_add(
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(y, z, w, y)
                ),
                Vector::<4, T, A>::new(
                    x.mul_add(z, y),
                    y.mul_add(w, z),
                    z.mul_add(y, w),
                    w.mul_add(x, y)
                )
            );
        });
    }

    #[test]
    fn test_div_euclid() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).div_euclid(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.div_euclid(z), y.div_euclid(w))
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).div_euclid(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.div_euclid(z), y.div_euclid(w), z.div_euclid(y))
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).div_euclid(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.div_euclid(z),
                    y.div_euclid(w),
                    z.div_euclid(y),
                    w.div_euclid(x)
                )
            );
        });
    }

    #[test]
    fn test_rem_euclid() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).rem_euclid(Vector::<2, T, A>::new(z, w)),
                Vector::<2, T, A>::new(x.rem_euclid(z), y.rem_euclid(w))
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).rem_euclid(Vector::<3, T, A>::new(z, w, y)),
                Vector::<3, T, A>::new(x.rem_euclid(z), y.rem_euclid(w), z.rem_euclid(y))
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).rem_euclid(Vector::<4, T, A>::new(z, w, y, x)),
                Vector::<4, T, A>::new(
                    x.rem_euclid(z),
                    y.rem_euclid(w),
                    z.rem_euclid(y),
                    w.rem_euclid(x)
                )
            );
        });
    }

    #[test]
    fn test_sqrt() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).sqrt(),
                Vector::<2, T, A>::new(x.sqrt(), y.sqrt())
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).sqrt(),
                Vector::<3, T, A>::new(x.sqrt(), y.sqrt(), z.sqrt())
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).sqrt(),
                Vector::<4, T, A>::new(x.sqrt(), y.sqrt(), z.sqrt(), w.sqrt())
            );
        });
    }

    #[test]
    fn test_exp() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).exp(),
                Vector::<2, T, A>::new(x.exp(), y.exp()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).exp(),
                Vector::<3, T, A>::new(x.exp(), y.exp(), z.exp()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).exp(),
                Vector::<4, T, A>::new(x.exp(), y.exp(), z.exp(), w.exp()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_exp2() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).exp2(),
                Vector::<2, T, A>::new(x.exp2(), y.exp2()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).exp2(),
                Vector::<3, T, A>::new(x.exp2(), y.exp2(), z.exp2()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).exp2(),
                Vector::<4, T, A>::new(x.exp2(), y.exp2(), z.exp2(), w.exp2()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_ln() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).ln(),
                Vector::<2, T, A>::new(x.ln(), y.ln()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).ln(),
                Vector::<3, T, A>::new(x.ln(), y.ln(), z.ln()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).ln(),
                Vector::<4, T, A>::new(x.ln(), y.ln(), z.ln(), w.ln()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_log2() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).log2(),
                Vector::<2, T, A>::new(x.log2(), y.log2()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).log2(),
                Vector::<3, T, A>::new(x.log2(), y.log2(), z.log2()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).log2(),
                Vector::<4, T, A>::new(x.log2(), y.log2(), z.log2(), w.log2()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_sin() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).sin(),
                Vector::<2, T, A>::new(x.sin(), y.sin()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).sin(),
                Vector::<3, T, A>::new(x.sin(), y.sin(), z.sin()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).sin(),
                Vector::<4, T, A>::new(x.sin(), y.sin(), z.sin(), w.sin()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_cos() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).cos(),
                Vector::<2, T, A>::new(x.cos(), y.cos()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).cos(),
                Vector::<3, T, A>::new(x.cos(), y.cos(), z.cos()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).cos(),
                Vector::<4, T, A>::new(x.cos(), y.cos(), z.cos(), w.cos()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_tan() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).tan(),
                Vector::<2, T, A>::new(x.tan(), y.tan()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).tan(),
                Vector::<3, T, A>::new(x.tan(), y.tan(), z.tan()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).tan(),
                Vector::<4, T, A>::new(x.tan(), y.tan(), z.tan(), w.tan()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_asin() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).asin(),
                Vector::<2, T, A>::new(x.asin(), y.asin()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).asin(),
                Vector::<3, T, A>::new(x.asin(), y.asin(), z.asin()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).asin(),
                Vector::<4, T, A>::new(x.asin(), y.asin(), z.asin(), w.asin()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_acos() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).acos(),
                Vector::<2, T, A>::new(x.acos(), y.acos()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).acos(),
                Vector::<3, T, A>::new(x.acos(), y.acos(), z.acos()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).acos(),
                Vector::<4, T, A>::new(x.acos(), y.acos(), z.acos(), w.acos()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_atan() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).atan(),
                Vector::<2, T, A>::new(x.atan(), y.atan()),
                abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).atan(),
                Vector::<3, T, A>::new(x.atan(), y.atan(), z.atan()),
                abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).atan(),
                Vector::<4, T, A>::new(x.atan(), y.atan(), z.atan(), w.atan()),
                abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
            );
        });
    }

    #[test]
    fn test_sin_cos() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).sin_cos(),
                (
                    Vector::<2, T, A>::new(x.sin_cos().0, y.sin_cos().0),
                    Vector::<2, T, A>::new(x.sin_cos().1, y.sin_cos().1)
                ),
                abs <= (
                    Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5,
                    Vector::<2, T, A>::new(x, y).abs() * 1e-5 + 1e-5
                )
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).sin_cos(),
                (
                    Vector::<3, T, A>::new(x.sin_cos().0, y.sin_cos().0, z.sin_cos().0),
                    Vector::<3, T, A>::new(x.sin_cos().1, y.sin_cos().1, z.sin_cos().1)
                ),
                abs <= (
                    Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5,
                    Vector::<3, T, A>::new(x, y, z).abs() * 1e-5 + 1e-5
                )
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).sin_cos(),
                (
                    Vector::<4, T, A>::new(
                        x.sin_cos().0,
                        y.sin_cos().0,
                        z.sin_cos().0,
                        w.sin_cos().0
                    ),
                    Vector::<4, T, A>::new(
                        x.sin_cos().1,
                        y.sin_cos().1,
                        z.sin_cos().1,
                        w.sin_cos().1
                    )
                ),
                abs <= (
                    Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5,
                    Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-5 + 1e-5
                )
            );
        });
    }

    #[test]
    fn test_lerp() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x) || !T::is_finite(y) || !T::is_finite(z) {
                return;
            }

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).lerp(Vector::<2, T, A>::new(z, w), 0.0),
                Vector::<2, T, A>::new(x, y),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).lerp(Vector::<2, T, A>::new(z, w), 0.5),
                Vector::<2, T, A>::new(x, y) * 0.5 + Vector::<2, T, A>::new(z, w) * 0.5,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).lerp(Vector::<2, T, A>::new(z, w), 1.0),
                Vector::<2, T, A>::new(z, w),
                0.0 = -0.0
            );

            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).lerp(Vector::<3, T, A>::new(z, w, y), 0.0),
                Vector::<3, T, A>::new(x, y, z),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).lerp(Vector::<3, T, A>::new(z, w, y), 0.5),
                Vector::<3, T, A>::new(x, y, z) * 0.5 + Vector::<3, T, A>::new(z, w, y) * 0.5,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).lerp(Vector::<3, T, A>::new(z, w, y), 1.0),
                Vector::<3, T, A>::new(z, w, y),
                0.0 = -0.0
            );

            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).lerp(Vector::<4, T, A>::new(z, w, y, x), 0.0),
                Vector::<4, T, A>::new(x, y, z, w),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).lerp(Vector::<4, T, A>::new(z, w, y, x), 0.5),
                Vector::<4, T, A>::new(x, y, z, w) * 0.5 + Vector::<4, T, A>::new(z, w, y, x) * 0.5,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).lerp(Vector::<4, T, A>::new(z, w, y, x), 1.0),
                Vector::<4, T, A>::new(z, w, y, x),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_midpoint() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).midpoint(Vector::<2, T, A>::new(z, w)),
                (Vector::<2, T, A>::new(x, y) + Vector::<2, T, A>::new(z, w)) * 0.5,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).midpoint(Vector::<3, T, A>::new(z, w, y)),
                (Vector::<3, T, A>::new(x, y, z) + Vector::<3, T, A>::new(z, w, y)) * 0.5,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(x, y, z, w).midpoint(Vector::<4, T, A>::new(z, w, y, x)),
                (Vector::<4, T, A>::new(x, y, z, w) + Vector::<4, T, A>::new(z, w, y, x)) * 0.5,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_move_towards() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if Vector::<2, T, A>::new(x, y)
                .distance(Vector::<2, T, A>::new(z, w))
                .is_finite()
            {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).move_towards(Vector::<2, T, A>::new(z, w), 0.0),
                    Vector::<2, T, A>::new(x, y),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).move_towards(Vector::<2, T, A>::new(z, w), T::MAX),
                    Vector::<2, T, A>::new(z, w),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y)
                        .move_towards(Vector::<2, T, A>::new(z, w), 1.0)
                        .distance(Vector::<2, T, A>::new(z, w)),
                    (Vector::<2, T, A>::new(x, y).distance(Vector::<2, T, A>::new(z, w)) - 1.0)
                        .max(0.0),
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5,
                    0.0 = -0.0
                );
            }

            if Vector::<3, T, A>::new(x, y, z)
                .distance(Vector::<3, T, A>::new(z, w, y))
                .is_finite()
            {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z)
                        .move_towards(Vector::<3, T, A>::new(z, w, y), 0.0),
                    Vector::<3, T, A>::new(x, y, z),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z)
                        .move_towards(Vector::<3, T, A>::new(z, w, y), T::MAX),
                    Vector::<3, T, A>::new(z, w, y),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z)
                        .move_towards(Vector::<3, T, A>::new(z, w, y), 1.0)
                        .distance(Vector::<3, T, A>::new(z, w, y)),
                    (Vector::<3, T, A>::new(x, y, z).distance(Vector::<3, T, A>::new(z, w, y))
                        - 1.0)
                        .max(0.0),
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5,
                    0.0 = -0.0
                );
            }

            if Vector::<4, T, A>::new(x, y, z, w)
                .distance(Vector::<4, T, A>::new(z, w, y, w))
                .is_finite()
            {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w)
                        .move_towards(Vector::<4, T, A>::new(z, w, y, w), 0.0),
                    Vector::<4, T, A>::new(x, y, z, w),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w)
                        .move_towards(Vector::<4, T, A>::new(z, w, y, w), T::MAX),
                    Vector::<4, T, A>::new(z, w, y, w),
                    0.0 = -0.0
                );
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w)
                        .move_towards(Vector::<4, T, A>::new(z, w, y, w), 1.0)
                        .distance(Vector::<4, T, A>::new(z, w, y, w)),
                    (Vector::<4, T, A>::new(x, y, z, w)
                        .distance(Vector::<4, T, A>::new(z, w, y, w))
                        - 1.0)
                        .max(0.0),
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_parameters!(|T: PrimitiveFloat, A, t| {
            let _: T = t;
            let t = (t + 5.0) % 10.0 - 5.0;
            let t = if t.is_finite() { t } else { 0.0 };

            for (vector, other) in [
                (
                    Vector::<2, T, A>::new(1.0, 0.3),
                    Vector::<2, T, A>::new(-1.5, 3.3),
                ),
                (
                    Vector::<2, T, A>::new(-10.0, 5.3),
                    Vector::<2, T, A>::new(1.5, 30.3),
                ),
                (
                    Vector::<2, T, A>::new(1.0, 0.03),
                    Vector::<2, T, A>::new(1.0, 0.0),
                ),
                (
                    Vector::<2, T, A>::new(1.0, 0.03),
                    Vector::<2, T, A>::new(20.0, 0.0),
                ),
                (
                    Vector::<2, T, A>::new(1.0, 0.03),
                    Vector::<2, T, A>::new(-1.0, 0.0),
                ),
                (
                    Vector::<2, T, A>::new(1.0, 0.03),
                    Vector::<2, T, A>::new(-20.0, 0.0),
                ),
            ] {
                assert_float_eq!(
                    vector.slerp(other, t).length(),
                    vector.length().lerp(other.length(), t).abs(),
                    abs <= vector.slerp(other, t).length() * 1e-2 + 1e-2
                );
                assert_float_eq!(
                    vector
                        .normalize()
                        .slerp(other.normalize(), t)
                        .angle_between(vector),
                    T::PI - (vector.angle_between(other) * t.abs() % T::TAU - T::PI).abs(),
                    abs <= 1e-2
                );
                assert_float_eq!(
                    vector
                        .normalize()
                        .slerp(other.normalize(), t)
                        .angle_between(other),
                    T::PI - (vector.angle_between(other) * (1.0 - t).abs() % T::TAU - T::PI).abs(),
                    abs <= 1e-2
                );
            }
            assert_debug_panic!(Vector::<2, T, A>::ZERO.slerp(Vector::ONE, t));
            assert_debug_panic!(Vector::<2, T, A>::ONE.slerp(Vector::ZERO, t));

            for (vector, other) in [
                (
                    Vector::<3, T, A>::new(1.0, 0.3, 1.4),
                    Vector::<3, T, A>::new(-1.5, 3.3, -0.3),
                ),
                (
                    Vector::<3, T, A>::new(-10.0, 5.3, 3.0),
                    Vector::<3, T, A>::new(1.5, 30.3, 1.3),
                ),
                (
                    Vector::<3, T, A>::new(1.0, 0.03, 3.0),
                    Vector::<3, T, A>::new(1.0, 0.0, 3.0001),
                ),
                (
                    Vector::<3, T, A>::new(1.0, 0.03, 500.0),
                    Vector::<3, T, A>::new(10.0, 0.0, 499.9),
                ),
                (
                    Vector::<3, T, A>::new(1.0, 0.03, 3.0),
                    Vector::<3, T, A>::new(1.0, 0.0, -3.0001),
                ),
                (
                    Vector::<3, T, A>::new(1.0, 0.03, 500.0),
                    Vector::<3, T, A>::new(1.0, 0.0, -499.9),
                ),
            ] {
                assert_float_eq!(
                    vector.slerp(other, t).length(),
                    vector.length().lerp(other.length(), t).abs(),
                    abs <= vector.slerp(other, t).length() * 1e-2 + 1e-2
                );
                assert_float_eq!(
                    vector
                        .normalize()
                        .slerp(other.normalize(), t)
                        .angle_between(vector),
                    T::PI - (vector.angle_between(other) * t.abs() % T::TAU - T::PI).abs(),
                    abs <= 1e-2
                );
                assert_float_eq!(
                    vector
                        .normalize()
                        .slerp(other.normalize(), t)
                        .angle_between(other),
                    T::PI - (vector.angle_between(other) * (1.0 - t).abs() % T::TAU - T::PI).abs(),
                    abs <= 1e-2
                );
            }
            assert_debug_panic!(Vector::<3, T, A>::ZERO.slerp(Vector::ONE, t));
            assert_debug_panic!(Vector::<3, T, A>::ONE.slerp(Vector::ZERO, t));

            for (vector, other) in [
                (
                    Vector::<4, T, A>::new(1.0, 0.3, 1.4, -2.0),
                    Vector::<4, T, A>::new(-1.5, 3.3, -0.3, -0.1),
                ),
                (
                    Vector::<4, T, A>::new(-10.0, 5.3, 3.0, 5.0),
                    Vector::<4, T, A>::new(1.5, 30.3, 1.3, -1.4),
                ),
                (
                    Vector::<4, T, A>::new(1.0, 0.03, 3.0, 4.2),
                    Vector::<4, T, A>::new(1.0, 0.0, 3.0001, 4.2),
                ),
                (
                    Vector::<4, T, A>::new(1.0, 0.03, 500.0, 2.0),
                    Vector::<4, T, A>::new(20.0, 0.0, 499.9, 2.0),
                ),
                (
                    Vector::<4, T, A>::new(1.0, 0.03, 3.0, 4.2),
                    Vector::<4, T, A>::new(1.0, 0.0, -3.0001, 4.2),
                ),
                (
                    Vector::<4, T, A>::new(1.0, 0.03, 500.0, 2.0),
                    Vector::<4, T, A>::new(20.0, 0.0, -499.9, 2.0),
                ),
            ] {
                assert_float_eq!(
                    vector.slerp(other, t).length(),
                    vector.length().lerp(other.length(), t).abs(),
                    abs <= vector.slerp(other, t).length() * 1e-2 + 1e-2
                );
                assert_float_eq!(
                    vector
                        .normalize()
                        .slerp(other.normalize(), t)
                        .angle_between(vector),
                    T::PI - (vector.angle_between(other) * t.abs() % T::TAU - T::PI).abs(),
                    abs <= 1e-2
                );
                assert_float_eq!(
                    vector
                        .normalize()
                        .slerp(other.normalize(), t)
                        .angle_between(other),
                    T::PI - (vector.angle_between(other) * (1.0 - t).abs() % T::TAU - T::PI).abs(),
                    abs <= 1e-2
                );
            }
            assert_debug_panic!(Vector::<4, T, A>::ZERO.slerp(Vector::ONE, t));
            assert_debug_panic!(Vector::<4, T, A>::ONE.slerp(Vector::ZERO, t));
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_parameters!(|T: PrimitiveFloat, A| {
            for max_angle in [0.0, 1.0, -1.0, 1.5, -1.5, 4.0, -4.0, 10.0, -1.0] {
                for (vector, target) in [
                    (
                        Vector::<2, T, A>::new(1.0, 0.3),
                        Vector::<2, T, A>::new(-1.5, 3.3),
                    ),
                    (
                        Vector::<2, T, A>::new(-10.0, 5.3),
                        Vector::<2, T, A>::new(1.5, 30.3),
                    ),
                    (
                        Vector::<2, T, A>::new(1.0, 0.03),
                        Vector::<2, T, A>::new(1.0, 0.0),
                    ),
                    (
                        Vector::<2, T, A>::new(1.0, 0.03),
                        Vector::<2, T, A>::new(20.0, 0.0),
                    ),
                    (
                        Vector::<2, T, A>::new(1.0, 0.03),
                        Vector::<2, T, A>::new(-1.0, 0.0),
                    ),
                    (
                        Vector::<2, T, A>::new(1.0, 0.03),
                        Vector::<2, T, A>::new(-20.0, 0.0),
                    ),
                ] {
                    assert_float_eq!(
                        vector.rotate_towards(target, max_angle).length(),
                        vector.length(),
                        r2nd <= 1e-2
                    );
                    if max_angle >= vector.angle_between(target) {
                        assert_float_eq!(
                            vector.rotate_towards(target, max_angle),
                            target.normalize() * vector.length(),
                            abs <= Vector::splat(1e-2)
                        );
                    } else if max_angle <= vector.angle_between(target) - T::PI {
                        assert_float_eq!(
                            vector.rotate_towards(target, max_angle),
                            -target.normalize() * vector.length(),
                            abs <= Vector::splat(1e-2)
                        );
                    } else {
                        assert_float_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(vector),
                            max_angle.abs(),
                            abs <= 1e-2
                        );
                        assert_float_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(target),
                            vector.angle_between(target) - max_angle,
                            abs <= 1e-2
                        );
                    }
                    assert_float_eq!(
                        vector.rotate_towards(target, -max_angle),
                        vector.rotate_towards(-target, max_angle),
                        abs <= Vector::splat(1e-2)
                    );

                    assert_float_eq!(
                        Vector::<2, T, A>::ZERO.rotate_towards(target, max_angle),
                        Vector::ZERO
                    );
                }
                assert_debug_panic!(Vector::<2, T, A>::ONE.rotate_towards(Vector::ZERO, max_angle));

                for (vector, target) in [
                    (
                        Vector::<3, T, A>::new(1.0, 0.3, 1.4),
                        Vector::<3, T, A>::new(-1.5, 3.3, -0.3),
                    ),
                    (
                        Vector::<3, T, A>::new(-10.0, 5.3, 3.0),
                        Vector::<3, T, A>::new(1.5, 30.3, 1.3),
                    ),
                    (
                        Vector::<3, T, A>::new(1.0, 0.03, 3.0),
                        Vector::<3, T, A>::new(1.0, 0.0, 3.0001),
                    ),
                    (
                        Vector::<3, T, A>::new(1.0, 0.03, 500.0),
                        Vector::<3, T, A>::new(10.0, 0.0, 499.9),
                    ),
                    (
                        Vector::<3, T, A>::new(1.0, 0.03, 3.0),
                        Vector::<3, T, A>::new(1.0, 0.0, -3.0001),
                    ),
                    (
                        Vector::<3, T, A>::new(1.0, 0.03, 500.0),
                        Vector::<3, T, A>::new(1.0, 0.0, -499.9),
                    ),
                ] {
                    assert_float_eq!(
                        vector.rotate_towards(target, max_angle).length(),
                        vector.length(),
                        r2nd <= 1e-2
                    );
                    if max_angle >= vector.angle_between(target) {
                        assert_float_eq!(
                            vector.rotate_towards(target, max_angle),
                            target.normalize() * vector.length(),
                            abs <= Vector::splat(1e-2)
                        );
                    } else if max_angle <= vector.angle_between(target) - T::PI {
                        assert_float_eq!(
                            vector.rotate_towards(target, max_angle),
                            -target.normalize() * vector.length(),
                            abs <= Vector::splat(1e-2)
                        );
                    } else {
                        assert_float_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(vector),
                            max_angle.abs(),
                            abs <= 1e-2
                        );
                        assert_float_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(target),
                            vector.angle_between(target) - max_angle,
                            abs <= 1e-2
                        );
                    }
                    assert_float_eq!(
                        vector.rotate_towards(target, -max_angle),
                        vector.rotate_towards(-target, max_angle),
                        abs <= Vector::splat(1e-2)
                    );

                    assert_float_eq!(
                        Vector::<3, T, A>::ZERO.rotate_towards(target, max_angle),
                        Vector::ZERO
                    );
                }
                assert_debug_panic!(Vector::<3, T, A>::ONE.rotate_towards(Vector::ZERO, max_angle));

                for (vector, target) in [
                    (
                        Vector::<4, T, A>::new(1.0, 0.3, 1.4, -2.0),
                        Vector::<4, T, A>::new(-1.5, 3.3, -0.3, -0.1),
                    ),
                    (
                        Vector::<4, T, A>::new(-10.0, 5.3, 3.0, 5.0),
                        Vector::<4, T, A>::new(1.5, 30.3, 1.3, -1.4),
                    ),
                    (
                        Vector::<4, T, A>::new(1.0, 0.03, 3.0, 4.2),
                        Vector::<4, T, A>::new(1.0, 0.0, 3.0001, 4.2),
                    ),
                    (
                        Vector::<4, T, A>::new(1.0, 0.03, 500.0, 2.0),
                        Vector::<4, T, A>::new(20.0, 0.0, 499.9, 2.0),
                    ),
                    (
                        Vector::<4, T, A>::new(1.0, 0.03, 3.0, 4.2),
                        Vector::<4, T, A>::new(1.0, 0.0, -3.0001, 4.2),
                    ),
                    (
                        Vector::<4, T, A>::new(1.0, 0.03, 500.0, 2.0),
                        Vector::<4, T, A>::new(20.0, 0.0, -499.9, 2.0),
                    ),
                ] {
                    assert_float_eq!(
                        vector.rotate_towards(target, max_angle).length(),
                        vector.length(),
                        r2nd <= 1e-2
                    );
                    if max_angle >= vector.angle_between(target) {
                        assert_float_eq!(
                            vector.rotate_towards(target, max_angle),
                            target.normalize() * vector.length(),
                            abs <= Vector::splat(1e-2)
                        );
                    } else if max_angle <= vector.angle_between(target) - T::PI {
                        assert_float_eq!(
                            vector.rotate_towards(target, max_angle),
                            -target.normalize() * vector.length(),
                            abs <= Vector::splat(1e-2)
                        );
                    } else {
                        assert_float_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(vector),
                            max_angle.abs(),
                            abs <= 1e-2
                        );
                        assert_float_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(target),
                            vector.angle_between(target) - max_angle,
                            abs <= 1e-2
                        );
                    }
                    assert_float_eq!(
                        vector.rotate_towards(target, -max_angle),
                        vector.rotate_towards(-target, max_angle),
                        abs <= Vector::splat(1e-2)
                    );

                    assert_float_eq!(
                        Vector::<4, T, A>::ZERO.rotate_towards(target, max_angle),
                        Vector::ZERO
                    );
                }
                assert_debug_panic!(Vector::<4, T, A>::ONE.rotate_towards(Vector::ZERO, max_angle));
            }
        });
    }

    #[test]
    fn test_length() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).length(),
                (x * x + y * y).sqrt()
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).length(),
                (x * x + y * y + z * z).sqrt()
            );
            assert!(
                float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).length(),
                    (x * x + y * y + z * z + w * w).sqrt()
                ) || float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).length(),
                    (x * x + y * y + (z * z + w * w)).sqrt()
                )
            );
        });
    }

    #[test]
    fn test_distance() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Vector::<2, T, A>::new(x, y).distance(Vector::<2, T, A>::new(z, w)),
                ((x - z) * (x - z) + (y - w) * (y - w)).sqrt()
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(x, y, z).distance(Vector::<3, T, A>::new(z, w, y)),
                ((x - z) * (x - z) + (y - w) * (y - w) + (z - y) * (z - y)).sqrt()
            );
            assert!(
                float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).distance(Vector::<4, T, A>::new(z, w, y, z)),
                    ((x - z) * (x - z) + (y - w) * (y - w) + (z - y) * (z - y) + (w - z) * (w - z))
                        .sqrt()
                ) || float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).distance(Vector::<4, T, A>::new(z, w, y, z)),
                    ((x - z) * (x - z)
                        + (y - w) * (y - w)
                        + ((z - y) * (z - y) + (w - z) * (w - z)))
                        .sqrt()
                )
            );
        });
    }

    #[test]
    fn test_normalize() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if Vector::<2, T, A>::new(x, y).length() != 0.0
                && Vector::<2, T, A>::new(x, y).length().is_finite()
            {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize()
                        * Vector::<2, T, A>::new(x, y).length(),
                    Vector::<2, T, A>::new(x, y),
                    abs <= Vector::<2, T, A>::new(x, y).abs() * 1e-7
                );
            } else {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).normalize());
            }

            if Vector::<3, T, A>::new(x, y, z).length() != 0.0
                && Vector::<3, T, A>::new(x, y, z).length().is_finite()
            {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize()
                        * Vector::<3, T, A>::new(x, y, z).length(),
                    Vector::<3, T, A>::new(x, y, z),
                    abs <= Vector::<3, T, A>::new(x, y, z).abs() * 1e-7
                );
            } else {
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).normalize());
            }

            if Vector::<4, T, A>::new(x, y, z, w).length() != 0.0
                && Vector::<4, T, A>::new(x, y, z, w).length().is_finite()
            {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize()
                        * Vector::<4, T, A>::new(x, y, z, w).length(),
                    Vector::<4, T, A>::new(x, y, z, w),
                    abs <= Vector::<4, T, A>::new(x, y, z, w).abs() * 1e-7
                );
            } else {
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).normalize());
            }
        });
    }

    #[test]
    fn test_try_normalize() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if let Some(try_normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize(),
                    try_normalize,
                    abs <= Vector::splat(1e-7)
                );
            } else {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).normalize());
            }

            if let Some(try_normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize(),
                    try_normalize,
                    abs <= Vector::splat(1e-7)
                );
            } else {
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).normalize());
            }

            if let Some(try_normalize) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize(),
                    try_normalize,
                    abs <= Vector::splat(1e-7)
                );
            } else {
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).normalize());
            }
        });
    }

    #[test]
    fn test_normalize_or() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if let Some(try_normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize_or(Vector::NAN),
                    try_normalize
                );
            } else {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize_or(Vector::NAN),
                    Vector::NAN
                );
            }

            if let Some(try_normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize_or(Vector::NAN),
                    try_normalize
                );
            } else {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize_or(Vector::NAN),
                    Vector::NAN
                );
            }

            if let Some(try_normalize) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_or(Vector::NAN),
                    try_normalize
                );
            } else {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_or(Vector::NAN),
                    Vector::NAN
                );
            }
        });
    }

    #[test]
    fn test_normalize_or_zero() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if let Some(try_normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize_or_zero(),
                    try_normalize
                );
            } else {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize_or_zero(),
                    Vector::ZERO
                );
            }

            if let Some(try_normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize_or_zero(),
                    try_normalize
                );
            } else {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize_or_zero(),
                    Vector::ZERO
                );
            }

            if let Some(try_normalize) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_or_zero(),
                    try_normalize
                );
            } else {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_or_zero(),
                    Vector::ZERO
                );
            }
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if let Some(try_normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize_and_length(),
                    (try_normalize, Vector::<2, T, A>::new(x, y).length())
                );
            } else {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).normalize_and_length(),
                    (Vector::ZERO, 0.0)
                );
            }

            if let Some(try_normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize_and_length(),
                    (try_normalize, Vector::<3, T, A>::new(x, y, z).length())
                );
            } else {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).normalize_and_length(),
                    (Vector::ZERO, 0.0)
                );
            }

            if let Some(try_normalize) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_and_length(),
                    (try_normalize, Vector::<4, T, A>::new(x, y, z, w).length())
                );
            } else {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).normalize_and_length(),
                    (Vector::ZERO, 0.0)
                );
            }
        });
    }

    #[test]
    fn test_is_normalized() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_eq!(
                Vector::<2, T, A>::new(x, y).is_normalized(),
                (1.0 - 1e-4..1.0 + 1e-4).contains(&(x * x + y * y).sqrt())
            );
            assert_eq!(
                Vector::<3, T, A>::new(x, y, z).is_normalized(),
                (1.0 - 1e-4..1.0 + 1e-4).contains(&(x * x + y * y + z * z).sqrt())
            );
            assert_eq!(
                Vector::<4, T, A>::new(x, y, z, w).is_normalized(),
                (1.0 - 1e-4..1.0 + 1e-4).contains(&(x * x + y * y + z * z + w * w).sqrt())
            );
        });
    }

    #[test]
    fn test_with_max_length() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = x + y;
            let a = y + z;

            if !T::is_finite(x) || !T::is_finite(y) || !T::is_finite(z) {
                return;
            }

            if a < 0.0 {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).with_max_length(a));
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).with_max_length(a));
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).with_max_length(a));

                return;
            }

            if (x * x + y * y).sqrt() <= a {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).with_max_length(a),
                    Vector::<2, T, A>::new(x, y)
                );
            } else if let Some(normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).with_max_length(a),
                    normalize * a,
                    abs <= normalize.abs() * a * 1e-6
                );
            }

            if (x * x + y * y + z * z).sqrt() <= a {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).with_max_length(a),
                    Vector::<3, T, A>::new(x, y, z)
                );
            } else if let Some(normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).with_max_length(a),
                    normalize * a,
                    abs <= normalize.abs() * a * 1e-6
                );
            }

            if (x * x + y * y + z * z + w * w).sqrt() <= a {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).with_max_length(a),
                    Vector::<4, T, A>::new(x, y, z, w)
                );
            } else if let Some(normalize) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).with_max_length(a),
                    normalize * a,
                    abs <= normalize.abs() * a * 1e-6
                );
            }
        });
    }

    #[test]
    fn test_with_min_length() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);
            let a = y.max(z);

            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return;
            }

            if a < 0.0 {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).with_min_length(a));
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).with_min_length(a));
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).with_min_length(a));

                return;
            }

            if (x * x + y * y).sqrt() >= a {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).with_min_length(a),
                    Vector::<2, T, A>::new(x, y)
                );
            } else if let Some(normalize) = Vector::<2, T, A>::new(x, y).try_normalize() {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).with_min_length(a),
                    normalize * a,
                    abs <= normalize.abs() * a * 1e-6
                );
            }

            if (x * x + y * y + z * z).sqrt() >= a {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).with_min_length(a),
                    Vector::<3, T, A>::new(x, y, z)
                );
            } else if let Some(normalize) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).with_min_length(a),
                    normalize * a,
                    abs <= normalize.abs() * a * 1e-6
                );
            }

            if (x * x + y * y + z * z + w * w).sqrt() >= a {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).with_min_length(a),
                    Vector::<4, T, A>::new(x, y, z, w)
                );
            } else if let Some(normalize) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).with_min_length(a),
                    normalize * a,
                    abs <= normalize.abs() * a * 1e-6
                );
            }
        });
    }

    #[test]
    fn test_clamp_length() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::midpoint(x, y);
            let a = x.max(y);
            let b = y.max(z);

            if !x.is_finite() || !y.is_finite() || !z.is_finite() {
                return;
            }

            if a < 0.0 || b < a {
                assert_debug_panic!(Vector::<2, T, A>::new(x, y).clamp_length(a, b));
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).clamp_length(a, b));
                assert_debug_panic!(Vector::<4, T, A>::new(x, y, z, w).clamp_length(a, b));

                return;
            }

            if (a..b).contains(&(x * x + y * y).sqrt()) {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).clamp_length(a, b),
                    Vector::<2, T, A>::new(x, y)
                );
            } else if (x * x + y * y).sqrt() >= b {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).clamp_length(a, b),
                    Vector::<2, T, A>::new(x, y).with_max_length(b)
                );
            } else {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).clamp_length(a, b),
                    Vector::<2, T, A>::new(x, y).with_min_length(a)
                );
            }

            if (a..b).contains(&(x * x + y * y + z * z).sqrt()) {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).clamp_length(a, b),
                    Vector::<3, T, A>::new(x, y, z)
                );
            } else if (x * x + y * y + z * z).sqrt() >= b {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).clamp_length(a, b),
                    Vector::<3, T, A>::new(x, y, z).with_max_length(b)
                );
            } else {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).clamp_length(a, b),
                    Vector::<3, T, A>::new(x, y, z).with_min_length(a)
                );
            }

            if (a..b).contains(&(x * x + y * y + z * z + w * w).sqrt()) {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).clamp_length(a, b),
                    Vector::<4, T, A>::new(x, y, z, w)
                );
            } else if (x * x + y * y + z * z + w * w).sqrt() >= b {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).clamp_length(a, b),
                    Vector::<4, T, A>::new(x, y, z, w).with_max_length(b)
                );
            } else {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w).clamp_length(a, b),
                    Vector::<4, T, A>::new(x, y, z, w).with_min_length(a)
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(1.0, 0.0).angle_between(Vector::<2, T, A>::new(1.0, 1.0)),
                T::to_radians(45.0),
                abs <= 1e-5
            );
            assert_float_eq!(
                Vector::<2, T, A>::new(2.0, 0.0)
                    .angle_between(Vector::<2, T, A>::new(1.0, T::sqrt(3.0))),
                T::to_radians(60.0),
                abs <= 1e-5
            );

            assert_float_eq!(
                Vector::<3, T, A>::new(1.0, 0.0, 0.0)
                    .angle_between(Vector::<3, T, A>::new(1.0, 1.0, 0.0)),
                T::to_radians(45.0),
                abs <= 1e-5
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(2.0, 0.0, 0.0).angle_between(Vector::<3, T, A>::new(
                    1.0,
                    T::sqrt(3.0),
                    0.0
                )),
                T::to_radians(60.0),
                abs <= 1e-5
            );

            assert_float_eq!(
                Vector::<4, T, A>::new(1.0, 0.0, 0.0, 0.0)
                    .angle_between(Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0)),
                T::to_radians(45.0),
                abs <= 1e-5
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(2.0, 0.0, 0.0, 0.0).angle_between(Vector::<4, T, A>::new(
                    1.0,
                    T::sqrt(3.0),
                    0.0,
                    0.0
                )),
                T::to_radians(60.0),
                abs <= 1e-5
            );
        });
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x * 2.0) || !T::is_finite(y * 2.0) || !T::is_finite(z * 2.0) {
                return;
            }

            if (x != 0.0 || y != 0.0) && (z != 0.0 || w != 0.0) {
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).angle_between(Vector::<2, T, A>::new(x, y)),
                    0.0,
                    abs <= x.abs().max(y.abs()).max(T::abs(z)) * 1e-5
                );
                assert_float_eq!(
                    Vector::<2, T, A>::new(x, y).angle_between(Vector::<2, T, A>::new(-x, -y)),
                    T::to_radians(180.0),
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5
                );
                assert!((0.0..=T::to_radians(180.0)).contains(
                    &Vector::<2, T, A>::new(x, y).angle_between(Vector::<2, T, A>::new(z, w))
                ));
            }

            if x != 0.0 || z != 0.0 || y != 0.0 && w != 0.0 {
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z).angle_between(Vector::<3, T, A>::new(x, y, z)),
                    0.0,
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5
                );
                assert_float_eq!(
                    Vector::<3, T, A>::new(x, y, z)
                        .angle_between(Vector::<3, T, A>::new(-x, -y, -z)),
                    T::to_radians(180.0),
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5
                );
                assert!((0.0..=T::to_radians(180.0)).contains(
                    &Vector::<3, T, A>::new(x, y, z).angle_between(Vector::<3, T, A>::new(z, w, x))
                ));
            }

            if z != 0.0 || w != 0.0 || x != 0.0 && y != 0.0 {
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w)
                        .angle_between(Vector::<4, T, A>::new(x, y, z, w)),
                    0.0,
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5
                );
                assert_float_eq!(
                    Vector::<4, T, A>::new(x, y, z, w)
                        .angle_between(Vector::<4, T, A>::new(-x, -y, -z, -w)),
                    T::to_radians(180.0),
                    abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5
                );
                assert!(
                    (0.0..=T::to_radians(180.0)).contains(
                        &Vector::<4, T, A>::new(x, y, z, w)
                            .angle_between(Vector::<4, T, A>::new(z, w, x, y))
                    )
                );
            }
        });
    }

    #[test]
    fn test_project_onto() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(4.0, 0.0).project_onto(Vector::<2, T, A>::new(1.0, 1.0)),
                Vector::<2, T, A>::new(2.0, 2.0)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .project_onto(Vector::<3, T, A>::new(1.0, 1.0, 0.0)),
                Vector::<3, T, A>::new(2.0, 2.0, 0.0)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0)
                    .project_onto(Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0)),
                Vector::<4, T, A>::new(2.0, 2.0, 0.0, 0.0)
            );

            assert_debug_panic!(Vector::<2, T, A>::new(4.0, 0.0).project_onto(Vector::ZERO));
            assert_debug_panic!(Vector::<3, T, A>::new(4.0, 0.0, 0.0).project_onto(Vector::ZERO));
            assert_debug_panic!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0).project_onto(Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_project_onto_normalized() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(4.0, 0.0)
                    .project_onto_normalized(Vector::<2, T, A>::new(1.0, 1.0).normalize()),
                Vector::<2, T, A>::new(2.0, 2.0),
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .project_onto_normalized(Vector::<3, T, A>::new(1.0, 1.0, 0.0).normalize()),
                Vector::<3, T, A>::new(2.0, 2.0, 0.0),
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0).project_onto_normalized(
                    Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0).normalize()
                ),
                Vector::<4, T, A>::new(2.0, 2.0, 0.0, 0.0),
                abs <= Vector::splat(1e-5)
            );

            assert_debug_panic!(
                Vector::<2, T, A>::new(4.0, 0.0).project_onto_normalized(Vector::ONE)
            );
            assert_debug_panic!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0).project_onto_normalized(Vector::ONE)
            );
            assert_debug_panic!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0).project_onto_normalized(Vector::ONE)
            );
        });
    }

    #[test]
    fn test_reject_from() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(4.0, 0.0).reject_from(Vector::<2, T, A>::new(1.0, 1.0)),
                Vector::<2, T, A>::new(2.0, -2.0)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .reject_from(Vector::<3, T, A>::new(1.0, 1.0, 0.0)),
                Vector::<3, T, A>::new(2.0, -2.0, 0.0)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0)
                    .reject_from(Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0)),
                Vector::<4, T, A>::new(2.0, -2.0, 0.0, 0.0)
            );

            assert_debug_panic!(Vector::<2, T, A>::new(4.0, 0.0).reject_from(Vector::ZERO));
            assert_debug_panic!(Vector::<3, T, A>::new(4.0, 0.0, 1.2).reject_from(Vector::ZERO));
            assert_debug_panic!(
                Vector::<4, T, A>::new(4.0, 0.0, 1.2, 5.4).reject_from(Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_reject_from_normalized() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(4.0, 0.0)
                    .reject_from_normalized(Vector::<2, T, A>::new(1.0, 1.0).normalize()),
                Vector::<2, T, A>::new(2.0, -2.0),
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .reject_from_normalized(Vector::<3, T, A>::new(1.0, 1.0, 0.0).normalize()),
                Vector::<3, T, A>::new(2.0, -2.0, 0.0),
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0)
                    .reject_from_normalized(Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0).normalize()),
                Vector::<4, T, A>::new(2.0, -2.0, 0.0, 0.0),
                abs <= Vector::splat(1e-5)
            );

            assert_debug_panic!(
                Vector::<2, T, A>::new(4.0, 0.0).reject_from_normalized(Vector::ONE)
            );
            assert_debug_panic!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0).reject_from_normalized(Vector::ONE)
            );
            assert_debug_panic!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0).reject_from_normalized(Vector::ONE)
            );
        });
    }

    #[test]
    fn test_reflect() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(3.0, 2.0).reflect(Vector::<2, T, A>::ONE.normalize()),
                Vector::<2, T, A>::new(-2.0, -3.0),
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(3.0, 2.0, 4.0).reflect(Vector::<3, T, A>::ONE.normalize()),
                Vector::<3, T, A>::new(-3.0, -4.0, -2.0),
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(3.0, 2.0, 4.0, 5.0)
                    .reflect(Vector::<4, T, A>::ONE.normalize()),
                Vector::<4, T, A>::new(-4.0, -5.0, -3.0, -2.0),
                abs <= Vector::splat(1e-5)
            );

            assert_debug_panic!(Vector::<2, T, A>::new(3.0, 2.0).reflect(Vector::<2, T, A>::ONE));
            assert_debug_panic!(
                Vector::<3, T, A>::new(3.0, 2.0, 4.0).reflect(Vector::<3, T, A>::ONE)
            );
            assert_debug_panic!(
                Vector::<4, T, A>::new(3.0, 2.0, 4.0, 5.0).reflect(Vector::<4, T, A>::ONE)
            );
        });
    }

    #[test]
    fn test_refract() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::new(1.0, -T::sqrt(3.0))
                    .normalize()
                    .refract(Vector::<2, T, A>::Y, T::recip(1.5)),
                Vector::<2, T, A>::new(1.0, -T::sqrt(8.0)).normalize(),
                abs <= Vector::splat(1e-8)
            );
            assert_float_eq!(
                Vector::<2, T, A>::new(2.0, -T::sqrt(3.0))
                    .normalize()
                    .refract(Vector::<2, T, A>::Y, 1.5),
                Vector::ZERO
            );

            assert_float_eq!(
                Vector::<3, T, A>::new(1.0, -T::sqrt(3.0), 0.0)
                    .normalize()
                    .refract(Vector::<3, T, A>::Y, T::recip(1.5)),
                Vector::<3, T, A>::new(1.0, -T::sqrt(8.0), 0.0).normalize(),
                abs <= Vector::splat(1e-8)
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(2.0, -T::sqrt(3.0), 0.0)
                    .normalize()
                    .refract(Vector::<3, T, A>::Y, 1.5),
                Vector::ZERO
            );

            assert_float_eq!(
                Vector::<4, T, A>::new(1.0, -T::sqrt(3.0), 0.0, 0.0)
                    .normalize()
                    .refract(Vector::<4, T, A>::Y, T::recip(1.5)),
                Vector::<4, T, A>::new(1.0, -T::sqrt(8.0), 0.0, 0.0).normalize(),
                abs <= Vector::splat(1e-8)
            );
            assert_float_eq!(
                Vector::<4, T, A>::new(2.0, -T::sqrt(3.0), 0.0, 0.0)
                    .normalize()
                    .refract(Vector::<4, T, A>::Y, 1.5),
                Vector::ZERO
            );

            assert_debug_panic!(
                Vector::<2, T, A>::new(1.0, -T::sqrt(3.0))
                    .refract(Vector::<2, T, A>::Y, T::recip(1.5))
            );
            assert_debug_panic!(
                Vector::<2, T, A>::new(1.0, -T::sqrt(3.0))
                    .normalize()
                    .refract(Vector::<2, T, A>::Y * 2.0, T::recip(1.5))
            );

            assert_debug_panic!(
                Vector::<3, T, A>::new(1.0, -T::sqrt(3.0), 0.0)
                    .refract(Vector::<3, T, A>::Y, T::recip(1.5))
            );
            assert_debug_panic!(
                Vector::<3, T, A>::new(1.0, -T::sqrt(3.0), 0.0)
                    .normalize()
                    .refract(Vector::<3, T, A>::Y * 2.0, T::recip(1.5))
            );

            assert_debug_panic!(
                Vector::<4, T, A>::new(1.0, -T::sqrt(3.0), 0.0, 0.0)
                    .refract(Vector::<4, T, A>::Y, T::recip(1.5))
            );
            assert_debug_panic!(
                Vector::<4, T, A>::new(1.0, -T::sqrt(3.0), 0.0, 0.0)
                    .normalize()
                    .refract(Vector::<4, T, A>::Y * 2.0, T::recip(1.5))
            );
        });
    }

    #[test]
    fn test_any_orthogonal_vector() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x + y;

            let vector = Vector::<2, T, A>::new(x, y);
            assert_float_eq!(vector.any_orthogonal_vector(), vector.perp());
            if vector != Vector::ZERO {
                assert!(vector.any_orthogonal_vector() != Vector::ZERO);
            }

            let vector = Vector::<3, T, A>::new(x, y, z);
            if vector.length().is_finite() {
                assert_float_eq!(vector.any_orthogonal_vector().dot(vector), 0.0);
                if vector != Vector::ZERO {
                    assert!(vector.any_orthogonal_vector() != Vector::ZERO);
                }
            }

            let vector = Vector::<4, T, A>::new(x, y, z, w);
            if vector.length().is_finite() {
                assert_float_eq!(vector.any_orthogonal_vector().dot(vector), 0.0);
                if vector != Vector::ZERO {
                    assert!(vector.any_orthogonal_vector() != Vector::ZERO);
                }
            }
        });
    }

    #[test]
    fn test_any_orthonormal_vector() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x * 0.3 + y * 0.1;

            let vector = Vector::<2, T, A>::new(x, y);
            if let Some(vector) = vector.try_normalize() {
                assert_float_eq!(vector.any_orthonormal_vector(), vector.perp());
            }
            if !vector.is_normalized() {
                assert_debug_panic!(vector.any_orthonormal_vector());
            }

            let vector = Vector::<3, T, A>::new(x, y, z);
            if let Some(vector) = vector.try_normalize() {
                assert_float_eq!(
                    vector.any_orthonormal_vector().dot(vector),
                    0.0,
                    abs <= 1e-5 * x.abs().max(y.abs()).max(z.abs()),
                    0.0 = -0.0
                );
                assert!(vector.any_orthonormal_vector().is_normalized());
            }
            if !vector.is_normalized() {
                assert_debug_panic!(vector.any_orthonormal_vector());
            }

            let vector = Vector::<4, T, A>::new(x, y, z, w);
            if let Some(vector) = vector.try_normalize() {
                assert_float_eq!(
                    vector.any_orthonormal_vector().dot(vector),
                    0.0,
                    abs <= 1e-5 * x.abs().max(y.abs()).max(z.abs()),
                    0.0 = -0.0
                );
                assert!(vector.any_orthonormal_vector().is_normalized());
            }
            if !vector.is_normalized() {
                assert_debug_panic!(vector.any_orthonormal_vector());
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat| {
            assert!(Vec2::<T>::new(0.0, 1.0).abs_diff_eq(Vec2::new(0.0, 1.0), 0.125));
            assert!(Vec2::<T>::new(0.0, 1.0).abs_diff_eq(Vec2::new(0.1, 0.9), 0.125));
            assert!(Vec2::<T>::new(5.0, 1.0).abs_diff_eq(Vec2::new(4.9, 1.0), 0.125));
            assert!(!Vec2::<T>::new(0.0, 1.0).abs_diff_eq(Vec2::new(0.2, 1.0), 0.125));
            assert!(!Vec2::<T>::new(0.0, 1.0).abs_diff_eq(Vec2::new(0.1, 0.8), 0.125));
            assert!(!Vec2::<T>::new(5.0, 1.0).abs_diff_eq(Vec2::new(4.5, 0.0), 0.125));
        });
    }

    #[test]
    fn test_angle_to() {
        for_parameters!(|T: PrimitiveFloat, A, x, y| {
            let _: [T; 2] = [x, y];
            let start = Vector::<2, T, A>::new(x, y);
            let end = Vector::<2, T, A>::new(x * 1.3 + y, x + y * 0.1);
            let angle = start.angle_to(end);

            if let Some(start) = start.try_normalize()
                && let Some(end) = end.try_normalize()
            {
                assert_float_eq!(
                    start.rotate(angle),
                    end,
                    abs <= Vector::splat(1e-4) * x.abs().max(y.abs())
                );
            }
        });
    }

    #[test]
    fn test_angle_from() {
        for_parameters!(|T: PrimitiveFloat, A, x, y| {
            let _: [T; 2] = [x, y];
            let start = Vector::<2, T, A>::new(x, y);
            let end = Vector::<2, T, A>::new(x * 1.3 + y, x + y * 0.1);
            let angle = end.angle_from(start);

            if let Some(start) = start.try_normalize()
                && let Some(end) = end.try_normalize()
            {
                assert_float_eq!(
                    start.rotate(angle),
                    end,
                    abs <= Vector::splat(1e-4) * x.abs().max(y.abs())
                );
            }
        });
    }

    #[test]
    fn test_rotate() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<2, T, A>::X.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::Y,
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<2, T, A>::Y.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::NEG_X,
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<2, T, A>::NEG_X.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::NEG_Y,
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<2, T, A>::NEG_Y.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::X,
                abs <= Vector::splat(1e-5)
            );
            assert_float_eq!(
                Vector::<2, T, A>::new(2.0, 0.0).rotate(T::to_radians(45.0)),
                Vector::<2, T, A>::new(2.0, 2.0).sqrt(),
                abs <= Vector::splat(1e-5)
            );
        });
    }

    #[test]
    fn test_from_homogeneous() {
        for_parameters!(|T: PrimitiveFloat, A, x| {
            let _: T = x;
            let [y, z, w] = [x + 1.0, x + 2.0, x + 3.0];

            assert_float_eq!(
                Vector::<3, T, A>::from_homogeneous(Vector::<4, T, A>::new(x, y, z, w)),
                Vector::<3, T, A>::new(x, y, z) / w
            );
        });
    }

    #[test]
    fn test_rotate_x() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<3, T, A>::X.rotate_x(T::to_radians(45.0)),
                Vector::<3, T, A>::X,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::Y.rotate_x(T::to_radians(90.0)),
                Vector::<3, T, A>::Z,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(0.0, -2.0, 0.0).rotate_x(T::to_radians(45.0)),
                -Vector::<3, T, A>::new(0.0, 2.0, 2.0).sqrt(),
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_rotate_y() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<3, T, A>::Y.rotate_y(T::to_radians(45.0)),
                Vector::<3, T, A>::Y,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::Z.rotate_y(T::to_radians(90.0)),
                Vector::<3, T, A>::X,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(0.0, 0.0, -2.0).rotate_y(T::to_radians(45.0)),
                -Vector::<3, T, A>::new(2.0, 0.0, 2.0).sqrt(),
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_rotate_z() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Vector::<3, T, A>::Z.rotate_z(T::to_radians(45.0)),
                Vector::<3, T, A>::Z,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::X.rotate_z(T::to_radians(90.0)),
                Vector::<3, T, A>::Y,
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, T, A>::new(-2.0, 0.0, 0.0).rotate_z(T::to_radians(45.0)),
                -Vector::<3, T, A>::new(2.0, 2.0, 0.0).sqrt(),
                abs <= Vector::splat(1e-5),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_any_orthonormal_pair() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            if !T::is_finite(x * 2.0)
                || !T::is_finite(y * 2.0)
                || !T::is_finite(z * 2.0)
                || x == 0.0 && y == 0.0 && z == 0.0
            {
                return;
            }

            let result = Vector::<3, T, A>::new(x, y, z)
                .normalize()
                .any_orthonormal_pair();
            assert_float_eq!(
                result.0.dot(Vector::<3, T, A>::new(x, y, z)),
                0.0,
                abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5,
                0.0 = -0.0
            );
            assert_float_eq!(
                result.1.dot(Vector::<3, T, A>::new(x, y, z)),
                0.0,
                abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5,
                0.0 = -0.0
            );
            assert_float_eq!(
                result.0.dot(result.1),
                0.0,
                abs <= x.abs().max(y.abs()).max(z.abs()) * 1e-5,
                0.0 = -0.0
            );
            assert!(result.0.is_normalized());
            assert!(result.1.is_normalized());

            if !Vector::<3, T, A>::new(x, y, z).is_normalized() {
                assert_debug_panic!(Vector::<3, T, A>::new(x, y, z).any_orthonormal_pair());
            }
        });
    }
}
