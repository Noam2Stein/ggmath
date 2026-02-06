#[cfg(feature = "libm")]
#[allow(unused_imports)]
use crate::libm::LibmFloatExt;
use crate::{Alignment, Length, Mask, Scalar, SupportedLength, Vector, utils::specialize};

macro_rules! impl_float {
    ($T:ident, $Backend:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            /// Returns `true` if any of the vector's components are NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(self) -> bool {
                self.nan_mask().any()
            }

            /// Returns a mask where each component is `true` if the
            /// corresponding component of `self` is NaN (Not a Number).
            ///
            /// Equivalent to `(self.x.is_nan(), self.y.is_nan(), ...)`.
            #[inline]
            #[must_use]
            pub fn nan_mask(self) -> Mask<N, $T, A> {
                specialize!(<$T as $Backend<N, A>>::vec_nan_mask(self))
            }

            /// Returns `true` if all of the vector's components are finite.
            ///
            /// Finite corresponds to not NaN and not positive/negative infinity.
            #[inline]
            #[must_use]
            pub fn is_finite(self) -> bool {
                self.finite_mask().all()
            }

            /// Returns a mask where each component is `true` if the
            /// corresponding component of `self` is finite.
            ///
            /// Finite corresponds to not NaN and not positive/negative infinity.
            ///
            /// Equivalent to `(self.x.is_finite(), self.y.is_finite(), ...)`.
            #[inline]
            #[must_use]
            pub fn finite_mask(self) -> Mask<N, $T, A> {
                specialize!(<$T as $Backend<N, A>>::vec_finite_mask(self))
            }

            /// Returns a mask where each component is `true` if the
            /// corresponding component of `self` has a positive sign.
            ///
            /// Equivalent to
            /// `(self.x.is_sign_positive(), self.y.is_sign_positive(), ...)`.
            #[inline]
            #[must_use]
            pub fn sign_positive_mask(self) -> Mask<N, $T, A> {
                specialize!(<$T as $Backend<N, A>>::vec_sign_positive_mask(self))
            }

            /// Returns a mask where each component is `true` if the
            /// corresponding component of `self` has a negative sign.
            ///
            /// Equivalent to
            /// `(self.x.is_sign_negative(), self.y.is_sign_negative(), ...)`.
            #[inline]
            #[must_use]
            pub fn sign_negative_mask(self) -> Mask<N, $T, A> {
                specialize!(<$T as $Backend<N, A>>::vec_sign_negative_mask(self))
            }

            /// Computes `1.0 / self`.
            #[inline]
            #[must_use]
            pub fn recip(self) -> Self {
                Self::ONE / self
            }

            /// Computes the sum of the vector's components.
            ///
            /// Equivalent to `self.x + self.y + ...`.
            ///
            /// The order of addition is unspecified and may differ between target
            /// architectures.
            #[inline]
            #[must_use]
            pub fn element_sum(self) -> $T {
                specialize!(<$T as $Backend<N, A>>::vec_element_sum(self))
            }

            /// Computes the product of the vector's components.
            ///
            /// Equivalent to `self.x * self.y * ...`.
            ///
            /// The order of multiplication is unspecified and may differ between target
            /// architectures.
            #[inline]
            #[must_use]
            pub fn element_product(self) -> $T {
                specialize!(<$T as $Backend<N, A>>::vec_element_product(self))
            }

            /// Returns the maximum between the components of `self` and `other`.
            ///
            /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
            ///
            /// This function is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any input is NaN.
            #[inline]
            #[must_use]
            pub fn max(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    !self.is_nan() && !other.is_nan(),
                    "NaN: {self:?}.max({other:?})"
                );

                specialize!(<$T as $Backend<N, A>>::vec_max(self, other))
            }

            /// Returns the minimum between the components of `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            ///
            /// This function is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any input is NaN.
            #[inline]
            #[must_use]
            pub fn min(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    !self.is_nan() && !other.is_nan(),
                    "NaN: {self:?}.min({other:?})"
                );

                specialize!(<$T as $Backend<N, A>>::vec_min(self, other))
            }

            /// Clamps the vector's components between the components of `min` and
            /// `max`.
            ///
            /// Equivalent to
            /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
            ///
            /// This function is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any input is NaN, or if `min > max`.
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

            /// Returns the maximum between the vector's components.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...`.
            ///
            /// This function is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any input is NaN.
            #[inline]
            #[must_use]
            pub fn max_element(self) -> $T {
                #[cfg(assertions)]
                assert!(!self.is_nan(), "NaN: {self:?}.max_element()");

                specialize!(<$T as $Backend<N, A>>::vec_max_element(self))
            }

            /// Returns the minimum between the vector's components.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...`.
            ///
            /// This function is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any input is NaN.
            #[inline]
            #[must_use]
            pub fn min_element(self) -> $T {
                #[cfg(assertions)]
                assert!(!self.is_nan(), "NaN: {self:?}.min_element()");

                specialize!(<$T as $Backend<N, A>>::vec_min_element(self))
            }

            /// Returns the absolute values of the vector's components.
            ///
            /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
            #[inline]
            #[must_use]
            pub fn abs(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_abs(self))
            }

            /// Returns the signum of the vector's components.
            ///
            /// Equivalent to `(self.x.signum(), self.y.signum(), ...)`.
            ///
            /// For each component:
            ///
            /// - `1.0` if the component is positive, `+0.0` or `INFINITY`
            /// - `-1.0` if the component is negative, `-0.0` or `NEG_INFINITY`
            /// - NaN if the component is NaN
            #[inline]
            #[must_use]
            pub fn signum(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_signum(self))
            }

            /// Returns a vector with the magnitudes of `self` and the signs of `sign`.
            ///
            /// Equivalent to `(self.x.copysign(sign.x), self.y.copysign(sign.y), ...)`.
            #[inline]
            #[must_use]
            pub fn copysign(self, sign: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_copysign(self, sign))
            }

            /// Computes the dot product of `self` and `rhs`.
            #[inline]
            #[must_use]
            pub fn dot(self, rhs: Self) -> $T {
                (self * rhs).element_sum()
            }

            /// Computes the squared length/magnitude of the vector.
            #[inline]
            #[must_use]
            pub fn length_squared(self) -> $T {
                (self * self).element_sum()
            }

            /// Computes the squared Euclidean distance between `self` and `other`.
            #[inline]
            #[must_use]
            pub fn distance_squared(self, other: Self) -> $T {
                (self - other).length_squared()
            }

            /// Computes the linear interpolation between `self` and `other` based on
            /// the value `t`.
            ///
            /// When `t` is `0.0`, the result is `self`.  When `t` is `1.0`, the result
            /// is `rhs`. When `t` is outside of the range `[0.0, 1.0]`, the result is
            /// linearly extrapolated.
            #[inline]
            #[must_use]
            pub fn lerp(self, other: Self, t: $T) -> Self {
                self * (1.0 - t) + other * t
            }

            /// Computes the middle point between `self` and `other`.
            ///
            /// Equivalent to `self.lerp(other, 0.5)` but is cheaper to compute and may
            /// return a slightly different value.
            #[inline]
            #[must_use]
            pub fn midpoint(self, other: Self) -> Self {
                (self + other) * 0.5
            }

            /// Returns whether the vector has the length `1.0` or not.
            ///
            /// This function uses a precision threshold of approximately `1e-4`.
            #[inline]
            #[must_use]
            pub fn is_normalized(self) -> bool {
                (self.length_squared() - 1.0).abs() <= 2e-4
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
            pub fn project_onto_normalized(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(other.is_normalized());

                other * self.dot(other)
            }

            /// Returns the vector rejection of `self` from `other`.
            ///
            /// Corresponds to a vector pointing at `self` from the projection of `self`
            /// onto `other`, or simply `self - self.project_onto(other)`.
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
            pub fn reject_from(self, other: Self) -> Self {
                self - self.project_onto(other)
            }

            /// Returns the vector rejection of `self` from `other`.
            ///
            /// Corresponds to a vector pointing at `self` from the projection of `self`
            /// onto `other`, or simply `self - self.project_onto(other)`.
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
            ///
            /// # Example
            ///
            /// ```
            /// use ggmath::{Vec2, vec2};
            ///
            /// let vec: Vec2<f32> = vec2!(1.0, 2.0);
            ///
            /// assert_eq!(vec.reflect(Vec2::X), vec2!(-1.0, 2.0));
            /// ```
            #[inline]
            #[must_use]
            pub fn reflect(self, normal: Self) -> Self {
                #[cfg(assertions)]
                assert!(normal.is_normalized());

                self - normal * (2.0 * self.dot(normal))
            }
        }

        impl<A: Alignment> Vector<3, $T, A> {
            /// Computes the cross product of `self` and `rhs`.
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
            /// [`Self::any_orthonormal_vector()`] instead.
            #[inline]
            #[must_use]
            pub fn any_orthogonal_vector(self) -> Self {
                if self.x.abs() > self.y.abs() {
                    // self.cross(Self::Y)
                    Self::new(-self.z, 0.0, self.x)
                } else {
                    // self.cross(Self::X)
                    Self::new(0.0, self.z, -self.y)
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
            pub fn any_orthonormal_vector(self) -> Self {
                #[cfg(assertions)]
                assert!(self.is_normalized());

                // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
                let sign = self.z.signum();
                let a = -1.0 / (sign + self.z);
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
            pub fn any_orthonormal_pair(self) -> (Self, Self) {
                #[cfg(assertions)]
                assert!(self.is_normalized());

                // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
                let sign = self.z.signum();
                let a = -1.0 / (sign + self.z);
                let b = self.x * self.y * a;
                (
                    Self::new(1.0 + sign * self.x * self.x * a, sign * b, -sign * self.x),
                    Self::new(b, sign + self.y * self.y * a, -self.y),
                )
            }
        }

        #[cfg(backend)]
        impl<const N: usize, A: Alignment> Vector<N, $T, A>
        where
            Length<N>: SupportedLength,
        {
            /// Rounds the vector's components down.
            #[inline]
            #[must_use]
            pub fn floor(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_floor(self))
            }

            /// Rounds the vector's components up.
            #[inline]
            #[must_use]
            pub fn ceil(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_ceil(self))
            }

            /// Rounds the vector's components to the nearest integer.
            #[inline]
            #[must_use]
            pub fn round(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_round(self))
            }

            /// Rounds the vector's components towards zero.
            #[inline]
            #[must_use]
            pub fn trunc(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_trunc(self))
            }

            /// Returns the fractional part of the vector.
            ///
            /// Equivalent to `self - self.trunc()`.
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
                specialize!(<$T as $Backend<N, A>>::vec_mul_add(self, a, b))
            }

            /// Euclidiean Division.
            ///
            /// Equivalent to `(self.x.div_euclid(rhs.x), self.y.div_euclid(rhs.y), ...)`.
            ///
            /// # Precision
            ///
            /// The result of this operation is guaranteed to be the rounded
            /// infinite-precision result.
            #[inline]
            #[must_use]
            pub fn div_euclid(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_div_euclid(self, rhs))
            }

            /// Euclidiean Remainder.
            ///
            /// Equivalent to `(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y), ...)`.
            ///
            /// # Precision
            ///
            /// The result of this operation is guaranteed to be the rounded
            /// infinite-precision result.
            #[inline]
            #[must_use]
            pub fn rem_euclid(self, rhs: Self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_rem_euclid(self, rhs))
            }

            /// Returns the square root of the vector's components.
            ///
            /// Equivalent to `(self.x.sqrt(), self.y.sqrt(), ...)`.
            ///
            /// # Precision
            ///
            /// The result of this operation is guaranteed to be the rounded
            /// infinite-precision result. It is specified by IEEE 754 as `squareRoot`
            /// and guaranteed not to change.
            #[inline]
            #[must_use]
            pub fn sqrt(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_sqrt(self))
            }

            /// Computes the sine of the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_sin(self))
            }

            /// Computes the cosine of the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_cos(self))
            }

            /// Computes the tangent of the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_tan(self))
            }

            /// Computes the arcsine of the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_asin(self))
            }

            /// Computes the arccosine of the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_acos(self))
            }

            /// Computes the arctangent of the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                specialize!(<$T as $Backend<N, A>>::vec_atan(self))
            }

            /// Simultaneously computes the sine and cosine of the vector's components.
            ///
            /// Equivalent to `(self.sin(), self.cos())` but may be more performant and
            /// might return a slightly different value.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn sin_cos(self) -> (Self, Self) {
                specialize!(<$T as $Backend<N, A>>::vec_sin_cos(self))
            }

            /// Returns the length/magnitude of the vector.
            #[inline]
            #[must_use]
            pub fn length(self) -> $T {
                self.dot(self).sqrt()
            }

            /// Computes the Euclidean distance between `self` and `other`.
            #[inline]
            #[must_use]
            pub fn distance(self, other: Self) -> $T {
                (self - other).length()
            }

            /// Moves `self` towards `other` by at most `max_delta`.
            ///
            /// When `max_delta` is `0.0`, the result is `self`. When `max_delta` is
            /// equal to or greater than `self.distance(other)`, the result is `other`.
            #[inline]
            #[must_use]
            pub fn move_towards(self, other: Self, max_delta: $T) -> Self {
                let delta = other - self;
                let delta_length = delta.length();

                if delta_length <= max_delta || delta_length <= 1e-4 {
                    other
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
            /// Panics if the input is a zero vector, or if the result is non finite or zero.
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

            /// Returns [`Self::normalize`], or `None` if the input is zero or if the
            /// result is non finite or zero.
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

            /// Returns [`Self::normalize`], or `fallback` if the input is zero or if
            /// the result is non finite or zero.
            #[inline]
            #[must_use]
            pub fn normalize_or(self, fallback: Self) -> Self {
                self.try_normalize().unwrap_or(fallback)
            }

            /// Returns [`Self::normalize`], or a zero vector if the input is zero or if
            /// the result is non finite.
            #[inline]
            #[must_use]
            pub fn normalize_or_zero(self) -> Self {
                self.normalize_or(Self::ZERO)
            }

            /// Simultaneously computes [`Self::normalize`] and [`Self::length`].
            ///
            /// If `self` is a zero vector, the result is `(Self::ZERO, 0.0)`.
            #[inline]
            #[must_use]
            pub fn normalize_and_length(self) -> (Self, $T) {
                let length = self.length();
                let recip = 1.0 / length;

                if recip.is_finite() && recip > 0.0 {
                    (self * recip, length)
                } else {
                    (Self::ZERO, 0.0)
                }
            }

            /// Returns the input vector but with a length of no more than `max`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `max` is negative.
            #[inline]
            #[must_use]
            pub fn with_max_length(self, max: $T) -> Self {
                #[cfg(assertions)]
                assert!(max >= 0.0, "negative maximum length");

                let length_squared = self.length_squared();
                if length_squared > max * max {
                    self / length_squared.sqrt() * max
                } else {
                    self
                }
            }

            /// Returns the input vector but with a length of no less than `min`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `min` is negative.
            #[inline]
            #[must_use]
            pub fn with_min_length(self, min: $T) -> Self {
                #[cfg(assertions)]
                assert!(min >= 0.0, "negative minimum length");

                let length_squared = self.length_squared();
                if length_squared < min * min {
                    self / length_squared.sqrt() * min
                } else {
                    self
                }
            }

            /// Returns the input vector buth with a length of no less than `min` and no
            /// more than `max`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `min > max`, or if `min` or `max` are negative.
            #[inline]
            #[must_use]
            pub fn clamp_length(self, min: $T, max: $T) -> Self {
                #[cfg(assertions)]
                assert!(min >= 0.0, "negative minimum length");

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
            /// `[0, +π]`.
            ///
            /// The vectors do not need to be unit vectors but they do need to be
            /// non-zero.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn angle_between(self, other: Self) -> $T {
                (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt()).acos()
            }

            /// Computes the exponential function `e^self` for the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                self.map($T::exp)
            }

            /// Computes `2^self` for the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn exp2(self) -> Self {
                self.map($T::exp2)
            }

            /// Computes the natural logarithm for the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                self.map($T::ln)
            }

            /// Computes the base 2 logarithm for the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                self.map($T::log2)
            }

            /// Computes `self^n` for the vector's components.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn powf(self, n: $T) -> Self {
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
            pub fn refract(self, normal: Self, eta: $T) -> Self {
                #[cfg(assertions)]
                assert!(self.is_normalized());

                #[cfg(assertions)]
                assert!(normal.is_normalized());

                let self_dot_normal = self.dot(normal);
                let k = 1.0 - eta * eta * (1.0 - self_dot_normal * self_dot_normal);
                if k >= 0.0 {
                    self * eta - normal * (eta * self_dot_normal + k.sqrt())
                } else {
                    Self::ZERO
                }
            }
        }

        #[cfg(backend)]
        impl<A: Alignment> Vector<2, $T, A> {
            /// Returns `self` rotated by 90 degrees.
            #[inline]
            #[must_use]
            pub fn perp(self) -> Self {
                Self::new(-self.y, self.x)
            }

            /// Rotates `self` by `angle` (in radians).
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn rotate(self, angle: $T) -> Self {
                let (angle_sin, angle_cos) = angle.sin_cos();
                Self::new(
                    self.x * angle_cos - self.y * angle_sin,
                    self.x * angle_sin + self.y * angle_cos,
                )
            }
        }

        #[cfg(backend)]
        impl<A: Alignment> Vector<3, $T, A> {
            /// Rotates `self` around the x axis by `angle` (in radians).
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn rotate_x(self, angle: $T) -> Self {
                let (angle_sin, angle_cos) = angle.sin_cos();
                Self::new(
                    self.x,
                    self.y * angle_cos - self.z * angle_sin,
                    self.y * angle_sin + self.z * angle_cos,
                )
            }

            /// Rotates `self` around the y axis by `angle` (in radians).
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn rotate_y(self, angle: $T) -> Self {
                let (angle_sin, angle_cos) = angle.sin_cos();
                Self::new(
                    self.x * angle_cos + self.z * angle_sin,
                    self.y,
                    self.x * -angle_sin + self.z * angle_cos,
                )
            }

            /// Rotates `self` around the z axis by `angle` (in radians).
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means it
            /// varies by platform, version, and can even differ within the same
            /// execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn rotate_z(self, angle: $T) -> Self {
                let (angle_sin, angle_cos) = angle.sin_cos();
                Self::new(
                    self.x * angle_cos - self.y * angle_sin,
                    self.x * angle_sin + self.y * angle_cos,
                    self.z,
                )
            }
        }

        pub(crate) trait $Backend<const N: usize, A: Alignment>: Scalar
        where
            Length<N>: SupportedLength,
        {
            #[inline]
            fn vec_nan_mask(vec: Vector<N, $T, A>) -> Mask<N, $T, A> {
                Mask::from_fn(|i| vec[i].is_nan())
            }

            #[inline]
            fn vec_finite_mask(vec: Vector<N, $T, A>) -> Mask<N, $T, A> {
                Mask::from_fn(|i| vec[i].is_finite())
            }

            #[inline]
            fn vec_sign_positive_mask(vec: Vector<N, $T, A>) -> Mask<N, $T, A> {
                Mask::from_fn(|i| vec[i].is_sign_positive())
            }

            #[inline]
            fn vec_sign_negative_mask(vec: Vector<N, $T, A>) -> Mask<N, $T, A> {
                Mask::from_fn(|i| vec[i].is_sign_negative())
            }

            #[inline]
            fn vec_element_sum(vec: Vector<N, $T, A>) -> $T {
                vec.as_array_ref().iter().copied().sum()
            }

            #[inline]
            fn vec_element_product(vec: Vector<N, $T, A>) -> $T {
                vec.as_array_ref().iter().copied().product()
            }

            #[inline]
            fn vec_max(vec: Vector<N, $T, A>, other: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| if vec[i] > other[i] { vec[i] } else { other[i] })
            }

            #[inline]
            fn vec_min(vec: Vector<N, $T, A>, other: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| if vec[i] < other[i] { vec[i] } else { other[i] })
            }

            #[inline]
            fn vec_max_element(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce(|a, b| if a > b { a } else { b }).unwrap()
            }

            #[inline]
            fn vec_min_element(vec: Vector<N, $T, A>) -> $T {
                vec.iter().reduce(|a, b| if a < b { a } else { b }).unwrap()
            }

            #[inline]
            fn vec_abs(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::abs)
            }

            #[inline]
            fn vec_signum(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::signum)
            }

            #[inline]
            fn vec_copysign(vec: Vector<N, $T, A>, sign: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].copysign(sign[i]))
            }

            #[cfg(backend)]
            #[inline]
            fn vec_floor(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::floor)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_ceil(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::ceil)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_round(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::round)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_trunc(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::trunc)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_mul_add(
                vec: Vector<N, $T, A>,
                a: Vector<N, $T, A>,
                b: Vector<N, $T, A>,
            ) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].mul_add(a[i], b[i]))
            }

            #[cfg(backend)]
            #[inline]
            fn vec_div_euclid(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].div_euclid(rhs[i]))
            }

            #[cfg(backend)]
            #[inline]
            fn vec_rem_euclid(vec: Vector<N, $T, A>, rhs: Vector<N, $T, A>) -> Vector<N, $T, A> {
                Vector::from_fn(|i| vec[i].rem_euclid(rhs[i]))
            }

            #[cfg(backend)]
            #[inline]
            fn vec_sqrt(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::sqrt)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_sin(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::sin)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_cos(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::cos)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_tan(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::tan)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_asin(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::asin)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_acos(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::acos)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_atan(vec: Vector<N, $T, A>) -> Vector<N, $T, A> {
                vec.map($T::atan)
            }

            #[cfg(backend)]
            #[inline]
            fn vec_sin_cos(vec: Vector<N, $T, A>) -> (Vector<N, $T, A>, Vector<N, $T, A>) {
                let array = vec.to_array().map(|x| x.sin_cos());

                (
                    Vector::from_fn(|i| array[i].0),
                    Vector::from_fn(|i| array[i].1),
                )
            }
        }
    };
}
impl_float!(f32, F32VectorBackend);
impl_float!(f64, F64VectorBackend);
