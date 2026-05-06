use wide::{CmpGe, CmpGt, CmpLe, CmpLt, f32x4, f32x8, f32x16, f64x2, f64x4, f64x8};

use crate::{Alignment, Length, SupportedLength, Vector, utils::transmute_generic};

macro_rules! impl_wide_float {
    ($Wide:ident, $powf:ident) => {
        impl<const N: usize, A: Alignment> Vector<N, $Wide, A>
        where
            Length<N>: SupportedLength,
        {
            /// For each lane, returns `true` if any element is NaN.
            #[inline]
            #[must_use]
            pub fn is_nan(self) -> $Wide {
                self.nan_mask().any()
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is NaN.
            ///
            /// Equivalent to `(self.x.is_nan(), self.y.is_nan(), ...)` for each
            /// lane.
            #[inline]
            #[must_use]
            pub fn nan_mask(self) -> Self {
                self.map($Wide::is_nan)
            }

            /// For each lane, returns `true` if all elements are neither
            /// infinite nor NaN.
            #[inline]
            #[must_use]
            pub fn is_finite(self) -> $Wide {
                self.finite_mask().all()
            }

            /// For each lane, returns a vector mask where each element is
            /// `true` if the corresponding element of `self` is neither
            /// infinite nor NaN.
            ///
            /// Equivalent to `(self.x.is_finite(), self.y.is_finite(), ...)`
            /// for each lane.
            #[inline]
            #[must_use]
            pub fn finite_mask(self) -> Self {
                self.map($Wide::is_finite)
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
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn max(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0) && other.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.max({other:?})"
                );

                Self::from_fn(|i| self[i].fast_max(other[i]))
            }

            /// Returns the minimum elements between `self` and `other`.
            ///
            /// Equivalent to `(self.x.min(other.x), self.y.min(other.y), ...)`.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn min(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0) && other.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.min({other:?})"
                );

                Self::from_fn(|i| self[i].fast_min(other[i]))
            }

            /// For each lane, clamps the elements of `self` between the
            /// elements of `min` and `max`.
            ///
            /// Equivalent to
            /// `(self.x.clamp(min.x, max.x), self.y.clamp(min.y, max.y), ...)`.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN, or if any element of `min` is
            /// greater than the corresponding element of `max`.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn clamp(self, min: Self, max: Self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0)
                        && min.is_nan() == $Wide::splat(0.0)
                        && max.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.clamp({min:?}, {max:?})"
                );

                #[cfg(assertions)]
                assert!(
                    min.simd_gt_mask(max).any() == $Wide::splat(0.0),
                    "min > max: {self:?}.clamp({min:?}, {max:?})"
                );

                self.max(min).min(max)
            }

            /// For each lane, returns the maximum between the elements of
            /// `self`.
            ///
            /// Equivalent to `self.x.max(self.y).max(self.z)...` for each lane.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn max_element(self) -> $Wide {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.max_element()"
                );

                match N {
                    2 => self[0].fast_max(self[1]),
                    3 => self[0].fast_max(self[1]).fast_max(self[2]),
                    4 => self[0]
                        .fast_max(self[1])
                        .fast_max(self[2])
                        .fast_max(self[3]),
                    _ => unreachable!(),
                }
            }

            /// For each lane, returns the minimum between the elements of
            /// `self`.
            ///
            /// Equivalent to `self.x.min(self.y).min(self.z)...` for each lane.
            ///
            /// This is not consistent with IEEE semantics in regards to NaN
            /// propagation and handling of `-0.0`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if any element is NaN.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn min_element(self) -> $Wide {
                #[cfg(assertions)]
                assert!(
                    self.is_nan() == $Wide::splat(0.0),
                    "NaN: {self:?}.min_element()"
                );

                match N {
                    2 => self[0].fast_min(self[1]),
                    3 => self[0].fast_min(self[1]).fast_min(self[2]),
                    4 => self[0]
                        .fast_min(self[1])
                        .fast_min(self[2])
                        .fast_min(self[3]),
                    _ => unreachable!(),
                }
            }

            /// Returns the absolute values of elements of `self`.
            ///
            /// Equivalent to `(self.x.abs(), self.y.abs(), ...)`.
            #[inline]
            #[must_use]
            pub fn abs(self) -> Self {
                self.map($Wide::abs)
            }

            /// Returns a vector with the element magnitudes of `self` and the
            /// element signs of `sign`.
            ///
            /// Equivalent to
            /// `(self.x.copysign(sign.x), self.y.copysign(sign.y), ...)`.
            #[inline]
            #[must_use]
            pub fn copysign(self, sign: Self) -> Self {
                Self::from_fn(|i| self[i].copysign(sign[i]))
            }

            /// Returns the largest integers less than or equal to the elements
            /// of `self`.
            ///
            /// This always returns the precise result.
            #[inline]
            #[must_use]
            pub fn floor(self) -> Self {
                self.map($Wide::floor)
            }

            /// Returns the smallest integers greater than or equal to the
            /// elements of `self`.
            ///
            /// This always returns the precise result.
            #[inline]
            #[must_use]
            pub fn ceil(self) -> Self {
                self.map($Wide::ceil)
            }

            /// Returns the nearest integers to the elements of `self`.
            ///
            /// This always returns the precise result.
            #[inline]
            #[must_use]
            pub fn round(self) -> Self {
                self.map($Wide::round)
            }

            /// Fused multiply-add. Computes `(self * a) + b`.
            ///
            /// When hardware FMA support is available, this computes the result
            /// with only one rounding error. Without FMA support, this falls
            /// back to separate multiply and add operations with two rounding
            /// errors.
            ///
            /// This is inconsistent with the scalar definition of `mul_add` that
            /// always computes the result with only one rounding error.
            #[inline]
            #[must_use]
            pub fn mul_add(self, a: Self, b: Self) -> Self {
                Self::from_fn(|i| self[i].mul_add(a[i], b[i]))
            }

            /// Computes `x^n` for the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn powf(self, n: $Wide) -> Self {
                self.map(|x| x.$powf(n))
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
                self.map($Wide::sqrt)
            }

            /// Computes the exponential function `e^x` for the elements of
            /// `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn exp(self) -> Self {
                self.map($Wide::exp)
            }

            /// Computes the natural logarithm for the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn ln(self) -> Self {
                self.map($Wide::ln)
            }

            /// Computes the base 2 logarithm for the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn log2(self) -> Self {
                self.map($Wide::log2)
            }

            /// Computes the sine of the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn sin(self) -> Self {
                self.map($Wide::sin)
            }

            /// Computes the cosine of the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn cos(self) -> Self {
                self.map($Wide::cos)
            }

            /// Computes the tangent of the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn tan(self) -> Self {
                self.map($Wide::tan)
            }

            /// Computes the arcsine of the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn asin(self) -> Self {
                self.map($Wide::asin)
            }

            /// Computes the arccosine of the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn acos(self) -> Self {
                self.map($Wide::acos)
            }

            /// Computes the arctangent of the elements of `self`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn atan(self) -> Self {
                self.map($Wide::atan)
            }

            /// Simultaneously computes the sine and cosine of the elements of
            /// `self`.
            ///
            /// Equivalent to `(self.sin(), self.cos())`, but may be more
            /// performant. This might return a slightly different value.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn sin_cos(self) -> (Self, Self) {
                let array = self.to_array().map($Wide::sin_cos);
                (
                    Vector::from_fn(|i| array[i].0),
                    Vector::from_fn(|i| array[i].1),
                )
            }

            /// Computes the linear interpolation between `self` and `other`
            /// based on the value `t`.
            ///
            /// When `t` is `0`, the result is `self`. When `t` is `1`, the
            /// result is `rhs`. When `t` is outside of the range `0..=1`, the
            /// result is linearly extrapolated.
            #[inline]
            #[must_use]
            pub fn lerp(self, other: Self, t: $Wide) -> Self {
                self * ($Wide::ONE - t) + other * t
            }

            /// Computes the middle point between `self` and `other`.
            ///
            /// Equivalent to `self.lerp(other, 0.5)`, but is cheaper to
            /// compute. This may return a slightly different value.
            #[inline]
            #[must_use]
            pub fn midpoint(self, other: Self) -> Self {
                (self + other) * $Wide::HALF
            }

            /// Moves `self` towards `other` by at most `max_delta`.
            ///
            /// When `max_delta` is `0`, the result is `self`. When `max_delta`
            /// is equal to or greater than `self.distance(other)`, the result
            /// is `other`.
            #[inline]
            #[must_use]
            pub fn move_towards(self, target: Self, max_delta: $Wide) -> Self {
                let delta = target - self;
                let delta_length = delta.length();

                Self::splat(
                    delta_length.simd_le(max_delta) | delta_length.simd_le($Wide::splat(1e-4)),
                )
                .blend(target, self + delta / delta_length * max_delta)
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
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if for any lane, `self` is a zero vector, or if the
            /// result is non finite or zero.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn normalize(self) -> Self {
                #[cfg(assertions)]
                assert!(
                    self.simd_ne(Self::ZERO).all(),
                    "cannot normalize a zero vector"
                );

                let result = self / self.length();

                #[cfg(assertions)]
                assert!(
                    (result.is_finite() & result.simd_ne(Self::ZERO)).all(),
                    "non finite result: {self:?}.normalize()"
                );

                result
            }

            /// Returns [`normalize`], or `None` if for any lane `self` is zero
            /// or if the result is non finite or zero.
            ///
            /// [`normalize`]: Self::normalize
            #[inline]
            #[must_use]
            pub fn try_normalize(self) -> Option<Self> {
                let length_recip = $Wide::ONE / self.length();
                if (length_recip.is_finite() & length_recip.simd_gt($Wide::ZERO)).all() {
                    Some(self * length_recip)
                } else {
                    None
                }
            }

            /// Returns [`normalize`] for each lane, or `fallback` if `self` is
            /// zero or if the result is non finite or zero.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            ///
            /// [`normalize`]: Self::normalize
            #[inline]
            #[must_use]
            pub fn normalize_or(self, fallback: Self) -> Self {
                let length_recip = $Wide::ONE / self.length();

                Self::splat(length_recip.is_finite() & length_recip.simd_gt($Wide::ZERO))
                    .blend(self * length_recip, fallback)
            }

            /// Returns [`normalize`] for each lane, or a zero vector if `self`
            /// is zero or if the result is non finite.
            ///
            /// The fallback is only applied for invalid lanes. Other lanes are
            /// not affected.
            ///
            /// [`normalize`]: Self::normalize
            #[inline]
            #[must_use]
            pub fn normalize_or_zero(self) -> Self {
                let length_recip = $Wide::ONE / self.length();

                Self::splat(length_recip.is_finite() & length_recip.simd_gt($Wide::ZERO))
                    .blend(self * length_recip, Self::ZERO)
            }

            /// Simultaneously computes [`normalize`] and [`length`].
            ///
            /// If `self` is a zero vector for a given lane, the result is
            /// `(Self::ZERO, 0)` for that lane.
            ///
            /// [`normalize`]: Self::normalize
            /// [`length`]: Self::length
            #[inline]
            #[must_use]
            pub fn normalize_and_length(self) -> (Self, $Wide) {
                let length = self.length();
                let length_recip = $Wide::ONE / length;

                let valid_mask = length_recip.is_finite() & length_recip.simd_gt($Wide::ZERO);
                (
                    Self::splat(valid_mask).blend(self * length_recip, Self::ZERO),
                    valid_mask.blend(length, $Wide::ZERO),
                )
            }

            /// For each lane, returns whether the vector has the length `1` or
            /// not.
            ///
            /// This uses a precision threshold of approximately `1e-4`.
            #[inline]
            #[must_use]
            pub fn is_normalized(self) -> $Wide {
                (self.length_squared() - $Wide::ONE)
                    .abs()
                    .simd_le($Wide::splat(2e-4))
            }

            /// For each lane, returns `self` with a length of no more than
            /// `max`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `max` is negative for any lane.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn with_max_length(self, max: $Wide) -> Self {
                #[cfg(assertions)]
                assert!(max.simd_ge($Wide::ZERO).all(), "negative maximum length");

                let length_squared = self.length_squared();
                Self::splat(length_squared.simd_gt(max * max))
                    .blend(self / length_squared.sqrt() * max, self)
            }

            /// For any lane, returns `self` with a length of no less than
            /// `min`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `min` is negative for any lane.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn with_min_length(self, min: $Wide) -> Self {
                #[cfg(assertions)]
                assert!(min.simd_ge($Wide::ZERO).all(), "negative minimum length");

                let length_squared = self.length_squared();
                Self::splat(length_squared.simd_lt(min * min))
                    .blend(self / length_squared.sqrt() * min, self)
            }

            /// For each lane, returns `self` with a length of no less than
            /// `min` and no more than `max`.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `min > max`, or if `min` is negative for any lane.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn clamp_length(self, min: $Wide, max: $Wide) -> Self {
                #[cfg(assertions)]
                assert!(min.simd_ge($Wide::ZERO).all(), "negative minimum length");

                #[cfg(assertions)]
                assert!(
                    min.simd_le(max).all(),
                    "minimum length is greater than maximum length"
                );

                let length_squared = self.length_squared();
                Self::splat(length_squared.simd_lt(min * min)).blend(
                    self / length_squared.sqrt() * min,
                    Self::splat(length_squared.simd_gt(max * max))
                        .blend(self / length_squared.sqrt() * max, self),
                )
            }

            /// For each lane, returns the angle (in radians) between `self` and
            /// `other` in the range `0..=+π`.
            ///
            /// The vectors do not need to be unit vectors but they do need to
            /// be non-zero.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn angle_between(self, other: Self) -> $Wide {
                (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt()).acos()
            }

            /// Returns the vector projection of `self` onto `other`.
            ///
            /// `other` must not be a zero vector.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if `other` is a zero vector for any lane.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn project_onto(self, other: Self) -> Self {
                let other_length_squared_recip = $Wide::ONE / other.length_squared();

                #[cfg(assertions)]
                assert!(other_length_squared_recip.is_finite().all());

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
            /// Panics if for any lane `other` is not normalized.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn project_onto_normalized(self, other: Self) -> Self {
                #[cfg(assertions)]
                assert!(other.is_normalized().all());

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
            /// Panics if `other` is a zero vector for any lane.
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
            /// Panics if for any lane `other` is not normalized.
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
            /// Panics if for any lane `normal` is not normalized.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn reflect(self, normal: Self) -> Self {
                #[cfg(assertions)]
                assert!(normal.is_normalized().all());

                self - normal * ($Wide::splat(2.0) * self.dot(normal))
            }

            /// Returns the vector refraction of `self` through `normal` and
            /// `eta`.
            ///
            /// `eta` is the incident refraction-index divided by the
            /// transmitted refraction-index.
            ///
            /// When total internal reflection occurs, the result is a zero
            /// vector.
            ///
            /// `self` and `normal` must be normalized.
            ///
            /// # Panics
            ///
            /// When assertions are enabled (see the crate documentation):
            ///
            /// Panics if for any lane `self` or `normal` are not normalized.
            #[inline]
            #[must_use]
            #[track_caller]
            pub fn refract(self, normal: Self, eta: $Wide) -> Self {
                #[cfg(assertions)]
                assert!(self.is_normalized().all());

                #[cfg(assertions)]
                assert!(normal.is_normalized().all());

                let self_dot_normal = self.dot(normal);
                let k = $Wide::ONE - eta * eta * ($Wide::ONE - self_dot_normal * self_dot_normal);

                Self::splat(k.simd_ge($Wide::ZERO)).blend(
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
                match N {
                    2 => {
                        // SAFETY: Because `N = 2`, `Vector<N, $Wide, A> = Vector<2, $Wide, A>`.
                        let self_ = unsafe {
                            transmute_generic::<Vector<N, $Wide, A>, Vector<2, $Wide, A>>(self)
                        };

                        let result = self_.perp();

                        // SAFETY: Because `N = 2`, `Vector<N, $Wide, A> = Vector<2, $Wide, A>`.
                        unsafe {
                            transmute_generic::<Vector<2, $Wide, A>, Vector<N, $Wide, A>>(result)
                        }
                    }
                    3 => {
                        // SAFETY: Because `N = 3`, `Vector<N, $Wide, A> = Vector<3, $Wide, A>`.
                        let self_ = unsafe {
                            transmute_generic::<Vector<N, $Wide, A>, Vector<3, $Wide, A>>(self)
                        };

                        let result =
                            Vector::<3, $Wide, A>::splat(self_.x.abs().simd_gt(self_.y.abs()))
                                .blend(
                                    Vector::<3, $Wide, A>::new(-self_.z, $Wide::ZERO, self_.x),
                                    Vector::<3, $Wide, A>::new($Wide::ZERO, self_.z, -self_.y),
                                );

                        // SAFETY: Because `N = 3`, `Vector<N, $Wide, A> = Vector<3, $Wide, A>`.
                        unsafe {
                            transmute_generic::<Vector<3, $Wide, A>, Vector<N, $Wide, A>>(result)
                        }
                    }
                    4 => {
                        // SAFETY: Because `N = 4`, `Vector<N, $Wide, A> = Vector<4, $Wide, A>`.
                        let self_ = unsafe {
                            transmute_generic::<Vector<N, $Wide, A>, Vector<4, $Wide, A>>(self)
                        };

                        let self_abs = self_.abs();
                        let result = Vector::<4, $Wide, A>::splat(self_abs.x.simd_gt(self_abs.y))
                            .blend(
                                Vector::<4, $Wide, A>::splat(self_abs.x.simd_gt(self_abs.z)).blend(
                                    Vector::<4, $Wide, A>::new(
                                        -self_.w,
                                        $Wide::ZERO,
                                        $Wide::ZERO,
                                        self_.x,
                                    ),
                                    Vector::<4, $Wide, A>::new(
                                        $Wide::ZERO,
                                        $Wide::ZERO,
                                        -self_.w,
                                        self_.z,
                                    ),
                                ),
                                Vector::<4, $Wide, A>::splat(self_abs.y.simd_gt(self_abs.z)).blend(
                                    Vector::<4, $Wide, A>::new(
                                        $Wide::ZERO,
                                        -self_.w,
                                        $Wide::ZERO,
                                        self_.y,
                                    ),
                                    Vector::<4, $Wide, A>::new(
                                        $Wide::ZERO,
                                        $Wide::ZERO,
                                        -self_.w,
                                        self_.z,
                                    ),
                                ),
                            );

                        // SAFETY: Because `N = 4`, `Vector<N, $Wide, A> = Vector<4, $Wide, A>`.
                        unsafe {
                            transmute_generic::<Vector<4, $Wide, A>, Vector<N, $Wide, A>>(result)
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }

        impl<A: Alignment> Vector<2, $Wide, A> {
            /// For each lane, rotates `self` by `angle` (in radians).
            ///
            /// This rotates `+X` to `+Y`.
            ///
            /// # Unspecified precision
            ///
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
            #[inline]
            #[must_use]
            pub fn rotate(self, angle: $Wide) -> Self {
                let (angle_sin, angle_cos) = angle.sin_cos();
                Self::new(
                    self.x * angle_cos - self.y * angle_sin,
                    self.x * angle_sin + self.y * angle_cos,
                )
            }
        }

        impl<A: Alignment> Vector<3, $Wide, A> {
            /// For each lane, creates a 3D vector from homogeneous coordinates
            /// by performing perspective divide.
            ///
            /// Equivalent to `homogeneous.xyz / homogeneous.w`.
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
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
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
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
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
            /// The precision of this function is non-deterministic. This means
            /// it varies by platform, version, and can even differ within the
            /// same execution from one invocation to the next.
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
        }

        // MISSING: sign_positive_mask
        // MISSING: sign_negative_mask
        // MISSING: signum
        // MISSING: trunc
        // MISSING: fract
        // MISSING: div_euclid
        // MISSING: rem_euclid
        // MISSING: exp2
        // MISSING: abs_diff_eq
        // MISSING: angle_to
        // MISSING: angle_from
        // MISSING: any_orthonormal_vector
        // MISSING: any_orthonormal_pair
    };
}
impl_wide_float!(f32x4, pow_f32x4);
impl_wide_float!(f32x8, pow_f32x8);
impl_wide_float!(f32x16, pow_f32x16);
impl_wide_float!(f64x2, pow_f64x2);
impl_wide_float!(f64x4, pow_f64x4);
impl_wide_float!(f64x8, pow_f64x8);

#[cfg(test)]
mod tests {
    use wide::{CmpGt, CmpLt};

    use crate::{
        Vector,
        utils::{assert_assertions_panic, assert_float_eq, assert_panic_float_eq, for_parameters},
    };

    #[test]
    fn test_is_nan() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).is_nan(),
                x.is_nan() | y.is_nan()
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan()
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).is_nan(),
                x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan()
            );
        });
    }

    #[test]
    fn test_nan_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).nan_mask(),
                Vector::<2, Wide, A>::new(x.is_nan(), y.is_nan())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).nan_mask(),
                Vector::<3, Wide, A>::new(x.is_nan(), y.is_nan(), z.is_nan())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).nan_mask(),
                Vector::<4, Wide, A>::new(x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan())
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).is_finite(),
                x.is_finite() & y.is_finite()
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite()
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).is_finite(),
                x.is_finite() & y.is_finite() & z.is_finite() & w.is_finite()
            );
        });
    }

    #[test]
    fn test_finite_mask() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).finite_mask(),
                Vector::<2, Wide, A>::new(x.is_finite(), y.is_finite())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).finite_mask(),
                Vector::<3, Wide, A>::new(x.is_finite(), y.is_finite(), z.is_finite())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).finite_mask(),
                Vector::<4, Wide, A>::new(
                    x.is_finite(),
                    y.is_finite(),
                    z.is_finite(),
                    w.is_finite()
                )
            );
        });
    }

    #[test]
    fn test_recip() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).recip(),
                Vector::<2, Wide, A>::new(1.0 / x, 1.0 / y)
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).recip(),
                Vector::<3, Wide, A>::new(1.0 / x, 1.0 / y, 1.0 / z)
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).recip(),
                Vector::<4, Wide, A>::new(1.0 / x, 1.0 / y, 1.0 / z, 1.0 / w)
            );
        });
    }

    #[test]
    fn test_max() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            if x.is_nan() == Wide::splat(0.0)
                && y.is_nan() == Wide::splat(0.0)
                && z.is_nan() == Wide::splat(0.0)
                && w.is_nan() == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Wide, A>::new(x, y).max(Vector::<2, Wide, A>::new(z, w)),
                    Vector::<2, Wide, A>::new(x.max(z), y.max(w))
                );
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, a).max(Vector::<3, Wide, A>::new(z, w, c)),
                    Vector::<3, Wide, A>::new(x.max(z), y.max(w), a.max(c))
                );
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, a, b)
                        .max(Vector::<4, Wide, A>::new(z, w, c, d)),
                    Vector::<4, Wide, A>::new(x.max(z), y.max(w), a.max(c), b.max(d))
                );
            } else {
                assert_assertions_panic!(
                    Vector::<2, Wide, A>::new(x, y).max(Vector::<2, Wide, A>::new(z, w))
                );
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, a).max(Vector::<
                    3,
                    Wide,
                    A,
                >::new(
                    z, w, c
                )));
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, a, b).max(Vector::<
                    4,
                    Wide,
                    A,
                >::new(
                    z, w, c, d
                )));
            }
        });
    }

    #[test]
    fn test_min() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = w.simd_gt(z).blend(x, y);
            let b = z.simd_gt(y).blend(y, z);
            let c = y.simd_gt(x).blend(z, w);
            let d = w.simd_gt(x).blend(w, x);

            if x.is_nan() == Wide::splat(0.0)
                && y.is_nan() == Wide::splat(0.0)
                && z.is_nan() == Wide::splat(0.0)
                && w.is_nan() == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Wide, A>::new(x, y).min(Vector::<2, Wide, A>::new(z, w)),
                    Vector::<2, Wide, A>::new(x.min(z), y.min(w))
                );
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, a).min(Vector::<3, Wide, A>::new(z, w, c)),
                    Vector::<3, Wide, A>::new(x.min(z), y.min(w), a.min(c))
                );
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, a, b)
                        .min(Vector::<4, Wide, A>::new(z, w, c, d)),
                    Vector::<4, Wide, A>::new(x.min(z), y.min(w), a.min(c), b.min(d))
                );
            } else {
                assert_assertions_panic!(
                    Vector::<2, Wide, A>::new(x, y).min(Vector::<2, Wide, A>::new(z, w))
                );
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, a).min(Vector::<
                    3,
                    Wide,
                    A,
                >::new(
                    z, w, c
                )));
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, a, b).min(Vector::<
                    4,
                    Wide,
                    A,
                >::new(
                    z, w, c, d
                )));
            }
        });
    }

    #[test]
    fn test_clamp() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let [w, a, b] = [x ^ y, x ^ z, y ^ z];
            let [c, d, e] = [x + y, x + z, y + z];
            let [f, g, h] = [c + d, c + e, d + e];

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (z.simd_gt(a) | w.simd_gt(b))
                == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<2, Wide, A>::new(x, y).clamp(
                        Vector::<2, Wide, A>::new(z, w),
                        Vector::<2, Wide, A>::new(a, b)
                    ),
                    Vector::<2, Wide, A>::new(x.max(z).min(a), y.max(w).min(b))
                );
            } else {
                assert_assertions_panic!(Vector::<2, Wide, A>::new(x, y).clamp(
                    Vector::<2, Wide, A>::new(z, w),
                    Vector::<2, Wide, A>::new(a, b)
                ));
            }

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (c.is_nan() | d.is_nan() | e.is_nan() | w.simd_gt(c) | a.simd_gt(d))
                | b.simd_gt(e)
                == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, z).clamp(
                        Vector::<3, Wide, A>::new(w, a, b),
                        Vector::<3, Wide, A>::new(c, d, e)
                    ),
                    Vector::<3, Wide, A>::new(x.max(w).min(c), y.max(a).min(d), z.max(b).min(e))
                );
            } else {
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, z).clamp(
                    Vector::<3, Wide, A>::new(w, a, b),
                    Vector::<3, Wide, A>::new(c, d, e)
                ));
            }

            if (x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() | a.is_nan() | b.is_nan())
                | (c.is_nan() | d.is_nan() | e.is_nan() | f.is_nan() | g.is_nan() | h.is_nan())
                | (a.simd_gt(e) | b.simd_gt(f) | c.simd_gt(g) | d.simd_gt(h))
                == Wide::splat(0.0)
            {
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, z, w).clamp(
                        Vector::<4, Wide, A>::new(a, b, c, d),
                        Vector::<4, Wide, A>::new(e, f, g, h)
                    ),
                    Vector::<4, Wide, A>::new(
                        x.max(a).min(e),
                        y.max(b).min(f),
                        z.max(c).min(g),
                        w.max(d).min(h)
                    )
                );
            } else {
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, z, w).clamp(
                    Vector::<4, Wide, A>::new(a, b, c, d),
                    Vector::<4, Wide, A>::new(e, f, g, h)
                ));
            }
        });
    }

    #[test]
    fn test_max_element() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            if x.is_nan() | y.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(Vector::<2, Wide, A>::new(x, y).max_element(), x.max(y));
            } else {
                assert_assertions_panic!(Vector::<2, Wide, A>::new(x, y).max_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, z).max_element(),
                    x.max(y).max(z)
                );
            } else {
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, z).max_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, z, w).max_element(),
                    x.max(y).max(z).max(w)
                );
            } else {
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, z, w).max_element());
            }
        });
    }

    #[test]
    fn test_min_element() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            if x.is_nan() | y.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(Vector::<2, Wide, A>::new(x, y).min_element(), x.min(y));
            } else {
                assert_assertions_panic!(Vector::<2, Wide, A>::new(x, y).min_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<3, Wide, A>::new(x, y, z).min_element(),
                    x.min(y).min(z)
                );
            } else {
                assert_assertions_panic!(Vector::<3, Wide, A>::new(x, y, z).min_element());
            }

            if x.is_nan() | y.is_nan() | z.is_nan() | w.is_nan() == Wide::splat(0.0) {
                assert_float_eq!(
                    Vector::<4, Wide, A>::new(x, y, z, w).min_element(),
                    x.min(y).min(z).min(w)
                );
            } else {
                assert_assertions_panic!(Vector::<4, Wide, A>::new(x, y, z, w).min_element());
            }
        });
    }

    #[test]
    fn test_abs() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).abs(),
                Vector::<2, Wide, A>::new(x.abs(), y.abs())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).abs(),
                Vector::<3, Wide, A>::new(x.abs(), y.abs(), z.abs())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).abs(),
                Vector::<4, Wide, A>::new(x.abs(), y.abs(), z.abs(), w.abs())
            );
        });
    }

    #[test]
    fn test_copysign() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).copysign(Vector::<2, Wide, A>::new(z, w)),
                Vector::<2, Wide, A>::new(x.copysign(z), y.copysign(w))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).copysign(Vector::<3, Wide, A>::new(z, w, x)),
                Vector::<3, Wide, A>::new(x.copysign(z), y.copysign(w), z.copysign(x))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .copysign(Vector::<4, Wide, A>::new(z, w, x, y)),
                Vector::<4, Wide, A>::new(
                    x.copysign(z),
                    y.copysign(w),
                    z.copysign(x),
                    w.copysign(y)
                )
            );
        });
    }

    #[test]
    fn test_floor() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).floor(),
                Vector::<2, Wide, A>::new(x.floor(), y.floor())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).floor(),
                Vector::<3, Wide, A>::new(x.floor(), y.floor(), z.floor())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).floor(),
                Vector::<4, Wide, A>::new(x.floor(), y.floor(), z.floor(), w.floor())
            );
        });
    }

    #[test]
    fn test_ceil() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).ceil(),
                Vector::<2, Wide, A>::new(x.ceil(), y.ceil())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).ceil(),
                Vector::<3, Wide, A>::new(x.ceil(), y.ceil(), z.ceil())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).ceil(),
                Vector::<4, Wide, A>::new(x.ceil(), y.ceil(), z.ceil(), w.ceil())
            );
        });
    }

    #[test]
    fn test_round() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).round(),
                Vector::<2, Wide, A>::new(x.round(), y.round())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).round(),
                Vector::<3, Wide, A>::new(x.round(), y.round(), z.round())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).round(),
                Vector::<4, Wide, A>::new(x.round(), y.round(), z.round(), w.round())
            );
        });
    }

    #[test]
    fn test_mul_add() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).mul_add(
                    Vector::<2, Wide, A>::new(z, w),
                    Vector::<2, Wide, A>::new(y, z)
                ),
                Vector::<2, Wide, A>::new(x.mul_add(z, y), y.mul_add(w, z))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).mul_add(
                    Vector::<3, Wide, A>::new(z, w, y),
                    Vector::<3, Wide, A>::new(y, z, w)
                ),
                Vector::<3, Wide, A>::new(x.mul_add(z, y), y.mul_add(w, z), z.mul_add(y, w))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).mul_add(
                    Vector::<4, Wide, A>::new(z, w, y, x),
                    Vector::<4, Wide, A>::new(y, z, w, y)
                ),
                Vector::<4, Wide, A>::new(
                    x.mul_add(z, y),
                    y.mul_add(w, z),
                    z.mul_add(y, w),
                    w.mul_add(x, y)
                )
            );
        });
    }

    #[test]
    fn test_powf() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let n = w.to_array()[0];

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).powf(Wide::splat(n)),
                Vector::<2, Wide, A>::new(x.powf(n), y.powf(n))
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).powf(Wide::splat(n)),
                Vector::<3, Wide, A>::new(x.powf(n), y.powf(n), z.powf(n))
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).powf(Wide::splat(n)),
                Vector::<4, Wide, A>::new(x.powf(n), y.powf(n), z.powf(n), w.powf(n))
            );
        });
    }

    #[test]
    fn test_sqrt() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).sqrt(),
                Vector::<2, Wide, A>::new(x.sqrt(), y.sqrt())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).sqrt(),
                Vector::<3, Wide, A>::new(x.sqrt(), y.sqrt(), z.sqrt())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).sqrt(),
                Vector::<4, Wide, A>::new(x.sqrt(), y.sqrt(), z.sqrt(), w.sqrt())
            );
        });
    }

    #[test]
    fn test_exp() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).exp(),
                Vector::<2, Wide, A>::new(x.exp(), y.exp())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).exp(),
                Vector::<3, Wide, A>::new(x.exp(), y.exp(), z.exp())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).exp(),
                Vector::<4, Wide, A>::new(x.exp(), y.exp(), z.exp(), w.exp())
            );
        });
    }

    #[test]
    fn test_ln() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).ln(),
                Vector::<2, Wide, A>::new(x.ln(), y.ln())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).ln(),
                Vector::<3, Wide, A>::new(x.ln(), y.ln(), z.ln())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).ln(),
                Vector::<4, Wide, A>::new(x.ln(), y.ln(), z.ln(), w.ln())
            );
        });
    }

    #[test]
    fn test_log2() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).log2(),
                Vector::<2, Wide, A>::new(x.log2(), y.log2())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).log2(),
                Vector::<3, Wide, A>::new(x.log2(), y.log2(), z.log2())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).log2(),
                Vector::<4, Wide, A>::new(x.log2(), y.log2(), z.log2(), w.log2())
            );
        });
    }

    #[test]
    fn test_sin() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).sin(),
                Vector::<2, Wide, A>::new(x.sin(), y.sin())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).sin(),
                Vector::<3, Wide, A>::new(x.sin(), y.sin(), z.sin())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).sin(),
                Vector::<4, Wide, A>::new(x.sin(), y.sin(), z.sin(), w.sin())
            );
        });
    }

    #[test]
    fn test_cos() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).cos(),
                Vector::<2, Wide, A>::new(x.cos(), y.cos())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).cos(),
                Vector::<3, Wide, A>::new(x.cos(), y.cos(), z.cos())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).cos(),
                Vector::<4, Wide, A>::new(x.cos(), y.cos(), z.cos(), w.cos())
            );
        });
    }

    #[test]
    fn test_tan() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).tan(),
                Vector::<2, Wide, A>::new(x.tan(), y.tan())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).tan(),
                Vector::<3, Wide, A>::new(x.tan(), y.tan(), z.tan())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).tan(),
                Vector::<4, Wide, A>::new(x.tan(), y.tan(), z.tan(), w.tan())
            );
        });
    }

    #[test]
    fn test_asin() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).asin(),
                Vector::<2, Wide, A>::new(x.asin(), y.asin())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).asin(),
                Vector::<3, Wide, A>::new(x.asin(), y.asin(), z.asin())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).asin(),
                Vector::<4, Wide, A>::new(x.asin(), y.asin(), z.asin(), w.asin())
            );
        });
    }

    #[test]
    fn test_acos() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).acos(),
                Vector::<2, Wide, A>::new(x.acos(), y.acos())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).acos(),
                Vector::<3, Wide, A>::new(x.acos(), y.acos(), z.acos())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).acos(),
                Vector::<4, Wide, A>::new(x.acos(), y.acos(), z.acos(), w.acos())
            );
        });
    }

    #[test]
    fn test_atan() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).atan(),
                Vector::<2, Wide, A>::new(x.atan(), y.atan())
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).atan(),
                Vector::<3, Wide, A>::new(x.atan(), y.atan(), z.atan())
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).atan(),
                Vector::<4, Wide, A>::new(x.atan(), y.atan(), z.atan(), w.atan())
            );
        });
    }

    #[test]
    fn test_sin_cos() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).sin_cos(),
                (
                    Vector::<2, Wide, A>::new(x.sin_cos().0, y.sin_cos().0),
                    Vector::<2, Wide, A>::new(x.sin_cos().1, y.sin_cos().1)
                )
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).sin_cos(),
                (
                    Vector::<3, Wide, A>::new(x.sin_cos().0, y.sin_cos().0, z.sin_cos().0),
                    Vector::<3, Wide, A>::new(x.sin_cos().1, y.sin_cos().1, z.sin_cos().1)
                )
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).sin_cos(),
                (
                    Vector::<4, Wide, A>::new(
                        x.sin_cos().0,
                        y.sin_cos().0,
                        z.sin_cos().0,
                        w.sin_cos().0
                    ),
                    Vector::<4, Wide, A>::new(
                        x.sin_cos().1,
                        y.sin_cos().1,
                        z.sin_cos().1,
                        w.sin_cos().1
                    )
                )
            );
        });
    }

    #[test]
    fn test_lerp() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let t = y ^ z;

            let a = Vector::<2, Wide, A>::new(x, y);
            let b = Vector::<2, Wide, A>::new(z, w);
            assert_float_eq!(
                a.lerp(b, t),
                Vector::<2, Wide, A>::from_lane_fn(|lane| a
                    .lane(lane)
                    .lerp(b.lane(lane), t.to_array()[lane]))
            );

            let a = Vector::<3, Wide, A>::new(x, y, z);
            let b = Vector::<3, Wide, A>::new(z, w, y);
            assert_float_eq!(
                a.lerp(b, t),
                Vector::<3, Wide, A>::from_lane_fn(|lane| a
                    .lane(lane)
                    .lerp(b.lane(lane), t.to_array()[lane]))
            );

            let a = Vector::<4, Wide, A>::new(x, y, z, w);
            let b = Vector::<4, Wide, A>::new(z, w, y, x);
            assert_float_eq!(
                a.lerp(b, t),
                Vector::<4, Wide, A>::from_lane_fn(|lane| a
                    .lane(lane)
                    .lerp(b.lane(lane), t.to_array()[lane]))
            );
        });
    }

    #[test]
    fn test_midpoint() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).midpoint(Vector::<2, Wide, A>::new(z, w)),
                (Vector::<2, Wide, A>::new(x, y) + Vector::<2, Wide, A>::new(z, w)) * Wide::HALF,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).midpoint(Vector::<3, Wide, A>::new(z, w, y)),
                (Vector::<3, Wide, A>::new(x, y, z) + Vector::<3, Wide, A>::new(z, w, y))
                    * Wide::HALF,
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .midpoint(Vector::<4, Wide, A>::new(z, w, y, x)),
                (Vector::<4, Wide, A>::new(x, y, z, w) + Vector::<4, Wide, A>::new(z, w, y, x))
                    * Wide::HALF,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_move_towards() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let max_delta = y ^ z;

            let vector = Vector::<2, Wide, A>::new(x, y);
            let target = Vector::<2, Wide, A>::new(z, w);
            assert_float_eq!(
                vector.move_towards(target, max_delta),
                Vector::<2, Wide, A>::from_lane_fn(|lane| vector
                    .lane(lane)
                    .move_towards(target.lane(lane), max_delta.to_array()[lane]))
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            let target = Vector::<3, Wide, A>::new(z, w, y);
            assert_float_eq!(
                vector.move_towards(target, max_delta),
                Vector::<3, Wide, A>::from_lane_fn(|lane| vector
                    .lane(lane)
                    .move_towards(target.lane(lane), max_delta.to_array()[lane]))
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            let target = Vector::<4, Wide, A>::new(z, w, y, x);
            assert_float_eq!(
                vector.move_towards(target, max_delta),
                Vector::<4, Wide, A>::from_lane_fn(|lane| vector
                    .lane(lane)
                    .move_towards(target.lane(lane), max_delta.to_array()[lane])),
                r2nd <= Vector::splat(Wide::splat(1e-4))
            );
        });
    }

    #[test]
    fn test_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).length(),
                (x * x + y * y).sqrt()
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).length(),
                (x * x + y * y + z * z).sqrt(),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w).length(),
                (x * x + y * y + (z * z + w * w)).sqrt()
            );
        });
    }

    #[test]
    fn test_distance() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            assert_float_eq!(
                Vector::<2, Wide, A>::new(x, y).distance(Vector::<2, Wide, A>::new(z, w)),
                ((x - z) * (x - z) + (y - w) * (y - w)).sqrt()
            );
            assert_float_eq!(
                Vector::<3, Wide, A>::new(x, y, z).distance(Vector::<3, Wide, A>::new(z, w, y)),
                ((x - z) * (x - z) + (y - w) * (y - w) + (z - y) * (z - y)).sqrt(),
                0.0 = -0.0
            );
            assert_float_eq!(
                Vector::<4, Wide, A>::new(x, y, z, w)
                    .distance(Vector::<4, Wide, A>::new(z, w, y, z)),
                ((x - z) * (x - z) + (y - w) * (y - w) + ((z - y) * (z - y) + (w - z) * (w - z)))
                    .sqrt()
            );
        });
    }

    #[test]
    fn test_normalize() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_panic_float_eq!(
                vector.normalize(),
                Vector::<2, Wide, A>::from_lane_fn(|lane| vector.lane(lane).normalize())
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_panic_float_eq!(
                vector.normalize(),
                Vector::<3, Wide, A>::from_lane_fn(|lane| vector.lane(lane).normalize())
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_panic_float_eq!(
                vector.normalize(),
                Vector::<4, Wide, A>::from_lane_fn(|lane| vector.lane(lane).normalize())
            );
        });
    }

    #[test]
    fn test_try_normalize() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            if let Some(try_normalize) = vector.try_normalize() {
                assert_float_eq!(
                    try_normalize,
                    Vector::<2, Wide, A>::from_lane_fn(|lane| vector
                        .lane(lane)
                        .try_normalize()
                        .unwrap())
                );
            } else {
                assert!((0..LANES).any(|lane| vector.lane(lane).try_normalize().is_none()));
            }

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            if let Some(try_normalize) = vector.try_normalize() {
                assert_float_eq!(
                    try_normalize,
                    Vector::<3, Wide, A>::from_lane_fn(|lane| vector
                        .lane(lane)
                        .try_normalize()
                        .unwrap())
                );
            } else {
                assert!((0..LANES).any(|lane| vector.lane(lane).try_normalize().is_none()));
            }

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            if let Some(try_normalize) = vector.try_normalize() {
                assert_float_eq!(
                    try_normalize,
                    Vector::<4, Wide, A>::from_lane_fn(|lane| vector
                        .lane(lane)
                        .try_normalize()
                        .unwrap())
                );
            } else {
                assert!((0..LANES).any(|lane| vector.lane(lane).try_normalize().is_none()));
            }
        });
    }

    #[test]
    fn test_normalize_or() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_float_eq!(
                vector.normalize_or(Vector::NAN),
                Vector::<2, Wide, A>::from_lane_fn(|lane| vector
                    .lane(lane)
                    .normalize_or(Vector::NAN))
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.normalize_or(Vector::NAN),
                Vector::<3, Wide, A>::from_lane_fn(|lane| vector
                    .lane(lane)
                    .normalize_or(Vector::NAN))
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_float_eq!(
                vector.normalize_or(Vector::NAN),
                Vector::<4, Wide, A>::from_lane_fn(|lane| vector
                    .lane(lane)
                    .normalize_or(Vector::NAN))
            );
        });
    }

    #[test]
    fn test_normalize_or_zero() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_float_eq!(
                vector.normalize_or_zero(),
                Vector::<2, Wide, A>::from_lane_fn(|lane| vector.lane(lane).normalize_or_zero())
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.normalize_or_zero(),
                Vector::<3, Wide, A>::from_lane_fn(|lane| vector.lane(lane).normalize_or_zero())
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_float_eq!(
                vector.normalize_or_zero(),
                Vector::<4, Wide, A>::from_lane_fn(|lane| vector.lane(lane).normalize_or_zero())
            );
        });
    }

    #[test]
    fn test_normalize_and_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_float_eq!(
                vector.normalize_and_length(),
                (
                    Vector::<2, Wide, A>::from_lane_fn(|lane| vector
                        .lane(lane)
                        .normalize_and_length()
                        .0),
                    Wide::new(core::array::from_fn(|lane| vector
                        .lane(lane)
                        .normalize_and_length()
                        .1))
                )
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.normalize_and_length(),
                (
                    Vector::<3, Wide, A>::from_lane_fn(|lane| vector
                        .lane(lane)
                        .normalize_and_length()
                        .0),
                    Wide::new(core::array::from_fn(|lane| vector
                        .lane(lane)
                        .normalize_and_length()
                        .1))
                )
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_float_eq!(
                vector.normalize_and_length(),
                (
                    Vector::<4, Wide, A>::from_lane_fn(|lane| vector
                        .lane(lane)
                        .normalize_and_length()
                        .0),
                    Wide::new(core::array::from_fn(|lane| vector
                        .lane(lane)
                        .normalize_and_length()
                        .1))
                )
            );
        });
    }

    #[test]
    fn test_is_normalized() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_float_eq!(
                vector.is_normalized(),
                Wide::new(core::array::from_fn(|lane| {
                    if vector.lane(lane).is_normalized() {
                        T::from_bits(!0)
                    } else {
                        0.0
                    }
                }))
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.is_normalized(),
                Wide::new(core::array::from_fn(|lane| {
                    if vector.lane(lane).is_normalized() {
                        T::from_bits(!0)
                    } else {
                        0.0
                    }
                }))
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_float_eq!(
                vector.is_normalized(),
                Wide::new(core::array::from_fn(|lane| {
                    if vector.lane(lane).is_normalized() {
                        T::from_bits(!0)
                    } else {
                        0.0
                    }
                }))
            );
        });
    }

    #[test]
    fn test_with_max_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_panic_float_eq!(
                vector.with_max_length(a),
                Vector::from_lane_fn(|lane| vector.lane(lane).with_max_length(a.to_array()[lane]))
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_panic_float_eq!(
                vector.with_max_length(a),
                Vector::from_lane_fn(|lane| vector.lane(lane).with_max_length(a.to_array()[lane]))
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_panic_float_eq!(
                vector.with_max_length(a),
                Vector::from_lane_fn(|lane| vector.lane(lane).with_max_length(a.to_array()[lane]))
            );
        });
    }

    #[test]
    fn test_with_min_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_panic_float_eq!(
                vector.with_min_length(a),
                Vector::from_lane_fn(|lane| vector.lane(lane).with_min_length(a.to_array()[lane]))
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_panic_float_eq!(
                vector.with_min_length(a),
                Vector::from_lane_fn(|lane| vector.lane(lane).with_min_length(a.to_array()[lane]))
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_panic_float_eq!(
                vector.with_min_length(a),
                Vector::from_lane_fn(|lane| vector.lane(lane).with_min_length(a.to_array()[lane]))
            );
        });
    }

    #[test]
    fn test_clamp_length() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_panic_float_eq!(
                vector.clamp_length(a, b),
                Vector::from_lane_fn(|lane| vector
                    .lane(lane)
                    .clamp_length(a.to_array()[lane], b.to_array()[lane]))
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_panic_float_eq!(
                vector.clamp_length(a, b),
                Vector::from_lane_fn(|lane| vector
                    .lane(lane)
                    .clamp_length(a.to_array()[lane], b.to_array()[lane]))
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_panic_float_eq!(
                vector.clamp_length(a, b),
                Vector::from_lane_fn(|lane| vector
                    .lane(lane)
                    .clamp_length(a.to_array()[lane], b.to_array()[lane]))
            );
        });
    }

    #[test]
    fn test_angle_between() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_float_eq!(
                vector_a.angle_between(vector_b),
                Wide::new(core::array::from_fn(|lane| vector_a
                    .lane(lane)
                    .angle_between(vector_b.lane(lane)))),
                r2nd <= Wide::splat(1e-5)
            );

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_float_eq!(
                vector_a.angle_between(vector_b),
                Wide::new(core::array::from_fn(|lane| vector_a
                    .lane(lane)
                    .angle_between(vector_b.lane(lane)))),
                r2nd <= Wide::splat(1e-5)
            );

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_float_eq!(
                vector_a.angle_between(vector_b),
                Wide::new(core::array::from_fn(|lane| vector_a
                    .lane(lane)
                    .angle_between(vector_b.lane(lane)))),
                r2nd <= Wide::splat(1e-5)
            );
        });
    }

    #[test]
    fn test_project_onto() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_panic_float_eq!(
                vector_a.project_onto(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).project_onto(vector_b.lane(lane)))
            );

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_panic_float_eq!(
                vector_a.project_onto(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).project_onto(vector_b.lane(lane)))
            );

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_panic_float_eq!(
                vector_a.project_onto(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).project_onto(vector_b.lane(lane)))
            );
        });
    }

    #[test]
    fn test_project_onto_normalized() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_panic_float_eq!(
                vector_a.project_onto_normalized(vector_b),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .project_onto_normalized(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.project_onto_normalized(vector_b),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .project_onto_normalized(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_panic_float_eq!(
                vector_a.project_onto_normalized(vector_b),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .project_onto_normalized(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.project_onto_normalized(vector_b),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .project_onto_normalized(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_panic_float_eq!(
                vector_a.project_onto_normalized(vector_b),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .project_onto_normalized(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.project_onto_normalized(vector_b),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .project_onto_normalized(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_reject_from() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_panic_float_eq!(
                vector_a.reject_from(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).reject_from(vector_b.lane(lane)))
            );

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_panic_float_eq!(
                vector_a.reject_from(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).reject_from(vector_b.lane(lane)))
            );

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_panic_float_eq!(
                vector_a.reject_from(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).reject_from(vector_b.lane(lane)))
            );
        });
    }

    #[test]
    fn test_reject_from_normalized() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_panic_float_eq!(
                vector_a.reject_from_normalized(vector_b),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .reject_from_normalized(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.reject_from_normalized(vector_b),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .reject_from_normalized(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_panic_float_eq!(
                vector_a.reject_from_normalized(vector_b),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .reject_from_normalized(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.reject_from_normalized(vector_b),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .reject_from_normalized(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_panic_float_eq!(
                vector_a.reject_from_normalized(vector_b),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .reject_from_normalized(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.reject_from_normalized(vector_b),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .reject_from_normalized(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_reflect() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_panic_float_eq!(
                vector_a.reflect(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).reflect(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_b) = vector_b.try_normalize() {
                assert_panic_float_eq!(
                    vector_a.reflect(vector_b),
                    Vector::from_lane_fn(|lane| vector_a.lane(lane).reflect(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_panic_float_eq!(
                vector_a.reflect(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).reflect(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_b) = vector_b.try_normalize() {
                assert_panic_float_eq!(
                    vector_a.reflect(vector_b),
                    Vector::from_lane_fn(|lane| vector_a.lane(lane).reflect(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_panic_float_eq!(
                vector_a.reflect(vector_b),
                Vector::from_lane_fn(|lane| vector_a.lane(lane).reflect(vector_b.lane(lane))),
                0.0 = -0.0
            );

            if let Some(vector_b) = vector_b.try_normalize() {
                assert_panic_float_eq!(
                    vector_a.reflect(vector_b),
                    Vector::from_lane_fn(|lane| vector_a.lane(lane).reflect(vector_b.lane(lane))),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_refract() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;
            let a = y ^ z;
            let b = w + a;

            let eta = (x + 0.1).abs() / (y + 0.2).abs();

            let vector_a = Vector::<2, Wide, A>::new(x, y);
            let vector_b = Vector::<2, Wide, A>::new(z, w);
            assert_panic_float_eq!(
                vector_a.refract(vector_b, eta),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .refract(vector_b.lane(lane), eta.to_array()[lane]))
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.refract(vector_b, eta),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .refract(vector_b.lane(lane), eta.to_array()[lane]))
                );
            }

            let vector_a = Vector::<3, Wide, A>::new(x, y, z);
            let vector_b = Vector::<3, Wide, A>::new(w, a, b);
            assert_panic_float_eq!(
                vector_a.refract(vector_b, eta),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .refract(vector_b.lane(lane), eta.to_array()[lane]))
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.refract(vector_b, eta),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .refract(vector_b.lane(lane), eta.to_array()[lane]))
                );
            }

            let vector_a = Vector::<4, Wide, A>::new(x, y, z, w);
            let vector_b = Vector::<4, Wide, A>::new(a, b, x, z);
            assert_panic_float_eq!(
                vector_a.refract(vector_b, eta),
                Vector::from_lane_fn(|lane| vector_a
                    .lane(lane)
                    .refract(vector_b.lane(lane), eta.to_array()[lane]))
            );

            if let Some(vector_a) = vector_a.try_normalize()
                && let Some(vector_b) = vector_b.try_normalize()
            {
                assert_panic_float_eq!(
                    vector_a.refract(vector_b, eta),
                    Vector::from_lane_fn(|lane| vector_a
                        .lane(lane)
                        .refract(vector_b.lane(lane), eta.to_array()[lane]))
                );
            }
        });
    }

    #[test]
    fn test_any_orthogonal_vector() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x + y;

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_float_eq!(
                vector.any_orthogonal_vector(),
                Vector::from_lane_fn(|lane| vector.lane(lane).any_orthogonal_vector())
            );

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.any_orthogonal_vector(),
                Vector::from_lane_fn(|lane| vector.lane(lane).any_orthogonal_vector())
            );

            let vector = Vector::<4, Wide, A>::new(x, y, z, w);
            assert_float_eq!(
                vector.any_orthogonal_vector(),
                Vector::from_lane_fn(|lane| vector.lane(lane).any_orthogonal_vector())
            );
        });
    }

    #[test]
    fn test_rotate() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];

            // `wide` trigonometric functions do not handle large values well.
            if !z.abs().simd_lt(1e8).all() {
                return;
            }

            let vector = Vector::<2, Wide, A>::new(x, y);
            assert_float_eq!(
                vector.rotate(z),
                Vector::from_lane_fn(|lane| vector.lane(lane).rotate(z.to_array()[lane])),
                r2nd <= Vector::splat(Wide::splat(1e-5)) * z.abs(),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_homogeneous() {
        for_parameters!(|Wide: WideFloat, A, x| {
            let _: Wide = x;
            let [y, z, w] = [x + 1.0, x + 2.0, x + 3.0];

            assert_float_eq!(
                Vector::<3, Wide, A>::from_homogeneous(Vector::<4, Wide, A>::new(x, y, z, w)),
                Vector::<3, Wide, A>::new(x, y, z) / w
            );
        });
    }

    #[test]
    fn test_rotate_x() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            // `wide` trigonometric functions do not handle large values well.
            if !w.abs().simd_lt(1e8).all() {
                return;
            }

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.rotate_x(w),
                Vector::from_lane_fn(|lane| vector.lane(lane).rotate_x(w.to_array()[lane])),
                r2nd <= Vector::splat(Wide::splat(1e-5)) * w.abs(),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_rotate_y() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            // `wide` trigonometric functions do not handle large values well.
            if !w.abs().simd_lt(1e8).all() {
                return;
            }

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.rotate_y(w),
                Vector::from_lane_fn(|lane| vector.lane(lane).rotate_y(w.to_array()[lane])),
                r2nd <= Vector::splat(Wide::splat(1e-5)) * w.abs(),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_rotate_z() {
        for_parameters!(|Wide: WideFloat, A, x, y, z| {
            let _: [Wide; 3] = [x, y, z];
            let w = x ^ y;

            // `wide` trigonometric functions do not handle large values well.
            if !w.abs().simd_lt(1e8).all() {
                return;
            }

            let vector = Vector::<3, Wide, A>::new(x, y, z);
            assert_float_eq!(
                vector.rotate_z(w),
                Vector::from_lane_fn(|lane| vector.lane(lane).rotate_z(w.to_array()[lane])),
                r2nd <= Vector::splat(Wide::splat(1e-5)) * w.abs(),
                0.0 = -0.0
            );
        });
    }
}
