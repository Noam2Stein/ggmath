use crate::{
    Alignment, Length, Matrix, Scalar, SupportedLength, Vector, num_primitive::PrimitiveFloat,
    transmute::transmute_ref,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PrimitiveFloat,
{
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
