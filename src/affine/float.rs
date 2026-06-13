use crate::{
    Affine, Alignment, EulerRot, Length, Matrix, PrimitiveFloat, Quaternion, SupportedLength,
    Vector,
};

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: PrimitiveFloat,
{
    /// An affine transform with all elements set to NaN (Not a Number).
    pub const NAN: Self =
        Self::from_submatrix_translation(Matrix::<N, T, A>::NAN, Vector::<N, T, A>::NAN);

    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let normal = Affine2::from_rows(&[
    ///     Vec2::new(1.0, 0.0),
    ///     Vec2::new(0.0, 1.0),
    ///     Vec2::new(2.0, 2.0),
    /// ]);
    /// let nan = Affine2::from_rows(&[
    ///     Vec2::new(1.0, 0.0),
    ///     Vec2::new(0.0, f32::NAN),
    ///     Vec2::new(2.0, 2.0),
    /// ]);
    ///
    /// assert!(!normal.is_nan());
    /// assert!(nan.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        self.submatrix.is_nan() || self.translation.is_nan()
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let finite = Affine2::from_rows(&[
    ///     Vec2::new(1.0, 0.0),
    ///     Vec2::new(0.0, 1.0),
    ///     Vec2::new(2.0, 2.0),
    /// ]);
    /// let infinite = Affine2::from_rows(&[
    ///     Vec2::new(1.0, 0.0),
    ///     Vec2::new(0.0, f32::INFINITY),
    ///     Vec2::new(2.0, 2.0),
    /// ]);
    ///
    /// assert!(finite.is_finite());
    /// assert!(!infinite.is_finite());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        self.submatrix.is_finite() && self.translation.is_finite()
    }

    /// Returns the inverse of `self`.
    ///
    /// If `self` is not invertable the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the determinant is `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(&self) -> Self {
        let submatrix = self.submatrix.inverse();
        let translation = -self.translation * submatrix;

        Self::from_submatrix_translation(submatrix, translation)
    }

    /// Returns the inverse of `self` or `None` if `self` is not invertable.
    #[inline]
    #[must_use]
    pub fn try_inverse(&self) -> Option<Self> {
        let submatrix = self.submatrix.try_inverse()?;
        let translation = -self.translation * submatrix;

        Some(Self::from_submatrix_translation(submatrix, translation))
    }

    /// Returns the inverse of `self` or `fallback` if `self` is not invertable.
    #[inline]
    #[must_use]
    pub fn inverse_or(&self, fallback: &Self) -> Self {
        self.try_inverse().unwrap_or(*fallback)
    }

    /// Returns the inverse of `self` or the zero transform if `self` is not
    /// invertable.
    #[inline]
    #[must_use]
    pub fn inverse_or_zero(&self) -> Self {
        self.try_inverse().unwrap_or(Self::ZERO)
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two affines that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool {
        self.submatrix.abs_diff_eq(&other.submatrix, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

impl<T, A: Alignment> Affine<2, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates an affine transform containing a rotation of `angle`
    /// (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        Self::from_submatrix(Matrix::<2, T, A>::from_angle(angle))
    }

    /// Creates an affine transform containing a rotation of `angle`
    /// (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: T, translation: Vector<2, T, A>) -> Self {
        Self::from_submatrix_translation(Matrix::<2, T, A>::from_angle(angle), translation)
    }

    /// Creates an affine transform containing a non-uniform `scale` and
    /// rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        Self::from_submatrix(Matrix::<2, T, A>::from_scale_angle(scale, angle))
    }

    /// Creates an affine transform containing a non-uniform `scale`, rotation
    /// of `angle` (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle_translation(
        scale: Vector<2, T, A>,
        angle: T,
        translation: Vector<2, T, A>,
    ) -> Self {
        Self::from_submatrix_translation(
            Matrix::<2, T, A>::from_scale_angle(scale, angle),
            translation,
        )
    }

    /// Returns the `scale` and `angle` of `self`.
    ///
    /// `self` must be reversible and not contain shearing. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle(&self) -> (Vector<2, T, A>, T) {
        self.submatrix.to_scale_angle()
    }

    /// Returns the `scale`, `angle` and `translation` of `self`.
    ///
    /// `self` must be reversible and not contain shearing. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle_translation(&self) -> (Vector<2, T, A>, T, Vector<2, T, A>) {
        let (scale, angle) = self.submatrix.to_scale_angle();
        (scale, angle, self.translation)
    }
}

impl<T, A: Alignment> Affine<3, T, A>
where
    T: PrimitiveFloat,
{
    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_rotation_x(angle))
    }

    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_rotation_y(angle))
    }

    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_rotation_z(angle))
    }

    /// Creates an affine transform containing a 3D rotation from a quaternion.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if the quaternion is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_quat(quat: Quaternion<T, A>) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_quat(quat))
    }

    /// Creates an affine transform containing a rotation from a rotation `axis`
    /// and `angle` (in radians).
    ///
    /// `axis` must be normalized. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `axis` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_axis_angle(axis, angle))
    }

    /// Creates an affine transform containing a rotation from an Euler rotation
    /// order/sequence and angles (in radians).
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: T, b: T, c: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_euler(order, a, b, c))
    }

    /// Creates an affine transform containing a non-uniform `scale` and a 3D
    /// `rotation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation(scale: Vector<3, T, A>, rotation: Quaternion<T, A>) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_scale_rotation(scale, rotation))
    }

    /// Creates an affine transform containing a 3D `rotation` and
    /// `translation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_rotation_translation(
        rotation: Quaternion<T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        Self::from_submatrix_translation(Matrix::<3, T, A>::from_quat(rotation), translation)
    }

    /// Creates an affine transform containing a non-uniform `scale`, a 3D
    /// `rotation` and `translation`.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `rotation` is not normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_scale_rotation_translation(
        scale: Vector<3, T, A>,
        rotation: Quaternion<T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        Self::from_submatrix_translation(
            Matrix::<3, T, A>::from_scale_rotation(scale, rotation),
            translation,
        )
    }

    /// Creates a left-handed view transform from a camera position, a facing
    /// direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
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
    pub fn look_to_lh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            dir.is_normalized() && up.is_normalized(),
            "directions are not normalized: look_to_lh({eye:?}, {dir:?}, {up:?})"
        );

        let forward = dir;

        let right = up.cross(forward);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "dir and up are parallel: look_to_lh({eye:?}, {dir:?}, {up:?})"
        );

        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, forward.x),
            Vector::<3, T, A>::new(right.y, up.y, forward.y),
            Vector::<3, T, A>::new(right.z, up.z, forward.z),
            Vector::<3, T, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward)),
        ])
    }

    /// Creates a right-handed view transform from a camera position, a facing
    /// direction and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
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
    pub fn look_to_rh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            dir.is_normalized() && up.is_normalized(),
            "directions are not normalized: look_to_rh({eye:?}, {dir:?}, {up:?})"
        );

        let forward = dir;

        let right = forward.cross(up);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "dir and up are parallel: look_to_rh({eye:?}, {dir:?}, {up:?})"
        );

        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, -forward.x),
            Vector::<3, T, A>::new(right.y, up.y, -forward.y),
            Vector::<3, T, A>::new(right.z, up.z, -forward.z),
            Vector::<3, T, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward)),
        ])
    }

    /// Creates a left-handed view transform from a camera position, a focal
    /// point and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=forward`.
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
    pub fn look_at_lh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            up.is_normalized(),
            "up is not normalized: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let forward = center - eye;
        let forward = forward / forward.length();
        debug_assert!(
            forward.is_finite() && forward != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let right = up.cross(forward);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_lh({eye:?}, {center:?}, {up:?})"
        );

        let up = forward.cross(right);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, forward.x),
            Vector::<3, T, A>::new(right.y, up.y, forward.y),
            Vector::<3, T, A>::new(right.z, up.z, forward.z),
            Vector::<3, T, A>::new(-eye.dot(right), -eye.dot(up), -eye.dot(forward)),
        ])
    }

    /// Creates a right-handed view transform from a camera position, a focal
    /// point and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
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
    pub fn look_at_rh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        debug_assert!(
            up.is_normalized(),
            "up is not normalized: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let forward = center - eye;
        let forward = forward / forward.length();
        debug_assert!(
            forward.is_finite() && forward != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let right = forward.cross(up);
        let right = right / right.length();
        debug_assert!(
            right.is_finite() && right != Vector::ZERO,
            "(center - eye) and up are parallel: look_at_rh({eye:?}, {center:?}, {up:?})"
        );

        let up = right.cross(forward);

        Self::from_rows(&[
            Vector::<3, T, A>::new(right.x, up.x, -forward.x),
            Vector::<3, T, A>::new(right.y, up.y, -forward.y),
            Vector::<3, T, A>::new(right.z, up.z, -forward.z),
            Vector::<3, T, A>::new(-eye.dot(right), -eye.dot(up), eye.dot(forward)),
        ])
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// `self` must not contain any non-rotation transformations, excluding
    /// translation. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains scaling or shearing.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        self.submatrix.to_euler(order)
    }

    /// Returns the `scale` and `rotation` of `self`.
    ///
    /// `self` must be reversible and not contain shearing. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation(&self) -> (Vector<3, T, A>, Quaternion<T, A>) {
        self.submatrix.to_scale_rotation()
    }

    /// Returns the `scale`, `rotation` and `translation` of `self`.
    ///
    /// `self` must be reversible and not contain shearing. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When debug assertions are enabled:
    ///
    /// Panics if `self` contains shearing or the determinant of `self` is zero.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation_translation(
        &self,
    ) -> (Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>) {
        let (scale, rotation) = self.submatrix.to_scale_rotation();
        (scale, rotation, self.translation)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Affine, Affine2, EulerRot, Matrix, Quaternion, Vec2, Vector,
        utils::{assert_debug_panic, assert_panic_test_eq, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_constants() {
        for_types!(|N, T: PrimitiveFloat, A| {
            assert_test_eq!(
                Affine::<N, T, A>::NAN,
                Affine::from_submatrix_translation(Matrix::<N, T, A>::NAN, Vector::<N, T, A>::NAN)
            );
        });
    }

    #[test]
    fn test_is_nan() {
        for_types!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let nan = Vector::<2, T, A>::NAN;
            assert!(!Affine::<2, T, A>::from_rows(&[one; 3]).is_nan());
            assert!(Affine::<2, T, A>::from_rows(&[nan, one, one]).is_nan());
            assert!(Affine::<2, T, A>::from_rows(&[one, nan, one]).is_nan());
            assert!(Affine::<2, T, A>::from_rows(&[one, one, nan]).is_nan());
            assert!(Affine::<2, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<3, T, A>::NAN;
            assert!(!Affine::<3, T, A>::from_rows(&[one; 4]).is_nan());
            assert!(Affine::<3, T, A>::from_rows(&[nan, one, one, one]).is_nan());
            assert!(Affine::<3, T, A>::from_rows(&[one, nan, one, one]).is_nan());
            assert!(Affine::<3, T, A>::from_rows(&[one, one, nan, one]).is_nan());
            assert!(Affine::<3, T, A>::from_rows(&[one, one, one, nan]).is_nan());
            assert!(Affine::<3, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::<4, T, A>::NAN;
            assert!(!Affine::<4, T, A>::from_rows(&[one; 5]).is_nan());
            assert!(Affine::<4, T, A>::from_rows(&[nan, one, one, one, one]).is_nan());
            assert!(Affine::<4, T, A>::from_rows(&[one, nan, one, one, one]).is_nan());
            assert!(Affine::<4, T, A>::from_rows(&[one, one, nan, one, one]).is_nan());
            assert!(Affine::<4, T, A>::from_rows(&[one, one, one, nan, one]).is_nan());
            assert!(Affine::<4, T, A>::from_rows(&[one, one, one, one, nan]).is_nan());
            assert!(Affine::<4, T, A>::NAN.is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_types!(|T: PrimitiveFloat, A| {
            for non_finite_value in [T::INFINITY, T::NEG_INFINITY, T::NAN] {
                let one = Vector::ONE;
                let non_finite = Vector::splat(non_finite_value);
                assert!(Affine::<2, T, A>::from_rows(&[one; 3]).is_finite());
                assert!(!Affine::<2, T, A>::from_rows(&[non_finite, one, one]).is_finite());
                assert!(!Affine::<2, T, A>::from_rows(&[one, non_finite, one]).is_finite());
                assert!(!Affine::<2, T, A>::from_rows(&[one, one, non_finite]).is_finite());
                assert!(!Affine::<2, T, A>::from_rows(&[non_finite; 3]).is_finite());

                let one = Vector::ONE;
                let non_finite = Vector::splat(non_finite_value);
                assert!(Affine::<3, T, A>::from_rows(&[one; 4]).is_finite());
                assert!(!Affine::<3, T, A>::from_rows(&[non_finite, one, one, one]).is_finite());
                assert!(!Affine::<3, T, A>::from_rows(&[one, non_finite, one, one]).is_finite());
                assert!(!Affine::<3, T, A>::from_rows(&[one, one, non_finite, one]).is_finite());
                assert!(!Affine::<3, T, A>::from_rows(&[one, one, one, non_finite]).is_finite());
                assert!(!Affine::<3, T, A>::from_rows(&[non_finite; 4]).is_finite());

                let one = Vector::ONE;
                let non_finite = Vector::splat(non_finite_value);
                assert!(Affine::<4, T, A>::from_rows(&[one; 5]).is_finite());
                assert!(
                    !Affine::<4, T, A>::from_rows(&[non_finite, one, one, one, one]).is_finite()
                );
                assert!(
                    !Affine::<4, T, A>::from_rows(&[one, non_finite, one, one, one]).is_finite()
                );
                assert!(
                    !Affine::<4, T, A>::from_rows(&[one, one, non_finite, one, one]).is_finite()
                );
                assert!(
                    !Affine::<4, T, A>::from_rows(&[one, one, one, non_finite, one]).is_finite()
                );
                assert!(
                    !Affine::<4, T, A>::from_rows(&[one, one, one, one, non_finite]).is_finite()
                );
                assert!(!Affine::<4, T, A>::from_rows(&[non_finite; 5]).is_finite());
            }
        });
    }

    #[test]
    fn test_inverse() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for affine in random_iter::<Affine<N, T, A>>() {
                if affine.submatrix.determinant() == 0.0 {
                    assert_debug_panic!(affine.inverse());
                }

                if !affine.is_finite()
                    || affine
                        .submatrix
                        .as_rows()
                        .iter()
                        .chain([&affine.translation])
                        .flatten()
                        .any(|x| x.abs() > 1e6)
                    || !(1e-2..=1e2).contains(
                        &(affine.submatrix.determinant()
                            / affine
                                .submatrix
                                .as_rows()
                                .iter()
                                .flatten()
                                .reduce(T::max)
                                .unwrap()
                                .powi(N as i32)),
                    )
                {
                    continue;
                }

                assert_test_eq!(
                    affine * affine.inverse(),
                    Affine::IDENTITY,
                    abs <= affine
                        .submatrix
                        .determinant()
                        .abs()
                        .max(affine.submatrix.determinant().recip().abs())
                        * 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for affine in random_iter::<Affine<N, T, A>>() {
                let Some(try_inverse) = affine.try_inverse() else {
                    assert_debug_panic!(affine.inverse());
                    continue;
                };

                assert_test_eq!(affine.inverse(), try_inverse);
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [affine, fallback] in random_iter::<[Affine<N, T, A>; 2]>() {
                let Some(inverse) = affine.try_inverse() else {
                    assert_test_eq!(affine.inverse_or(&fallback), fallback);
                    continue;
                };

                assert_test_eq!(affine.inverse_or(&fallback), inverse);
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for affine in random_iter::<Affine<N, T, A>>() {
                let Some(inverse) = affine.try_inverse() else {
                    assert_test_eq!(affine.inverse_or_zero(), Affine::ZERO);
                    continue;
                };

                assert_test_eq!(affine.inverse_or_zero(), inverse);
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_types!(|T: PrimitiveFloat| {
            assert!(
                Affine2::<T>::from_rows(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_rows(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.05),
                        Vec2::new(4.1, 4.9)
                    ]),
                    0.125
                )
            );
            assert!(
                !Affine2::<T>::from_rows(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_rows(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.5),
                        Vec2::new(4.1, 4.9)
                    ]),
                    0.125
                )
            );
            assert!(
                !Affine2::<T>::from_rows(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_rows(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.05),
                        Vec2::new(4.6, 4.9)
                    ]),
                    0.125
                )
            );
        });
    }

    #[test]
    fn test_from_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (angle, point) in random_iter::<(T, Vector<2, T, A>)>() {
                assert_test_eq!(
                    Affine::<2, T, A>::from_angle(angle).transform_point(point),
                    point.rotate(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                assert_panic_test_eq!(
                    Affine::<2, T, A>::from_scale_angle(scale, angle),
                    Affine::<2, T, A>::from_submatrix(Matrix::<2, T, A>::from_scale_angle(
                        scale, angle
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (angle, translation) in random_iter::<(T, Vector<2, T, A>)>() {
                assert_test_eq!(
                    Affine::<2, T, A>::from_angle_translation(angle, translation),
                    Affine::<2, T, A>::from_submatrix_translation(
                        Matrix::<2, T, A>::from_angle(angle),
                        translation
                    )
                );
            }
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                assert_test_eq!(
                    Affine::<2, T, A>::from_scale_angle_translation(scale, angle, translation),
                    Affine::<2, T, A>::from_submatrix_translation(
                        Matrix::<2, T, A>::from_scale_angle(scale, angle),
                        translation
                    )
                );
            }
        });
    }

    #[test]
    fn test_to_scale_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                let affine =
                    Affine::<2, T, A>::from_scale_angle_translation(scale, angle, translation);

                assert_panic_test_eq!(affine.to_scale_angle(), affine.submatrix.to_scale_angle());
            }
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, angle, translation) in
                random_iter::<(Vector<2, T, A>, T, Vector<2, T, A>)>()
            {
                let affine =
                    Affine::<2, T, A>::from_scale_angle_translation(scale, angle, translation);

                assert_panic_test_eq!(
                    affine.to_scale_angle_translation(),
                    (
                        affine.submatrix.to_scale_angle().0,
                        affine.submatrix.to_scale_angle().1,
                        affine.translation
                    )
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                assert_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::from_rotation_x(angle)),
                    Matrix::<4, T, A>::from_rotation_x(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                assert_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::from_rotation_y(angle)),
                    Matrix::<4, T, A>::from_rotation_y(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_types!(|T: PrimitiveFloat, A| {
            for angle in random_iter::<T>() {
                assert_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::from_rotation_z(angle)),
                    Matrix::<4, T, A>::from_rotation_z(angle),
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_from_quat() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_panic_test_eq!(
                    Affine::<3, T, A>::from_quat(quat),
                    Affine::from_submatrix(Matrix::<3, T, A>::from_quat(quat))
                );
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_types!(|T: PrimitiveFloat, A| {
            for (axis, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                assert_panic_test_eq!(
                    Affine::<3, T, A>::from_axis_angle(axis, angle),
                    Affine::from_submatrix(Matrix::<3, T, A>::from_axis_angle(axis, angle))
                );
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for [a, b, c] in random_iter::<[T; 3]>() {
                    assert_test_eq!(
                        Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::from_euler(
                            order, a, b, c
                        )),
                        Matrix::<4, T, A>::from_euler(order, a, b, c)
                    );
                }
            }
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation) in random_iter::<(Vector<3, T, A>, Quaternion<T, A>)>() {
                assert_panic_test_eq!(
                    Affine::<3, T, A>::from_scale_rotation(scale, rotation),
                    Affine::from_submatrix(Matrix::<3, T, A>::from_scale_rotation(scale, rotation))
                );
            }
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotation, translation) in random_iter::<(Quaternion<T, A>, Vector<3, T, A>)>() {
                assert_panic_test_eq!(
                    Affine::<3, T, A>::from_rotation_translation(rotation, translation),
                    Affine::<3, T, A>::from_matrix(Matrix::<4, T, A>::from_rotation_translation(
                        rotation,
                        translation
                    ))
                );
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                assert_panic_test_eq!(
                    Affine::<3, T, A>::from_scale_rotation_translation(
                        scale,
                        rotation,
                        translation
                    ),
                    Affine::<3, T, A>::from_matrix(
                        Matrix::<4, T, A>::from_scale_rotation_translation(
                            scale,
                            rotation,
                            translation
                        )
                    )
                );
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, dir, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_to_lh(eye, dir, up)),
                    Matrix::<4, T, A>::look_to_lh(eye, dir, up)
                );

                let dir = dir.normalize_or(Vector::<3, T, A>::Z);
                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_to_lh(eye, dir, up)),
                    Matrix::<4, T, A>::look_to_lh(eye, dir, up)
                );
            }
        });
    }

    #[test]
    fn test_look_to_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, dir, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_to_rh(eye, dir, up)),
                    Matrix::<4, T, A>::look_to_rh(eye, dir, up)
                );

                let dir = dir.normalize_or(Vector::<3, T, A>::Z);
                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_to_rh(eye, dir, up)),
                    Matrix::<4, T, A>::look_to_rh(eye, dir, up)
                );
            }
        });
    }

    #[test]
    fn test_look_at_lh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_at_lh(eye, center, up)),
                    Matrix::<4, T, A>::look_at_lh(eye, center, up)
                );

                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_at_lh(eye, center, up)),
                    Matrix::<4, T, A>::look_at_lh(eye, center, up)
                );
            }
        });
    }

    #[test]
    fn test_look_at_rh() {
        for_types!(|T: PrimitiveFloat, A| {
            for [eye, center, up] in random_iter::<[Vector<3, T, A>; 3]>() {
                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_at_rh(eye, center, up)),
                    Matrix::<4, T, A>::look_at_rh(eye, center, up)
                );

                let up = up.normalize_or(Vector::<3, T, A>::Y);

                assert_panic_test_eq!(
                    Matrix::<4, T, A>::from_affine(&Affine::<3, T, A>::look_at_rh(eye, center, up)),
                    Matrix::<4, T, A>::look_at_rh(eye, center, up)
                );
            }
        });
    }

    #[test]
    fn test_to_euler() {
        for_types!(|T: PrimitiveFloat, A| {
            for order in EulerRot::values() {
                for [a, b, c] in random_iter::<[T; 3]>() {
                    let affine = Affine::<3, T, A>::from_euler(order, a, b, c);

                    assert_panic_test_eq!(
                        affine.to_euler(order),
                        Matrix::<4, T, A>::from_affine(&affine).to_euler(order)
                    );
                }
            }
        });
    }

    #[test]
    fn test_to_scale_rotation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                let affine = Affine::<3, T, A>::from_scale_rotation_translation(
                    scale,
                    rotation,
                    translation,
                );

                assert_panic_test_eq!(
                    affine.to_scale_rotation(),
                    affine.submatrix.to_scale_rotation()
                );
            }
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_types!(|T: PrimitiveFloat, A| {
            for (scale, rotation, translation) in
                random_iter::<(Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>)>()
            {
                let rotation = rotation.normalize_or(Quaternion::IDENTITY).normalize();

                let affine = Affine::<3, T, A>::from_scale_rotation_translation(
                    scale,
                    rotation,
                    translation,
                );

                assert_panic_test_eq!(
                    affine.to_scale_rotation_translation(),
                    (
                        affine.to_scale_rotation().0,
                        affine.to_scale_rotation().1,
                        affine.translation
                    )
                );
            }
        });
    }
}
