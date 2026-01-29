#[cfg(feature = "libm")]
#[allow(unused_imports)]
use crate::libm::LibmFloatExt;
use crate::{Alignment, Length, Scalar, SupportedLength, Vector, utils::specialize};

impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Returns true if any component of `self` is NaN.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        specialize!(<T as FloatBackend<N, A>>::vec_is_nan(self))
    }

    /// Returns true if all components of `self` are finite.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        specialize!(<T as FloatBackend<N, A>>::vec_is_finite(self))
    }

    /// Computes `1.0 / self`.
    #[inline]
    #[must_use]
    pub fn recip(self) -> Self {
        Self::ONE / self
    }

    /// Computes the sum of the vector's elements.
    ///
    /// The order of addition is unspecified and may differ between target
    /// architectures.
    #[inline]
    #[must_use]
    pub fn element_sum(self) -> T {
        specialize!(<T as FloatBackend<N, A>>::vec_element_sum(self))
    }

    /// Computes the product of the vector's elements.
    ///
    /// The order of multiplication is unspecified and may differ between target
    /// architectures.
    #[inline]
    #[must_use]
    pub fn element_product(self) -> T {
        specialize!(<T as FloatBackend<N, A>>::vec_element_product(self))
    }

    /// Returns the maximum between the components of `self` and `other`.
    ///
    /// This function is not consistent with IEEE semantics in regards to NaN
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
    /// This function is not consistent with IEEE semantics in regards to NaN
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

    /// Clamps the components of `self` between the components of `min` and
    /// `max`.
    ///
    /// This function is not consistent with IEEE semantics in regards to NaN
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

    /// Returns the maximum component of the vector.
    ///
    /// This function is not consistent with IEEE semantics in regards to NaN
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
    /// This function is not consistent with IEEE semantics in regards to NaN
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

    /// Returns the absolute values of the components of `self`.
    #[inline]
    #[must_use]
    pub fn abs(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_abs(self))
    }

    /// Returns the signum of the components of `self`.
    ///
    /// For each component:
    ///
    /// - `1.0` if the component is positive or `+0.0`.
    /// - `-1.0` if the component is negative or `-0.0`.
    /// - `NaN` if the component is NaN.
    #[inline]
    #[must_use]
    pub fn signum(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_signum(self))
    }

    /// Returns a vector with the magnitudes of `self` and the signs of `sign`.
    #[inline]
    #[must_use]
    pub fn copysign(self, sign: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_copysign(self, sign))
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

    /// Computes the squared euclidean distance between `self` and `other`.
    #[inline]
    #[must_use]
    pub fn distance_squared(self, other: Self) -> T {
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
    pub fn lerp(self, other: Self, t: T) -> Self {
        self * (1.0 - t) + other * t
    }

    /// Computes the midpoint between `self` and `other`.
    ///
    /// This function is equivalent to `self.lerp(other, 0.5)` but is cheaper to
    /// compute.
    ///
    /// This function may return a slightly different value than `lerp`.
    #[inline]
    #[must_use]
    pub fn midpoint(self, other: Self) -> Self {
        (self + other) * 0.5
    }

    /// Returns whether `self` has the length `1.0` or not.
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
    /// When assertions are enabled, this function panics if `other` is a zero
    /// vector.
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
    /// When assertions are enabled, this function panics if `other` is not
    /// normalized.
    #[inline]
    #[must_use]
    pub fn project_onto_normalized(self, other: Self) -> Self {
        #[cfg(assertions)]
        assert!(other.is_normalized());

        other * self.dot(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// Vector rejection is the vector pointing from the projection to the
    /// original vector. Basically: `self - self.project_onto(other)`.
    ///
    /// `other` must not be a zero vector.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if `other` is a zero
    /// vector.
    #[inline]
    #[must_use]
    pub fn reject_from(self, other: Self) -> Self {
        self - self.project_onto(other)
    }

    /// Returns the vector rejection of `self` from `other`.
    ///
    /// Vector rejection is the vector pointing from the projection to the
    /// original vector. Basically: `self - self.project_onto(other)`.
    ///
    /// `other` must be normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if `other` is not
    /// normalized.
    #[inline]
    #[must_use]
    pub fn reject_from_normalized(self, other: Self) -> Self {
        self - self.project_onto_normalized(other)
    }

    /// Returns the vector reflection for `self` with `normal`.
    ///
    /// Vector reflection is the reflection of the vector on a surface with the
    /// given normal.
    ///
    /// `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if `normal` is not
    /// normalized.
    #[inline]
    #[must_use]
    pub fn reflect(self, normal: Self) -> Self {
        #[cfg(assertions)]
        assert!(normal.is_normalized());

        self - normal * (2.0 * self.dot(normal))
    }
}

impl<A: Alignment> Vector<3, T, A> {
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
    /// The output vector is not necessarily normalized. For that use
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
    /// When assertions are enabled, this function panics if `self` is not
    /// normalized.
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
    /// vectors are orthogonal to each other and are normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if `self` is not
    /// normalized.
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
impl<const N: usize, A: Alignment> Vector<N, T, A>
where
    Length<N>: SupportedLength,
{
    /// Rounds the components of `self` down.
    #[inline]
    #[must_use]
    pub fn floor(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_floor(self))
    }

    /// Rounds the components of `self` up.
    #[inline]
    #[must_use]
    pub fn ceil(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_ceil(self))
    }

    /// Rounds the components of `self` to the nearest integer.
    #[inline]
    #[must_use]
    pub fn round(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_round(self))
    }

    /// Rounds the components of `self` towards zero.
    #[inline]
    #[must_use]
    pub fn trunc(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_trunc(self))
    }

    /// Returns the fractional part of `self`.
    ///
    /// This function is equivalent to `self - self.trunc()`.
    #[inline]
    #[must_use]
    pub fn fract(self) -> Self {
        self - self.trunc()
    }

    /// Fused Multiply Add. Computes `self * a + b` with only one rounding error
    /// instead of two.
    ///
    /// This is slower than an unfused multiply add for most target
    /// architectures.
    ///
    /// This function is guaranteed to return the exact same value as the
    /// standard library.
    #[inline]
    #[must_use]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_mul_add(self, a, b))
    }

    /// Euclidiean Division.
    ///
    /// This function is guaranteed to return the exact same value as the
    /// standard library.
    #[inline]
    #[must_use]
    pub fn div_euclid(self, rhs: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_div_euclid(self, rhs))
    }

    /// Euclidiean Remainder.
    ///
    /// This function is guaranteed to return the exact same value as the
    /// standard library.
    #[inline]
    #[must_use]
    pub fn rem_euclid(self, rhs: Self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_rem_euclid(self, rhs))
    }

    /// Computes the square root of the components of `self`.
    ///
    /// This function is guaranteed to return the exact same value as the
    /// standard library.
    #[inline]
    #[must_use]
    pub fn sqrt(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_sqrt(self))
    }

    /// Computes the sine of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn sin(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_sin(self))
    }

    /// Computes the cosine of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn cos(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_cos(self))
    }

    /// Computes the tangent of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn tan(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_tan(self))
    }

    /// Computes the arc sine of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn asin(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_asin(self))
    }

    /// Computes the arc cosine of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn acos(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_acos(self))
    }

    /// Computes the arc tangent of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn atan(self) -> Self {
        specialize!(<T as FloatBackend<N, A>>::vec_atan(self))
    }

    /// Simultaneously computes the sine and cosine of the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn sin_cos(self) -> (Self, Self) {
        specialize!(<T as FloatBackend<N, A>>::vec_sin_cos(self))
    }

    /// Returns the length/magnitude of `self`.
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.dot(self).sqrt()
    }

    /// Computes the euclidean distance between `self` and `other`.
    #[inline]
    #[must_use]
    pub fn distance(self, other: Self) -> T {
        (self - other).length()
    }

    /// Moves `self` towards `other` by the value `max_delta`.
    ///
    /// When `max_delta` is `0.0`, the result is `self`. When `max_delta` is
    /// equal to or greater than `self.distance(other)`, the result is `other`.
    #[inline]
    #[must_use]
    pub fn move_towards(self, other: Self, max_delta: T) -> Self {
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

    /// Computes [`self.normalize`](Self::normalize) or returns `None` if the
    /// input is zero or if the result is non finite or zero.
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

    /// Computes [`self.normalize`](Self::normalize) or returns `fallback` if
    /// the input is zero or if the result is non finite or zero.
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        self.try_normalize().unwrap_or(fallback)
    }

    /// Computes [`self.normalize`](Self::normalize) or returns a zero vector if
    /// the input is zero or if the result is non finite or zero.
    #[inline]
    #[must_use]
    pub fn normalize_or_zero(self) -> Self {
        self.normalize_or(Self::ZERO)
    }

    /// Computes [`self.normalize()`](Self::normalize) and
    /// [`self.length()`](Self::length).
    ///
    /// If `self` is a zero vector, the function returns
    /// `(Self::ZERO, 0.0)`.
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        let length = self.length();
        let recip = 1.0 / length;

        if recip.is_finite() && recip > 0.0 {
            (self * recip, length)
        } else {
            (Self::ZERO, 0.0)
        }
    }

    /// Returns `self` with a length of no more than `max`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if `max` is negative.
    #[inline]
    #[must_use]
    pub fn with_max_length(self, max: T) -> Self {
        #[cfg(assertions)]
        assert!(max >= 0.0, "negative maximum length");

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
    /// When assertions are enabled, this function panics if `min` is negative.
    #[inline]
    #[must_use]
    pub fn with_min_length(self, min: T) -> Self {
        #[cfg(assertions)]
        assert!(min >= 0.0, "negative minimum length");

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
    /// When assertions are enabled, this function panics if `min` is greater
    /// than `max`, or if either `min` or `max` are negative.
    #[inline]
    #[must_use]
    pub fn clamp_length(self, min: T, max: T) -> Self {
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

    /// Computes the angle (in radians) between two vectors in the range
    /// `[0, +π]`.
    ///
    /// The vectors do not need to be unit vectors but they do need to be
    /// non-zero.
    ///
    /// This function has unspecified precision.
    #[inline]
    #[must_use]
    pub fn angle_between(self, other: Self) -> T {
        (self.dot(other) / (self.length_squared() * other.length_squared()).sqrt()).acos()
    }

    /// Computes the exponential function `e^self` for the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn exp(self) -> Self {
        self.map(T::exp)
    }

    /// Computes `2^self` for the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn exp2(self) -> Self {
        self.map(T::exp2)
    }

    /// Computes the natural logarithm for the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn ln(self) -> Self {
        self.map(T::ln)
    }

    /// Computes the base 2 logarithm for the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn log2(self) -> Self {
        self.map(T::log2)
    }

    /// Computes `self^n` for the components of `self`.
    ///
    /// This function has unspecified precision and may return a different value
    /// than the standard library.
    #[inline]
    #[must_use]
    pub fn powf(self, n: T) -> Self {
        self.map(|x| x.powf(n))
    }

    /// Returns the vector refraction of `self` through `normal` and `eta`.
    ///
    /// `eta` is the incident index divided by the transmitted index.
    ///
    /// When total internal reflection occurs, this function returns a zero
    /// vector.
    ///
    /// `self` and `normal` must be normalized.
    ///
    /// # Panics
    ///
    /// When assertions are enabled, this function panics if `self` or `normal`
    /// are not normalized.
    #[inline]
    #[must_use]
    pub fn refract(self, normal: Self, eta: T) -> Self {
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
impl<A: Alignment> Vector<2, T, A> {
    /// Returns `self` rotated by 90 degrees.
    #[inline]
    #[must_use]
    pub fn perp(self) -> Self {
        Self::new(-self.y, self.x)
    }

    /// Rotates `self` by `angle` (in radians).
    ///
    /// This function has unspecified precision.
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
impl<A: Alignment> Vector<3, T, A> {
    /// Rotates `self` around the x axis by `angle` (in radians).
    ///
    /// This function has unspecified precision.
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
    /// This function has unspecified precision.
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
    /// This function has unspecified precision.
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
    fn vec_element_sum(vec: Vector<N, T, A>) -> T {
        vec.as_array_ref().iter().copied().sum()
    }

    #[inline]
    fn vec_element_product(vec: Vector<N, T, A>) -> T {
        vec.as_array_ref().iter().copied().product()
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
    fn vec_max_element(vec: Vector<N, T, A>) -> T {
        vec.iter().reduce(|a, b| if a > b { a } else { b }).unwrap()
    }

    #[inline]
    fn vec_min_element(vec: Vector<N, T, A>) -> T {
        vec.iter().reduce(|a, b| if a < b { a } else { b }).unwrap()
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

    #[cfg(backend)]
    #[inline]
    fn vec_sin_cos(vec: Vector<N, T, A>) -> (Vector<N, T, A>, Vector<N, T, A>) {
        let array = vec.to_array().map(|x| x.sin_cos());

        (
            Vector::from_fn(|i| array[i].0),
            Vector::from_fn(|i| array[i].1),
        )
    }
}
