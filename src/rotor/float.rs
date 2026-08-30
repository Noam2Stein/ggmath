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
    /// TODO
    pub const NAN: Self = todo!();

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc(_from: Vector<N, T, A>, _to: Vector<N, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_arc_colinear(_from: Vector<N, T, A>, _to: Vector<N, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_matrix(_matrix: &Matrix<N, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_affine(_affine: &Affine<N, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_projective(_projective: &Projective<N, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn is_nan(self) -> bool {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(self) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn angle_between(self, _other: Self) -> T {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn lerp(self, _other: Self, _t: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn slerp(self, _other: Self, _t: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn rotate_towards(self, _target: Self, _max_angle: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn length(self) -> T {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn normalize(self) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn try_normalize(self) -> Option<Self> {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn normalize_or(self, _fallback: Self) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn normalize_and_length(self) -> (Self, T) {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn is_normalized(self) -> bool {
        todo!()
    }

    /// TODO
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
    /// TODO
    #[inline]
    #[must_use]
    pub fn from_angle(_angle: T) -> Self {
        todo!()
    }

    /// TODO
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
    /// TODO
    #[inline]
    #[must_use]
    pub fn from_rotation_xy(_angle: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn from_rotation_xz(_angle: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn from_rotation_yz(_angle: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(_axis: Vector<3, T, A>, _angle: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn from_scaled_axis(_scaled_axis: Vector<3, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn from_euler(_order: EulerRot, _a: T, _b: T, _c: T) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(_dir: Vector<3, T, A>, _up: Vector<3, T, A>) -> Self {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(_dir: Vector<3, T, A>, _up: Vector<3, T, A>) -> Self {
        todo!()
    }

    /// TODO
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

    /// TODO
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

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_axis_angle(self) -> (Vector<3, T, A>, T) {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub fn to_scaled_axis(self) -> Vector<3, T, A> {
        todo!()
    }

    /// TODO
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
