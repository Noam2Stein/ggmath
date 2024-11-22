use std::{
    any::{type_name, TypeId},
    mem::{transmute, transmute_copy},
};

use super::*;

#[allow(private_bounds)]
pub trait MatrixMajorAxis: Seal + Sized + 'static {
    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>: Construct
    where
        ScalarCount<C>: VecLen,
        ScalarCount<R>: VecLen;
}

pub struct ColumnMajor;
pub struct RowMajor;

pub enum MajorAxisResolvedMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    ColumnMajor(Matrix<C, R, T, A, ColumnMajor>),
    RowMajor(Matrix<C, R, T, A, RowMajor>),
}
pub enum MajorAxisResolvedMatrixRef<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    ColumnMajor(&'a Matrix<C, R, T, A, ColumnMajor>),
    RowMajor(&'a Matrix<C, R, T, A, RowMajor>),
}
pub enum MajorAxisResolvedMatrixMut<'a, const C: usize, const R: usize, T: Scalar, A: VecAlignment>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    ColumnMajor(&'a mut Matrix<C, R, T, A, ColumnMajor>),
    RowMajor(&'a mut Matrix<C, R, T, A, RowMajor>),
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    pub fn into_column_major(self) -> Matrix<C, R, T, A, ColumnMajor> {
        Matrix::from_columns(self.into_columns())
    }
    #[inline(always)]
    pub fn into_row_major(self) -> Matrix<C, R, T, A, RowMajor> {
        Matrix::from_rows(self.into_rows())
    }
    #[inline(always)]
    pub fn into_major_axis<MOutput: MatrixMajorAxis>(self) -> Matrix<C, R, T, A, MOutput> {
        Matrix::from_resolved_major_axis_fns(|| self.into_column_major(), || self.into_row_major())
    }

    #[inline(always)]
    pub fn resolve_major_axis(self) -> MajorAxisResolvedMatrix<C, R, T, A> {
        unsafe {
            if TypeId::of::<M>() == TypeId::of::<ColumnMajor>() {
                MajorAxisResolvedMatrix::ColumnMajor(transmute_copy(&self))
            } else if TypeId::of::<M>() == TypeId::of::<RowMajor>() {
                MajorAxisResolvedMatrix::RowMajor(transmute_copy(&self))
            } else {
                panic!("invalid MatrixMajorAxis: {}", type_name::<M>())
            }
        }
    }
    #[inline(always)]
    pub fn resolve_major_axis_ref(&self) -> MajorAxisResolvedMatrixRef<C, R, T, A> {
        unsafe {
            if TypeId::of::<M>() == TypeId::of::<ColumnMajor>() {
                MajorAxisResolvedMatrixRef::ColumnMajor(transmute(self))
            } else if TypeId::of::<M>() == TypeId::of::<RowMajor>() {
                MajorAxisResolvedMatrixRef::RowMajor(transmute(self))
            } else {
                panic!("invalid MatrixMajorAxis: {}", type_name::<M>())
            }
        }
    }
    #[inline(always)]
    pub fn resolve_major_axis_mut(&mut self) -> MajorAxisResolvedMatrixMut<C, R, T, A> {
        unsafe {
            if TypeId::of::<M>() == TypeId::of::<ColumnMajor>() {
                MajorAxisResolvedMatrixMut::ColumnMajor(transmute(self))
            } else if TypeId::of::<M>() == TypeId::of::<RowMajor>() {
                MajorAxisResolvedMatrixMut::RowMajor(transmute(self))
            } else {
                panic!("invalid MatrixMajorAxis: {}", type_name::<M>())
            }
        }
    }

    #[inline(always)]
    pub fn from_resolved_major_axis_fns(
        f_column_major: impl FnOnce() -> Matrix<C, R, T, A, ColumnMajor>,
        f_row_major: impl FnOnce() -> Matrix<C, R, T, A, RowMajor>,
    ) -> Self {
        unsafe {
            if TypeId::of::<M>() == TypeId::of::<ColumnMajor>() {
                transmute_copy(&f_column_major())
            } else if TypeId::of::<M>() == TypeId::of::<RowMajor>() {
                transmute_copy(&f_row_major())
            } else {
                panic!("invalid MatrixMajorAxis: {}", type_name::<M>())
            }
        }
    }
}

impl MatrixMajorAxis for ColumnMajor {
    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment> =
        [Vector<R, T, A>; C]
    where
        ScalarCount<C>: VecLen,
        ScalarCount<R>: VecLen;
}

impl MatrixMajorAxis for RowMajor {
    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment> =
        [Vector<C, T, A>; R]
    where
        ScalarCount<C>: VecLen,
        ScalarCount<R>: VecLen;
}

trait Seal {}
impl Seal for ColumnMajor {}
impl Seal for RowMajor {}
