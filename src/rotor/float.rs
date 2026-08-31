use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, PrimitiveFloat, Projective, Rotor, Vector,
    length::TwoOrThree,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: PrimitiveFloat,
{
    /// A rotor with all elements set to NaN (Not a Number).
    pub const NAN: Self = todo!();

    /// Returns the minimal rotation transforming `from` to `to`.
    ///
    /// The rotation is in the plane spanned by `from` and `to`. Rotates up to
    /// 180 degrees.
    ///
    /// When `from≈to` this is only accurate to about `0.001` (for `f32`).
    ///
    /// `from` and `to` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc(_from: Vector<N, T, A>, _to: Vector<N, T, A>) -> Self {
        todo!()
    }

    /// Returns the minimal rotation transforming `from` to either `to` or
    /// `-to`. This rotates `from` so that it is colinear with `to`.
    ///
    /// The rotation is in the plane spanned by `from` and `to`. Rotates up to
    /// 90 degrees.
    ///
    /// When `from≈to` or `from≈-to` this is only accurate to about `0.001` (for
    /// `f32`).
    ///
    /// `from` and `to` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `from` or `to` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc_colinear(_from: Vector<N, T, A>, _to: Vector<N, T, A>) -> Self {
        todo!()
    }

    /// Converts a rotation matrix to a rotor.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `matrix` is not a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_matrix(_matrix: &Matrix<N, T, A>) -> Self {
        todo!()
    }

    /// Converts an affine transform with rotation to a rotor.
    ///
    /// This function assumes the transform only contains rotation, and possibly
    /// translation, which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `affine.matrix` is not a rotation matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_affine(_affine: &Affine<N, T, A>) -> Self {
        todo!()
    }

    /// Converts a projective transform with rotation to a rotor.
    ///
    /// This function assumes the transform only contains rotation, and possibly
    /// translation, which is ignored.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `projective` is not a rotation transform.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_projective(_projective: &Projective<N, T, A>) -> Self {
        todo!()
    }

    /// Returns `true` if any element is NaN.
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        todo!()
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        todo!()
    }

    /// Returns the inverse of a rotor.
    ///
    /// The only difference between this and [`conjugate`] is that this asserts
    /// [`self.is_normalized()`] when debug assertions are enabled. Use
    /// whichever function makes your intentions clearer.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    ///
    /// [`conjugate`]: Self::conjugate
    /// [`self.is_normalized()`]: Self::is_normalized
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(self) -> Self {
        todo!()
    }

    /// Returns the angle (in radians) for the minimal rotation for transforming
    /// `self` into `other`.
    ///
    /// `self` and `other` must be normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_between(self, _other: Self) -> T {
        todo!()
    }

    /// Computes the linear interpolation between `self` and `other` based on
    /// the value `t`, then normalizes the result.
    ///
    /// When `t` is 0, the result is `self`.  When `t` is 1, the result is
    /// `rhs`.
    ///
    /// Note that this does *not* interpolate the angle. For that, use [`slerp`].
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    ///
    /// [`slerp`]: Self::slerp
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lerp(self, _other: Self, _t: T) -> Self {
        todo!()
    }

    /// Computes the spherical linear interpolation between `self` and `other`
    /// based on the value `t`.
    ///
    /// When `t` is `0`, the result is `self`.  When `t` is `1`, the result is
    /// `other`.
    ///
    /// This function assumes both rotors are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `other` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn slerp(self, _other: Self, _t: T) -> Self {
        todo!()
    }

    /// Rotates `self` towards `target` by at most `max_angle` (in radians).
    ///
    /// When `max_angle` is `0`, the result is `self`. When `max_angle` is equal
    /// to or greater than `self.angle_between(target)`, the result is `target`.
    /// When `max_angle` is negative, rotates towards the opposite of `target`.
    ///
    /// This assumes `self` and `target` are normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` or `target` are not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn rotate_towards(self, _target: Self, _max_angle: T) -> Self {
        todo!()
    }

    /// Returns the length/magnitude of `self`.
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        todo!()
    }

    /// Returns `self` normalized to length `1`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is a zero rotor, or if the result is non finite or
    /// zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        todo!()
    }

    /// Returns [`normalize`], or `None` if `self` is zero or if the result is
    /// non finite or zero.
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        todo!()
    }

    /// Returns [`normalize`], or `fallback` if `self` is zero or if the result
    /// is non finite or zero.
    ///
    /// [`normalize`]: Self::normalize
    #[inline]
    #[must_use]
    pub fn normalize_or(self, _fallback: Self) -> Self {
        todo!()
    }

    /// Simultaneously computes [`normalize`] and [`length`].
    ///
    /// This assumes the rotor is not zero (so the output for that will be
    /// garbage). Consider manually checking for that case.
    ///
    /// [`normalize`]: Self::normalize
    /// [`length`]: Self::length
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        todo!()
    }

    /// Returns whether the rotor has the length 1 or not.
    ///
    /// This uses a precision threshold of approximately `1e-4`.
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        todo!()
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two rotors that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(self, _other: Self, _max_abs_diff: T) -> bool {
        todo!()
    }
}

impl<T, A: Alignment> Rotor<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(_angle: T) -> Self {
        todo!()
    }

    /// Converts a 2D rotor to an angle (in radians) rotating `+X` to `+Y`.
    ///
    /// This assumes the rotor is normalized.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    pub fn to_angle(self) -> T {
        todo!()
    }
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_xy(_angle: T) -> Self {
        todo!()
    }

    /// Creates a rotor from an `angle` (in radians) rotating `+X` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_xz(_angle: T) -> Self {
        todo!()
    }

    /// Creates a rotor from an `angle` (in radians) rotating `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_yz(_angle: T) -> Self {
        todo!()
    }

    /// Creates a rotor from a rotation `axis` and `angle` (in radians), using
    /// the right-hand rule.
    ///
    /// This assumes `axis` is normalized.
    ///
    /// If you are using this to initialize a static rotation, consider using
    /// [`from_rotation_arc`] instead. That function makes it clearer what
    /// direction the rotation happens in, whereas this function requires
    /// remembering the right-hand rule.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `axis` is not normalized.
    ///
    /// [`from_rotation_arc`]: Self::from_rotation_arc
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(_axis: Vector<3, T, A>, _angle: T) -> Self {
        todo!()
    }

    /// Creates a rotor that rotates `scaled_axis.length()` radians around
    /// `scaled_axis.normalize()`, using the right-hand rule.
    ///
    /// If you are using this to initialize a static rotation, consider using
    /// [`from_rotation_arc`] instead. That function makes it clearer what
    /// direction the rotation happens in, whereas this function requires
    /// remembering the right-hand rule.
    ///
    /// [`from_rotation_arc`]: Self::from_rotation_arc
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(_scaled_axis: Vector<3, T, A>) -> Self {
        todo!()
    }

    /// Creates a rotor from an Euler rotation order/sequence and angles (in
    /// radians).
    #[inline]
    #[must_use]
    pub fn from_euler(_order: EulerRot, _a: T, _b: T, _c: T) -> Self {
        todo!()
    }

    /// Creates a rotor from a facing direction and an up direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=forward`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `dir` or `up` are not normalized
    /// - `dir` and `up` are parallel
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(_dir: Vector<3, T, A>, _up: Vector<3, T, A>) -> Self {
        todo!()
    }

    /// Creates a rotor from a facing direction and an up direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=back`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `dir` or `up` are not normalized
    /// - `dir` and `up` are parallel
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(_dir: Vector<3, T, A>, _up: Vector<3, T, A>) -> Self {
        todo!()
    }

    /// Creates a rotor from a camera position, a focal point and an up
    /// direction.
    ///
    /// For a left-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=forward`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `up` is not normalized
    /// - `center` is equal to `eye`
    /// - The resulting forward direction is parallel to `up`
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_lh(
        _eye: Vector<3, T, A>,
        _center: Vector<3, T, A>,
        _up: Vector<3, T, A>,
    ) -> Self {
        todo!()
    }

    /// Creates a rotor from a camera position, a focal point and an up
    /// direction.
    ///
    /// For a right-handed view coordinate system with `+X=right`, `+Y=up` and
    /// `+Z=back`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if:
    ///
    /// - `up` is not normalized
    /// - `center` is equal to `eye`
    /// - The resulting forward direction is parallel to `up`
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_rh(
        _eye: Vector<3, T, A>,
        _center: Vector<3, T, A>,
        _up: Vector<3, T, A>,
    ) -> Self {
        todo!()
    }

    /// Converts the rotor `self` to a normalized rotation axis and an angle (in
    /// radians), using the right-hand rule.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_axis_angle(self) -> (Vector<3, T, A>, T) {
        todo!()
    }

    // Converts the rotor `self` to a rotation axis scaled by an angle (in
    /// radians), using the right-hand rule.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    pub fn to_scaled_axis(self) -> Vector<3, T, A> {
        todo!()
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(self, _order: EulerRot) -> (T, T, T) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_constants() {
        todo!()
    }

    #[test]
    fn test_from_rotation_arc() {
        todo!()
    }

    #[test]
    fn test_from_rotation_arc_colinear() {
        todo!()
    }

    #[test]
    fn test_from_matrix() {
        todo!()
    }

    #[test]
    fn test_from_affine() {
        todo!()
    }

    #[test]
    fn test_from_projective() {
        todo!()
    }

    #[test]
    fn test_is_nan() {
        todo!()
    }

    #[test]
    fn test_is_finite() {
        todo!()
    }

    #[test]
    fn test_inverse() {
        todo!()
    }

    #[test]
    fn test_angle_between() {
        todo!()
    }

    #[test]
    fn test_lerp() {
        todo!()
    }

    #[test]
    fn test_slerp() {
        todo!()
    }

    #[test]
    fn test_rotate_towards() {
        todo!()
    }

    #[test]
    fn test_length() {
        todo!()
    }

    #[test]
    fn test_normalize() {
        todo!()
    }

    #[test]
    fn test_try_normalize() {
        todo!()
    }

    #[test]
    fn test_normalize_or() {
        todo!()
    }

    #[test]
    fn test_normalize_and_length() {
        todo!()
    }

    #[test]
    fn test_is_normalized() {
        todo!()
    }

    #[test]
    fn test_abs_diff_eq() {
        todo!()
    }

    #[test]
    fn test_from_angle() {
        todo!()
    }

    #[test]
    fn test_to_angle() {
        todo!()
    }

    #[test]
    fn test_from_rotation_xy() {
        todo!()
    }

    #[test]
    fn test_from_rotation_xz() {
        todo!()
    }

    #[test]
    fn test_from_rotation_yz() {
        todo!()
    }

    #[test]
    fn test_from_axis_angle() {
        todo!()
    }

    #[test]
    fn test_from_scaled_axis() {
        todo!()
    }

    #[test]
    fn test_from_euler() {
        todo!()
    }

    #[test]
    fn test_look_to_lh() {
        todo!()
    }

    #[test]
    fn test_look_to_rh() {
        todo!()
    }

    #[test]
    fn test_look_at_lh() {
        todo!()
    }

    #[test]
    fn test_look_at_rh() {
        todo!()
    }

    #[test]
    fn test_to_axis_angle() {
        todo!()
    }

    #[test]
    fn test_to_scaled_axis() {
        todo!()
    }

    #[test]
    fn test_to_euler() {
        todo!()
    }
}
