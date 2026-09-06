use crate::{Affine, Alignment, FloatExt, Matrix, PrimitiveFloat, Projective, Rotation2, Vector};

impl<T, A: Alignment> Rotation2<T, A>
where
    T: PrimitiveFloat,
{
    /// A 2D rotation with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_cos_sin(T::NAN, T::NAN);

    /// Creates a 2D rotation from an `angle` (in radians) rotating `+X` to
    /// `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_cos_sin(cos, sin)
    }

    /// Returns the rotation transforming `from` to `to`.
    ///
    /// This assumes `from` and `to` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc(from: Vector<2, T, A>, to: Vector<2, T, A>) -> Self {
        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc({from:?}, {to:?})"
        );

        Self::from_cos_sin(from.dot(to), from.perp_dot(to))
    }

    /// Returns the rotation transforming `from` to either `to` or `-to`,
    /// rotating up to 90 degrees.
    ///
    /// This assumes `from` and `to` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc_colinear(from: Vector<2, T, A>, to: Vector<2, T, A>) -> Self {
        debug_assert!(
            from.is_normalized() && to.is_normalized(),
            "vectors are not normalized: from_rotation_arc_colinear({from:?}, {to:?})"
        );

        let dot = from.dot(to);

        Self::from_cos_sin(dot, from.perp_dot(to)) * dot.signum()
    }

    /// Converts a rotation matrix to a 2D rotation represented by a complex
    /// number.
    ///
    /// This assumes `matrix` only contains rotation.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `matrix` is not approximately a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_matrix(matrix: &Matrix<2, T, A>) -> Self {
        debug_assert!(
            matrix
                .x_axis
                .length_squared()
                .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && matrix
                    .y_axis
                    .length_squared()
                    .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && matrix
                    .x_axis
                    .perp_dot(matrix.y_axis)
                    .abs_diff_eq(T::ONE, T::as_from(1e-4)),
            "not a rotation matrix: Rot2::from_matrix({matrix:?})"
        );

        Self(matrix.x_axis)
    }

    /// Converts an affine transform to a 2D rotation represented by a complex
    /// number.
    ///
    /// This assumes `affine` only contains rotation, and translation which is
    /// ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `affine.matrix` is not approximately a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_affine(affine: &Affine<2, T, A>) -> Self {
        Self::from_matrix(&affine.matrix)
    }

    /// Converts a projective transform to a 2D rotation represented by a
    /// complex number.
    ///
    /// This assumes `projective` only contains rotation, and translation which
    /// is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `projective` is not approximately a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_projective(projective: &Projective<2, T, A>) -> Self {
        debug_assert!(
            projective
                .x_axis
                .truncate()
                .length_squared()
                .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && projective
                    .y_axis
                    .truncate()
                    .length_squared()
                    .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && projective
                    .x_axis
                    .truncate()
                    .perp_dot(projective.y_axis.truncate())
                    .abs_diff_eq(T::ONE, T::as_from(1e-4))
                && projective
                    .z_axis
                    .abs_diff_eq(Vector::<3, T, A>::Z, T::as_from(1e-4)),
            "not a rotation: Rot2::from_projective({projective:?})"
        );

        Self(projective.x_axis.truncate())
    }

    /// Converts a 2D rotation to an angle (in radians) rotating `+X` to `+Y`.
    ///
    /// This assumes `self` is normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_angle(self) -> T {
        debug_assert!(
            self.is_normalized(),
            "rotation is not normalized: {self:?}.to_angle()"
        );

        self.sin.atan2(self.cos)
    }

    /// Returns `true` if any element is NaN.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// Returns the inverse of a 2D rotation.
    ///
    /// This assumes `self` is normalized.
    ///
    /// This is the same as [`conjugate`], but asserts that `self` is
    /// normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    ///
    /// [`conjugate`]: Self::conjugate
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(self) -> Self {
        debug_assert!(
            self.is_normalized(),
            "2D rotation is not normalized: {self:?}.inverse()"
        );

        self.conjugate()
    }

    /// Returns the absolute angle (in radians) between two rotations.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_between(self, other: Self) -> T {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "2D rotations are not normalized: {self:?}.angle_between({other:?})"
        );

        self.dot(other).acos_approx()
    }

    /// Returns the signed angle (in radians) transforming `self` to `other`.
    ///
    /// The result is in the range `-π..π`. Positive is counter-clockwise and
    /// negative is clockwise.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// `self.angle_to(other)` is identical to `other.angle_from(self)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_to(self, other: Self) -> T {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "2D rotations are not normalized: {self:?}.angle_to({other:?})"
        );

        self.dot(other).acos_approx() * self.perp_dot(other).signum()
    }

    /// Returns the signed angle (in radians) transforming `other` to `self`.
    ///
    /// The result is in the range `-π..π`. Positive is counter-clockwise and
    /// negative is clockwise.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// `self.angle_from(other)` is identical to `other.angle_to(self)`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panic ifs `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_from(self, other: Self) -> T {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "2D rotations are not normalized: {self:?}.angle_from({other:?})"
        );

        self.dot(other).acos_approx() * other.perp_dot(self).signum()
    }

    /// Computes the linear interpolation between two rotations, then normalizes
    /// the result.
    ///
    /// When `t` is `0`, the result is `self`. When `t` is `1`, the result is
    /// `other`.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// This does not interpolate the angle at a constant speed. For that use
    /// [`slerp`]. This function is more efficient as it avoids calling
    /// trigonometric functions. This function breaks when rotations are exactly
    /// 180 degrees apart, so only use this if you know that is not a
    /// possibility.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized, or if `self` and `other`
    /// are exactly 180 degrees apart.
    ///
    /// [`slerp`]: Self::slerp
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lerp(self, other: Self, t: T) -> Self {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "2D rotations are not normalized: {self:?}.lerp({other:?}, {t:?})"
        );
        debug_assert!(
            self.dot(other) > T::as_from(-0.999),
            "attempt to lerp opposite 2D rotations: {self:?}.lerp({other:?}, {t:?})"
        );

        (self * (T::ONE - t) + other * t).normalize()
    }

    /// Computes the spherical linear interpolation between two rotations.
    ///
    /// When `t` is `0`, the result is `self`. When `t` is `1`, the result is
    /// `other`. This interpolates the angle at a constant speed, always taking
    /// the shorter path.
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn slerp(self, other: Self, t: T) -> Self {
        debug_assert!(
            self.is_normalized() && other.is_normalized(),
            "2D rotations are not normalized: {self:?}.slerp({other:?}, {t:?})"
        );

        // diff = other * self.inverse()
        let diff = Self::from_cos_sin(
            other.cos * self.cos + other.sin * self.sin,
            other.sin * self.cos - other.cos * self.sin,
        );

        Self::from_angle(diff.to_angle() * t) * self
    }

    /// Rotates one rotation towards another by at most `max_angle` (in
    /// radians).
    ///
    /// This assumes `self` and `other` are normalized.
    ///
    /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
    /// to or greater than `self.angle_between(target)`, the result is `target`.
    /// When `max_angle` is negative, this rotates towards the opposite of
    /// `target`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn rotate_towards(self, target: Self, max_angle: T) -> Self {
        debug_assert!(
            self.is_normalized() && target.is_normalized(),
            "2D rotations are not normalized: {self:?}.rotate_towards({target:?}, {max_angle:?})"
        );

        // diff = target * self.inverse()
        let diff = Self::from_cos_sin(
            target.cos * self.cos + target.sin * self.sin,
            target.sin * self.cos - target.cos * self.sin,
        );

        // Handle negative `max_angle` by rotating towards `-target`
        let diff = diff * max_angle.signum();
        let max_angle = max_angle.abs();

        Self::from_angle(diff.to_angle().clamp(-max_angle, max_angle)) * self
    }

    /// Returns the length/magnitude of a complex number.
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.0.length()
    }

    /// Returns `self` normalized to length `1`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is a zero vector, or if the result is non finite or
    /// zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        let result = self / self.length();

        debug_assert!(
            result.is_finite() && result != Self(Vector::ZERO),
            "2D rotation is zero or non-finite: {self:?}.normalize()"
        );

        result
    }

    /// Returns [`normalize`], or `None` if `self` is zero or if the result is
    /// non finite or zero.
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        self.0.try_normalize().map(Self)
    }

    /// Returns [`normalize`], or `fallback` if `self` is zero or if the result
    /// is non finite or zero.
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        Self(self.0.normalize_or(fallback.0))
    }

    /// Simultaneously computes [`normalize`] and [`length`].
    ///
    /// [`normalize`]: Self::normalize
    /// [`length`]: Self::length
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        let (normalize, length) = self.0.normalize_and_length();
        (Self(normalize), length)
    }

    /// Returns whether the rotation has the length `1` or not.
    ///
    /// This uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        self.0.is_normalized()
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two rotations that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Matrix, Projective, Rotation2, Vector,
        test_utils::{assert_debug_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_from_rotation_arc() {
        for_types!(|T: PrimitiveFloat, A| {
            for [from, to] in random_iter::<[Vector<2, T, A>; 2]>() {
                let [from, to] =
                    [from, to].map(|v| v.normalize_or(Vector::<2, T, A>::X).normalize());

                assert_test_eq!(
                    from * Rotation2::<T, A>::from_rotation_arc(from, to),
                    to,
                    abs <= 1e-5,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        for_types!(|T: PrimitiveFloat, A| {
            for [from, to] in random_iter::<[Vector<2, T, A>; 2]>() {
                let [from, to] =
                    [from, to].map(|v| v.normalize_or(Vector::<2, T, A>::X).normalize());

                assert_test_eq!(
                    Rotation2::<T, A>::from_rotation_arc_colinear(from, to),
                    if from.dot(to).is_sign_positive() {
                        Rotation2::<T, A>::from_rotation_arc(from, to)
                    } else {
                        Rotation2::<T, A>::from_rotation_arc(from, -to)
                    },
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in
                random_iter::<(Vector<2, T, A>, T)>().filter(|(_, angle)| angle.is_finite())
            {
                let matrix = Matrix::<2, T, A>::from_angle(angle);

                assert_test_eq!(
                    vector * Rotation2::<T, A>::from_matrix(&matrix),
                    vector * matrix
                );
            }
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in
                random_iter::<(Vector<2, T, A>, T)>().filter(|(_, angle)| angle.is_finite())
            {
                let projective = Projective::<2, T, A>::from_angle(angle);

                assert_test_eq!(
                    vector * Rotation2::<T, A>::from_projective(&projective),
                    projective.transform_point(vector),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_to_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>().filter(|angle| angle.is_finite()) {
                let angle = angle % 3.0;

                assert_test_eq!(
                    Rotation2::<T, A>::from_angle(angle).to_angle(),
                    angle,
                    abs <= 1e-4
                );
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotation in random_iter::<Rotation2<T, A>>() {
                let rotation = rotation.normalize_or(Rotation2::IDENTITY).normalize();

                assert_test_eq!(
                    rotation * rotation.inverse(),
                    Rotation2::IDENTITY,
                    abs <= 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_angle_between() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotation2<T, A>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());

                assert_test_eq!(a.angle_between(b), a.0.angle_between(b.0), abs <= 1e-2);
            }
        });
    }

    #[test]
    fn test_angle_to() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotation2<T, A>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());

                assert_test_eq!(a.angle_to(b), a.0.angle_to(b.0), abs <= 1e-2);
            }
        });
    }

    #[test]
    fn test_angle_from() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotation2<T, A>; 2]>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());

                assert_test_eq!(a.angle_from(b), a.0.angle_from(b.0), abs <= 1e-2);
            }
        });
    }

    #[test]
    fn test_lerp() {
        for_types!(|T: PrimitiveFloat, A| {
            for ([a, b], t) in random_iter::<([Rotation2<T, A>; 2], T)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());
                let t = if t.is_finite() { t % 5.0 } else { 0.0 };

                assert_debug_panic!(a.lerp(-a, t));

                if a.dot(b) > -0.99 {
                    assert_test_eq!(a.lerp(b, t), Rotation2(a.0.lerp(b.0, t).normalize()));
                }
            }
        });
    }

    #[test]
    fn test_slerp() {
        for_types!(|T: PrimitiveFloat, A| {
            for ([a, b], t) in random_iter::<([Rotation2<T, A>; 2], T)>() {
                let [a, b] = [a, b].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());
                let t = if t.is_finite() { t % 5.0 } else { 0.0 };

                assert_test_eq!(a.slerp(b, 0.0), a, abs <= 1e-3, 0.0 = -0.0);
                assert_test_eq!(a.slerp(b, 1.0), b, abs <= 1e-3, 0.0 = -0.0);

                assert_test_eq!(
                    a.slerp(b, t),
                    a * Rotation2::<T, A>::from_angle(a.angle_to(b) * t),
                    abs <= 2e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_rotate_towards() {
        for_types!(|T: PrimitiveFloat, A| {
            for ([current, target], max_angle) in random_iter::<([Rotation2<T, A>; 2], T)>() {
                let [current, target] =
                    [current, target].map(|r| r.normalize_or(Rotation2::IDENTITY).normalize());
                let max_angle = if max_angle.is_finite() {
                    max_angle % 10.0
                } else {
                    0.0
                };

                if max_angle.is_sign_negative() {
                    if current != target {
                        assert_test_eq!(
                            current.rotate_towards(target, max_angle),
                            current.rotate_towards(-target, -max_angle)
                        )
                    } else {
                        assert_test_eq!(
                            current
                                .rotate_towards(target, max_angle)
                                .angle_between(-target),
                            (current.angle_between(-target) - max_angle.abs()).max(0.0),
                            abs <= 1e-3,
                            0.0 = -0.0
                        );
                    }
                } else {
                    assert_test_eq!(
                        current
                            .rotate_towards(target, max_angle)
                            .angle_between(target),
                        (current.angle_between(target) - max_angle).max(0.0),
                        abs <= 1e-3,
                        0.0 = -0.0
                    );
                }
            }
        });
    }
}
