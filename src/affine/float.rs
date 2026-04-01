use crate::{Affine, Alignment, Length, Scalar, SupportedLength, utils::PrimitiveFloat};

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

#[cfg(test)]
mod tests {
    use crate::{Affine, Affine2, Vec2, utils::for_parameters};

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
}
