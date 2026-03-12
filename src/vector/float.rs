use crate::{
    Alignment, Length, Mask, Scalar, SupportedLength, Vector, num_primitive::PrimitiveFloat,
    specialize::specialize,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
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
    /// let vec = Vec3::new(1.0, 2.0, f32::NAN);
    /// let mask = vec.nan_mask();
    ///
    /// assert_eq!(mask, Mask3::new(false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn nan_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_nan_mask(self))
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
    /// let vec = Vec3::new(1.0, f32::INFINITY, f32::NAN);
    /// let mask = vec.finite_mask();
    ///
    /// assert_eq!(mask, Mask3::new(true, false, false));
    /// ```
    #[inline]
    #[must_use]
    pub fn finite_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_finite_mask(self))
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
    /// let vec = Vec4::new(1.0, -2.0, -3.0, f32::INFINITY);
    /// let mask = vec.sign_positive_mask();
    ///
    /// assert_eq!(mask, Mask4::new(true, false, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn sign_positive_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_sign_positive_mask(
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
    /// let vec = Vec4::new(1.0, -2.0, 3.0, f32::NEG_INFINITY);
    /// let mask = vec.sign_negative_mask();
    ///
    /// assert_eq!(mask, Mask4::new(false, true, false, true));
    /// ```
    #[inline]
    #[must_use]
    pub fn sign_negative_mask(self) -> Mask<N, T, A> {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_sign_negative_mask(
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
    /// let vec = Vec3::new(2.0, 3.0, 4.0);
    /// let recip = vec.recip();
    /// let div = Vec3::ONE / vec;
    ///
    /// assert_eq!(recip, div);
    /// ```
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Computes the sum of the elements of `self`.
    ///
    /// Equivalent to `self.x + self.y + ...`.
    ///
    /// The order of addition is unspecified and may differ between target
    /// architectures.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> T {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_element_sum(self))
    }

    /// Computes the product of the elements of `self`.
    ///
    /// Equivalent to `self.x * self.y * ...`.
    ///
    /// The order of multiplication is unspecified and may differ between target
    /// architectures.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> T {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_element_product(
            self
        ))
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
    /// When assertions are enabled (see the crate documentation):
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
        #[cfg(assertions)]
        assert!(
            !self.is_nan() && !other.is_nan(),
            "NaN: {self:?}.max({other:?})"
        );

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_max(self, other))
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
    /// When assertions are enabled (see the crate documentation):
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
        #[cfg(assertions)]
        assert!(
            !self.is_nan() && !other.is_nan(),
            "NaN: {self:?}.min({other:?})"
        );

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_min(self, other))
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if any element is NaN, or if any element of `min` is greater than
    /// the corresponding element of `max`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec4;
    /// #
    /// let vec = Vec4::new(1.0, 2.0, 3.0, 0.0);
    /// let min = Vec4::new(0.0, 5.0, 1.0, -2.0);
    /// let max = Vec4::new(3.0, 6.0, 2.0, -1.0);
    /// let clamp = vec.clamp(min, max);
    ///
    /// assert_eq!(clamp, Vec4::new(1.0, 5.0, 2.0, -1.0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
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

    /// Returns the maximum between the elements of `self`.
    ///
    /// Equivalent to `self.x.max(self.y).max(self.z)...`.
    ///
    /// This is not consistent with IEEE semantics in regards to NaN propagation
    /// and handling of `-0.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = Vec3::new(-1.0, 7.0, 3.0);
    ///
    /// assert_eq!(vec.max_element(), 7.0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn max_element(self) -> T {
        #[cfg(assertions)]
        assert!(!self.is_nan(), "NaN: {self:?}.max_element()");

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_max_element(self))
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = Vec3::new(7.0, -1.0, 3.0);
    ///
    /// assert_eq!(vec.min_element(), -1.0);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn min_element(self) -> T {
        #[cfg(assertions)]
        assert!(!self.is_nan(), "NaN: {self:?}.min_element()");

        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_min_element(self))
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
    /// let vec = Vec3::new(7.0, -1.0, -3.0);
    ///
    /// assert_eq!(vec.abs(), Vec3::new(7.0, 1.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_abs(self))
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
    /// let vec = Vec4::new(7.0, -1.0, -3.0, f32::NAN);
    ///
    /// assert_eq!(vec.signum().x, 1.0);
    /// assert_eq!(vec.signum().y, -1.0);
    /// assert_eq!(vec.signum().z, -1.0);
    /// assert!(vec.signum().w.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_signum(self))
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
    /// let vec = Vec3::new(7.0, -1.0, -3.0);
    /// let sign = Vec3::new(-5.0, -2.0, 1.0);
    /// let copysign = vec.copysign(sign);
    ///
    /// assert_eq!(copysign, Vec3::new(-7.0, -1.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_copysign(self, sign))
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> T {
        (self * rhs).element_sum()
    }

    /// Computes the squared length/magnitude of `self`.
    #[inline]
    #[must_use]
    pub fn length_squared(self) -> T {
        (self * self).element_sum()
    }

    /// Computes the squared Euclidean distance between `self` and `other`.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, other: Self) -> T {
        (self - other).length_squared()
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

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must not be a zero vector.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `other` is a zero vector.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn project_onto(self, other: Self) -> Self {
        let other_length_squared_recip = other.length_squared().recip();

        #[cfg(assertions)]
        assert!(other_length_squared_recip.is_finite());

        other * self.dot(other) * other_length_squared_recip
    }

    /// Returns the vector projection of `self` onto `other`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `other` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        #[cfg(assertions)]
        assert!(other.is_normalized());

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
    /// When assertions are enabled (see the crate documentation):
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
    /// When assertions are enabled (see the crate documentation):
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `normal` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn reflect(self, normal: Self) -> Self {
        #[cfg(assertions)]
        assert!(normal.is_normalized());

        self - normal * (T::as_from(2.0) * self.dot(normal))
    }
}

#[expect(private_bounds)]
impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Computes the cross product of `self` and `rhs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let x = Vec3::new(1.0, 0.0, 0.0);
    /// let y = Vec3::new(0.0, 1.0, 0.0);
    ///
    /// assert_eq!(x.cross(y), Vec3::new(0.0, 0.0, 1.0));
    /// assert_eq!(y.cross(x), Vec3::new(0.0, 0.0, -1.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn cross(self, rhs: Self) -> Self {
        (self.zxy() * rhs - self * rhs.zxy()).zxy()
    }

    /// Returns some vector that is orthogonal to `self`.
    ///
    /// `self` must be finite and not a zero vector.
    ///
    /// The result is not necessarily normalized. For that use
    /// [`any_orthonormal_vector`] instead.
    ///
    /// [`any_orthonormal_vector`]: Self::any_orthonormal_vector
    #[inline]
    #[must_use]
    pub fn any_orthogonal_vector(self) -> Self {
        if self.x.abs() > self.y.abs() {
            // self.cross(Self::Y)
            Self::new(-self.z, T::as_from(0.0), self.x)
        } else {
            // self.cross(Self::X)
            Self::new(T::as_from(0.0), self.z, -self.y)
        }
    }

    /// Returns some unit vector that is orthogonal to `self`.
    ///
    /// `self` must normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn any_orthonormal_vector(self) -> Self {
        #[cfg(assertions)]
        assert!(self.is_normalized());

        // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
        let sign = self.z.signum();
        let a = T::as_from(-1.0) / (sign + self.z);
        let b = self.x * self.y * a;
        Self::new(b, sign + self.y * self.y * a, -self.y)
    }

    /// Returns two unit vectors that are orthogonal to `self` and to each
    /// other.
    ///
    /// Together with `self`, they form an orthonormal basis where the three
    /// vectors are all orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn any_orthonormal_pair(self) -> (Self, Self) {
        #[cfg(assertions)]
        assert!(self.is_normalized());

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

#[cfg(backend)]
#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
{
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
    /// let vec = Vec3::new(3.7, 3.0, -3.7);
    ///
    /// assert_eq!(vec.floor(), Vec3::new(3.0, 3.0, -4.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_floor(self))
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
    /// let vec = Vec3::new(3.01, 4.0, -4.99);
    ///
    /// assert_eq!(vec.ceil(), Vec3::new(4.0, 4.0, -4.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_ceil(self))
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
    /// let vec = Vec3::new(3.3, -3.3, 3.5);
    ///
    /// assert_eq!(vec.round(), Vec3::new(3.0, -3.0, 4.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_round(self))
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
    /// let vec = Vec3::new(3.7, 3.0, -3.7);
    ///
    /// assert_eq!(vec.trunc(), Vec3::new(3.0, 3.0, -3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_trunc(self))
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
    /// let vec = Vec2::new(3.25, -3.25);
    ///
    /// assert_eq!(vec.fract(), Vec2::new(0.25, -0.25));
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_mul_add(self, a, b))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_div_euclid(
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_rem_euclid(
            self, rhs
        ))
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
    /// let vec = Vec3::<f32>::new(4.0, 16.0, -4.0);
    ///
    /// assert_eq!(vec.sqrt().x, 2.0);
    /// assert_eq!(vec.sqrt().y, 4.0);
    /// assert!(vec.sqrt().z.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_sqrt(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_sin(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_cos(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_tan(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_asin(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_acos(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_atan(self))
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
        specialize!(<T as PrimitiveFloatBackend<N, A>>::vec_sin_cos(self))
    }

    /// Returns the length/magnitude of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = Vec3::new(2.0, 3.0, 1.0);
    ///
    /// assert_eq!(vec.length(), 14.0_f32.sqrt());
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

    /// Moves `self` towards `other` by at most `max_delta`.
    ///
    /// When `max_delta` is `0.0`, the result is `self`. When `max_delta` is
    /// equal to or greater than `self.distance(other)`, the result is `other`.
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = Vec3::new(2.0, 0.0, 0.0);
    /// let target = Vec3::new(5.0, 0.0, 0.0);
    /// let max_delta = 1.0;
    /// let move_towards = vec.move_towards(target, max_delta);
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

    /// Returns a vector with the direction of `self` and length `1.0`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `self` is a zero vector, or if the result is non finite or
    /// zero.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec3;
    /// #
    /// let vec = Vec3::new(1.0, 2.0, 3.0);
    ///
    /// assert_eq!(vec.normalize(), vec / vec.length());
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
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
    /// let vec = Vec3::new(1.0, 2.0, 3.0);
    /// let (normalize, length) = vec.normalize_and_length();
    ///
    /// assert_eq!(normalize, vec.normalize());
    /// assert_eq!(length, vec.length());
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

    /// Returns `self` with a length of no more than `max`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
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
        #[cfg(assertions)]
        assert!(max >= T::as_from(0.0), "negative maximum length");

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
    /// When assertions are enabled (see the crate documentation):
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
        #[cfg(assertions)]
        assert!(min >= T::as_from(0.0), "negative minimum length");

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
    /// When assertions are enabled (see the crate documentation):
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
        #[cfg(assertions)]
        assert!(min >= T::as_from(0.0), "negative minimum length");

        #[cfg(assertions)]
        assert!(min <= max, "minimum length is greater than maximum length");

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
        (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt()).acos()
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
    /// let vec = Vec3::new(2.0, 4.0, 8.0);
    ///
    /// assert_eq!(vec.log2(), Vec3::new(1.0, 2.0, 3.0));
    /// ```
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        self.map(T::log2)
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `self` or `normal` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn refract(self, normal: Self, eta: T) -> Self {
        #[cfg(assertions)]
        assert!(self.is_normalized());

        #[cfg(assertions)]
        assert!(normal.is_normalized());

        let self_dot_normal = self.dot(normal);
        let k = T::as_from(1.0) - eta * eta * (T::as_from(1.0) - self_dot_normal * self_dot_normal);
        if k >= T::as_from(0.0) {
            self * eta - normal * (eta * self_dot_normal + k.sqrt())
        } else {
            Self::ZERO
        }
    }
}

#[cfg(backend)]
#[expect(private_bounds)]
impl<T, A: Alignment> Vector<2, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Returns `self` rotated by 90 degrees.
    ///
    /// This rotates `+X` to `+Y`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Vec2;
    /// #
    /// let x = Vec2::new(1.0, 0.0);
    /// let y = Vec2::new(0.0, 1.0);
    ///
    /// assert_eq!(x.perp(), y);
    /// assert_eq!(y.perp(), -x);
    /// assert_eq!((-x).perp(), -y);
    /// assert_eq!((-y).perp(), x);
    /// ```
    #[inline]
    #[must_use]
    pub fn perp(self) -> Self {
        Self::new(-self.y, self.x)
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

#[cfg(backend)]
#[expect(private_bounds)]
impl<T, A: Alignment> Vector<3, T, A>
where
    T: Scalar + PrimitiveFloat,
{
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
}

pub(crate) trait PrimitiveFloatBackend<const N: usize, A: Alignment>: Scalar
where
    Length<N>: SupportedLength,
{
    #[inline]
    fn vec_nan_mask(vec: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vec[i].is_nan())
    }

    #[inline]
    fn vec_finite_mask(vec: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vec[i].is_finite())
    }

    #[inline]
    fn vec_sign_positive_mask(vec: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vec[i].is_sign_positive())
    }

    #[inline]
    fn vec_sign_negative_mask(vec: Vector<N, Self, A>) -> Mask<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Mask::from_fn(|i| vec[i].is_sign_negative())
    }

    #[inline]
    fn vec_element_sum(vec: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveFloat,
    {
        vec.as_array_ref().iter().copied().sum()
    }

    #[inline]
    fn vec_element_product(vec: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveFloat,
    {
        vec.as_array_ref().iter().copied().product()
    }

    #[inline]
    fn vec_max(vec: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| if vec[i] > other[i] { vec[i] } else { other[i] })
    }

    #[inline]
    fn vec_min(vec: Vector<N, Self, A>, other: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| if vec[i] < other[i] { vec[i] } else { other[i] })
    }

    #[inline]
    fn vec_max_element(vec: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveFloat,
    {
        vec.iter().reduce(|a, b| if a > b { a } else { b }).unwrap()
    }

    #[inline]
    fn vec_min_element(vec: Vector<N, Self, A>) -> Self
    where
        Self: PrimitiveFloat,
    {
        vec.iter().reduce(|a, b| if a < b { a } else { b }).unwrap()
    }

    #[inline]
    fn vec_abs(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::abs)
    }

    #[inline]
    fn vec_signum(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::signum)
    }

    #[inline]
    fn vec_copysign(vec: Vector<N, Self, A>, sign: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vec[i].copysign(sign[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_floor(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::floor)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_ceil(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::ceil)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_round(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::round)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_trunc(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::trunc)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_mul_add(
        vec: Vector<N, Self, A>,
        a: Vector<N, Self, A>,
        b: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_div_euclid(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_rem_euclid(vec: Vector<N, Self, A>, rhs: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
    }

    #[cfg(backend)]
    #[inline]
    fn vec_sqrt(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::sqrt)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_sin(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::sin)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_cos(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::cos)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_tan(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::tan)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_asin(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::asin)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_acos(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::acos)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_atan(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        Self: PrimitiveFloat,
    {
        vec.map(Self::atan)
    }

    #[cfg(backend)]
    #[inline]
    fn vec_sin_cos(vec: Vector<N, Self, A>) -> (Vector<N, Self, A>, Vector<N, Self, A>)
    where
        Self: PrimitiveFloat,
    {
        let array = vec.to_array().map(|x| x.sin_cos());

        (
            Vector::from_fn(|i| array[i].0),
            Vector::from_fn(|i| array[i].1),
        )
    }
}
