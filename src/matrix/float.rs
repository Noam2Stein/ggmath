use crate::{
    Alignment, Length, Matrix, Scalar, SupportedLength, Vector,
    num_primitive::PrimitiveFloat,
    transmute::{transmute_generic, transmute_ref},
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
{
    /// Returns `true` if any element is NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let normal = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let nan = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, f32::NAN),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    ///
    /// assert!(!normal.is_nan());
    /// assert!(nan.is_nan());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_nan(&self) -> bool {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe { transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self).is_nan() },
            3 => self.column(0).is_nan() || self.column(1).is_nan() || self.column(2).is_nan(),
            4 => {
                self.column(0).is_nan()
                    || self.column(1).is_nan()
                    || self.column(2).is_nan()
                    || self.column(3).is_nan()
            }
            _ => unreachable!(),
        }
    }

    /// Returns `true` if all elements are neither infinite nor NaN.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let finite = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, 0.0),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    /// let infinite = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, 1.0, f32::INFINITY),
    ///     Vec3::new(1.0, 0.0, 1.0),
    /// ]);
    ///
    /// assert!(finite.is_finite());
    /// assert!(!infinite.is_finite());
    /// ```
    #[inline]
    #[must_use]
    pub fn is_finite(&self) -> bool {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe { transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self).is_finite() },
            3 => {
                self.column(0).is_finite()
                    && self.column(1).is_finite()
                    && self.column(2).is_finite()
            }
            4 => {
                self.column(0).is_finite()
                    && self.column(1).is_finite()
                    && self.column(2).is_finite()
                    && self.column(3).is_finite()
            }
            _ => unreachable!(),
        }
    }

    /// Returns the element-wise reciprocal (inverse) of a matrix, `1 / self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let mat = Mat3::from_columns(&[
    ///     Vec3::new(2.0, 4.0, 1.0),
    ///     Vec3::new(1.0, 2.0, 4.0),
    ///     Vec3::new(4.0, 1.0, 2.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     mat.recip(),
    ///     Mat3::from_columns(&[
    ///         Vec3::new(0.5, 0.25, 1.0),
    ///         Vec3::new(1.0, 0.5, 0.25),
    ///         Vec3::new(0.25, 1.0, 0.5),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn recip(&self) -> Self {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Matrix<N, T, A>>(mat.recip())
            },
            _ => Self::from_column_fn(|i| self.column(i).recip()),
        }
    }

    /// Returns the absolute values of the elements of `self`.
    ///
    /// Equivalent to `(self.x_axis.abs(), self.y_axis.abs(), ...)`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec3};
    /// #
    /// let mat = Mat3::from_columns(&[
    ///     Vec3::new(1.0, 0.0, 0.0),
    ///     Vec3::new(0.0, -1.0, 0.0),
    ///     Vec3::new(0.0, 0.0, -1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     mat.abs(),
    ///     Mat3::from_columns(&[
    ///         Vec3::new(1.0, 0.0, 0.0),
    ///         Vec3::new(0.0, 1.0, 0.0),
    ///         Vec3::new(0.0, 0.0, 1.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn abs(&self) -> Self {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Matrix<N, T, A>>(mat.abs())
            },
            _ => Self::from_column_fn(|i| self.column(i).abs()),
        }
    }

    /// Returns `true` if the absolute difference of all elements between `self`
    /// and `other` is less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare two matrices that should be equal, but may
    /// have a slight difference due to operations having rounding errors.
    #[inline]
    #[must_use]
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: T) -> bool {
        match N {
            // SAFETY: `Matrix<N, T, A>` is `Matrix<2, T, A>` which has the
            // memory layout of `Vector<4, T, A>`.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(self);
                let other = transmute_ref::<Matrix<N, T, A>, Vector<4, T, A>>(other);

                mat.abs_diff_eq(*other, max_abs_diff)
            },
            3 => {
                self.column(0).abs_diff_eq(other.column(0), max_abs_diff)
                    && self.column(1).abs_diff_eq(other.column(1), max_abs_diff)
                    && self.column(2).abs_diff_eq(other.column(2), max_abs_diff)
            }
            4 => {
                self.column(0).abs_diff_eq(other.column(0), max_abs_diff)
                    && self.column(1).abs_diff_eq(other.column(1), max_abs_diff)
                    && self.column(2).abs_diff_eq(other.column(2), max_abs_diff)
                    && self.column(3).abs_diff_eq(other.column(3), max_abs_diff)
            }
            _ => unreachable!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Matrix, Vector, test_utils::for_parameters};

    #[test]
    fn test_is_nan() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let nan = Vector::NAN;
            assert!(!Matrix::<2, T, A>::from_columns(&[one, one]).is_nan());
            assert!(Matrix::<2, T, A>::from_columns(&[nan, one]).is_nan());
            assert!(Matrix::<2, T, A>::from_columns(&[one, nan]).is_nan());
            assert!(Matrix::<2, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::NAN;
            assert!(!Matrix::<3, T, A>::from_columns(&[one, one, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_columns(&[nan, one, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_columns(&[one, nan, one]).is_nan());
            assert!(Matrix::<3, T, A>::from_columns(&[one, one, nan]).is_nan());
            assert!(Matrix::<3, T, A>::NAN.is_nan());

            let one = Vector::ONE;
            let nan = Vector::NAN;
            assert!(!Matrix::<4, T, A>::from_columns(&[one, one, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[nan, one, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[one, nan, one, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[one, one, nan, one]).is_nan());
            assert!(Matrix::<4, T, A>::from_columns(&[one, one, one, nan]).is_nan());
            assert!(Matrix::<4, T, A>::NAN.is_nan());
        });
    }

    #[test]
    fn test_is_finite() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let one = Vector::ONE;
            let inf = Vector::INFINITY;
            assert!(Matrix::<2, T, A>::from_columns(&[one, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_columns(&[inf, one]).is_finite());
            assert!(!Matrix::<2, T, A>::from_columns(&[one, inf]).is_finite());
            assert!(!Matrix::<2, T, A>::from_columns(&[inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::INFINITY;
            assert!(Matrix::<3, T, A>::from_columns(&[one, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[inf, one, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[one, inf, one]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[one, one, inf]).is_finite());
            assert!(!Matrix::<3, T, A>::from_columns(&[inf, inf, inf]).is_finite());

            let one = Vector::ONE;
            let inf = Vector::INFINITY;
            assert!(Matrix::<4, T, A>::from_columns(&[one, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[inf, one, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[one, inf, one, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[one, one, inf, one]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[one, one, one, inf]).is_finite());
            assert!(!Matrix::<4, T, A>::from_columns(&[inf, inf, inf, inf]).is_finite());
        });
    }

    #[test]
    fn test_recip() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(1.0, 2.0),
                    Vector::<2, T, A>::new(0.5, 4.0)
                ])
                .recip(),
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(1.0, 0.5),
                    Vector::<2, T, A>::new(2.0, 0.25)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(1.0, 2.0, 0.5),
                    Vector::<3, T, A>::new(4.0, 0.25, 8.0),
                    Vector::<3, T, A>::new(0.125, 16.0, 0.0625)
                ])
                .recip(),
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(1.0, 0.5, 2.0),
                    Vector::<3, T, A>::new(0.25, 4.0, 0.125),
                    Vector::<3, T, A>::new(8.0, 0.0625, 16.0)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(1.0, 2.0, 0.5, 4.0),
                    Vector::<4, T, A>::new(0.25, 8.0, 0.125, 16.0),
                    Vector::<4, T, A>::new(0.0625, 32.0, 0.03125, 64.0),
                    Vector::<4, T, A>::new(0.015625, 128.0, 0.0078125, 256.0)
                ])
                .recip(),
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(1.0, 0.5, 2.0, 0.25),
                    Vector::<4, T, A>::new(4.0, 0.125, 8.0, 0.0625),
                    Vector::<4, T, A>::new(16.0, 0.03125, 32.0, 0.015625),
                    Vector::<4, T, A>::new(64.0, 0.0078125, 128.0, 0.00390625)
                ])
            );
        });
    }

    #[test]
    fn test_abs() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(-1.0, 2.0),
                    Vector::<2, T, A>::new(-3.0, 4.0)
                ])
                .abs(),
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(1.0, 2.0),
                    Vector::<2, T, A>::new(3.0, 4.0)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(-1.0, 2.0, -3.0),
                    Vector::<3, T, A>::new(4.0, -5.0, 6.0),
                    Vector::<3, T, A>::new(-7.0, 8.0, -9.0)
                ])
                .abs(),
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(1.0, 2.0, 3.0),
                    Vector::<3, T, A>::new(4.0, 5.0, 6.0),
                    Vector::<3, T, A>::new(7.0, 8.0, 9.0)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(-1.0, 2.0, -3.0, 4.0),
                    Vector::<4, T, A>::new(-5.0, 6.0, -7.0, 8.0),
                    Vector::<4, T, A>::new(9.0, -10.0, 11.0, -12.0),
                    Vector::<4, T, A>::new(13.0, -14.0, 15.0, -16.0)
                ])
                .abs(),
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(1.0, 2.0, 3.0, 4.0),
                    Vector::<4, T, A>::new(5.0, 6.0, 7.0, 8.0),
                    Vector::<4, T, A>::new(9.0, 10.0, 11.0, 12.0),
                    Vector::<4, T, A>::new(13.0, 14.0, 15.0, 16.0)
                ])
            );
        });
    }

    #[test]
    fn test_abs_diff_eq() {
        for_parameters!(|T: PrimitiveFloat, A| {
            let col0 = Vector::<2, T, A>::new(0.0, 1.0);
            let col1 = Vector::<2, T, A>::new(2.0, 3.0);
            assert!(
                Matrix::<2, T, A>::from_columns(&[col0, col1])
                    .abs_diff_eq(&Matrix::from_columns(&[col0 + 0.1, col1 - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_columns(&[col0, col1])
                    .abs_diff_eq(&Matrix::from_columns(&[col0 + 0.5, col1 - 0.1]), 0.125)
            );
            assert!(
                !Matrix::<2, T, A>::from_columns(&[col0, col1])
                    .abs_diff_eq(&Matrix::from_columns(&[col0 + 0.1, col1 - 0.5]), 0.125)
            );

            let col0 = Vector::<3, T, A>::new(0.0, 1.0, 2.0);
            let col1 = Vector::<3, T, A>::new(3.0, 4.0, 5.0);
            let col2 = Vector::<3, T, A>::new(6.0, 7.0, 8.0);
            assert!(
                Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.5, col1 - 0.1, col2 + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.5, col2 + 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<3, T, A>::from_columns(&[col0, col1, col2]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.5]),
                    0.125
                )
            );

            let col0 = Vector::<4, T, A>::new(0.0, 1.0, 2.0, 3.0);
            let col1 = Vector::<4, T, A>::new(4.0, 5.0, 6.0, 7.0);
            let col2 = Vector::<4, T, A>::new(8.0, 9.0, 10.0, 11.0);
            let col3 = Vector::<4, T, A>::new(12.0, 13.0, 14.0, 15.0);
            assert!(
                Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.05, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.5, col1 - 0.1, col2 + 0.05, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.5, col2 + 0.05, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.5, col3 - 0.05]),
                    0.125
                )
            );
            assert!(
                !Matrix::<4, T, A>::from_columns(&[col0, col1, col2, col3]).abs_diff_eq(
                    &Matrix::from_columns(&[col0 + 0.1, col1 - 0.1, col2 + 0.05, col3 - 0.5]),
                    0.125
                )
            );
        });
    }
}
