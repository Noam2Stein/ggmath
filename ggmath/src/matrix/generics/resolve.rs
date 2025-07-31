use std::mem::{transmute, transmute_copy};

use super::*;

/// See [`Matrix::resolve`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Column major matrix.
    ColumnMajor(Matrix<C, R, T, A, ColMajor>),
    /// Row major matrix.
    RowMajor(Matrix<C, R, T, A, RowMajor>),
}

/// See [`Matrix::resolve_ref`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedMatrixRef<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Column major matrix.
    ColumnMajor(&'a Matrix<C, R, T, A, ColMajor>),
    /// Row major matrix.
    RowMajor(&'a Matrix<C, R, T, A, RowMajor>),
}

/// See [`Matrix::resolve_mut`].
#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedMatrixMut<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Column major matrix.
    ColumnMajor(&'a mut Matrix<C, R, T, A, ColMajor>),
    /// Row major matrix.
    RowMajor(&'a mut Matrix<C, R, T, A, RowMajor>),
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Splits the matrix based on its major axis.
    /// This can be used to specialize function implementations based on the major axis.
    ///
    /// ### Example
    ///
    /// ```
    /// fn foo(matrix: Matrix<2, 2, f32, VecAligned, impl MatrixMajorAxis>) {
    ///     match matrix.resolve() {
    ///         ResolvedMatrix::ColumnMajor(matrix) => {
    ///             println!("Column major matrix");
    ///         }
    ///         ResolvedMatrix::RowMajor(matrix) => {
    ///             println!("Row major matrix");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedMatrix<C, R, T, A> {
        unsafe {
            match M::ENUM {
                MatrixMajorAxisEnum::ColumnMajor => {
                    ResolvedMatrix::ColumnMajor(transmute_copy::<
                        Matrix<C, R, T, A, M>,
                        Matrix<C, R, T, A, ColMajor>,
                    >(&self))
                }

                MatrixMajorAxisEnum::RowMajor => ResolvedMatrix::RowMajor(transmute_copy::<
                    Matrix<C, R, T, A, M>,
                    Matrix<C, R, T, A, RowMajor>,
                >(&self)),
            }
        }
    }

    /// Splits the matrix reference based on its major axis.
    /// This can be used to specialize function implementations based on the major axis.
    ///
    /// ### Example
    ///
    /// ```
    /// fn foo(matrix: &Matrix<2, 2, f32, VecAligned, impl MatrixMajorAxis>) {
    ///     match matrix.resolve_ref() {
    ///         ResolvedMatrixRef::ColumnMajor(matrix) => {
    ///             println!("Column major matrix");
    ///         }
    ///         ResolvedMatrixRef::RowMajor(matrix) => {
    ///             println!("Row major matrix");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedMatrixRef<C, R, T, A> {
        unsafe {
            match M::ENUM {
                MatrixMajorAxisEnum::ColumnMajor => {
                    ResolvedMatrixRef::ColumnMajor(transmute::<
                        &Matrix<C, R, T, A, M>,
                        &Matrix<C, R, T, A, ColMajor>,
                    >(self))
                }

                MatrixMajorAxisEnum::RowMajor => {
                    ResolvedMatrixRef::RowMajor(transmute::<
                        &Matrix<C, R, T, A, M>,
                        &Matrix<C, R, T, A, RowMajor>,
                    >(self))
                }
            }
        }
    }

    /// Splits the mutable matrix reference based on its major axis.
    /// This can be used to specialize function implementations based on the major axis.
    ///
    /// ### Example
    ///
    /// ```
    /// fn foo(matrix: &mut Matrix<2, 2, f32, VecAligned, impl MatrixMajorAxis>) {
    ///     match matrix.resolve_mut() {
    ///         ResolvedMatrixMut::ColumnMajor(matrix) => {
    ///             println!("Column major matrix");
    ///         }
    ///         ResolvedMatrixMut::RowMajor(matrix) => {
    ///             println!("Row major matrix");
    ///         }
    ///     }
    /// }
    /// ```
    #[inline(always)]
    pub const fn resolve_mut(&mut self) -> ResolvedMatrixMut<C, R, T, A> {
        unsafe {
            match M::ENUM {
                MatrixMajorAxisEnum::ColumnMajor => {
                    ResolvedMatrixMut::ColumnMajor(transmute::<
                        &mut Matrix<C, R, T, A, M>,
                        &mut Matrix<C, R, T, A, ColMajor>,
                    >(self))
                }

                MatrixMajorAxisEnum::RowMajor => {
                    ResolvedMatrixMut::RowMajor(transmute::<
                        &mut Matrix<C, R, T, A, M>,
                        &mut Matrix<C, R, T, A, RowMajor>,
                    >(self))
                }
            }
        }
    }

    /// "Unresolves" a matrix based on its major axis.
    /// Selects only one of the inputs based on the generic major axis.
    #[inline(always)]
    pub const fn resolved(
        c_major: Matrix<C, R, T, A, ColMajor>,
        r_major: Matrix<C, R, T, A, RowMajor>,
    ) -> Self {
        match M::ENUM {
            MatrixMajorAxisEnum::ColumnMajor => unsafe {
                transmute_copy::<Matrix<C, R, T, A, ColMajor>, Matrix<C, R, T, A, M>>(&c_major)
            },

            MatrixMajorAxisEnum::RowMajor => unsafe {
                transmute_copy::<Matrix<C, R, T, A, RowMajor>, Matrix<C, R, T, A, M>>(&r_major)
            },
        }
    }
}
