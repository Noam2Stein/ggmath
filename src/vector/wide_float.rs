use wide::{f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, u32x4, u32x8, u32x16, u64x2, u64x4, u64x8};

use crate::{
    Alignment, FloatExt, Length, Quaternion, SupportedLength, Vector,
    utils::{FloatUtils, specialize, transmute_generic},
};

macro_rules! items {
    ($Wide:ident, $UnsignedWide:ty, $powf:ident) => {
        /// A vector with all elements set to [`MIN`].
        ///
        /// [`MIN`]: f32::MIN
        pub const MIN: Self = Self::splat($Wide::MIN);

        /// A vector with all elements set to [`MAX`].
        ///
        /// [`MAX`]: f32::MAX
        pub const MAX: Self = Self::splat($Wide::MAX);

        /// A vector with all elements set to NaN (Not a Number).
        pub const NAN: Self = Self::splat($Wide::NAN);

        /// A vector with all elements set to [`INFINITY`].
        ///
        /// [`INFINITY`]: f32::INFINITY
        pub const INFINITY: Self = Self::splat($Wide::INFINITY);

        /// A vector with all elements set to [`NEG_INFINITY`].
        ///
        /// [`NEG_INFINITY`]: f32::NEG_INFINITY
        pub const NEG_INFINITY: Self = Self::splat($Wide::NEG_INFINITY);

        /// For each lane, returns `true` if any element is NaN.
        #[inline]
        #[must_use]
        pub fn is_nan(self) -> $Wide {
            self.nan_mask().any()
        }

        /// For each lane, returns a vector mask where each element is `true` if
        /// the corresponding element of `self` is NaN.
        ///
        /// Equivalent to `(self.x.is_nan(), self.y.is_nan(), ...)` for each
        /// lane.
        #[inline]
        #[must_use]
        pub fn nan_mask(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::nan_mask_backend(self))
        }

        /// For each lane, returns `true` if all elements are neither infinite
        /// nor NaN.
        #[inline]
        #[must_use]
        pub fn is_finite(self) -> $Wide {
            self.finite_mask().all()
        }

        /// For each lane, returns a vector mask where each element is `true` if
        /// the corresponding element of `self` is neither infinite nor NaN.
        ///
        /// Equivalent to `(self.x.is_finite(), self.y.is_finite(), ...)` for
        /// each lane.
        #[inline]
        #[must_use]
        pub fn finite_mask(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::finite_mask_backend(self))
        }

        /// For each lane, returns a vector mask where each element is `true` if
        /// the corresponding element of `self` has a positive sign, including
        /// `+0.0`, NaNs with positive sign bit and positive infinity.
        ///
        /// Equivalent to
        /// `(self.x.is_sign_positive(), self.y.is_sign_positive(), ...)`
        /// for each lane.
        #[inline]
        #[must_use]
        pub fn sign_positive_mask(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::sign_positive_mask_backend(self))
        }

        /// For each lane, returns a vector mask where each element is `true` if
        /// the corresponding element of `self` has a negative sign, including
        /// `-0.0`, NaNs with negative sign bit and negative infinity.
        ///
        /// Equivalent to
        /// `(self.x.is_sign_negative(), self.y.is_sign_negative(), ...)`
        /// for each lane.
        #[inline]
        #[must_use]
        pub fn sign_negative_mask(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::sign_negative_mask_backend(self))
        }

        /// Returns the element-wise reciprocal (inverse) of a vector,
        /// `1 / self`.
        #[inline]
        #[must_use]
        pub fn recip(self) -> Self {
            Self::ONE / self
        }

        /// Returns the maximum elements between `self` and `other`.
        ///
        /// Equivalent to `(self.x.max(other.x), self.y.max(other.y), ...)`.
        ///
        /// This is not consistent with IEEE semantics in regards to NaN
        /// propagation and handling of `-0.0`.
        #[inline]
        #[must_use]
        pub fn max(self, other: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::max_backend(self, other))
        }

        /// Returns the minimum elements between `self` and `other`.
        ///
        /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
        ///
        /// This is not consistent with IEEE semantics in regards to NaN
        /// propagation and handling of `-0.0`.
        #[inline]
        #[must_use]
        pub fn min(self, other: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::min_backend(self, other))
        }

        /// For each lane, clamps the elements of `self` between the elements of
        /// `min` and `max`.
        ///
        /// Equivalent to
        /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
        ///
        /// This is not consistent with IEEE semantics in regards to NaN
        /// propagation and handling of `-0.0`.
        #[inline]
        #[must_use]
        pub fn clamp(self, min: Self, max: Self) -> Self {
            self.max(min).min(max)
        }

        /// For each lane, returns the maximum between the elements of `self`.
        ///
        /// Equivalent to `self.x.max(self.y).max(self.z)...` for each lane.
        ///
        /// This is not consistent with IEEE semantics in regards to NaN
        /// propagation and handling of `-0.0`.
        #[inline]
        #[must_use]
        pub fn max_element(self) -> $Wide {
            specialize!(Vector::<N, $Wide, A>::max_element_backend(self))
        }

        /// For each lane, returns the minimum between the elements of `self`.
        ///
        /// Equivalent to `self.x.min(self.y).min(self.z)...` for each lane.
        ///
        /// This is not consistent with IEEE semantics in regards to NaN
        /// propagation and handling of `-0.0`.
        #[inline]
        #[must_use]
        pub fn min_element(self) -> $Wide {
            specialize!(Vector::<N, $Wide, A>::min_element_backend(self))
        }

        /// Returns the absolute values of elements of `self`.
        ///
        /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
        #[inline]
        #[must_use]
        pub fn abs(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::abs_backend(self))
        }

        /// Returns the signum of the elements of `self`.
        ///
        /// Equivalent to `(self.x.signum(), self.y.signum(), ...)`.
        #[inline]
        #[must_use]
        pub fn signum(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::signum_backend(self))
        }

        /// Returns a vector with the element magnitudes of `self` and the
        /// element signs of `sign`.
        ///
        /// Equivalent to
        /// `(self.x.copysign(sign.x), self.y.copysign(sign.y), ...)`.
        #[inline]
        #[must_use]
        pub fn copysign(self, sign: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::copysign_backend(self, sign))
        }

        /// Returns the largest integers less than or equal to the elements of
        /// `self`.
        ///
        /// This always returns the precise result.
        #[inline]
        #[must_use]
        pub fn floor(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::floor_backend(self))
        }

        /// Returns the smallest integers greater than or equal to the elements
        /// of `self`.
        ///
        /// This always returns the precise result.
        #[inline]
        #[must_use]
        pub fn ceil(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::ceil_backend(self))
        }

        /// Returns the nearest integers to the elements of `self`.
        ///
        /// This always returns the precise result.
        #[inline]
        #[must_use]
        pub fn round(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::round_backend(self))
        }

        /// Returns the integer part of the elements of `self`. This means that
        /// non-integer numbers are always truncated towards zero.
        ///
        /// This always returns the precise result.
        #[inline]
        #[must_use]
        pub fn trunc(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::trunc_backend(self))
        }

        /// Returns the fractional part of `self`. Equivalent to
        /// `self - self.trunc()`.
        ///
        /// This always returns the precise result.
        #[inline]
        #[must_use]
        pub fn fract(self) -> Self {
            self - self.trunc()
        }

        /// Fused multiply-add. Computes `(self * a) + b`.
        ///
        /// When hardware FMA support is available, this computes the result
        /// with only one rounding error. Without FMA support, this falls back
        /// to separate multiply and add operations with two rounding errors.
        ///
        /// This is inconsistent with the scalar definition of `mul_add` that
        /// always computes the result with only one rounding error.
        #[inline]
        #[must_use]
        pub fn mul_add(self, a: Self, b: Self) -> Self {
            specialize!(Vector::<N, $Wide, A>::mul_add_backend(self, a, b))
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
            specialize!(Vector::<N, $Wide, A>::div_euclid_backend(self, rhs))
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
            specialize!(Vector::<N, $Wide, A>::rem_euclid_backend(self, rhs))
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
        pub fn powf(self, n: $Wide) -> Self {
            specialize!(Vector::<N, $Wide, A>::powf_backend(self, n))
        }

        /// Returns the square root of the elements of `self`.
        ///
        /// Equivalent to `(self.x.sqrt(), self.y.sqrt(), ...)`.
        ///
        /// # Precision
        ///
        /// The result of this operation is guaranteed to be the rounded
        /// infinite-precision result. It is specified by IEEE 754 as
        /// `squareRoot` and guaranteed not to change.
        #[inline]
        #[must_use]
        pub fn sqrt(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::sqrt_backend(self))
        }

        /// Computes the exponential function `e^x` for the elements of
        /// `self`.
        ///
        /// # Unspecified precision
        ///
        /// The precision of this function is non-deterministic. This means it
        /// varies by platform, version, and can even differ within the same
        /// execution from one invocation to the next.
        #[inline]
        #[must_use]
        pub fn exp(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::exp_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::exp2_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::ln_backend(self))
        }

        /// Computes the base 2 logarithm for the elements of `self`.
        ///
        /// # Unspecified precision
        ///
        /// The precision of this function is non-deterministic. This means it
        /// varies by platform, version, and can even differ within the same
        /// execution from one invocation to the next.
        #[inline]
        #[must_use]
        pub fn log2(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::log2_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::sin_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::cos_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::tan_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::asin_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::acos_backend(self))
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
            specialize!(Vector::<N, $Wide, A>::atan_backend(self))
        }

        /// Simultaneously computes the sine and cosine of the elements of
        /// `self`.
        ///
        /// Equivalent to `(self.sin(), self.cos())`, but may be more
        /// performant. This might return a slightly different value.
        ///
        /// # Unspecified precision
        ///
        /// The precision of this function is non-deterministic. This means it
        /// varies by platform, version, and can even differ within the same
        /// execution from one invocation to the next.
        #[inline]
        #[must_use]
        pub fn sin_cos(self) -> (Self, Self) {
            specialize!(Vector::<N, $Wide, A>::sin_cos_backend(self))
        }

        /// Computes the linear interpolation between `self` and `other` based
        /// on the value `t`.
        ///
        /// When `t` is `0`, the result is `self`. When `t` is `1`, the result
        /// is `rhs`. When `t` is outside of the range `0..=1`, the result is
        /// linearly extrapolated.
        #[inline]
        #[must_use]
        pub fn lerp(self, other: Self, t: $Wide) -> Self {
            self * ($Wide::ONE - t) + other * t
        }

        /// Computes the middle point between `self` and `other`.
        ///
        /// Equivalent to `self.lerp(other, 0.5)`, but is cheaper to compute.
        /// This may return a slightly different value.
        #[inline]
        #[must_use]
        pub fn midpoint(self, other: Self) -> Self {
            (self + other) * $Wide::HALF
        }

        /// Moves `self` towards `other` by at most `max_delta`.
        ///
        /// When `max_delta` is `0`, the result is `self`. When `max_delta` is
        /// equal to or greater than `self.distance(other)`, the result is
        /// `other`.
        #[inline]
        #[must_use]
        pub fn move_towards(self, target: Self, max_delta: $Wide) -> Self {
            let delta = target - self;
            let delta_length = delta.length();

            (delta_length.simd_le(max_delta) | delta_length.simd_le($Wide::splat(1e-4)))
                .select(target, self + delta / delta_length * max_delta)
        }

        /// For each lane, computes the spherical linear interpolation between
        /// `self` and `other` based on the value `t`.
        ///
        /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result
        /// is `other`. When `t` is outside of the range `0..=1`, the result is
        /// spherically linearly extrapolated.
        ///
        /// The vectors do not need to be unit vectors but they do need to be
        /// non-zero.
        #[inline]
        #[must_use]
        pub fn slerp(self, other: Self, t: $Wide) -> Self {
            specialize!(Vector::<N, $Wide, A>::slerp_backend(self, other, t))
        }

        /// For each lane, rotates `self` towards `target` by at most
        /// `max_angle` (in radians).
        ///
        /// When `max_angle` is `0`, the result is `self`. When `max_angle` is
        /// equal to or greater than `self.angle_between(target)`, the result is
        /// `target`. When `max_angle` is negative, this rotates towards
        /// `-target`.
        ///
        /// The vectors do not need to be unit vectors but `target` does need to
        /// be non-zero.
        #[inline]
        #[must_use]
        pub fn rotate_towards(self, target: Self, max_angle: $Wide) -> Self {
            specialize!(Vector::<N, $Wide, A>::rotate_towards_backend(
                self, target, max_angle
            ))
        }

        /// Returns the length/magnitude of `self`.
        #[inline]
        #[must_use]
        pub fn length(self) -> $Wide {
            self.dot(self).sqrt()
        }

        /// Computes the Euclidean distance between `self` and `other`.
        #[inline]
        #[must_use]
        pub fn distance(self, other: Self) -> $Wide {
            (self - other).length()
        }

        /// For each lane, returns a vector with the direction of `self` and
        /// length `1`.
        #[inline]
        #[must_use]
        pub fn normalize(self) -> Self {
            self / self.length()
        }

        // `try_normalize` is exluded on purpose. It would not be useful because
        // it would only return `Some` if all lanes succeed.

        /// Returns [`normalize`] for each lane, or `fallback` if `self` is zero
        /// or if the result is non finite or zero.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn normalize_or(self, fallback: Self) -> Self {
            let length_recip = $Wide::ONE / self.length();

            (length_recip.is_finite() & length_recip.simd_gt($Wide::ZERO))
                .select(self * length_recip, fallback)
        }

        /// Returns [`normalize`] for each lane, or a zero vector if `self` is
        /// zero or if the result is non finite.
        ///
        /// The fallback is only applied for invalid lanes. Other lanes are not
        /// affected.
        ///
        /// [`normalize`]: Self::normalize
        #[inline]
        #[must_use]
        pub fn normalize_or_zero(self) -> Self {
            let length_recip = $Wide::ONE / self.length();

            (length_recip.is_finite() & length_recip.simd_gt($Wide::ZERO))
                .select(self * length_recip, Self::ZERO)
        }

        /// Simultaneously computes [`normalize`] and [`length`].
        ///
        /// If `self` is a zero vector, the result for that lane is length `0`
        /// and an unspecified vector. Consider manually checking for
        /// `length == 0.0`.
        ///
        /// [`normalize`]: Self::normalize
        /// [`length`]: Self::length
        #[inline]
        #[must_use]
        pub fn normalize_and_length(self) -> (Self, $Wide) {
            let length = self.length();
            (self / length, length)
        }

        /// For each lane, returns whether the vector has the length `1` or not.
        ///
        /// This uses a precision threshold of approximately `1e-4`.
        #[inline]
        #[must_use]
        pub fn is_normalized(self) -> $Wide {
            (self.length_squared() - $Wide::ONE)
                .abs()
                .simd_le($Wide::splat(2e-4))
        }

        /// For each lane, returns `self` with a length of no more than `max`.
        #[inline]
        #[must_use]
        pub fn with_max_length(self, max: $Wide) -> Self {
            let length_squared = self.length_squared();
            length_squared
                .simd_gt(max * max)
                .select(self / length_squared.sqrt() * max, self)
        }

        /// For any lane, returns `self` with a length of no less than `min`.
        ///
        /// If `min` is negative, this returns `self` for that lane.
        #[inline]
        #[must_use]
        pub fn with_min_length(self, min: $Wide) -> Self {
            let length_squared = self.length_squared();
            length_squared
                .simd_lt(min * min.abs())
                .select(self / length_squared.sqrt() * min, self)
        }

        /// For each lane, returns `self` with a length of no less than `min`
        /// and no more than `max`.
        ///
        /// If `min` is negative it is ignored.
        #[inline]
        #[must_use]
        pub fn clamp_length(self, min: $Wide, max: $Wide) -> Self {
            let length_squared = self.length_squared();
            length_squared.simd_lt(min * min.abs()).select(
                self / length_squared.sqrt() * min,
                length_squared
                    .simd_gt(max * max)
                    .select(self / length_squared.sqrt() * max, self),
            )
        }

        /// For each lane, returns the angle (in radians) between `self` and
        /// `other` in the range `0..=+π`.
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
        pub fn angle_between(self, other: Self) -> $Wide {
            (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt())
                .acos_approx()
        }

        /// Returns the vector projection of `self` onto `other`.
        ///
        /// `other` must not be a zero vector.
        #[inline]
        #[must_use]
        pub fn project_onto(self, other: Self) -> Self {
            let other_length_squared_recip = $Wide::ONE / other.length_squared();

            other * self.dot(other) * other_length_squared_recip
        }

        /// Returns the vector projection of `self` onto `other`.
        ///
        /// `other` must be normalized.
        #[inline]
        #[must_use]
        pub fn project_onto_normalized(self, other: Self) -> Self {
            other * self.dot(other)
        }

        /// Returns the vector rejection of `self` from `other`.
        ///
        /// Equivalent to `self - self.project_onto(other)`.
        ///
        /// `other` must not be a zero vector.
        #[inline]
        #[must_use]
        pub fn reject_from(self, other: Self) -> Self {
            self - self.project_onto(other)
        }

        /// Returns the vector rejection of `self` from `other`.
        ///
        /// Equivalent to `self - self.project_onto(other)`.
        ///
        /// `other` must be normalized.
        #[inline]
        #[must_use]
        pub fn reject_from_normalized(self, other: Self) -> Self {
            self - self.project_onto_normalized(other)
        }

        /// Returns the reflection of `self` through `normal`.
        ///
        /// `normal` must be normalized.
        #[inline]
        #[must_use]
        pub fn reflect(self, normal: Self) -> Self {
            self - normal * ($Wide::splat(2.0) * self.dot(normal))
        }

        /// Returns the vector refraction of `self` through `normal` and `eta`.
        ///
        /// `eta` is the incident refraction-index divided by the transmitted
        /// refraction-index.
        ///
        /// When total internal reflection occurs, the result is a zero vector.
        ///
        /// `self` and `normal` must be normalized.
        #[inline]
        #[must_use]
        pub fn refract(self, normal: Self, eta: $Wide) -> Self {
            let self_dot_normal = self.dot(normal);
            let k = $Wide::ONE - eta * eta * ($Wide::ONE - self_dot_normal * self_dot_normal);

            k.simd_ge($Wide::ZERO).select(
                self * eta - normal * (eta * self_dot_normal + k.sqrt()),
                Self::ZERO,
            )
        }

        /// For each lane, returns some vector that is orthogonal to `self`.
        ///
        /// The result is not necessarily normalized.
        ///
        /// For 2D vectors this is equivalent to [`perp`].
        ///
        /// [`perp`]: Vector::perp
        #[inline]
        #[must_use]
        pub fn any_orthogonal_vector(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::any_orthogonal_vector_backend(self))
        }

        /// For each lane, returns some unit vector that is orthogonal to
        /// `self`.
        ///
        /// `self` must normalized.
        ///
        /// For 2D vectors this is equivalent to [`perp`].
        ///
        /// [`perp`]: Self::perp
        #[inline]
        #[must_use]
        pub fn any_orthonormal_vector(self) -> Self {
            specialize!(Vector::<N, $Wide, A>::any_orthonormal_vector_backend(self))
        }

        /// Returns `true` if the absolute difference of all elements between
        /// `self` and `other` is less than or equal to `max_abs_diff` for all
        /// lanes.
        ///
        /// This can be used to compare two vectors that should be equal, but
        /// may have a slight difference due to operations having rounding
        /// errors.
        #[inline]
        #[must_use]
        pub fn abs_diff_eq(self, other: Self, max_abs_diff: $Wide) -> bool {
            (self - other)
                .abs()
                .simd_le_mask(Self::splat(max_abs_diff))
                .all()
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
        pub const fn to_bits(self) -> Vector<N, $UnsignedWide, A> {
            // SAFETY: Both types accept all bit-patterns.
            unsafe { transmute_generic::<Vector<N, $Wide, A>, Vector<N, $UnsignedWide, A>>(self) }
        }

        /// Raw transmutation from unsigned integer vector.
        ///
        /// Note that this function is distinct from [`as`] conversions, which
        /// attempt to preserve the *numeric* value, and not the bitwise value.
        ///
        /// [`as`]: https://rust-for-c-programmers.com/ch16/16_2_primitive_casting_with_as.html
        #[inline]
        #[must_use]
        pub const fn from_bits(value: Vector<N, $UnsignedWide, A>) -> Self {
            // SAFETY: Both types accept all bit-patterns.
            unsafe { transmute_generic::<Vector<N, $UnsignedWide, A>, Vector<N, $Wide, A>>(value) }
        }
    };
}

macro_rules! items_2 {
    ($Wide:ident, $UnsignedWide:ty, $powf:ident) => {
        /// For each lane, creates a 2D vector from homogeneous coordinates by
        /// performing perspective divide.
        ///
        /// Equivalent to `homogeneous.xy / homogeneous.z`.
        #[inline]
        #[must_use]
        pub fn from_homogeneous(homogeneous: Vector<3, $Wide, A>) -> Self {
            homogeneous.xy() / homogeneous.z
        }

        /// For each lane, returns the angle (in radians) that rotates `self` to
        /// `other` in the range `-π..=+π`.
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
        #[inline]
        #[must_use]
        pub fn angle_to(self, other: Self) -> $Wide {
            let outer_product = (self.x * other.y) - (self.y * other.x);
            self.angle_between(other) * outer_product.signum()
        }

        /// For each lane, returns the angle (in radians) that rotates `other`
        /// to `self` in the range `-π..=+π`.
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
        #[inline]
        #[must_use]
        pub fn angle_from(self, other: Self) -> $Wide {
            let outer_product = (other.x * self.y) - (other.y * self.x);
            self.angle_between(other) * outer_product.signum()
        }

        /// For each lane, rotates `self` by `angle` (in radians).
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
        pub fn rotate(self, angle: $Wide) -> Self {
            let (angle_sin, angle_cos) = angle.sin_cos();
            Self::new(
                self.x * angle_cos - self.y * angle_sin,
                self.x * angle_sin + self.y * angle_cos,
            )
        }
    };
}

macro_rules! items_3 {
    ($Wide:ident, $UnsignedWide:ty, $powf:ident) => {
        /// For each lane, creates a 3D vector from homogeneous coordinates by
        /// performing perspective divide.
        ///
        /// Equivalent to `homogeneous.xyz / homogeneous.w`.
        #[inline]
        #[must_use]
        pub fn from_homogeneous(homogeneous: Vector<4, $Wide, A>) -> Self {
            homogeneous.xyz() / homogeneous.w
        }

        /// For each lane, rotates `self` around the x axis by `angle` (in
        /// radians).
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
        pub fn rotate_x(self, angle: $Wide) -> Self {
            let (angle_sin, angle_cos) = angle.sin_cos();
            Self::new(
                self.x,
                self.y * angle_cos - self.z * angle_sin,
                self.y * angle_sin + self.z * angle_cos,
            )
        }

        /// For each lane, rotates `self` around the y axis by `angle` (in
        /// radians).
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
        pub fn rotate_y(self, angle: $Wide) -> Self {
            let (angle_sin, angle_cos) = angle.sin_cos();
            Self::new(
                self.x * angle_cos + self.z * angle_sin,
                self.y,
                self.x * -angle_sin + self.z * angle_cos,
            )
        }

        /// For each lane, rotates `self` around the z axis by `angle` (in
        /// radians).
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
        pub fn rotate_z(self, angle: $Wide) -> Self {
            let (angle_sin, angle_cos) = angle.sin_cos();
            Self::new(
                self.x * angle_cos - self.y * angle_sin,
                self.x * angle_sin + self.y * angle_cos,
                self.z,
            )
        }

        /// For each lane, returns two unit vectors that are orthogonal to
        /// `self` and to each other.
        ///
        /// Together with `self`, they form an orthonormal basis where the three
        /// vectors are all orthogonal to each other and are normalized.
        #[inline]
        #[must_use]
        pub fn any_orthonormal_pair(self) -> (Self, Self) {
            // From https://graphics.pixar.com/library/OrthonormalB/paper.pdf
            let sign = self.z.signum();
            let a = -$Wide::ONE / (sign + self.z);
            let b = self.x * self.y * a;
            (
                Self::new(
                    $Wide::ONE + sign * self.x * self.x * a,
                    sign * b,
                    -sign * self.x,
                ),
                Self::new(b, sign + self.y * self.y * a, -self.y),
            )
        }
    };
}

// Since all wide-float functions have names that conflict with normal float
// functions, We cannot implement this API using generics. Duplicating the API
// for each supported wide-float type works, but then documentation shows the
// duplicated API, making it hard to read.
//
// When generating documentation, Rust does not care that these items are
// conflicting. This allows us to cheat by showing these items in a generic
// context in documentation, but making them separate in all other cases.

#[cfg(doc)]
#[doc(hidden)]
pub trait WideFloat: crate::Scalar {
    type Bits: crate::Scalar;
}

/// Functionality for [SoA] (Structure of Arrays) float vectors.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<const N: usize, Wide, A: Alignment> Vector<N, Wide, A>
where
    Length<N>: SupportedLength,
    Wide: WideFloat,
{
    items!(Wide, <Wide as WideFloat>::Bits, powf);
}

/// Functionality for [SoA] (Structure of Arrays) float vector2s.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Vector<2, Wide, A>
where
    Wide: WideFloat,
{
    items_2!(Wide, <Wide as WideFloat>::Bits, powf);
}

/// Functionality for [SoA] (Structure of Arrays) float vector3s.
///
/// This is gated behind the `wide` feature flag.
///
/// This functionality is shown with generics to make it easier to read. This
/// works with all float types from the [`wide`] crate.
///
/// [SoA]: crate#soa
/// [`wide`]: https://crates.io/crates/wide
#[cfg(doc)]
impl<Wide, A: Alignment> Vector<3, Wide, A>
where
    Wide: WideFloat,
{
    items_3!(Wide, <Wide as WideFloat>::Bits, powf);
}

macro_rules! impl_items {
    ($Wide:ident, $UnsignedWide:ty, $powf:ident) => {
        #[cfg(not(doc))]
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            items!($Wide, $UnsignedWide, $powf);
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Vector<2, $Wide, A> {
            items_2!($Wide, $UnsignedWide, $powf);

            #[inline(always)]
            fn nan_mask_backend(self) -> Self {
                Self::new(self.x.is_nan(), self.y.is_nan())
            }

            #[inline(always)]
            fn finite_mask_backend(self) -> Self {
                Self::new(self.x.is_finite(), self.y.is_finite())
            }

            #[inline(always)]
            fn sign_positive_mask_backend(self) -> Self {
                Self::new(self.x.is_sign_positive(), self.y.is_sign_positive())
            }

            #[inline(always)]
            fn sign_negative_mask_backend(self) -> Self {
                Self::new(self.x.is_sign_negative(), self.y.is_sign_negative())
            }

            #[inline(always)]
            fn max_backend(self, other: Self) -> Self {
                Self::new(self.x.fast_max(other.x), self.y.fast_max(other.y))
            }

            #[inline(always)]
            fn min_backend(self, other: Self) -> Self {
                Self::new(self.x.fast_min(other.x), self.y.fast_min(other.y))
            }

            #[inline(always)]
            fn max_element_backend(self) -> $Wide {
                self.x.fast_max(self.y)
            }

            #[inline(always)]
            fn min_element_backend(self) -> $Wide {
                self.x.fast_min(self.y)
            }

            #[inline(always)]
            fn abs_backend(self) -> Self {
                Self::new(self.x.abs(), self.y.abs())
            }

            #[inline(always)]
            fn signum_backend(self) -> Self {
                Self::new(self.x.signum(), self.y.signum())
            }

            #[inline(always)]
            fn copysign_backend(self, sign: Self) -> Self {
                Self::new(self.x.copysign(sign.x), self.y.copysign(sign.y))
            }

            #[inline(always)]
            fn floor_backend(self) -> Self {
                Self::new(self.x.floor(), self.y.floor())
            }

            #[inline(always)]
            fn ceil_backend(self) -> Self {
                Self::new(self.x.ceil(), self.y.ceil())
            }

            #[inline(always)]
            fn round_backend(self) -> Self {
                Self::new(self.x.round(), self.y.round())
            }

            #[inline(always)]
            fn trunc_backend(self) -> Self {
                Self::new(self.x.trunc(), self.y.trunc())
            }

            #[inline(always)]
            fn mul_add_backend(self, a: Self, b: Self) -> Self {
                Self::new(self.x.mul_add(a.x, b.x), self.y.mul_add(a.y, b.y))
            }

            #[inline(always)]
            fn div_euclid_backend(self, rhs: Self) -> Self {
                Self::new(self.x.div_euclid(rhs.x), self.y.div_euclid(rhs.y))
            }

            #[inline(always)]
            fn rem_euclid_backend(self, rhs: Self) -> Self {
                Self::new(self.x.rem_euclid(rhs.x), self.y.rem_euclid(rhs.y))
            }

            #[inline(always)]
            fn powf_backend(self, n: $Wide) -> Self {
                Self::new(self.x.$powf(n), self.y.$powf(n))
            }

            #[inline(always)]
            fn sqrt_backend(self) -> Self {
                Self::new(self.x.sqrt(), self.y.sqrt())
            }

            #[inline(always)]
            fn exp_backend(self) -> Self {
                Self::new(self.x.exp(), self.y.exp())
            }

            #[inline(always)]
            fn exp2_backend(self) -> Self {
                Self::new(self.x.exp2(), self.y.exp2())
            }

            #[inline(always)]
            fn ln_backend(self) -> Self {
                Self::new(self.x.ln(), self.y.ln())
            }

            #[inline(always)]
            fn log2_backend(self) -> Self {
                Self::new(self.x.log2(), self.y.log2())
            }

            #[inline(always)]
            fn sin_backend(self) -> Self {
                Self::new(self.x.sin(), self.y.sin())
            }

            #[inline(always)]
            fn cos_backend(self) -> Self {
                Self::new(self.x.cos(), self.y.cos())
            }

            #[inline(always)]
            fn tan_backend(self) -> Self {
                Self::new(self.x.tan(), self.y.tan())
            }

            #[inline(always)]
            fn asin_backend(self) -> Self {
                Self::new(self.x.asin(), self.y.asin())
            }

            #[inline(always)]
            fn acos_backend(self) -> Self {
                Self::new(self.x.acos(), self.y.acos())
            }

            #[inline(always)]
            fn atan_backend(self) -> Self {
                Self::new(self.x.atan(), self.y.atan())
            }

            #[inline(always)]
            fn sin_cos_backend(self) -> (Self, Self) {
                let x_sin_cos = self.x.sin_cos();
                let y_sin_cos = self.y.sin_cos();
                (
                    Self::new(x_sin_cos.0, y_sin_cos.0),
                    Self::new(x_sin_cos.1, y_sin_cos.1),
                )
            }

            #[inline(always)]
            fn slerp_backend(self, other: Self, t: $Wide) -> Self {
                let self_length = self.length();
                let other_length = other.length();

                let self_normalized = self / self_length;
                let angle_cos = self_normalized.dot(other) / other_length;
                let angle = angle_cos.acos_approx() * self_normalized.wedge(other).signum();

                let result_length = self_length.lerp(other_length, t);
                self_normalized.rotate(angle * t) * result_length
            }

            #[inline(always)]
            fn rotate_towards_backend(self, target: Self, max_angle: $Wide) -> Self {
                let self_length = self.length();
                let target_length = target.length();

                if self == Self::ZERO {
                    return self;
                }

                let target_angle = (self.dot(target) / self_length / target_length).acos_approx();
                let angle_sign = self.wedge(target).signum();
                let angle = max_angle.simd_lt(target_angle - $Wide::PI).select(
                    target_angle - $Wide::PI,
                    max_angle
                        .simd_gt(target_angle)
                        .select(target_angle, max_angle),
                ) * angle_sign;

                self.simd_eq(Self::ZERO).select(self, self.rotate(angle))
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

        #[cfg(not(doc))]
        impl<A: Alignment> Vector<3, $Wide, A> {
            items_3!($Wide, $UnsignedWide, $powf);

            #[inline(always)]
            fn nan_mask_backend(self) -> Self {
                Self::new(self.x.is_nan(), self.y.is_nan(), self.z.is_nan())
            }

            #[inline(always)]
            fn finite_mask_backend(self) -> Self {
                Self::new(self.x.is_finite(), self.y.is_finite(), self.z.is_finite())
            }

            #[inline(always)]
            fn sign_positive_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_sign_positive(),
                    self.y.is_sign_positive(),
                    self.z.is_sign_positive(),
                )
            }

            #[inline(always)]
            fn sign_negative_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_sign_negative(),
                    self.y.is_sign_negative(),
                    self.z.is_sign_negative(),
                )
            }

            #[inline(always)]
            fn max_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.fast_max(other.x),
                    self.y.fast_max(other.y),
                    self.z.fast_max(other.z),
                )
            }

            #[inline(always)]
            fn min_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.fast_min(other.x),
                    self.y.fast_min(other.y),
                    self.z.fast_min(other.z),
                )
            }

            #[inline(always)]
            fn max_element_backend(self) -> $Wide {
                self.x.fast_max(self.y).fast_max(self.z)
            }

            #[inline(always)]
            fn min_element_backend(self) -> $Wide {
                self.x.fast_min(self.y).fast_min(self.z)
            }

            #[inline(always)]
            fn abs_backend(self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs())
            }

            #[inline(always)]
            fn signum_backend(self) -> Self {
                Self::new(self.x.signum(), self.y.signum(), self.z.signum())
            }

            #[inline(always)]
            fn copysign_backend(self, sign: Self) -> Self {
                Self::new(
                    self.x.copysign(sign.x),
                    self.y.copysign(sign.y),
                    self.z.copysign(sign.z),
                )
            }

            #[inline(always)]
            fn floor_backend(self) -> Self {
                Self::new(self.x.floor(), self.y.floor(), self.z.floor())
            }

            #[inline(always)]
            fn ceil_backend(self) -> Self {
                Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil())
            }

            #[inline(always)]
            fn round_backend(self) -> Self {
                Self::new(self.x.round(), self.y.round(), self.z.round())
            }

            #[inline(always)]
            fn trunc_backend(self) -> Self {
                Self::new(self.x.trunc(), self.y.trunc(), self.z.trunc())
            }

            #[inline(always)]
            fn mul_add_backend(self, a: Self, b: Self) -> Self {
                Self::new(
                    self.x.mul_add(a.x, b.x),
                    self.y.mul_add(a.y, b.y),
                    self.z.mul_add(a.z, b.z),
                )
            }

            #[inline(always)]
            fn div_euclid_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.div_euclid(rhs.x),
                    self.y.div_euclid(rhs.y),
                    self.z.div_euclid(rhs.z),
                )
            }

            #[inline(always)]
            fn rem_euclid_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.rem_euclid(rhs.x),
                    self.y.rem_euclid(rhs.y),
                    self.z.rem_euclid(rhs.z),
                )
            }

            #[inline(always)]
            fn powf_backend(self, n: $Wide) -> Self {
                Self::new(self.x.$powf(n), self.y.$powf(n), self.z.$powf(n))
            }

            #[inline(always)]
            fn sqrt_backend(self) -> Self {
                Self::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt())
            }

            #[inline(always)]
            fn exp_backend(self) -> Self {
                Self::new(self.x.exp(), self.y.exp(), self.z.exp())
            }

            #[inline(always)]
            fn exp2_backend(self) -> Self {
                Self::new(self.x.exp2(), self.y.exp2(), self.z.exp2())
            }

            #[inline(always)]
            fn ln_backend(self) -> Self {
                Self::new(self.x.ln(), self.y.ln(), self.z.ln())
            }

            #[inline(always)]
            fn log2_backend(self) -> Self {
                Self::new(self.x.log2(), self.y.log2(), self.z.log2())
            }

            #[inline(always)]
            fn sin_backend(self) -> Self {
                Self::new(self.x.sin(), self.y.sin(), self.z.sin())
            }

            #[inline(always)]
            fn cos_backend(self) -> Self {
                Self::new(self.x.cos(), self.y.cos(), self.z.cos())
            }

            #[inline(always)]
            fn tan_backend(self) -> Self {
                Self::new(self.x.tan(), self.y.tan(), self.z.tan())
            }

            #[inline(always)]
            fn asin_backend(self) -> Self {
                Self::new(self.x.asin(), self.y.asin(), self.z.asin())
            }

            #[inline(always)]
            fn acos_backend(self) -> Self {
                Self::new(self.x.acos(), self.y.acos(), self.z.acos())
            }

            #[inline(always)]
            fn atan_backend(self) -> Self {
                Self::new(self.x.atan(), self.y.atan(), self.z.atan())
            }

            #[inline(always)]
            fn sin_cos_backend(self) -> (Self, Self) {
                let x_sin_cos = self.x.sin_cos();
                let y_sin_cos = self.y.sin_cos();
                let z_sin_cos = self.z.sin_cos();
                (
                    Self::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0),
                    Self::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1),
                )
            }

            #[inline(always)]
            fn slerp_backend(self, other: Self, t: $Wide) -> Self {
                // Ported from `https://github.com/bitshifter/glam-rs`.

                let self_length = self.length();
                let other_length = other.length();

                let angle_cos = self.dot(other) / (self_length * other_length);

                // If `angle_cos` is close to `1` or `-1` or is NaN the normal
                // calculation breaks down.

                angle_cos.abs().simd_lt($Wide::splat(1.0 - 3e-7)).select(
                    {
                        let angle = angle_cos.acos_approx();
                        let angle_sin = angle.sin();
                        let self_factor = (angle * ($Wide::ONE - t)).sin();
                        let other_factor = (angle * t).sin();

                        let result_length = self_length.lerp(other_length, t);

                        (self * (result_length / self_length) * self_factor
                            + other * (result_length / other_length) * other_factor)
                            / angle_sin
                    },
                    angle_cos.is_sign_negative().select(
                        {
                            // Vectors are almost parallel in opposing directions.

                            let axis = self.any_orthogonal_vector().normalize();
                            let rotation =
                                Quaternion::<$Wide, A>::from_axis_angle(axis, t * $Wide::PI);

                            let result_length = self_length.lerp(other_length, t);
                            self * rotation * (result_length / self_length)
                        },
                        {
                            // Vectors are almost parallel in the same direction.
                            self.lerp(other, t)
                        },
                    ),
                )
            }

            #[inline(always)]
            fn rotate_towards_backend(self, target: Self, max_angle: $Wide) -> Self {
                // Ported from `https://github.com/bitshifter/glam-rs`.

                let self_length = self.length();
                let target_length = target.length();

                if self == Self::ZERO {
                    return self;
                }

                let target_angle = (self.dot(target) / (self_length * target_length)).acos_approx();
                let angle = max_angle.simd_lt(target_angle - $Wide::PI).select(
                    target_angle - $Wide::PI,
                    max_angle
                        .simd_gt(target_angle)
                        .select(target_angle, max_angle),
                );
                let axis = self
                    .cross(target)
                    .normalize_or(self.any_orthonormal_vector());

                self.simd_eq(Self::ZERO).select(
                    self,
                    self * Quaternion::<$Wide, A>::from_axis_angle(axis, angle),
                )
            }

            #[inline(always)]
            fn any_orthogonal_vector_backend(self) -> Self {
                self.x.abs().simd_gt(self.y.abs()).select(
                    Self::new(-self.z, $Wide::ZERO, self.x),
                    Self::new($Wide::ZERO, self.z, -self.y),
                )
            }

            #[inline(always)]
            fn any_orthonormal_vector_backend(self) -> Self {
                // Ported from https://github.com/bitshifter/glam-rs.

                let sign = self.z.signum();
                let a = -$Wide::ONE / (sign + self.z);
                let b = self.x * self.y * a;
                Self::new(b, sign + self.y * self.y * a, -self.y)
            }
        }

        #[cfg(not(doc))]
        impl<A: Alignment> Vector<4, $Wide, A> {
            #[inline(always)]
            fn nan_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_nan(),
                    self.y.is_nan(),
                    self.z.is_nan(),
                    self.w.is_nan(),
                )
            }

            #[inline(always)]
            fn finite_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_finite(),
                    self.y.is_finite(),
                    self.z.is_finite(),
                    self.w.is_finite(),
                )
            }

            #[inline(always)]
            fn sign_positive_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_sign_positive(),
                    self.y.is_sign_positive(),
                    self.z.is_sign_positive(),
                    self.w.is_sign_positive(),
                )
            }

            #[inline(always)]
            fn sign_negative_mask_backend(self) -> Self {
                Self::new(
                    self.x.is_sign_negative(),
                    self.y.is_sign_negative(),
                    self.z.is_sign_negative(),
                    self.w.is_sign_negative(),
                )
            }

            #[inline(always)]
            fn max_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.fast_max(other.x),
                    self.y.fast_max(other.y),
                    self.z.fast_max(other.z),
                    self.w.fast_max(other.w),
                )
            }

            #[inline(always)]
            fn min_backend(self, other: Self) -> Self {
                Self::new(
                    self.x.fast_min(other.x),
                    self.y.fast_min(other.y),
                    self.z.fast_min(other.z),
                    self.w.fast_min(other.w),
                )
            }

            #[inline(always)]
            fn max_element_backend(self) -> $Wide {
                self.x.fast_max(self.y).fast_max(self.z).fast_max(self.w)
            }

            #[inline(always)]
            fn min_element_backend(self) -> $Wide {
                self.x.fast_min(self.y).fast_min(self.z).fast_min(self.w)
            }

            #[inline(always)]
            fn abs_backend(self) -> Self {
                Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
            }

            #[inline(always)]
            fn signum_backend(self) -> Self {
                Self::new(
                    self.x.signum(),
                    self.y.signum(),
                    self.z.signum(),
                    self.w.signum(),
                )
            }

            #[inline(always)]
            fn copysign_backend(self, sign: Self) -> Self {
                Self::new(
                    self.x.copysign(sign.x),
                    self.y.copysign(sign.y),
                    self.z.copysign(sign.z),
                    self.w.copysign(sign.w),
                )
            }

            #[inline(always)]
            fn floor_backend(self) -> Self {
                Self::new(
                    self.x.floor(),
                    self.y.floor(),
                    self.z.floor(),
                    self.w.floor(),
                )
            }

            #[inline(always)]
            fn ceil_backend(self) -> Self {
                Self::new(self.x.ceil(), self.y.ceil(), self.z.ceil(), self.w.ceil())
            }

            #[inline(always)]
            fn round_backend(self) -> Self {
                Self::new(
                    self.x.round(),
                    self.y.round(),
                    self.z.round(),
                    self.w.round(),
                )
            }

            #[inline(always)]
            fn trunc_backend(self) -> Self {
                Self::new(
                    self.x.trunc(),
                    self.y.trunc(),
                    self.z.trunc(),
                    self.w.trunc(),
                )
            }

            #[inline(always)]
            fn mul_add_backend(self, a: Self, b: Self) -> Self {
                Self::new(
                    self.x.mul_add(a.x, b.x),
                    self.y.mul_add(a.y, b.y),
                    self.z.mul_add(a.z, b.z),
                    self.w.mul_add(a.w, b.w),
                )
            }

            #[inline(always)]
            fn div_euclid_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.div_euclid(rhs.x),
                    self.y.div_euclid(rhs.y),
                    self.z.div_euclid(rhs.z),
                    self.w.div_euclid(rhs.w),
                )
            }

            #[inline(always)]
            fn rem_euclid_backend(self, rhs: Self) -> Self {
                Self::new(
                    self.x.rem_euclid(rhs.x),
                    self.y.rem_euclid(rhs.y),
                    self.z.rem_euclid(rhs.z),
                    self.w.rem_euclid(rhs.w),
                )
            }

            #[inline(always)]
            fn powf_backend(self, n: $Wide) -> Self {
                Self::new(
                    self.x.$powf(n),
                    self.y.$powf(n),
                    self.z.$powf(n),
                    self.w.$powf(n),
                )
            }

            #[inline(always)]
            fn sqrt_backend(self) -> Self {
                Self::new(self.x.sqrt(), self.y.sqrt(), self.z.sqrt(), self.w.sqrt())
            }

            #[inline(always)]
            fn exp_backend(self) -> Self {
                Self::new(self.x.exp(), self.y.exp(), self.z.exp(), self.w.exp())
            }

            #[inline(always)]
            fn exp2_backend(self) -> Self {
                Self::new(self.x.exp2(), self.y.exp2(), self.z.exp2(), self.w.exp2())
            }

            #[inline(always)]
            fn ln_backend(self) -> Self {
                Self::new(self.x.ln(), self.y.ln(), self.z.ln(), self.w.ln())
            }

            #[inline(always)]
            fn log2_backend(self) -> Self {
                Self::new(self.x.log2(), self.y.log2(), self.z.log2(), self.w.log2())
            }

            #[inline(always)]
            fn sin_backend(self) -> Self {
                Self::new(self.x.sin(), self.y.sin(), self.z.sin(), self.w.sin())
            }

            #[inline(always)]
            fn cos_backend(self) -> Self {
                Self::new(self.x.cos(), self.y.cos(), self.z.cos(), self.w.cos())
            }

            #[inline(always)]
            fn tan_backend(self) -> Self {
                Self::new(self.x.tan(), self.y.tan(), self.z.tan(), self.w.tan())
            }

            #[inline(always)]
            fn asin_backend(self) -> Self {
                Self::new(self.x.asin(), self.y.asin(), self.z.asin(), self.w.asin())
            }

            #[inline(always)]
            fn acos_backend(self) -> Self {
                Self::new(self.x.acos(), self.y.acos(), self.z.acos(), self.w.acos())
            }

            #[inline(always)]
            fn atan_backend(self) -> Self {
                Self::new(self.x.atan(), self.y.atan(), self.z.atan(), self.w.atan())
            }

            #[inline(always)]
            fn sin_cos_backend(self) -> (Self, Self) {
                let x_sin_cos = self.x.sin_cos();
                let y_sin_cos = self.y.sin_cos();
                let z_sin_cos = self.z.sin_cos();
                let w_sin_cos = self.w.sin_cos();
                (
                    Self::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0, w_sin_cos.0),
                    Self::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1, w_sin_cos.1),
                )
            }

            #[inline(always)]
            fn slerp_backend(self, other: Self, t: $Wide) -> Self {
                // Ported from `https://github.com/bitshifter/glam-rs`.

                let self_length = self.length();
                let other_length = other.length();

                let angle_cos = self.dot(other) / (self_length * other_length);

                // If `angle_cos` is close to `1` or `-1` or is NaN the normal
                // calculation breaks down.
                angle_cos.abs().simd_lt($Wide::splat(1.0 - 3e-7)).select(
                    {
                        let angle = angle_cos.acos_approx();
                        let angle_sin = angle.sin();
                        let t1 = (angle * ($Wide::ONE - t)).sin();
                        let t2 = (angle * t).sin();

                        let result_length = self_length.lerp(other_length, t);

                        (self * (result_length / self_length) * t1
                            + other * (result_length / other_length) * t2)
                            / angle_sin
                    },
                    angle_cos.is_sign_negative().select(
                        {
                            // Vectors are almost parallel in opposing directions.

                            let axis = self.any_orthogonal_vector().normalize();
                            let (sin, cos) = (t * $Wide::PI).sin_cos();

                            let result_dir = self * cos + axis * sin;
                            let result_length = self_length.lerp(other_length, t);
                            result_dir * (result_length / result_dir.length())
                        },
                        {
                            // Vectors are almost parallel in the same direction.
                            self.lerp(other, t)
                        },
                    ),
                )
            }

            #[inline(always)]
            fn rotate_towards_backend(self, target: Self, max_angle: $Wide) -> Self {
                // Ported from `https://github.com/bitshifter/glam-rs`.

                let self_length = self.length();
                let target_length = target.length();

                if self == Self::ZERO {
                    return self;
                }

                let target_angle_cos = self.dot(target) / (self_length * target_length);
                let target_angle = target_angle_cos.acos_approx();
                let angle = max_angle.simd_lt(target_angle - $Wide::PI).select(
                    target_angle - $Wide::PI,
                    max_angle
                        .simd_gt(target_angle)
                        .select(target_angle, max_angle),
                );

                if angle == $Wide::ZERO {
                    return self;
                }

                // If `target_angle_cos` is close to `1` or `-1` or is NaN the
                // normal calculation breaks down.
                let result = target_angle_cos
                    .abs()
                    .simd_le($Wide::splat(1.0 - 3e-7))
                    .select(
                        {
                            let self_factor = (target_angle - angle).sin();
                            let target_factor = angle.sin();
                            let result = self * self_factor
                                + target * (self_length / target_length) * target_factor;

                            result / result.length() * self_length
                        },
                        target_angle_cos.is_sign_negative().select(
                            {
                                // Vectors are almost parallel in opposing directions.

                                let axis = self.any_orthogonal_vector();
                                let axis = axis / axis.length();
                                let (sin, cos) = angle.sin_cos();

                                let result_dir = self * cos + axis * sin;
                                result_dir * (self_length / result_dir.length())
                            },
                            {
                                // Vectors are almost parallel in the same direction.
                                target / target_length * self_length
                            },
                        ),
                    );

                (self.simd_eq(Self::ZERO) | angle.simd_eq($Wide::ZERO)).select(self, result)
            }

            #[inline(always)]
            fn any_orthogonal_vector_backend(self) -> Self {
                let self_abs = self.abs();

                self_abs.x.simd_gt(self_abs.y).select(
                    self_abs.x.simd_gt(self_abs.z).select(
                        Self::new(-self.w, $Wide::ZERO, $Wide::ZERO, self.x),
                        Self::new($Wide::ZERO, $Wide::ZERO, -self.w, self.z),
                    ),
                    self_abs.y.simd_gt(self_abs.z).select(
                        Self::new($Wide::ZERO, -self.w, $Wide::ZERO, self.y),
                        Self::new($Wide::ZERO, $Wide::ZERO, -self.w, self.z),
                    ),
                )
            }

            #[inline(always)]
            fn any_orthonormal_vector_backend(self) -> Self {
                self.any_orthogonal_vector().normalize()
            }
        }
    };
}
impl_items!(f32x4, u32x4, pow_f32x4);
impl_items!(f32x8, u32x8, pow_f32x8);
impl_items!(f32x16, u32x16, pow_f32x16);
impl_items!(f64x2, u64x2, pow_f64x2);
impl_items!(f64x4, u64x4, pow_f64x4);
impl_items!(f64x8, u64x8, pow_f64x8);

#[cfg(test)]
mod tests {
    extern crate std;

    use wide::f32x4;

    use crate::{
        Unaligned, Vec2, Vec3, Vec3A, Vec4, Vector,
        test_utils::{assert_test_eq, assert_test_eq_or_panic, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, Wide: WideFloat| {
            assert_eq!(Vector::<N, Wide, Unaligned>::MIN, Vector::splat(Wide::MIN));
            assert_eq!(Vector::<N, Wide, Unaligned>::MAX, Vector::splat(Wide::MAX));
            assert_test_eq!(Vector::<N, Wide, Unaligned>::NAN, Vector::splat(Wide::NAN));
            assert_eq!(
                Vector::<N, Wide, Unaligned>::INFINITY,
                Vector::splat(Wide::INFINITY)
            );
            assert_eq!(
                Vector::<N, Wide, Unaligned>::NEG_INFINITY,
                Vector::splat(Wide::NEG_INFINITY)
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for [x, y, z, w] in random_iter::<[f32x4; 4]>() {
            assert_test_eq!(Vec2::new(x, y).is_nan(), x.is_nan() | y.is_nan());
            assert_test_eq!(
                Vec3::new(x, y, z).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan()
            );
            assert_test_eq!(
                Vec4::new(x, y, z, w).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
            );
        }
    }

    #[test]
    fn test_nan_mask() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.nan_mask(), vector.map(f32x4::is_nan));
            }
        });
    }

    #[test]
    fn test_is_finite() {
        for [x, y, z, w] in random_iter::<[f32x4; 4]>() {
            assert_test_eq!(Vec2::new(x, y).is_finite(), x.is_finite() & y.is_finite());
            assert_test_eq!(
                Vec3::new(x, y, z).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite()
            );
            assert_test_eq!(
                Vec4::new(x, y, z, w).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
            );
        }
    }

    #[test]
    fn test_finite_mask() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.finite_mask(), vector.map(f32x4::is_finite));
            }
        });
    }

    #[test]
    fn test_sign_positive_mask() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(
                    vector.sign_positive_mask(),
                    vector.map(f32x4::is_sign_positive)
                );
            }
        });
    }

    #[test]
    fn test_sign_negative_mask() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(
                    vector.sign_negative_mask(),
                    vector.map(f32x4::is_sign_negative)
                );
            }
        });
    }

    #[test]
    fn test_recip() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(
                    vector.recip(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).recip())
                );
            }
        });
    }

    #[test]
    fn test_max() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, f32x4, Unaligned>; 2]>() {
                let a = a.nan_mask().select(Vector::ZERO, a);
                let b = b.nan_mask().select(Vector::ZERO, b);

                assert_test_eq!(a.max(b), Vector::from_fn(|i| a[i].max(b[i])), 0.0 = -0.0);
            }
        });
    }

    #[test]
    fn test_min() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, f32x4, Unaligned>; 2]>() {
                let a = a.nan_mask().select(Vector::ZERO, a);
                let b = b.nan_mask().select(Vector::ZERO, b);

                assert_test_eq!(a.min(b), Vector::from_fn(|i| a[i].min(b[i])), 0.0 = -0.0);
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_types!(|N| {
            for [vector, min, max] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                let vector = vector.nan_mask().select(Vector::ZERO, vector);
                let min = min.nan_mask().select(Vector::ZERO, min);
                let max = min.nan_mask().select(Vector::ZERO, max);
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
        for vector in random_iter::<Vec4<f32x4>>() {
            let vector = vector.nan_mask().select(Vector::ZERO, vector);

            assert_test_eq!(vector.xy().max_element(), vector.x.max(vector.y));
            assert_test_eq!(
                vector.xyz().max_element(),
                vector.x.max(vector.y).max(vector.z)
            );
            assert_test_eq!(
                vector.max_element(),
                vector.x.max(vector.y).max(vector.z).max(vector.w)
            );
        }
    }

    #[test]
    fn test_min_element() {
        for vector in random_iter::<Vec4<f32x4>>() {
            let vector = vector.nan_mask().select(Vector::ZERO, vector);

            assert_test_eq!(vector.xy().min_element(), vector.x.min(vector.y));
            assert_test_eq!(
                vector.xyz().min_element(),
                vector.x.min(vector.y).min(vector.z)
            );
            assert_test_eq!(
                vector.min_element(),
                vector.x.min(vector.y).min(vector.z).min(vector.w)
            );
        }
    }

    #[test]
    fn test_abs() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.abs(), vector.map(f32x4::abs));
            }
        });
    }

    #[test]
    fn test_signum() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.signum(), vector.map(f32x4::signum));
            }
        });
    }

    #[test]
    fn test_copysign() {
        for_types!(|N| {
            for [vector, sign] in random_iter::<[Vector<N, f32x4, Unaligned>; 2]>() {
                assert_test_eq!(
                    vector.copysign(sign),
                    Vector::from_fn(|i| vector[i].copysign(sign[i]))
                );
            }
        });
    }

    #[test]
    fn test_floor() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.floor(), vector.map(f32x4::floor));
            }
        });
    }

    #[test]
    fn test_ceil() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.ceil(), vector.map(f32x4::ceil));
            }
        });
    }

    #[test]
    fn test_round() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.round(), vector.map(f32x4::round));
            }
        });
    }

    #[test]
    fn test_trunc() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.trunc(), vector.map(f32x4::trunc));
            }
        });
    }

    #[test]
    fn test_fract() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.fract(), vector.map(f32x4::fract));
            }
        });
    }

    #[test]
    fn test_mul_add() {
        for_types!(|N| {
            for [vector, a, b] in random_iter::<[Vector<N, f32x4, Unaligned>; 3]>() {
                assert_test_eq!(
                    vector.mul_add(a, b),
                    Vector::from_fn(|i| vector[i].mul_add(a[i], b[i]))
                );
            }
        });
    }

    #[test]
    fn test_div_euclid() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, f32x4, Unaligned>; 2]>() {
                assert_test_eq!(a.div_euclid(b), Vector::from_fn(|i| a[i].div_euclid(b[i])));
            }
        });
    }

    #[test]
    fn test_rem_euclid() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, f32x4, Unaligned>; 2]>() {
                assert_test_eq!(a.rem_euclid(b), Vector::from_fn(|i| a[i].rem_euclid(b[i])));
            }
        });
    }

    #[test]
    fn test_powf() {
        for_types!(|N| {
            for (vector, n) in random_iter::<(Vector<N, f32x4, Unaligned>, f32x4)>() {
                assert_test_eq!(vector.powf(n), Vector::from_fn(|i| vector[i].pow_f32x4(n)));
            }
        });
    }

    #[test]
    fn test_sqrt() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.sqrt(), vector.map(f32x4::sqrt));
            }
        });
    }

    #[test]
    fn test_exp() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.exp(), vector.map(f32x4::exp));
            }
        });
    }

    #[test]
    fn test_exp2() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.exp2(), vector.map(f32x4::exp2));
            }
        });
    }

    #[test]
    fn test_ln() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.ln(), vector.map(f32x4::ln));
            }
        });
    }

    #[test]
    fn test_log2() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.log2(), vector.map(f32x4::log2));
            }
        });
    }

    #[test]
    fn test_sin() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.sin(), vector.map(f32x4::sin));
            }
        });
    }

    #[test]
    fn test_cos() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.cos(), vector.map(f32x4::cos));
            }
        });
    }

    #[test]
    fn test_tan() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.tan(), vector.map(f32x4::tan));
            }
        });
    }

    #[test]
    fn test_asin() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.asin(), vector.map(f32x4::asin));
            }
        });
    }

    #[test]
    fn test_acos() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.acos(), vector.map(f32x4::acos));
            }
        });
    }

    #[test]
    fn test_atan() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(vector.atan(), vector.map(f32x4::atan));
            }
        });
    }

    #[test]
    fn test_sin_cos() {
        for_types!(|N| {
            for vector in random_iter::<Vector<N, f32x4, Unaligned>>() {
                assert_test_eq!(
                    vector.sin_cos(),
                    (vector.map(|x| x.sin_cos().0), vector.map(|x| x.sin_cos().1))
                );
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|N, Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Vector<N, Wide, Unaligned>; 2], Wide)>() {
                assert_test_eq!(
                    a.lerp(b, t),
                    Vector::from_lane_fn(|lane| a
                        .lane(lane)
                        .lerp(b.lane(lane), t.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_midpoint() {
        for_types!(|N| {
            for [a, b] in random_iter::<[Vector<N, f32x4, Unaligned>; 2]>() {
                assert_test_eq!(a.midpoint(b), Vector::from_fn(|i| a[i].midpoint(b[i])));
            }
        });
    }

    #[test]
    fn test_move_towards() {
        for_types!(|N, Wide: WideFloat| {
            for ([vector, target], max_delta) in
                random_iter::<([Vector<N, Wide, Unaligned>; 2], Wide)>()
            {
                assert_test_eq!(
                    vector.move_towards(target, max_delta),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .move_towards(target.lane(lane), max_delta.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|N, Wide: WideFloat| {
            for ([a, b], t) in random_iter::<([Vector<N, Wide, Unaligned>; 2], Wide)>() {
                let condition = a.is_finite()
                    & b.is_finite()
                    & a.length().simd_lt(1e4)
                    & b.length().simd_lt(1e4);
                let [a, b] = [a, b].map(|v| condition.select(v, Vector::ZERO));
                let t = condition.select(
                    (t / 10.0).clamp(Wide::splat(-100.0), Wide::splat(100.0)),
                    Wide::ZERO,
                );

                assert_test_eq_or_panic!(
                    a.slerp(b, t),
                    Vector::from_lane_fn(|lane| a
                        .lane(lane)
                        .slerp(b.lane(lane), t.to_array()[lane])),
                    abs <= a.length().max(b.length()) * t.abs().max(Wide::ONE) * 1e-3 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|N, Wide: WideFloat| {
            for ([vector, target], max_delta) in
                random_iter::<([Vector<N, Wide, Unaligned>; 2], Wide)>()
            {
                let condition = vector.is_finite()
                    & target.is_finite()
                    & vector.length().simd_lt(1e6)
                    & target.length().simd_lt(1e6);
                let [vector, target] = [vector, target].map(|v| v & condition);

                assert_test_eq_or_panic!(
                    vector.rotate_towards(target, max_delta),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .rotate_towards(target.lane(lane), max_delta.to_array()[lane])),
                    abs <= vector.length().max(target.length()) * 1e-3 + 1e-3,
                    0.0 = -0.0,
                    "  vector: {vector:?}\n  target: {target:?}\nmax_delta: {max_delta:?}"
                );
            }
        });
    }

    #[test]
    fn test_length() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    vector.length(),
                    Wide::new(std::array::from_fn(|lane| vector.lane(lane).length()))
                );
            }
        });
    }

    #[test]
    fn test_distance() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>() {
                assert_test_eq!(
                    a.distance(b),
                    Wide::new(std::array::from_fn(|lane| a
                        .lane(lane)
                        .distance(b.lane(lane))))
                );
            }
        });
    }

    #[test]
    fn test_normalize() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>() {
                assert_test_eq_or_panic!(
                    vector.normalize(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).normalize())
                );
            }
        });
    }

    // `try_normalize` is excluded on purpose.

    #[test]
    fn test_normalize_or() {
        for_types!(|N, Wide: WideFloat| {
            for [vector, fallback] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>() {
                assert_test_eq!(
                    vector.normalize_or(fallback),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .normalize_or(fallback.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_normalize_or_zero() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    vector.normalize_or_zero(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).normalize_or_zero())
                );
            }
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    vector.normalize_and_length(),
                    (
                        Vector::from_lane_fn(|lane| vector.lane(lane).normalize_and_length().0),
                        Wide::new(std::array::from_fn(|lane| vector
                            .lane(lane)
                            .normalize_and_length()
                            .1))
                    )
                );
            }
        });
    }

    #[test]
    fn test_is_normalized() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    vector.is_normalized(),
                    Wide::new(std::array::from_fn(|lane| {
                        if vector.lane(lane).is_normalized() {
                            T::from_bits(!0)
                        } else {
                            0.0
                        }
                    }))
                );
            }
        });
    }

    #[test]
    fn test_with_max_length() {
        for_types!(|N, Wide: WideFloat| {
            for (vector, max_length) in random_iter::<(Vector<N, Wide, Unaligned>, Wide)>() {
                assert_test_eq_or_panic!(
                    vector.with_max_length(max_length),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .with_max_length(max_length.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_with_min_length() {
        for_types!(|N, Wide: WideFloat| {
            for (vector, min_length) in random_iter::<(Vector<N, Wide, Unaligned>, Wide)>() {
                assert_test_eq_or_panic!(
                    vector.with_min_length(min_length),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .with_min_length(min_length.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_clamp_length() {
        for_types!(|N, Wide: WideFloat| {
            for (vector, [min_length, max_length]) in
                random_iter::<(Vector<N, Wide, Unaligned>, [Wide; 2])>()
            {
                assert_test_eq_or_panic!(
                    vector.clamp_length(min_length, max_length),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .clamp_length(min_length.to_array()[lane], max_length.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>() {
                assert_test_eq_or_panic!(
                    a.angle_between(b),
                    Wide::new(std::array::from_fn(|lane| {
                        a.lane(lane).angle_between(b.lane(lane))
                    })),
                    abs <= a.angle_between(b) * 1e-3 + 1e-3
                );
            }
        });
    }

    #[test]
    fn test_project_onto() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>() {
                assert_test_eq_or_panic!(
                    a.project_onto(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).project_onto(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_project_onto_normalized() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>()
                .flat_map(|[a, b]| [[a, b], [a, b.normalize_or(Vector::ONE).normalize()]])
            {
                assert_test_eq_or_panic!(
                    a.project_onto_normalized(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).project_onto_normalized(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_reject_from() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>() {
                assert_test_eq_or_panic!(
                    a.reject_from(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).reject_from(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_reject_from_normalized() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>()
                .flat_map(|[a, b]| [[a, b], [a, b.normalize_or(Vector::ONE).normalize()]])
            {
                assert_test_eq_or_panic!(
                    a.reject_from_normalized(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).reject_from_normalized(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_reflect() {
        for_types!(|N, Wide: WideFloat| {
            for [a, b] in random_iter::<[Vector<N, Wide, Unaligned>; 2]>()
                .flat_map(|[a, b]| [[a, b], [a, b.normalize_or(Vector::ONE).normalize()]])
            {
                assert_test_eq_or_panic!(
                    a.reflect(b),
                    Vector::from_lane_fn(|lane| a.lane(lane).reflect(b.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_refract() {
        for_types!(|N, Wide: WideFloat| {
            for (vector, normal, eta) in random_iter::<([Vector<N, Wide, Unaligned>; 2], Wide)>()
                .flat_map(|([vector, normal], eta)| {
                    [
                        (vector, normal, eta),
                        (
                            vector.normalize_or(Vector::ONE).normalize(),
                            normal.normalize_or(Vector::ONE).normalize(),
                            eta,
                        ),
                    ]
                })
            {
                assert_test_eq_or_panic!(
                    vector.refract(normal, eta),
                    Vector::from_lane_fn(|lane| vector
                        .lane(lane)
                        .refract(normal.lane(lane), eta.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_any_orthogonal_vector() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>() {
                assert_test_eq!(
                    vector.any_orthogonal_vector(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).any_orthogonal_vector())
                );
            }
        });
    }

    #[test]
    fn test_any_orthonormal_vector() {
        for_types!(|N, Wide: WideFloat| {
            for vector in random_iter::<Vector<N, Wide, Unaligned>>()
                .flat_map(|vector| [vector, vector.normalize_or(Vector::ONE).normalize()])
            {
                assert_test_eq_or_panic!(
                    vector.any_orthonormal_vector(),
                    Vector::from_lane_fn(|lane| vector.lane(lane).any_orthonormal_vector())
                );
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|N, Wide: WideFloat| {
            for ([a, b], max_abs_diff) in random_iter::<([Vector<N, Wide, Unaligned>; 2], Wide)>() {
                assert_test_eq_or_panic!(
                    a.abs_diff_eq(b, max_abs_diff),
                    (0..LANES).all(|lane| a
                        .lane(lane)
                        .abs_diff_eq(b.lane(lane), max_abs_diff.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_to_bits() {
        for_types!(|Wide: WideFloat| {
            let vector = Vec3A::new(Wide::splat(3.1), -Wide::ZERO, Wide::splat(T::NAN));
            assert_eq!(
                vector.to_bits(),
                Vec3A::from_lane_fn(|lane| vector.lane(lane).to_bits())
            );
        });
    }

    #[test]
    fn test_from_bits() {
        for_types!(|Wide: WideFloat| {
            let vector = Vec3A::new(Wide::splat(3.1), -Wide::ZERO, Wide::splat(T::NAN));
            assert_eq!(
                Vec3A::<Wide>::from_bits(vector.to_bits()).to_bits(),
                vector.to_bits()
            );
        });
    }

    #[test]
    fn test_angle_to() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in random_iter::<[Vec2<Wide>; 2]>() {
                assert_test_eq_or_panic!(
                    a.angle_to(b),
                    Wide::new(core::array::from_fn(|lane| a
                        .lane(lane)
                        .angle_to(b.lane(lane)))),
                    abs <= a.angle_to(b).abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_angle_from() {
        for_types!(|Wide: WideFloat| {
            for [a, b] in random_iter::<[Vec2<Wide>; 2]>() {
                assert_test_eq_or_panic!(
                    a.angle_from(b),
                    Wide::new(core::array::from_fn(|lane| a
                        .lane(lane)
                        .angle_from(b.lane(lane)))),
                    abs <= a.angle_from(b).abs() * 1e-5 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_rotate() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec2<Wide>, Wide)>() {
                assert_test_eq!(
                    vector.rotate(angle),
                    Vector::from_lane_fn(|lane| vector.lane(lane).rotate(angle.to_array()[lane])),
                    abs <= (vector.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_from_homogeneous() {
        for_types!(|Wide: WideFloat| {
            for vector4 in random_iter::<Vec4<Wide>>() {
                let vector3 = vector4.xyz();
                assert_test_eq!(
                    Vec2::<Wide>::from_homogeneous(vector3),
                    Vec2::from_lane_fn(|lane| Vec2::<T>::from_homogeneous(vector3.lane(lane)))
                );
                assert_test_eq!(
                    Vec3::<Wide>::from_homogeneous(vector4),
                    Vec3::from_lane_fn(|lane| Vec3::<T>::from_homogeneous(vector4.lane(lane)))
                );
            }
        });
    }

    #[test]
    fn test_rotate_x() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                assert_test_eq!(
                    vector.rotate_x(angle),
                    Vector::from_lane_fn(|lane| vector.lane(lane).rotate_x(angle.to_array()[lane])),
                    abs <= (vector.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_rotate_y() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                assert_test_eq!(
                    vector.rotate_y(angle),
                    Vector::from_lane_fn(|lane| vector.lane(lane).rotate_y(angle.to_array()[lane])),
                    abs <= (vector.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_rotate_z() {
        for_types!(|Wide: WideFloat| {
            for (vector, angle) in random_iter::<(Vec3<Wide>, Wide)>() {
                assert_test_eq!(
                    vector.rotate_z(angle),
                    Vector::from_lane_fn(|lane| vector.lane(lane).rotate_z(angle.to_array()[lane])),
                    abs <= (vector.length() * angle.abs() * 1e-4).max(Wide::splat(1e-3)),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_any_orthonormal_pair() {
        for_types!(|Wide: WideFloat| {
            for vector in random_iter::<Vec3<Wide>>()
                .flat_map(|vector| [vector, vector.normalize_or(Vector::ONE).normalize()])
            {
                assert_test_eq_or_panic!(
                    vector.any_orthonormal_pair(),
                    (
                        Vector::from_lane_fn(|lane| vector.lane(lane).any_orthonormal_pair().0),
                        Vector::from_lane_fn(|lane| vector.lane(lane).any_orthonormal_pair().1)
                    )
                );
            }
        });
    }
}
