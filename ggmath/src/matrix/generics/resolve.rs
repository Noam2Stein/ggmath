use std::mem::{transmute, transmute_copy};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    ColumnMajor(Matrix<C, R, T, A, ColumnMajor>),
    RowMajor(Matrix<C, R, T, A, RowMajor>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedMatrixRef<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    ColumnMajor(&'a Matrix<C, R, T, A, ColumnMajor>),
    RowMajor(&'a Matrix<C, R, T, A, RowMajor>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedMatrixMut<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    ColumnMajor(&'a mut Matrix<C, R, T, A, ColumnMajor>),
    RowMajor(&'a mut Matrix<C, R, T, A, RowMajor>),
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedMatrix<C, R, T, A> {
        unsafe {
            if M::IS_COLUMN_MAJOR {
                ResolvedMatrix::ColumnMajor(transmute_copy::<
                    Matrix<C, R, T, A, M>,
                    Matrix<C, R, T, A, ColumnMajor>,
                >(&self))
            } else {
                ResolvedMatrix::RowMajor(transmute_copy::<
                    Matrix<C, R, T, A, M>,
                    Matrix<C, R, T, A, RowMajor>,
                >(&self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedMatrixRef<C, R, T, A> {
        unsafe {
            if M::IS_COLUMN_MAJOR {
                ResolvedMatrixRef::ColumnMajor(transmute::<
                    &Matrix<C, R, T, A, M>,
                    &Matrix<C, R, T, A, ColumnMajor>,
                >(self))
            } else {
                ResolvedMatrixRef::RowMajor(transmute::<
                    &Matrix<C, R, T, A, M>,
                    &Matrix<C, R, T, A, RowMajor>,
                >(self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_mut(&mut self) -> ResolvedMatrixMut<C, R, T, A> {
        unsafe {
            if M::IS_COLUMN_MAJOR {
                ResolvedMatrixMut::ColumnMajor(transmute::<
                    &mut Matrix<C, R, T, A, M>,
                    &mut Matrix<C, R, T, A, ColumnMajor>,
                >(self))
            } else {
                ResolvedMatrixMut::RowMajor(transmute::<
                    &mut Matrix<C, R, T, A, M>,
                    &mut Matrix<C, R, T, A, RowMajor>,
                >(self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolved(
        c_major: Matrix<C, R, T, A, ColumnMajor>,
        r_major: Matrix<C, R, T, A, RowMajor>,
    ) -> Self {
        if M::IS_COLUMN_MAJOR {
            unsafe {
                transmute_copy::<Matrix<C, R, T, A, ColumnMajor>, Matrix<C, R, T, A, M>>(&c_major)
            }
        } else {
            unsafe {
                transmute_copy::<Matrix<C, R, T, A, RowMajor>, Matrix<C, R, T, A, M>>(&r_major)
            }
        }
    }
}
