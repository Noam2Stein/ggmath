use crate::{Alignment, FloatExt, Matrix, PrimitiveFloat, Rotation2, Vector};

impl<T, A: Alignment> Rotation2<T, A>
where
    T: PrimitiveFloat,
{
    /// TODO
    pub const NAN: Self = Self::from_sin_cos(T::NAN, T::NAN);

    /// TODO
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        let (sin, cos) = angle.sin_cos();
        Self::from_sin_cos(sin, cos)
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

        Self::from_sin_cos(from.wedge(to), from.dot(to))
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

        Self::from_sin_cos(from.wedge(to), dot) * dot.signum()
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
                    .wedge(matrix.y_axis)
                    .abs_diff_eq(T::ONE, T::as_from(1e-4)),
            "not a rotation matrix: Rot2::from_matrix({matrix:?})"
        );

        Self::from_vector(matrix.x_axis)
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_angle(self) -> T {
        todo!()
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

        self.dot(other).abs().acos_approx()
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

        let other = if self.dot(other).is_sign_negative() {
            -other
        } else {
            other
        };

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

        todo!()
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

        todo!()
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
