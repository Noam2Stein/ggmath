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
        Self::from_matrix_translation(Matrix::<N, T, A>::NAN, Vector::<N, T, A>::NAN);

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
        self.matrix.is_nan() || self.translation.is_nan()
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
        self.matrix.is_finite() && self.translation.is_finite()
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
        let matrix = self.matrix.inverse();
        let translation = -self.translation * matrix;

        Self::from_matrix_translation(matrix, translation)
    }

    /// Returns the inverse of `self` or `None` if `self` is not invertable.
    #[inline]
    #[must_use]
    pub fn try_inverse(&self) -> Option<Self> {
        let matrix = self.matrix.try_inverse()?;
        let translation = -self.translation * matrix;

        Some(Self::from_matrix_translation(matrix, translation))
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
        self.matrix.abs_diff_eq(&other.matrix, max_abs_diff)
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
        Self::from_matrix(Matrix::<2, T, A>::from_angle(angle))
    }

    /// Creates an affine transform containing a rotation of `angle`
    /// (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: T, translation: Vector<2, T, A>) -> Self {
        Self::from_matrix_translation(Matrix::<2, T, A>::from_angle(angle), translation)
    }

    /// Creates an affine transform containing a non-uniform `scale` and
    /// rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        Self::from_matrix(Matrix::<2, T, A>::from_scale_angle(scale, angle))
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
        Self::from_matrix_translation(
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
        self.matrix.to_scale_angle()
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
        let (scale, angle) = self.matrix.to_scale_angle();
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
        Self::from_matrix(Matrix::<3, T, A>::from_rotation_x(angle))
    }

    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        Self::from_matrix(Matrix::<3, T, A>::from_rotation_y(angle))
    }

    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        Self::from_matrix(Matrix::<3, T, A>::from_rotation_z(angle))
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
        Self::from_matrix(Matrix::<3, T, A>::from_quat(quat))
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
        Self::from_matrix(Matrix::<3, T, A>::from_axis_angle(axis, angle))
    }

    /// Creates an affine transform containing a rotation from an Euler rotation
    /// order/sequence and angles (in radians).
    #[inline]
    #[must_use]
    pub fn from_euler(order: EulerRot, a: T, b: T, c: T) -> Self {
        Self::from_matrix(Matrix::<3, T, A>::from_euler(order, a, b, c))
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
        Self::from_matrix(Matrix::<3, T, A>::from_scale_rotation(scale, rotation))
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
        Self::from_matrix_translation(Matrix::<3, T, A>::from_quat(rotation), translation)
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
        Self::from_matrix_translation(
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
        self.matrix.to_euler(order)
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
        self.matrix.to_scale_rotation()
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
        let (scale, rotation) = self.matrix.to_scale_rotation();
        (scale, rotation, self.translation)
    }
}
