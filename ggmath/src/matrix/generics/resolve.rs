use std::mem::{transmute, transmute_copy};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    ColumnMajor(Matrix<C, R, T, A, ColMajor>),
    RowMajor(Matrix<C, R, T, A, RowMajor>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedMatrixRef<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    ColumnMajor(&'a Matrix<C, R, T, A, ColMajor>),
    RowMajor(&'a Matrix<C, R, T, A, RowMajor>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedMatrixMut<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    ColumnMajor(&'a mut Matrix<C, R, T, A, ColMajor>),
    RowMajor(&'a mut Matrix<C, R, T, A, RowMajor>),
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
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
