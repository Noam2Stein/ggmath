use std::mem::{transmute, transmute_copy};

use super::*;

// Major Axis

/// Marker trait for matrix major axis.
///
/// Is implemented for `ColMajor` and `RowMajor`.
pub unsafe trait MatMajorAxis {
    /// Enum that is used to specialize function implementations based on the major axis.
    const ENUM: MatMajorAxisEnum;

    /// The inner array type that is inside a matrix.
    /// This has to be controlled by the major axis.
    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>: Construct
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}

/// Marker type for column major matrices.
pub struct ColMajor;

/// Marker type for row major matrices.
pub struct RowMajor;

/// Enum that mirrors the `MatMajorAxis` trait and its impls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatMajorAxisEnum {
    /// Column major matrix.
    ColumnMajor,
    /// Row major matrix.
    RowMajor,
}

#[allow(private_interfaces)]
unsafe impl MatMajorAxis for ColMajor {
    const ENUM: MatMajorAxisEnum = MatMajorAxisEnum::ColumnMajor;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
        = [Vector<R, T, A>; C]
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}

#[allow(private_interfaces)]
unsafe impl MatMajorAxis for RowMajor {
    const ENUM: MatMajorAxisEnum = MatMajorAxisEnum::RowMajor;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
        = [Vector<C, T, A>; R]
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}

// Resolve

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

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
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
    /// use ggmath::*;
    ///
    /// fn foo(matrix: Matrix<2, 2, f32, VecAligned, impl MatMajorAxis>) {
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
                MatMajorAxisEnum::ColumnMajor => {
                    ResolvedMatrix::ColumnMajor(transmute_copy::<
                        Matrix<C, R, T, A, M>,
                        Matrix<C, R, T, A, ColMajor>,
                    >(&self))
                }

                MatMajorAxisEnum::RowMajor => ResolvedMatrix::RowMajor(transmute_copy::<
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
    /// use ggmath::*;
    ///
    /// fn foo(matrix: &Matrix<2, 2, f32, VecAligned, impl MatMajorAxis>) {
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
    pub const fn resolve_ref(&'_ self) -> ResolvedMatrixRef<'_, C, R, T, A> {
        unsafe {
            match M::ENUM {
                MatMajorAxisEnum::ColumnMajor => {
                    ResolvedMatrixRef::ColumnMajor(transmute::<
                        &Matrix<C, R, T, A, M>,
                        &Matrix<C, R, T, A, ColMajor>,
                    >(self))
                }

                MatMajorAxisEnum::RowMajor => ResolvedMatrixRef::RowMajor(transmute::<
                    &Matrix<C, R, T, A, M>,
                    &Matrix<C, R, T, A, RowMajor>,
                >(self)),
            }
        }
    }

    /// Splits the mutable matrix reference based on its major axis.
    /// This can be used to specialize function implementations based on the major axis.
    ///
    /// ### Example
    ///
    /// ```
    /// use ggmath::*;
    ///
    /// fn foo(matrix: &mut Matrix<2, 2, f32, VecAligned, impl MatMajorAxis>) {
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
    pub const fn resolve_mut(&'_ mut self) -> ResolvedMatrixMut<'_, C, R, T, A> {
        unsafe {
            match M::ENUM {
                MatMajorAxisEnum::ColumnMajor => {
                    ResolvedMatrixMut::ColumnMajor(transmute::<
                        &mut Matrix<C, R, T, A, M>,
                        &mut Matrix<C, R, T, A, ColMajor>,
                    >(self))
                }

                MatMajorAxisEnum::RowMajor => ResolvedMatrixMut::RowMajor(transmute::<
                    &mut Matrix<C, R, T, A, M>,
                    &mut Matrix<C, R, T, A, RowMajor>,
                >(self)),
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
            MatMajorAxisEnum::ColumnMajor => unsafe {
                transmute_copy::<Matrix<C, R, T, A, ColMajor>, Matrix<C, R, T, A, M>>(&c_major)
            },

            MatMajorAxisEnum::RowMajor => unsafe {
                transmute_copy::<Matrix<C, R, T, A, RowMajor>, Matrix<C, R, T, A, M>>(&r_major)
            },
        }
    }
}
