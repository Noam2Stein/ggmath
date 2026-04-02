use crate::{Affine, Alignment, Length, Scalar, SupportedLength, utils::PrimitiveFloat};
#[cfg(backend)]
use crate::{Matrix, Vector};

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

#[cfg(test)]
mod tests {
    use crate::{
        Affine, Affine2, Vec2, Vector,
        utils::{assert_float_eq, assert_panic, for_parameters},
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
}
