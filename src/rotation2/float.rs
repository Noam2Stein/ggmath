use crate::{Alignment, FloatExt, Matrix, PrimitiveFloat, Rotation2, Vector};

impl<T, A: Alignment> Rotation2<T, A>
where
    T: PrimitiveFloat,
{
    /// TODO
    pub const NAN: Self = Self::from_cos_sin(T::NAN, T::NAN);

    /// TODO
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_cos_sin(cos, sin)
    }

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
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

    /// TODO
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        self.0.length()
    }

    /// TODO
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

    /// TODO
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        self.0.try_normalize().map(Self)
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn normalize_or(self, fallback: Self) -> Self {
        Self(self.0.normalize_or(fallback.0))
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        let (normalize, length) = self.0.normalize_and_length();
        (Self(normalize), length)
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        self.0.is_normalized()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, other: Self, max_abs_diff: T) -> bool {
        self.0.abs_diff_eq(other.0, max_abs_diff)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Matrix, Rotation2, Vector,
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
