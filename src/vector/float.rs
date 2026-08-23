use core::cmp::Ordering;

use crate::{
    Alignment, FloatExt, Length, Mask, PrimitiveFloat, Quaternion, SupportedLength, Vector,
    backend::FloatVectorBackend,
    utils::{PrimitiveFloatUtils, specialize, transmute_generic},
};

type Bits<T> = <T as PrimitiveFloat>::Bits;

impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveFloat,
{
    /// A vector with all elements set to [`MIN`].
    ///
    /// [`MIN`]: f32::MIN
    pub const MIN: Self = Self::splat(T::MIN);

    /// A vector with all elements set to [`MAX`].
    ///
    /// [`MAX`]: f32::MAX
    pub const MAX: Self = Self::splat(T::MAX);

    /// A vector with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::splat(T::NAN);

    /// A vector with all elements set to [`INFINITY`].
    ///
    /// [`INFINITY`]: f32::INFINITY
    pub const INFINITY: Self = Self::splat(T::INFINITY);

    /// A vector with all elements set to [`NEG_INFINITY`].
    ///
    /// [`NEG_INFINITY`]: f32::NEG_INFINITY
    pub const NEG_INFINITY: Self = Self::splat(T::NEG_INFINITY);

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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_nan_mask(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_finite_mask(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_sign_positive_mask(
            self
        ))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_sign_negative_mask(
            self
        ))
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
            "cannot compare NaN: {self:?}.max({other:?})"
        );

        specialize!(<T as FloatVectorBackend<N, A>>::vector_max(self, other))
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
            "cannot compare NaN: {self:?}.min({other:?})"
        );

        specialize!(<T as FloatVectorBackend<N, A>>::vector_min(self, other))
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
            min.le_mask(max).all() && !self.is_nan() && !min.is_nan() && !max.is_nan(),
            "min > max, or either was NaN: {self:?}.clamp({min:?}, {max:?})"
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
        debug_assert!(!self.is_nan(), "cannot compare NaN: {self:?}.max_element()");

        specialize!(<T as FloatVectorBackend<N, A>>::vector_max_element(self))
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
        debug_assert!(!self.is_nan(), "cannot compare NaN: {self:?}.min_element()");

        specialize!(<T as FloatVectorBackend<N, A>>::vector_min_element(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_abs(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_signum(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_copysign(self, sign))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_floor(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_ceil(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_round(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_trunc(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_mul_add(self, a, b))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_div_euclid(
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_rem_euclid(
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_powf(self, n))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_sqrt(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_exp(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_exp2(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_ln(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_log2(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_sin(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_cos(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_tan(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_asin(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_acos(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_atan(self))
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
        specialize!(<T as FloatVectorBackend<N, A>>::vector_sin_cos(self))
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
        self * (T::ONE - t) + other * t
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
        specialize!(Vector::<N, T, A>::slerp_backend(self, other, t))
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
        specialize!(Vector::<N, T, A>::rotate_towards_backend(
            self, target, max_angle
        ))
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
        PrimitiveFloatUtils::sqrt(self.dot(self))
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
        let result = self / self.length();

        debug_assert!(
            result.is_finite() && result != Self::ZERO,
            "vector is zero or non-finite: {self:?}.normalize()"
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
        let recip = T::ONE / self.length();
        if recip.is_finite() && recip > T::ZERO {
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
    /// If `self` is a zero vector, the result is length `0` and an unspecified
    /// vector. Consider manually checking for `length == 0.0`.
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
        (self / length, length)
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
        (self.length_squared() - T::ONE).abs() <= T::as_from(2e-4)
    }

    /// Returns `self` with a length of no more than `max`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `max` is negative or `self` cannot be normalized.
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
        debug_assert!(
            matches!(
                max.partial_cmp(&T::ZERO),
                None | Some(Ordering::Greater | Ordering::Equal)
            ),
            "negative maximum length: {self:?}.with_max_length({max:?})"
        );

        let length_squared = self.length_squared();
        if length_squared > max * max {
            let normalized = self / PrimitiveFloatUtils::sqrt(length_squared);

            debug_assert!(
                normalized.is_finite() && normalized != Self::ZERO,
                "vector cannot be normalized: {self:?}.with_max_length({max:?})"
            );

            normalized * max
        } else {
            self
        }
    }

    /// Returns `self` with a length of no less than `min`.
    ///
    /// If `min` is negative, this returns `self`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` cannot be normalized.
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
        let length_squared = self.length_squared();
        if length_squared < min * min.abs() {
            let normalized = self / PrimitiveFloatUtils::sqrt(length_squared);

            debug_assert!(
                normalized.is_finite() && normalized != Self::ZERO,
                "vector cannot be normalized: {self:?}.with_min_length({min:?})"
            );

            normalized * min
        } else {
            self
        }
    }

    /// Returns `self` with a length of no less than `min` and no more than
    /// `max`.
    ///
    /// If `min` is negative it is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `min > max`, `max` is negative or `self` cannot be normalized.
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
        debug_assert!(
            matches!(
                max.partial_cmp(&T::ZERO),
                None | Some(Ordering::Greater | Ordering::Equal)
            ) && matches!(
                min.partial_cmp(&max),
                None | Some(Ordering::Less | Ordering::Equal)
            ),
            "max_length < min_length or max_length < 0: {self:?}.clamp_length({min:?}, {max:?})"
        );

        let length_squared = self.length_squared();
        if length_squared < min * min.abs() {
            let normalized = self / PrimitiveFloatUtils::sqrt(length_squared);

            debug_assert!(
                normalized.is_finite() && normalized != Self::ZERO,
                "invalid vector: {self:?}.clamp_length({min:?}, {max:?})"
            );

            normalized * min
        } else if length_squared > max * max {
            let normalized = self / PrimitiveFloatUtils::sqrt(length_squared);

            debug_assert!(
                normalized.is_finite() && normalized != Self::ZERO,
                "invalid vector: {self:?}.clamp_length({min:?}, {max:?})"
            );

            normalized * max
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
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are zero vectors.
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
        let length_product =
            PrimitiveFloatUtils::sqrt(self.length_squared() * other.length_squared());

        debug_assert!(
            length_product.recip().is_finite(),
            "vectors cannot be normalized: {self:?}.angle_between({other:?})"
        );

        (self.dot(other) / length_product).acos_approx()
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

        debug_assert!(
            other_length_squared_recip.is_finite(),
            "other cannot be normalized: {self:?}.project_onto({other:?})"
        );

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
        debug_assert!(
            other.is_normalized(),
            "other is not normalized: {self:?}.project_onto_normalized({other:?})"
        );

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
        let other_length_squared_recip = other.length_squared().recip();

        debug_assert!(
            other_length_squared_recip.is_finite(),
            "other cannot be normalized: {self:?}.reject_from({other:?})"
        );

        self - other * self.dot(other) * other_length_squared_recip
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
        debug_assert!(
            other.is_normalized(),
            "other is not normalized: {self:?}.reject_from_normalized({other:?})"
        );

        self - other * self.dot(other)
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
        debug_assert!(
            normal.is_normalized(),
            "normal is not normalized: {self:?}.reflect({normal:?})"
        );

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
        debug_assert!(
            self.is_normalized() && normal.is_normalized(),
            "vector or normal are not normalized: {self:?}.refract({normal:?}, {eta:?})"
        );

        let self_dot_normal = self.dot(normal);
        let k = T::ONE - eta * eta * (T::ONE - self_dot_normal * self_dot_normal);
        if k >= T::ZERO {
            self * eta - normal * (eta * self_dot_normal + PrimitiveFloatUtils::sqrt(k))
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
        specialize!(Vector::<N, T, A>::any_orthogonal_vector_backend(self))
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
        debug_assert!(
            self.is_normalized(),
            "vector is not normalized: {self:?}.any_orthonormal_vector()"
        );

        specialize!(Vector::<N, T, A>::any_orthonormal_vector_backend(self))
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

    /// Raw transmutation to unsigned integer vector.
    ///
    /// Note that this function is distinct from [`as`] conversions, which
    /// attempt to preserve the *numeric* value, and not the bitwise value.
    ///
    /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
    #[inline]
    #[must_use]
    pub const fn to_bits(self) -> Vector<N, Bits<T>, A> {
        if const { size_of::<Vector<N, T, A>>() == size_of::<Vector<N, Bits<T>, A>>() } {
            // SAFETY: Both types accept all bit-patterns.
            unsafe { transmute_generic::<Vector<N, T, A>, Vector<N, Bits<T>, A>>(self) }
        } else {
            // SAFETY: Both types accept all bit-patterns.
            Vector::from_array(unsafe {
                transmute_generic::<[T; N], [Bits<T>; N]>(self.to_array())
            })
        }
    }

    /// Raw transmutation from unsigned integer vector.
    ///
    /// Note that this function is distinct from [`as`] conversions, which
    /// attempt to preserve the *numeric* value, and not the bitwise value.
    ///
    /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
    #[inline]
    #[must_use]
    pub const fn from_bits(value: Vector<N, Bits<T>, A>) -> Self {
        if const { size_of::<Vector<N, T, A>>() == size_of::<Vector<N, Bits<T>, A>>() } {
            // SAFETY: Both types accept all bit-patterns.
            unsafe { transmute_generic::<Vector<N, Bits<T>, A>, Vector<N, T, A>>(value) }
        } else {
            // SAFETY: Both types accept all bit-patterns.
            Vector::from_array(unsafe {
                transmute_generic::<[Bits<T>; N], [T; N]>(value.to_array())
            })
        }
    }
}

impl<T, A: Alignment> Vector<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a 2D vector from homogeneous coordinates by performing
    /// perspective divide.
    ///
    /// Equivalent to `homogeneous.xy / homogeneous.z`.
    #[inline]
    #[must_use]
    pub fn from_homogeneous(homogeneous: Vector<3, T, A>) -> Self {
        homogeneous.xy() / homogeneous.z
    }

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
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are zero vectors.
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
    #[track_caller]
    pub fn angle_to(self, other: Self) -> T {
        let length_product =
            PrimitiveFloatUtils::sqrt(self.length_squared() * other.length_squared());

        debug_assert!(
            length_product.recip().is_finite(),
            "vectors cannot be normalized: {self:?}.angle_to({other:?})"
        );

        let angle_between = (self.dot(other) / length_product).acos_approx();
        let outer_product = self.x * other.y - self.y * other.x;
        angle_between * outer_product.signum()
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
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are zero vectors.
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
        let length_product =
            PrimitiveFloatUtils::sqrt(self.length_squared() * other.length_squared());

        debug_assert!(
            length_product.recip().is_finite(),
            "vectors cannot be normalized: {self:?}.angle_from({other:?})"
        );

        let angle_between = (self.dot(other) / length_product).acos_approx();
        let outer_product = other.x * self.y - other.y * self.x;
        angle_between * outer_product.signum()
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
        let (angle_sin, angle_cos) = PrimitiveFloatUtils::sin_cos(angle);
        Self::new(
            self.x * angle_cos - self.y * angle_sin,
            self.x * angle_sin + self.y * angle_cos,
        )
    }

    #[track_caller]
    #[inline(always)]
    fn slerp_backend(self, other: Self, t: T) -> Self {
        let self_length = self.length();
        let other_length = other.length();

        debug_assert!(
            self_length >= T::as_from(1e-7) && other_length >= T::as_from(1e-7),
            "zero vector: {self:?}.slerp({other:?})"
        );

        let self_normalized = self / self_length;
        let angle_cos = self_normalized.dot(other) / other_length;
        let angle = angle_cos.acos_approx() * self_normalized.wedge(other).signum();

        let result_length = self_length.lerp(other_length, t);
        self_normalized.rotate(angle * t) * result_length
    }

    #[track_caller]
    #[inline(always)]
    fn rotate_towards_backend(self, target: Self, max_angle: T) -> Self {
        let self_length = self.length();
        let target_length = target.length();

        debug_assert!(
            target_length >= T::as_from(1e-7),
            "target is zero: {self:?}.rotate_towards({target:?}, {max_angle:?})"
        );

        if self == Self::ZERO {
            return self;
        }

        let target_angle = (self.dot(target) / self_length / target_length).acos_approx();
        let angle_sign = self.wedge(target).signum();
        let angle = if max_angle < target_angle - T::PI {
            target_angle - T::PI
        } else if max_angle > target_angle {
            target_angle
        } else {
            max_angle
        } * angle_sign;

        self.rotate(angle)
    }

    #[inline(always)]
    fn any_orthogonal_vector_backend(self) -> Self {
        self.perp()
    }

    #[inline(always)]
    fn any_orthonormal_vector_backend(self) -> Self {
        self.perp()
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
    #[inline]
    #[must_use]
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
        let (angle_sin, angle_cos) = PrimitiveFloatUtils::sin_cos(angle);
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
        let (angle_sin, angle_cos) = PrimitiveFloatUtils::sin_cos(angle);
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
        let (angle_sin, angle_cos) = PrimitiveFloatUtils::sin_cos(angle);
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
        debug_assert!(
            self.is_normalized(),
            "vector is not normalized: {self:?}.any_orthonormal_pair()"
        );

        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = self.z.signum();
        let a = T::NEG_ONE / (sign + self.z);
        let b = self.x * self.y * a;
        (
            Self::new(
                T::ONE + sign * self.x * self.x * a,
                sign * b,
                -sign * self.x,
            ),
            Self::new(b, sign + self.y * self.y * a, -self.y),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn slerp_backend(self, other: Self, t: T) -> Self {
        // Ported from `https://github.com/bitshifter/glam-rs`.

        let self_length = self.length();
        let other_length = other.length();

        debug_assert!(
            self_length >= T::as_from(1e-7) && other_length >= T::as_from(1e-7),
            "zero vector: {self:?}.slerp({other:?})"
        );

        let angle_cos = self.dot(other) / (self_length * other_length);

        // If `angle_cos` is close to `1` or `-1` or is NaN the normal
        // calculation breaks down.
        if angle_cos.abs() < T::as_from(1.0 - 3e-7) {
            let angle = angle_cos.acos_approx();
            let angle_sin = PrimitiveFloatUtils::sin(angle);
            let self_factor = PrimitiveFloatUtils::sin(angle * (T::ONE - t));
            let other_factor = PrimitiveFloatUtils::sin(angle * t);

            let result_length = self_length.lerp(other_length, t);

            (self * (result_length / self_length) * self_factor
                + other * (result_length / other_length) * other_factor)
                / angle_sin
        } else if angle_cos.is_sign_negative() {
            // Vectors are almost parallel in opposing directions.

            let axis = self.any_orthogonal_vector().normalize();
            let rotation = Quaternion::<T, A>::from_axis_angle(axis, t * T::PI);

            let result_length = self_length.lerp(other_length, t);
            self * rotation * (result_length / self_length)
        } else {
            // Vectors are almost parallel in the same direction.
            self.lerp(other, t)
        }
    }

    #[track_caller]
    #[inline(always)]
    fn rotate_towards_backend(self, target: Self, max_angle: T) -> Self {
        // Ported from `https://github.com/bitshifter/glam-rs`.

        let self_length = self.length();
        let target_length = target.length();

        debug_assert!(
            target_length >= T::as_from(1e-7),
            "target is zero: {self:?}.rotate_towards({target:?}, {max_angle:?})"
        );

        if self == Self::ZERO {
            return self;
        }

        let target_angle = (self.dot(target) / (self_length * target_length)).acos_approx();
        let angle = if max_angle < target_angle - T::PI {
            target_angle - T::PI
        } else if max_angle > target_angle {
            target_angle
        } else {
            max_angle
        };
        let axis = self
            .cross(target)
            .try_normalize()
            .unwrap_or_else(|| self.any_orthonormal_vector());

        self * Quaternion::<T, A>::from_axis_angle(axis, angle)
    }

    #[track_caller]
    #[inline(always)]
    fn any_orthogonal_vector_backend(self) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs.

        if self.x.abs() > self.y.abs() {
            Self::new(-self.z, T::ZERO, self.x)
        } else {
            Self::new(T::ZERO, self.z, -self.y)
        }
    }

    #[inline(always)]
    fn any_orthonormal_vector_backend(self) -> Self {
        // Ported from https://github.com/bitshifter/glam-rs.

        let sign = self.z.signum();
        let a = T::NEG_ONE / (sign + self.z);
        let b = self.x * self.y * a;

        Self::new(b, sign + self.y * self.y * a, -self.y)
    }
}

impl<T, A: Alignment> Vector<4, T, A>
where
    T: PrimitiveFloat,
{
    #[track_caller]
    #[inline(always)]
    fn slerp_backend(self, other: Self, t: T) -> Self {
        // Ported from `https://github.com/bitshifter/glam-rs`.

        let self_length = self.length();
        let other_length = other.length();

        debug_assert!(
            self_length >= T::as_from(1e-7) && other_length >= T::as_from(1e-7),
            "zero vector: {self:?}.slerp({other:?})"
        );

        let angle_cos = self.dot(other) / (self_length * other_length);

        // If `angle_cos` is close to `1` or `-1` or is NaN the normal
        // calculation breaks down.
        if angle_cos.abs() < T::as_from(1.0 - 3e-7) {
            let angle = angle_cos.acos_approx();
            let angle_sin = PrimitiveFloatUtils::sin(angle);
            let t1 = PrimitiveFloatUtils::sin(angle * (T::ONE - t));
            let t2 = PrimitiveFloatUtils::sin(angle * t);

            let result_length = self_length.lerp(other_length, t);

            (self * (result_length / self_length) * t1
                + other * (result_length / other_length) * t2)
                / angle_sin
        } else if angle_cos.is_sign_negative() {
            // Vectors are almost parallel in opposing directions.

            let axis = self.any_orthogonal_vector().normalize();
            let (sin, cos) = PrimitiveFloatUtils::sin_cos(t * T::PI);

            let result_dir = self * cos + axis * sin;
            let result_length = self_length.lerp(other_length, t);
            result_dir * (result_length / result_dir.length())
        } else {
            // Vectors are almost parallel in the same direction.
            self.lerp(other, t)
        }
    }

    #[track_caller]
    #[inline(always)]
    fn rotate_towards_backend(self, target: Self, max_angle: T) -> Self {
        let self_length = self.length();
        let target_length = target.length();

        debug_assert!(
            target_length >= T::as_from(1e-7),
            "target is zero: {self:?}.rotate_towards({target:?}, {max_angle:?})"
        );

        if self == Self::ZERO {
            return self;
        }

        let target_angle_cos = self.dot(target) / (self_length * target_length);
        let target_angle = target_angle_cos.acos_approx();
        let angle = if max_angle < target_angle - T::PI {
            target_angle - T::PI
        } else if max_angle > target_angle {
            target_angle
        } else {
            max_angle
        };

        if angle == T::ZERO {
            return self;
        }

        // If `target_angle_cos` is close to `1` or `-1` or is NaN the
        // normal calculation breaks down.
        if target_angle_cos.abs() <= T::as_from(1.0 - 3e-7) {
            let self_factor = PrimitiveFloatUtils::sin(target_angle - angle);
            let target_factor = PrimitiveFloatUtils::sin(angle);

            (self * self_factor + target * (self_length / target_length) * target_factor)
                .normalize()
                * self_length
        } else if target_angle_cos.is_sign_negative() {
            // Vectors are almost parallel in opposing directions.

            let axis = self.any_orthogonal_vector().normalize();
            let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);

            let result_dir = self * cos + axis * sin;
            result_dir * (self_length / result_dir.length())
        } else {
            // Vectors are almost parallel in the same direction.
            target / target_length * self_length
        }
    }

    #[track_caller]
    #[inline(always)]
    fn any_orthogonal_vector_backend(self) -> Self {
        let self_abs = self.abs();
        if self_abs.x > self_abs.y {
            if self_abs.x > self_abs.z {
                Self::new(-self.w, T::ZERO, T::ZERO, self.x)
            } else {
                Self::new(T::ZERO, T::ZERO, -self.w, self.z)
            }
        } else if self_abs.y > self_abs.z {
            Self::new(T::ZERO, -self.w, T::ZERO, self.y)
        } else {
            Self::new(T::ZERO, T::ZERO, -self.w, self.z)
        }
    }

    #[inline(always)]
    fn any_orthonormal_vector_backend(self) -> Self {
        self.any_orthogonal_vector().normalize()
    }
}

#[cfg(test)]
mod tests {

    use crate::{
        FloatExt, Mask, Vec2A, Vec3A, Vector,
        test_utils::{
            assert_debug_panic, assert_panic_test_eq, assert_test_eq, assert_test_eq_or_panic,
            for_types, random_iter,
        },
        utils::PrimitiveFloatUtils,
    };

    #[test]
    fn test_constants() {
        for_types!(|N, T: PrimitiveFloat, A| {
            assert_eq!(Vector::<N, T, A>::MIN, Vector::splat(T::MIN));
            assert_eq!(Vector::<N, T, A>::MAX, Vector::splat(T::MAX));
            assert_test_eq!(Vector::<N, T, A>::NAN, Vector::splat(T::NAN));
            assert_eq!(Vector::<N, T, A>::INFINITY, Vector::splat(T::INFINITY));
            assert_eq!(
                Vector::<N, T, A>::NEG_INFINITY,
                Vector::splat(T::NEG_INFINITY)
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(vector.is_nan(), vector.iter().any(T::is_nan));
            }
        });
    }

    #[test]
    fn test_nan_mask() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(vector.nan_mask(), Mask::from_fn(|i| vector[i].is_nan()));
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(vector.is_finite(), vector.iter().all(T::is_finite));
            }
        });
    }

    #[test]
    fn test_finite_mask() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(
                    vector.finite_mask(),
                    Mask::from_fn(|i| vector[i].is_finite())
                );
            }
        });
    }

    #[test]
    fn test_sign_positive_mask() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(
                    vector.sign_positive_mask(),
                    Mask::from_fn(|i| vector[i].is_sign_positive())
                );
            }
        });
    }

    #[test]
    fn test_sign_negative_mask() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(
                    vector.sign_negative_mask(),
                    Mask::from_fn(|i| vector[i].is_sign_negative())
                );
            }
        });
    }

    #[test]
    fn test_recip() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(vector.recip(), vector.map(T::recip));
            }
        });
    }

    #[test]
    fn test_max() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<N, T, A>; 2]>() {
                if vector.is_nan() || other.is_nan() {
                    assert_debug_panic!(vector.max(other));
                    continue;
                }

                assert_test_eq!(
                    vector.max(other),
                    Vector::from_fn(|i| vector[i].max(other[i])),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_min() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<N, T, A>; 2]>() {
                if vector.is_nan() || other.is_nan() {
                    assert_debug_panic!(vector.min(other));
                    continue;
                }

                assert_test_eq!(
                    vector.min(other),
                    Vector::from_fn(|i| vector[i].min(other[i])),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, min, max] in random_iter::<[Vector<N, T, A>; 3]>() {
                if vector.is_nan() || min.is_nan() || max.is_nan() || min.gt_mask(max).any() {
                    assert_debug_panic!(vector.clamp(min, max));
                }

                if vector.is_nan() || min.is_nan() || max.is_nan() {
                    continue;
                }
                let max = max.max(min);

                assert_test_eq!(
                    vector.clamp(min, max),
                    Vector::from_fn(|i| vector[i].clamp(min[i], max[i])),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                if vector.is_nan() {
                    assert_debug_panic!(vector.max_element());
                    continue;
                }

                assert_test_eq!(
                    vector.max_element(),
                    vector.iter().reduce(T::max).unwrap(),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                if vector.is_nan() {
                    assert_debug_panic!(vector.min_element());
                    continue;
                }

                assert_test_eq!(
                    vector.min_element(),
                    vector.iter().reduce(T::min).unwrap(),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_abs() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(vector.abs(), vector.map(T::abs));
            }
        });
    }

    #[test]
    fn test_signum() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(vector.signum(), vector.map(T::signum));
            }
        });
    }

    #[test]
    fn test_copysign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, sign] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector.copysign(sign),
                    Vector::from_fn(|i| vector[i].copysign(sign[i]))
                );
            }
        });
    }

    #[test]
    fn test_floor() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [0.0, 0.1, 0.5, 0.7, 3.0, 3.1, 3.5, 3.7, 4.0, 4.1, 4.5, 4.7]
                .into_iter()
                .flat_map(|x| [x, -x])
                .map(Vector::<N, T, A>::splat)
                .chain(random_iter())
            {
                assert_test_eq!(vector.floor(), vector.map(T::floor));
            }
        });
    }

    #[test]
    fn test_ceil() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [0.0, 0.1, 0.5, 0.7, 3.0, 3.1, 3.5, 3.7, 4.0, 4.1, 4.5, 4.7]
                .into_iter()
                .flat_map(|x| [x, -x])
                .map(Vector::<N, T, A>::splat)
                .chain(random_iter())
            {
                assert_test_eq!(vector.ceil(), vector.map(T::ceil));
            }
        });
    }

    #[test]
    fn test_round() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [0.0, 0.1, 0.5, 0.7, 3.0, 3.1, 3.5, 3.7, 4.0, 4.1, 4.5, 4.7]
                .into_iter()
                .flat_map(|x| [x, -x])
                .flat_map(|x: T| [x, x.next_down(), x.next_up()])
                .map(Vector::<N, T, A>::splat)
                .chain(random_iter())
            {
                assert_test_eq!(vector.round(), vector.map(T::round));
            }
        });
    }

    #[test]
    fn test_trunc() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [0.0, 0.1, 0.5, 0.7, 3.0, 3.1, 3.5, 3.7, 4.0, 4.1, 4.5, 4.7]
                .into_iter()
                .flat_map(|x| [x, -x])
                .map(Vector::<N, T, A>::splat)
                .chain(random_iter())
            {
                assert_test_eq!(vector.trunc(), vector.map(T::trunc));
            }
        });
    }

    #[test]
    fn test_fract() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [0.0, 0.1, 0.5, 0.7, 3.0, 3.1, 3.5, 3.7, 4.0, 4.1, 4.5, 4.7]
                .into_iter()
                .flat_map(|x| [x, -x])
                .map(Vector::<N, T, A>::splat)
                .chain(random_iter())
            {
                assert_test_eq!(vector.fract(), vector.map(T::fract));
            }
        });
    }

    #[test]
    fn test_mul_add() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, a, b] in random_iter::<[Vector<N, T, A>; 3]>() {
                assert_test_eq!(
                    vector.mul_add(a, b),
                    Vector::from_fn(|i| vector[i].mul_add(a[i], b[i]))
                );
            }
        });
    }

    #[test]
    fn test_div_euclid() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector_1, vector_2] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector_1.div_euclid(vector_2),
                    Vector::from_fn(|i| vector_1[i].div_euclid(vector_2[i]))
                );
            }
        });
    }

    #[test]
    fn test_rem_euclid() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector_1, vector_2] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(
                    vector_1.rem_euclid(vector_2),
                    Vector::from_fn(|i| vector_1[i].rem_euclid(vector_2[i]))
                );
            }
        });
    }

    #[test]
    fn test_sqrt() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(vector.sqrt(), vector.map(T::sqrt));
            }
        });
    }

    #[test]
    fn test_exp() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.exp(),
                    vector.map(T::exp),
                    abs <= vector.map(T::exp).abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_exp2() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.exp2(),
                    vector.map(T::exp2),
                    abs <= vector.map(T::exp2).abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_ln() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.ln(),
                    vector.map(T::ln),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_log2() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.log2(),
                    vector.map(T::log2),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_sin() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.sin(),
                    vector.map(T::sin),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_cos() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.cos(),
                    vector.map(T::cos),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_tan() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.tan(),
                    vector.map(T::tan),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_asin() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.asin(),
                    vector.map(T::asin),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_acos() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.acos(),
                    vector.map(T::acos),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_atan() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.atan(),
                    vector.map(T::atan),
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_sin_cos() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_test_eq!(
                    vector.sin_cos(),
                    (vector.map(|x| x.sin_cos().0), vector.map(|x| x.sin_cos().1)),
                    abs <= (vector.abs() * 1e-5 + 1e-5, vector.abs() * 1e-5 + 1e-5)
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Vector<N, T, A>; 2]>() {
                if !a.is_finite() || !b.is_finite() {
                    continue;
                }

                assert_test_eq!(a.lerp(b, 0.0), a, 0.0 = -0.0);
                assert_test_eq!(a.lerp(b, 0.5), a * 0.5 + b * 0.5, 0.0 = -0.0);
                assert_test_eq!(a.lerp(b, 1.0), b, 0.0 = -0.0);
            }
        });
    }

    #[test]
    fn test_midpoint() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Vector<N, T, A>; 2]>() {
                assert_test_eq!(a.midpoint(b), (a + b) * 0.5, 0.0 = -0.0);
            }
        });
    }

    #[test]
    fn test_move_towards() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, target] in random_iter::<[Vector<N, T, A>; 2]>() {
                if !vector.distance(target).is_finite() {
                    continue;
                }

                if vector.distance(target) <= 1e-4 {
                    assert_test_eq!(vector.move_towards(target, 0.0), target, 0.0 = -0.0);
                } else {
                    assert_test_eq!(vector.move_towards(target, 0.0), vector, 0.0 = -0.0);
                }
                assert_test_eq!(vector.move_towards(target, T::MAX), target, 0.0 = -0.0);
                assert_test_eq!(
                    vector.move_towards(target, 1.0).distance(target),
                    (vector.distance(target) - 1.0).max(0.0),
                    abs <= vector.abs().max_element().max(target.abs().max_element()) * 1e-5 + 1e-5,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for t in [
                -4.9, -2.3, -1.1, -0.01, 0.0, 0.01, 0.34, 0.5, 0.74, 0.97, 1.0, 1.01, 2.3, 4.9,
            ] {
                assert_debug_panic!(Vector::<N, T, A>::ZERO.slerp(Vector::ONE, t));
                assert_debug_panic!(Vector::<N, T, A>::ONE.slerp(Vector::ZERO, t));

                for [vector, other] in [
                    [[1.0, 0.3, 1.4, -2.0], [-1.5, 3.3, -0.3, -0.1]],
                    [[-10.0, 5.3, 3.0, 5.0], [1.5, 30.3, 1.3, -1.4]],
                    [[1.0, 0.03, 3.0, 4.2], [1.0, 0.0, 3.0001, 4.2]],
                    [[1.0, 0.03, 500.0, 2.0], [20.0, 0.0, 499.9, 2.0]],
                    [[1.0, 0.03, 3.0, 4.2], [1.0, 0.0, -3.0001, 4.2]],
                    [[1.0, 0.03, 500.0, 2.0], [20.0, 0.0, -499.9, 2.0]],
                ]
                .into_iter()
                .map(|values| {
                    values
                        .map(|array| Vector::<N, T, A>::from_array(array[0..N].try_into().unwrap()))
                }) {
                    assert_test_eq!(
                        vector.slerp(other, t).length(),
                        vector.length().lerp(other.length(), t).abs(),
                        abs <= vector.slerp(other, t).length() * 1e-2 + 1e-2
                    );
                    assert_test_eq!(
                        vector
                            .normalize()
                            .slerp(other.normalize(), t)
                            .angle_between(vector),
                        T::PI - (vector.angle_between(other) * t.abs() % T::TAU - T::PI).abs(),
                        abs <= 1e-2
                    );
                    assert_test_eq!(
                        vector
                            .normalize()
                            .slerp(other.normalize(), t)
                            .angle_between(other),
                        T::PI
                            - (vector.angle_between(other) * (1.0 - t).abs() % T::TAU - T::PI)
                                .abs(),
                        abs <= 1e-2
                    );
                }
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for max_angle in [0.0, 1.0, -1.0, 1.5, -1.5, 4.0, -4.0, 10.0, -1.0] {
                assert_debug_panic!(Vector::<N, T, A>::ONE.rotate_towards(Vector::ZERO, max_angle));

                for [vector, target] in [
                    [[1.0, 0.3, 1.4, -2.0], [-1.5, 3.3, -0.3, -0.1]],
                    [[-10.0, 5.3, 3.0, 5.0], [1.5, 30.3, 1.3, -1.4]],
                    [[1.0, 0.03, 3.0, 4.2], [1.0, 0.0, 3.0001, 4.2]],
                    [[1.0, 0.03, 500.0, 2.0], [20.0, 0.0, 499.9, 2.0]],
                    [[1.0, 0.03, 3.0, 4.2], [1.0, 0.0, -3.0001, 4.2]],
                    [[1.0, 0.03, 500.0, 2.0], [20.0, 0.0, -499.9, 2.0]],
                ]
                .map(|values| {
                    values
                        .map(|array| Vector::<N, T, A>::from_array(array[0..N].try_into().unwrap()))
                }) {
                    assert_test_eq!(
                        vector.rotate_towards(target, max_angle).length(),
                        vector.length(),
                        abs <= vector.length() * 1e-2
                    );

                    if max_angle >= vector.angle_between(target) {
                        assert_test_eq!(
                            vector.rotate_towards(target, max_angle),
                            target.normalize() * vector.length(),
                            abs <= 1e-2
                        );
                    } else if max_angle <= vector.angle_between(target) - T::PI {
                        assert_test_eq!(
                            vector.rotate_towards(target, max_angle),
                            -target.normalize() * vector.length(),
                            abs <= 1e-2
                        );
                    } else {
                        assert_test_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(vector),
                            max_angle.abs(),
                            abs <= 1e-2
                        );
                        assert_test_eq!(
                            vector
                                .rotate_towards(target, max_angle)
                                .angle_between(target),
                            vector.angle_between(target) - max_angle,
                            abs <= 1e-2
                        );
                    }

                    assert_test_eq!(
                        vector.rotate_towards(target, -max_angle),
                        vector.rotate_towards(-target, max_angle),
                        abs <= 1e-2
                    );

                    assert_test_eq!(
                        Vector::<N, T, A>::ZERO.rotate_towards(target, max_angle),
                        Vector::ZERO
                    );
                }
            }
        });
    }

    #[test]
    fn test_length() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in [
                [0.0, 0.0],
                [-0.0, -0.0],
                [0.0, -0.0],
                [-0.0, 0.0],
                [1.3, 0.1],
            ]
            .into_iter()
            .map(Vector::<2, T, A>::from_array)
            .chain(random_iter())
            {
                let [x, y] = vector.to_array();

                assert_test_eq!(vector.length(), (x * x + y * y).sqrt());
            }

            for vector in [
                [0.0, 0.0, 0.0],
                [-0.0, -0.0, -0.0],
                [0.0, -0.0, -0.0],
                [-0.0, 0.0, 0.0],
                [1.3, 0.1, -0.3],
            ]
            .into_iter()
            .map(Vector::<3, T, A>::from_array)
            .chain(random_iter())
            {
                let [x, y, z] = vector.to_array();

                assert_test_eq!(vector.length(), (x * x + y * y + z * z).sqrt());
            }

            for vector in [
                [0.0, 0.0, 0.0, 0.0],
                [-0.0, -0.0, -0.0, -0.0],
                [0.0, 0.0, -0.0, -0.0],
                [-0.0, -0.0, 0.0, 0.0],
                [1.3, 0.1, -0.3, -0.1],
            ]
            .into_iter()
            .map(Vector::<4, T, A>::from_array)
            .chain(random_iter())
            {
                let [x, y, z, w] = vector.to_array();

                assert_test_eq!(vector.length(), (x * x + y * y + (z * z + w * w)).sqrt());
            }
        });
    }

    #[test]
    fn test_distance() {
        for_types!(|T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<2, T, A>; 2]>() {
                let [x1, y1] = vector.to_array();
                let [x2, y2] = other.to_array();

                assert_test_eq!(
                    vector.distance(other),
                    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
                );
            }

            for [vector, other] in random_iter::<[Vector<3, T, A>; 2]>() {
                let [x1, y1, z1] = vector.to_array();
                let [x2, y2, z2] = other.to_array();

                assert_test_eq!(
                    vector.distance(other),
                    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2) + (z1 - z2) * (z1 - z2)).sqrt()
                );
            }

            for [vector, other] in random_iter::<[Vector<4, T, A>; 2]>() {
                let [x1, y1, z1, w1] = vector.to_array();
                let [x2, y2, z2, w2] = other.to_array();

                assert_test_eq!(
                    vector.distance(other),
                    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2))
                        + ((z1 - z2) * (z1 - z2) + (w1 - w2) * (w1 - w2)))
                        .sqrt()
                );
            }
        });
    }

    #[test]
    fn test_normalize() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [Vector::<N, T, A>::ZERO].into_iter().chain(random_iter()) {
                if !vector.length().is_finite() || !vector.length().recip().is_finite() {
                    assert_debug_panic!(vector.normalize());
                    continue;
                }

                assert_test_eq!(
                    vector.normalize() * vector.length(),
                    vector,
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_try_normalize() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [Vector::<N, T, A>::ZERO].into_iter().chain(random_iter()) {
                let Some(try_normalize) = vector.try_normalize() else {
                    assert_debug_panic!(vector.normalize());
                    continue;
                };

                assert_test_eq!(
                    try_normalize * vector.length(),
                    vector,
                    abs <= vector.abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_normalize_or() {
        for_types!(|N, T: PrimitiveFloat, A| {
            let fallback = Vector::splat(2401.0);
            for vector in [Vector::<N, T, A>::ZERO].into_iter().chain(random_iter()) {
                let Some(try_normalize) = vector.try_normalize() else {
                    assert_test_eq!(vector.normalize_or(fallback), fallback);
                    continue;
                };

                assert_test_eq!(vector.normalize_or(fallback), try_normalize);
            }
        });
    }

    #[test]
    fn test_normalize_or_zero() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [Vector::<N, T, A>::ZERO].into_iter().chain(random_iter()) {
                let Some(try_normalize) = vector.try_normalize() else {
                    assert_test_eq!(vector.normalize_or_zero(), Vector::ZERO);
                    continue;
                };

                assert_test_eq!(vector.normalize_or_zero(), try_normalize);
            }
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in [Vector::<N, T, A>::ZERO].into_iter().chain(random_iter()) {
                assert_test_eq_or_panic!(
                    vector.normalize_and_length(),
                    (vector.normalize(), vector.length())
                );
            }
        });
    }

    #[test]
    fn test_is_normalized() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                assert_eq!(
                    vector.is_normalized(),
                    (1.0 - 1e-4..1.0 + 1e-4).contains(&vector.length())
                );
            }
        });
    }

    #[test]
    fn test_with_max_length() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (vector, max_length) in random_iter::<(Vector<N, T, A>, T)>() {
                if max_length < 0.0 {
                    assert_debug_panic!(vector.with_max_length(max_length));
                }

                let max_length = max_length.abs();

                if vector.length_squared() > max_length * max_length {
                    assert_panic_test_eq!(
                        vector.with_max_length(max_length),
                        vector.normalize() * max_length
                    );
                } else {
                    assert_test_eq!(vector.with_max_length(max_length), vector);
                }
            }
        });
    }

    #[test]
    fn test_with_min_length() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (vector, min_length) in random_iter::<(Vector<N, T, A>, T)>() {
                if vector.length_squared() < min_length * min_length
                    && min_length.is_sign_positive()
                {
                    assert_panic_test_eq!(
                        vector.with_min_length(min_length),
                        vector.normalize() * min_length
                    );
                } else {
                    assert_test_eq!(vector.with_min_length(min_length), vector);
                }
            }
        });
    }

    #[test]
    fn test_clamp_length() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (vector, min_length, max_length) in random_iter::<(Vector<N, T, A>, T, T)>() {
                if max_length < 0.0 || max_length < min_length {
                    assert_debug_panic!(vector.clamp_length(min_length, max_length));
                }

                let min_length = min_length.abs();
                let max_length = max_length.abs().max(min_length);

                if (min_length..=max_length).contains(&vector.length()) {
                    assert_test_eq!(vector.clamp_length(min_length, max_length), vector);
                } else if vector.length() > max_length {
                    assert_panic_test_eq!(
                        vector.clamp_length(min_length, max_length),
                        vector.with_max_length(max_length)
                    );
                } else {
                    assert_panic_test_eq!(
                        vector.clamp_length(min_length, max_length),
                        vector.with_min_length(min_length)
                    );
                }
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_debug_panic!(Vector::<2, T, A>::ZERO.angle_between(Vector::<2, T, A>::X));
            assert_debug_panic!(Vector::<2, T, A>::X.angle_between(Vector::<2, T, A>::ZERO));

            assert_test_eq!(
                Vector::<2, T, A>::X.angle_between(Vector::<2, T, A>::ONE),
                (45.0 as T).to_radians(),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<2, T, A>::new(2.0, 0.0)
                    .angle_between(Vector::<2, T, A>::new(1.0, (3.0 as T).sqrt())),
                (60.0 as T).to_radians(),
                abs <= 1e-5
            );

            assert_test_eq!(
                Vector::<3, T, A>::new(1.0, 0.0, 0.0)
                    .angle_between(Vector::<3, T, A>::new(1.0, 1.0, 0.0)),
                (45.0 as T).to_radians(),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(2.0, 0.0, 0.0).angle_between(Vector::<3, T, A>::new(
                    1.0,
                    (3.0 as T).sqrt(),
                    0.0
                )),
                (60.0 as T).to_radians(),
                abs <= 1e-5
            );

            assert_test_eq!(
                Vector::<4, T, A>::new(1.0, 0.0, 0.0, 0.0)
                    .angle_between(Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0)),
                (45.0 as T).to_radians(),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<4, T, A>::new(2.0, 0.0, 0.0, 0.0).angle_between(Vector::<4, T, A>::new(
                    1.0,
                    (3.0 as T).sqrt(),
                    0.0,
                    0.0
                )),
                (60.0 as T).to_radians(),
                abs <= 1e-5
            );
        });
        for_types!(|N, T: PrimitiveFloat, A| {
            for [vector, other] in random_iter::<[Vector<N, T, A>; 2]>() {
                if [vector, other]
                    .iter()
                    .any(|v| !(1e-5..1e10).contains(&v.length()))
                {
                    continue;
                }

                assert_test_eq!(
                    vector.angle_between(vector),
                    0.0,
                    abs <= vector
                        .abs()
                        .max_element()
                        .max(vector.recip().abs().max_element())
                        * 1e-5
                );
                assert_test_eq!(
                    vector.angle_between(-vector),
                    T::PI,
                    abs <= vector
                        .abs()
                        .max_element()
                        .max(vector.recip().abs().max_element())
                        * 1e-5
                );
                assert!((0.0..=T::TAU / 2.0).contains(&vector.angle_between(other)));
            }
        });
    }

    #[test]
    fn test_project_onto() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::new(4.0, 0.0).project_onto(Vector::<2, T, A>::new(1.0, 1.0)),
                Vector::<2, T, A>::new(2.0, 2.0)
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .project_onto(Vector::<3, T, A>::new(1.0, 1.0, 0.0)),
                Vector::<3, T, A>::new(2.0, 2.0, 0.0)
            );
            assert_test_eq!(
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
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::new(4.0, 0.0)
                    .project_onto_normalized(Vector::<2, T, A>::new(1.0, 1.0).normalize()),
                Vector::<2, T, A>::new(2.0, 2.0),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .project_onto_normalized(Vector::<3, T, A>::new(1.0, 1.0, 0.0).normalize()),
                Vector::<3, T, A>::new(2.0, 2.0, 0.0),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0).project_onto_normalized(
                    Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0).normalize()
                ),
                Vector::<4, T, A>::new(2.0, 2.0, 0.0, 0.0),
                abs <= 1e-5
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
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::new(4.0, 0.0).reject_from(Vector::<2, T, A>::new(1.0, 1.0)),
                Vector::<2, T, A>::new(2.0, -2.0)
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .reject_from(Vector::<3, T, A>::new(1.0, 1.0, 0.0)),
                Vector::<3, T, A>::new(2.0, -2.0, 0.0)
            );
            assert_test_eq!(
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
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::new(4.0, 0.0)
                    .reject_from_normalized(Vector::<2, T, A>::new(1.0, 1.0).normalize()),
                Vector::<2, T, A>::new(2.0, -2.0),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(4.0, 0.0, 0.0)
                    .reject_from_normalized(Vector::<3, T, A>::new(1.0, 1.0, 0.0).normalize()),
                Vector::<3, T, A>::new(2.0, -2.0, 0.0),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<4, T, A>::new(4.0, 0.0, 0.0, 0.0)
                    .reject_from_normalized(Vector::<4, T, A>::new(1.0, 1.0, 0.0, 0.0).normalize()),
                Vector::<4, T, A>::new(2.0, -2.0, 0.0, 0.0),
                abs <= 1e-5
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
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::new(3.0, 2.0).reflect(Vector::<2, T, A>::ONE.normalize()),
                Vector::<2, T, A>::new(-2.0, -3.0),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(3.0, 2.0, 4.0).reflect(Vector::<3, T, A>::ONE.normalize()),
                Vector::<3, T, A>::new(-3.0, -4.0, -2.0),
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<4, T, A>::new(3.0, 2.0, 4.0, 5.0)
                    .reflect(Vector::<4, T, A>::ONE.normalize()),
                Vector::<4, T, A>::new(-4.0, -5.0, -3.0, -2.0),
                abs <= 1e-5
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
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::new(1.0, -T::sqrt(3.0))
                    .normalize()
                    .refract(Vector::<2, T, A>::Y, T::recip(1.5)),
                Vector::<2, T, A>::new(1.0, -T::sqrt(8.0)).normalize(),
                abs <= 1e-8
            );
            assert_test_eq!(
                Vector::<2, T, A>::new(2.0, -T::sqrt(3.0))
                    .normalize()
                    .refract(Vector::<2, T, A>::Y, 1.5),
                Vector::ZERO
            );

            assert_test_eq!(
                Vector::<3, T, A>::new(1.0, -T::sqrt(3.0), 0.0)
                    .normalize()
                    .refract(Vector::<3, T, A>::Y, T::recip(1.5)),
                Vector::<3, T, A>::new(1.0, -T::sqrt(8.0), 0.0).normalize(),
                abs <= 1e-8
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(2.0, -T::sqrt(3.0), 0.0)
                    .normalize()
                    .refract(Vector::<3, T, A>::Y, 1.5),
                Vector::ZERO
            );

            assert_test_eq!(
                Vector::<4, T, A>::new(1.0, -T::sqrt(3.0), 0.0, 0.0)
                    .normalize()
                    .refract(Vector::<4, T, A>::Y, T::recip(1.5)),
                Vector::<4, T, A>::new(1.0, -T::sqrt(8.0), 0.0, 0.0).normalize(),
                abs <= 1e-8
            );
            assert_test_eq!(
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
        for_types!(|T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<2, T, A>>() {
                assert_test_eq!(vector.any_orthogonal_vector(), vector.perp());
            }
        });
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                let vector = if vector.length().is_finite() {
                    vector
                } else {
                    Vector::ZERO
                };

                assert_test_eq!(vector.any_orthogonal_vector().dot(vector), 0.0);
                if vector != Vector::ZERO {
                    assert!(vector.any_orthogonal_vector() != Vector::ZERO);
                }
            }
        });
    }

    #[test]
    fn test_any_orthonormal_vector() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<2, T, A>>() {
                let vector = vector.normalize_or(Vector::<2, T, A>::X).normalize();

                assert_test_eq!(vector.any_orthonormal_vector(), vector.perp());
            }
        });
        for_types!(|N, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                if !vector.is_normalized() {
                    assert_debug_panic!(vector.any_orthonormal_vector());
                }

                let vector = vector.normalize_or(Vector::<N, T, A>::ONE).normalize();

                assert_test_eq!(
                    vector.any_orthonormal_vector().dot(vector),
                    0.0,
                    abs <= 1e-5 * vector.abs().max_element(),
                    0.0 = -0.0
                );
                assert!(vector.any_orthonormal_vector().is_normalized());
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat| {
            assert!(Vec2A::<T>::new(0.0, 1.0).abs_diff_eq(Vec2A::new(0.0, 1.0), 0.125));
            assert!(Vec2A::<T>::new(0.0, 1.0).abs_diff_eq(Vec2A::new(0.1, 0.9), 0.125));
            assert!(Vec2A::<T>::new(5.0, 1.0).abs_diff_eq(Vec2A::new(4.9, 1.0), 0.125));
            assert!(!Vec2A::<T>::new(0.0, 1.0).abs_diff_eq(Vec2A::new(0.2, 1.0), 0.125));
            assert!(!Vec2A::<T>::new(0.0, 1.0).abs_diff_eq(Vec2A::new(0.1, 0.8), 0.125));
            assert!(!Vec2A::<T>::new(5.0, 1.0).abs_diff_eq(Vec2A::new(4.5, 0.0), 0.125));
        });
    }

    #[test]
    fn test_to_bits() {
        for_types!(|T: PrimitiveFloat| {
            let vector = Vec3A::new(3.1, -0.0, T::NAN);
            assert_eq!(vector.to_bits(), vector.map(T::to_bits));
        });
    }

    #[test]
    fn test_from_bits() {
        for_types!(|T: PrimitiveFloat| {
            let vector = Vec3A::<T>::new(3.1, -0.0, T::NAN);
            assert_eq!(
                Vec3A::<T>::from_bits(vector.to_bits()).to_bits(),
                vector.to_bits()
            );
        });
    }

    #[test]
    fn test_angle_to() {
        for_types!(|T: PrimitiveFloat, A| {
            for [start, end] in random_iter::<[Vector<2, T, A>; 2]>() {
                assert_panic_test_eq!(
                    {
                        let _ = start.angle_to(end);
                    },
                    {
                        let _ = start.angle_between(end);
                    }
                );

                if start.try_normalize().is_none() || end.try_normalize().is_none() {
                    continue;
                }

                let result = start.angle_to(end);
                assert_test_eq!(
                    start.normalize().rotate(result),
                    end.normalize(),
                    abs <= start
                        .length()
                        .max(end.length())
                        .max(start.length().recip())
                        .max(end.length().recip())
                        * 1e-5
                        + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_angle_from() {
        for_types!(|T: PrimitiveFloat, A| {
            for [start, end] in random_iter::<[Vector<2, T, A>; 2]>() {
                assert_panic_test_eq!(end.angle_from(start), start.angle_to(end));
            }
        });
    }

    #[test]
    fn test_rotate() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<2, T, A>::X.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::Y,
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<2, T, A>::Y.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::NEG_X,
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<2, T, A>::NEG_X.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::NEG_Y,
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<2, T, A>::NEG_Y.rotate(T::to_radians(90.0)),
                Vector::<2, T, A>::X,
                abs <= 1e-5
            );
            assert_test_eq!(
                Vector::<2, T, A>::new(2.0, 0.0).rotate(T::to_radians(45.0)),
                Vector::<2, T, A>::new(2.0, 2.0).sqrt(),
                abs <= 1e-5
            );
        });
    }

    #[test]
    fn test_from_homogeneous() {
        for_types!(|T: PrimitiveFloat, A| {
            let vector3 = Vector::<3, T, A>::new(0.3, 0.6, 0.1);
            let vector4 = Vector::<4, T, A>::new(0.3, 0.6, 0.1, 0.4);

            assert_test_eq!(
                Vector::<2, T, A>::from_homogeneous(vector3),
                vector3.xy() / vector3.z
            );
            assert_test_eq!(
                Vector::<3, T, A>::from_homogeneous(vector4),
                vector4.xyz() / vector4.w
            );
        });
    }

    #[test]
    fn test_rotate_x() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<3, T, A>::X.rotate_x(T::to_radians(45.0)),
                Vector::<3, T, A>::X,
                abs <= 1e-5,
                0.0 = -0.0
            );
            assert_test_eq!(
                Vector::<3, T, A>::Y.rotate_x(T::to_radians(90.0)),
                Vector::<3, T, A>::Z,
                abs <= 1e-5,
                0.0 = -0.0
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(0.0, -2.0, 0.0).rotate_x(T::to_radians(45.0)),
                -Vector::<3, T, A>::new(0.0, 2.0, 2.0).sqrt(),
                abs <= 1e-5,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_rotate_y() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<3, T, A>::Y.rotate_y(T::to_radians(45.0)),
                Vector::<3, T, A>::Y,
                abs <= 1e-5,
                0.0 = -0.0
            );
            assert_test_eq!(
                Vector::<3, T, A>::Z.rotate_y(T::to_radians(90.0)),
                Vector::<3, T, A>::X,
                abs <= 1e-5,
                0.0 = -0.0
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(0.0, 0.0, -2.0).rotate_y(T::to_radians(45.0)),
                -Vector::<3, T, A>::new(2.0, 0.0, 2.0).sqrt(),
                abs <= 1e-5,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_rotate_z() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_test_eq!(
                Vector::<3, T, A>::Z.rotate_z(T::to_radians(45.0)),
                Vector::<3, T, A>::Z,
                abs <= 1e-5,
                0.0 = -0.0
            );
            assert_test_eq!(
                Vector::<3, T, A>::X.rotate_z(T::to_radians(90.0)),
                Vector::<3, T, A>::Y,
                abs <= 1e-5,
                0.0 = -0.0
            );
            assert_test_eq!(
                Vector::<3, T, A>::new(-2.0, 0.0, 0.0).rotate_z(T::to_radians(45.0)),
                -Vector::<3, T, A>::new(2.0, 2.0, 0.0).sqrt(),
                abs <= 1e-5,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_any_orthonormal_pair() {
        for_types!(|T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<3, T, A>>() {
                if !vector.is_normalized() {
                    assert_debug_panic!(vector.any_orthonormal_pair());
                }

                let vector = vector.normalize_or(Vector::<3, T, A>::X).normalize();

                let pair = vector.any_orthonormal_pair();
                assert_test_eq!(
                    pair.0.dot(vector),
                    0.0,
                    abs <= vector.abs().max_element() * 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    pair.1.dot(vector),
                    0.0,
                    abs <= vector.abs().max_element() * 1e-5,
                    0.0 = -0.0
                );
                assert_test_eq!(
                    pair.0.dot(pair.1),
                    0.0,
                    abs <= vector.abs().max_element() * 1e-5,
                    0.0 = -0.0
                );
                assert!(pair.0.is_normalized());
                assert!(pair.1.is_normalized());
            }
        });
    }
}
