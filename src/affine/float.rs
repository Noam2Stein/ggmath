#[cfg(backend)]
use crate::EulerRot;
use crate::{
    Affine, Alignment, Length, Matrix, Quaternion, Scalar, SupportedLength, Vector,
    utils::PrimitiveFloat,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
{
    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let normal = Affine2::from_columns(&[
    ///     Vec2::new(1.0, 0.0),
    ///     Vec2::new(0.0, 1.0),
    ///     Vec2::new(2.0, 2.0),
    /// ]);
    /// let nan = Affine2::from_columns(&[
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
    /// let finite = Affine2::from_columns(&[
    ///     Vec2::new(1.0, 0.0),
    ///     Vec2::new(0.0, 1.0),
    ///     Vec2::new(2.0, 2.0),
    /// ]);
    /// let infinite = Affine2::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the determinant is `0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn inverse(&self) -> Self {
        let submatrix = self.matrix.inverse();
        let translation = -(submatrix * self.translation);

        Self::from_submatrix_translation(submatrix, translation)
    }

    /// Returns the inverse of `self` or `None` if `self` is not invertable.
    #[inline]
    #[must_use]
    pub fn try_inverse(&self) -> Option<Self> {
        let submatrix = self.matrix.try_inverse()?;
        let translation = -(submatrix * self.translation);

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
        self.matrix.abs_diff_eq(&other.matrix, max_abs_diff)
            && self
                .translation
                .abs_diff_eq(other.translation, max_abs_diff)
    }
}

#[expect(private_bounds)]
impl<T, A: Alignment> Affine<2, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Creates an affine transform containing a rotation of `angle`
    /// (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_angle(angle: T) -> Self {
        Self::from_submatrix(Matrix::<2, T, A>::from_angle(angle))
    }

    /// Creates an affine transform containing a rotation of `angle`
    /// (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_angle_translation(angle: T, translation: Vector<2, T, A>) -> Self {
        Self::from_submatrix_translation(Matrix::<2, T, A>::from_angle(angle), translation)
    }

    /// Creates an affine transform containing a non-uniform `scale` and
    /// rotation of `angle` (in radians).
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_scale_angle(scale: Vector<2, T, A>, angle: T) -> Self {
        Self::from_submatrix(Matrix::<2, T, A>::from_scale_angle(scale, angle))
    }

    /// Creates an affine transform containing a non-uniform `scale`, rotation
    /// of `angle` (in radians) and `translation`.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
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

    /// Returns the `scale`, `angle` and `translation` of `self`.
    ///
    /// `self` must be reversible and not contain shearing. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the determinant of `self` is zero.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_angle_translation(&self) -> (Vector<2, T, A>, T, Vector<2, T, A>) {
        let determinant = self.matrix.determinant();

        #[cfg(assertions)]
        assert!(determinant != T::ZERO);

        let scale = Vector::<2, T, A>::new(
            self.matrix.x_axis.length() * determinant.signum(),
            self.matrix.y_axis.length(),
        );

        let angle = (-self.matrix.y_axis.x).atan2(self.matrix.y_axis.y);

        (scale, angle, self.translation)
    }
}

#[expect(private_bounds)]
impl<T, A: Alignment> Affine<3, T, A>
where
    T: Scalar + PrimitiveFloat,
{
    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the x axis.
    ///
    /// This rotates `+Y` to `+Z`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_x(angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_rotation_x(angle))
    }

    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the y axis.
    ///
    /// This rotates `+Z` to `+X`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_y(angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_rotation_y(angle))
    }

    /// Creates an affine transform containing a 3D rotation from `angle` (in
    /// radians) around the z axis.
    ///
    /// This rotates `+X` to `+Y`.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    pub fn from_rotation_z(angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_rotation_z(angle))
    }

    /// Creates an affine transform containing a 3D rotation from a quaternion.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `axis` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_axis_angle(axis: Vector<3, T, A>, angle: T) -> Self {
        Self::from_submatrix(Matrix::<3, T, A>::from_axis_angle(axis, angle))
    }

    /// Creates an affine transform containing a rotation from an Euler rotation
    /// order/sequence and angles (in radians).
    #[cfg(backend)]
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
    /// When assertions are enabled (see the crate documentation):
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
    /// When assertions are enabled (see the crate documentation):
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
    /// When assertions are enabled (see the crate documentation):
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_lh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        #[cfg(assertions)]
        assert!(dir.is_normalized());
        #[cfg(assertions)]
        assert!(up.is_normalized());

        let forward = dir;
        let right = up.cross(forward).normalize();
        let up = forward.cross(right);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `dir` or `up` are not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_to_rh(eye: Vector<3, T, A>, dir: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        #[cfg(assertions)]
        assert!(dir.is_normalized());
        #[cfg(assertions)]
        assert!(up.is_normalized());

        let forward = dir;
        let right = forward.cross(up).normalize();
        let up = right.cross(forward);

        Self::from_columns(&[
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
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `up` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_lh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::look_to_lh(eye, (center - eye).normalize(), up)
    }

    /// Creates a right-handed view transform from a camera position, a focal
    /// point and an up direction.
    ///
    /// For a view coordinate system with `+X=right`, `+Y=up` and `+Z=back`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if `up` is not normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn look_at_rh(eye: Vector<3, T, A>, center: Vector<3, T, A>, up: Vector<3, T, A>) -> Self {
        Self::look_to_rh(eye, (center - eye).normalize(), up)
    }

    /// Returns the Euler angles forming `self` for the given Euler rotation
    /// order/sequence.
    ///
    /// `self` must not contain any non-rotation transformations, excluding
    /// translation. Otherwise the result is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if any column of `self`, excluding the translation column, is not
    /// normalized.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_euler(&self, order: EulerRot) -> (T, T, T) {
        self.matrix.to_euler(order)
    }

    /// Returns the `scale`, `rotation` and `translation` of `self`.
    ///
    /// `self` must be reversible and not contain shearing. Otherwise the result
    /// is unspecified.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the determinant of `self` is zero.
    #[cfg(backend)]
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn to_scale_rotation_translation(
        &self,
    ) -> (Vector<3, T, A>, Quaternion<T, A>, Vector<3, T, A>) {
        self.to_matrix().to_scale_rotation_translation()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        Affine, Affine2, Matrix, Quaternion, Vec2, Vector,
        utils::{assert_float_eq, assert_panic, assert_panic_float_eq, for_parameters},
    };

    #[test]
    fn test_is_nan() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z, w, a, b, c, d| {
            let [x, y, z, w, a, b, c, d] =
                [x, y, z, w, a, b, c, d].map(|b| if b { T::NAN } else { 1.0 });

            let affine = Affine::<2, T, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_eq!(
                affine.is_nan(),
                affine.as_columns().iter().any(|column| column.is_nan())
            );

            let affine =
                Affine::<3, T, A>::from_column_array(&[x, y, z, w, x, y, z, w, a, b, c, d]);
            assert_eq!(
                affine.is_nan(),
                affine.as_columns().iter().any(|column| column.is_nan())
            );

            let affine = Affine::<4, T, A>::from_column_array(&[
                x, y, z, w, x, y, z, w, x, y, z, w, x, y, z, w, a, b, c, d,
            ]);
            assert_eq!(
                affine.is_nan(),
                affine.as_columns().iter().any(|column| column.is_nan())
            );
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z, w, a, b, c, d| {
            let [x, y, z, w, a, b, c, d] =
                [x, y, z, w, a, b, c, d].map(|b| if b { T::INFINITY } else { 1.0 });

            let affine = Affine::<2, T, A>::from_column_array(&[x, y, z, w, a, b]);
            assert_eq!(
                affine.is_finite(),
                affine.as_columns().iter().all(|column| column.is_finite())
            );

            let affine =
                Affine::<3, T, A>::from_column_array(&[x, y, z, w, x, y, z, w, a, b, c, d]);
            assert_eq!(
                affine.is_finite(),
                affine.as_columns().iter().all(|column| column.is_finite())
            );

            let affine = Affine::<4, T, A>::from_column_array(&[
                x, y, z, w, x, y, z, w, x, y, z, w, x, y, z, w, a, b, c, d,
            ]);
            assert_eq!(
                affine.is_finite(),
                affine.as_columns().iter().all(|column| column.is_finite())
            );
        });
    }

    #[test]
    fn test_inverse() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 10000.0
                || y.abs() > 10000.0
                || z.abs() > 10000.0
            {
                return;
            }

            let affine = Affine::<2, T, A>::from_column_array(&[x, y, z, w, a, b]);
            if affine.matrix.determinant() != 0.0 {
                let tol = (affine
                    .matrix
                    .determinant()
                    .abs()
                    .log2()
                    .abs()
                    .exp2()
                    .powi(2)
                    + affine.translation.length_squared())
                    * 1e-5;

                assert_float_eq!(
                    affine * affine.inverse(),
                    Affine::IDENTITY,
                    abs <= Affine::<2, T, A>::from_column_array(&[tol; 6]),
                    0.0 = -0.0
                );
            } else if cfg!(assertions) {
                assert_panic!(affine.inverse());
            }

            let affine =
                Affine::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            if affine.matrix.determinant() != 0.0 {
                let tol = (affine
                    .matrix
                    .determinant()
                    .abs()
                    .log2()
                    .abs()
                    .exp2()
                    .powi(2)
                    + affine.translation.length_squared())
                    * 1e-5;

                assert_float_eq!(
                    affine * affine.inverse(),
                    Affine::IDENTITY,
                    abs <= Affine::<3, T, A>::from_column_array(&[tol; 12]),
                    0.0 = -0.0
                );
            } else if cfg!(assertions) {
                assert_panic!(affine.inverse());
            }
        });
    }

    #[test]
    fn test_try_inverse() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, T, A>::from_column_array(&[x, y, z, w, a, b]);
            if let Some(inverse) = affine.try_inverse() {
                assert_float_eq!(affine.inverse(), inverse);
            } else if cfg!(assertions) {
                assert_panic!(affine.inverse());
            }

            let affine =
                Affine::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            if let Some(inverse) = affine.try_inverse() {
                assert_float_eq!(affine.inverse(), inverse);
            } else if cfg!(assertions) {
                assert_panic!(affine.inverse());
            }
        });
    }

    #[test]
    fn test_inverse_or() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, T, A>::from_column_array(&[x, y, z, w, a, b]);
            if let Some(inverse) = affine.try_inverse() {
                assert_float_eq!(affine.inverse_or(&Affine::NAN), inverse);
            } else {
                assert_float_eq!(affine.inverse_or(&Affine::NAN), Affine::NAN);
            }

            let affine =
                Affine::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            if let Some(inverse) = affine.try_inverse() {
                assert_float_eq!(affine.inverse_or(&Affine::NAN), inverse);
            } else {
                assert_float_eq!(affine.inverse_or(&Affine::NAN), Affine::NAN);
            }
        });
    }

    #[test]
    fn test_inverse_or_zero() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];

            let affine = Affine::<2, T, A>::from_column_array(&[x, y, z, w, a, b]);
            if let Some(inverse) = affine.try_inverse() {
                assert_float_eq!(affine.inverse_or_zero(), inverse);
            } else {
                assert_float_eq!(affine.inverse_or_zero(), Affine::ZERO);
            }

            let affine =
                Affine::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e, f, g, h]);
            if let Some(inverse) = affine.try_inverse() {
                assert_float_eq!(affine.inverse_or_zero(), inverse);
            } else {
                assert_float_eq!(affine.inverse_or_zero(), Affine::ZERO);
            }
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat| {
            assert!(
                Affine2::<T>::from_columns(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_columns(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.05),
                        Vec2::new(4.1, 4.9)
                    ]),
                    0.125
                )
            );
            assert!(
                !Affine2::<T>::from_columns(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_columns(&[
                        Vec2::new(0.1, 0.9),
                        Vec2::new(1.95, 3.5),
                        Vec2::new(4.1, 4.9)
                    ]),
                    0.125
                )
            );
            assert!(
                !Affine2::<T>::from_columns(&[
                    Vec2::new(0.0, 1.0),
                    Vec2::new(2.0, 3.0),
                    Vec2::new(4.0, 5.0)
                ])
                .abs_diff_eq(
                    &Affine2::<T>::from_columns(&[
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
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            assert_float_eq!(
                Affine::<2, T, A>::from_angle(z).transform_point(Vector::<2, T, A>::new(x, y)),
                Vector::<2, T, A>::new(x, y).rotate(z),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_angle() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<2, T, A>::new(x, y);
            assert_float_eq!(
                Affine::<2, T, A>::from_scale_angle(scale, z),
                Affine::<2, T, A>::from_angle(z) * Affine::<2, T, A>::from_scale(scale),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_angle_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let translation = Vector::<2, T, A>::new(x, y);
            assert_float_eq!(
                Affine::<2, T, A>::from_angle_translation(z, translation),
                Affine::<2, T, A>::from_translation(translation) * Affine::<2, T, A>::from_angle(z),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_scale_angle_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<2, T, A>::new(x, y);
            let translation = Vector::<2, T, A>::new(x + 1.0, y + 2.0);
            assert_float_eq!(
                Affine::<2, T, A>::from_scale_angle_translation(scale, z, translation),
                Affine::<2, T, A>::from_translation(translation)
                    * Affine::<2, T, A>::from_angle(z)
                    * Affine::<2, T, A>::from_scale(scale),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_to_scale_angle_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a] = [x * 0.3 + 0.5, x + 1.0];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 1e5
                || y.abs() > 1e5
                || z.abs() > 1e5
            {
                return;
            }

            let affine = Affine::<2, T, A>::from_scale_angle_translation(
                Vector::<2, T, A>::new(x, y),
                z,
                Vector::<2, T, A>::new(w, a),
            );
            if affine.matrix.determinant() != 0.0 {
                assert_float_eq!(
                    affine.to_scale_angle_translation(),
                    affine.to_matrix().to_scale_angle_translation()
                );
            } else if cfg!(assertions) {
                assert_panic!(affine.to_scale_angle_translation());
            }
        });
    }

    #[test]
    fn test_from_rotation_x() {
        for_parameters!(|T: PrimitiveFloat, A, x| {
            assert_float_eq!(
                Affine::<3, T, A>::from_rotation_x(x).to_matrix(),
                Matrix::<4, T, A>::from_rotation_x(x),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_y() {
        for_parameters!(|T: PrimitiveFloat, A, x| {
            assert_float_eq!(
                Affine::<3, T, A>::from_rotation_y(x).to_matrix(),
                Matrix::<4, T, A>::from_rotation_y(x),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_rotation_z() {
        for_parameters!(|T: PrimitiveFloat, A, x| {
            assert_float_eq!(
                Affine::<3, T, A>::from_rotation_z(x).to_matrix(),
                Matrix::<4, T, A>::from_rotation_z(x),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_quat() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.midpoint(y);

            if let Some(normalized) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                let quat = Quaternion::from_vec(normalized);
                assert_float_eq!(
                    Affine::<3, T, A>::from_quat(quat).to_matrix(),
                    Matrix::<4, T, A>::from_quat(quat),
                    0.0 = -0.0
                );
            }

            let invalid_quat = Quaternion::new(x, y, z, w);
            if cfg!(assertions) && !invalid_quat.to_vec().is_normalized() {
                assert_panic!(Affine::<3, T, A>::from_quat(invalid_quat));
            }
        });
    }

    #[test]
    fn test_from_axis_angle() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.midpoint(y);

            if let Some(axis) = Vector::<3, T, A>::new(x, y, z).try_normalize() {
                assert_float_eq!(
                    Affine::<3, T, A>::from_axis_angle(axis, w).to_matrix(),
                    Matrix::<4, T, A>::from_axis_angle(axis, w),
                    0.0 = -0.0
                );
            }

            let invalid_axis = Vector::<3, T, A>::new(x, y, z);
            if cfg!(assertions) && !invalid_axis.is_normalized() {
                assert_panic!(Affine::<3, T, A>::from_axis_angle(invalid_axis, w));
            }
        });
    }

    #[test]
    fn test_from_euler() {
        for_parameters!(|T: PrimitiveFloat, A, order, x, y, z| {
            assert_float_eq!(
                Affine::<3, T, A>::from_euler(order, x, y, z).to_matrix(),
                Matrix::<4, T, A>::from_euler(order, x, y, z)
            );
        });
    }

    #[test]
    fn test_from_scale_rotation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.midpoint(y);

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<3, T, A>::new(x, y, z);
            if let Some(normalized) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                let rotation = Quaternion::from_vec(normalized);
                assert_float_eq!(
                    Affine::<3, T, A>::from_scale_rotation(scale, rotation),
                    Affine::<3, T, A>::from_quat(rotation) * Affine::<3, T, A>::from_scale(scale),
                    0.0 = -0.0
                );
            }

            let invalid_rotation = Quaternion::new(x, y, z, w);
            if cfg!(assertions) && !invalid_rotation.to_vec().is_normalized() {
                assert_panic!(Affine::<3, T, A>::from_scale_rotation(
                    scale,
                    invalid_rotation
                ));
            }
        });
    }

    #[test]
    fn test_from_rotation_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.midpoint(y);

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let translation = Vector::<3, T, A>::new(x, y, z);
            if let Some(normalized) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                let rotation = Quaternion::from_vec(normalized);
                assert_float_eq!(
                    Affine::<3, T, A>::from_rotation_translation(rotation, translation),
                    Affine::<3, T, A>::from_translation(translation)
                        * Affine::<3, T, A>::from_quat(rotation),
                    0.0 = -0.0
                );
            }

            let invalid_rotation = Quaternion::new(x, y, z, w);
            if cfg!(assertions) && !invalid_rotation.to_vec().is_normalized() {
                assert_panic!(Affine::<3, T, A>::from_rotation_translation(
                    invalid_rotation,
                    translation
                ));
            }
        });
    }

    #[test]
    fn test_from_scale_rotation_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.midpoint(y);

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<3, T, A>::new(y, z, x);
            let translation = Vector::<3, T, A>::new(x, y, z);
            if let Some(normalized) = Vector::<4, T, A>::new(x, y, z, w).try_normalize() {
                let rotation = Quaternion::from_vec(normalized);
                assert_float_eq!(
                    Affine::<3, T, A>::from_scale_rotation_translation(
                        scale,
                        rotation,
                        translation
                    ),
                    Affine::<3, T, A>::from_translation(translation)
                        * Affine::<3, T, A>::from_quat(rotation)
                        * Affine::<3, T, A>::from_scale(scale),
                    0.0 = -0.0
                );
            }

            let invalid_rotation = Quaternion::new(x, y, z, w);
            if cfg!(assertions) && !invalid_rotation.to_vec().is_normalized() {
                assert_panic!(Affine::<3, T, A>::from_scale_rotation_translation(
                    scale,
                    invalid_rotation,
                    translation
                ));
            }
        });
    }

    #[test]
    fn test_look_to_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let Some(dir) = Vector::<3, T, A>::new(x, y, z).try_normalize() else {
                return;
            };

            let eye = dir * 0.3 + dir.yzx().with_z(0.6);
            let up = (dir * 0.4 + dir.zxy().with_z(0.3)).normalize();

            assert_float_eq!(
                Affine::<3, T, A>::look_to_lh(eye, dir, up).to_matrix(),
                Matrix::<4, T, A>::look_to_lh(eye, dir, up)
            );

            let xyz = Vector::<3, T, A>::new(x, y, z);
            if cfg!(assertions) && !xyz.is_normalized() {
                assert_panic!(Affine::<3, T, A>::look_to_lh(eye, xyz, up));
                assert_panic!(Affine::<3, T, A>::look_to_lh(eye, dir, xyz));
            }
        })
    }

    #[test]
    fn test_look_to_rh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let Some(dir) = Vector::<3, T, A>::new(x, y, z).try_normalize() else {
                return;
            };

            let eye = dir * 0.3 + dir.yzx().with_z(0.6);
            let up = (dir * 0.4 + dir.zxy().with_z(0.3)).normalize();

            assert_float_eq!(
                Affine::<3, T, A>::look_to_rh(eye, dir, up).to_matrix(),
                Matrix::<4, T, A>::look_to_rh(eye, dir, up)
            );

            let xyz = Vector::<3, T, A>::new(x, y, z);
            if cfg!(assertions) && !xyz.is_normalized() {
                assert_panic!(Affine::<3, T, A>::look_to_rh(eye, xyz, up));
                assert_panic!(Affine::<3, T, A>::look_to_rh(eye, dir, xyz));
            }
        })
    }

    #[test]
    fn test_look_at_lh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let eye = Vector::<3, T, A>::new(x, y, z);
            let center = eye * 0.6 + eye.yzx();
            let Some(up) = (eye * 0.4 + center.zxy().with_z(0.6)).try_normalize() else {
                return;
            };

            assert_panic_float_eq!(
                Affine::<3, T, A>::look_at_lh(eye, center, up).to_matrix(),
                Matrix::<4, T, A>::look_at_lh(eye, center, up)
            );

            let xyz = Vector::<3, T, A>::new(x, y, z);
            if cfg!(assertions) && !xyz.is_normalized() {
                assert_panic!(Affine::<3, T, A>::look_at_lh(eye, center, xyz));
            }
        })
    }

    #[test]
    fn test_look_at_rh() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let eye = Vector::<3, T, A>::new(x, y, z);
            let center = eye * 0.6 + eye.yzx();
            let Some(up) = (eye * 0.4 + center.zxy().with_z(0.6)).try_normalize() else {
                return;
            };

            assert_panic_float_eq!(
                Affine::<3, T, A>::look_at_rh(eye, center, up).to_matrix(),
                Matrix::<4, T, A>::look_at_rh(eye, center, up)
            );

            let xyz = Vector::<3, T, A>::new(x, y, z);
            if cfg!(assertions) && !xyz.is_normalized() {
                assert_panic!(Affine::<3, T, A>::look_at_rh(eye, center, xyz));
            }
        })
    }

    #[test]
    fn test_to_euler() {
        for_parameters!(|T: PrimitiveFloat, A, order, x, y, z| {
            let affine = Affine::<3, T, A>::from_translation(Vector::<3, T, A>::new(x, y, z))
                * Affine::<3, T, A>::from_euler(order, x, y, z);

            assert_panic_float_eq!(affine.to_euler(order), affine.to_matrix().to_euler(order));

            let scale = Vector::<3, T, A>::new(x, y, z);
            let invalid_affine = affine * Affine::from_scale(scale);
            assert_panic_float_eq!(
                invalid_affine.to_euler(order),
                invalid_affine.to_matrix().to_euler(order)
            );
        });
    }

    #[test]
    fn test_to_scale_rotation_translation() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b, c] = [x * 0.3 + 0.5, x + 1.0, y + 2.0, z + 3.0];

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 100000.0
                || y.abs() > 100000.0
                || z.abs() > 100000.0
            {
                return;
            }

            let scale = Vector::<3, T, A>::new(x, y, z);
            let rotation = Quaternion::from_vec(Vector::<4, T, A>::new(x, y, z, w).normalize());
            let translation = Vector::<3, T, A>::new(a, b, c);

            let affine =
                Affine::<3, T, A>::from_scale_rotation_translation(scale, rotation, translation);

            assert_panic_float_eq!(
                affine.to_scale_rotation_translation(),
                affine.to_matrix().to_scale_rotation_translation()
            );
        });
    }
}
