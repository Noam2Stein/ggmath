use crate::{
    construct::*,
    scalar::Scalar,
    vector::{alignment::*, length::*, *},
};

pub mod major_axis;
use major_axis::*;

mod api;
pub use api::*;

mod impl_std;

pub struct Matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    inner: <M as major_axis_seal::MatrixMajorAxis>::InnerMatrix<C, R, T, A>,
}

pub type Mat2<T, M> = Matrix<2, 2, T, VecAligned, M>;
pub type Mat2x3<T, M> = Matrix<2, 3, T, VecAligned, M>;
pub type Mat2x4<T, M> = Matrix<2, 4, T, VecAligned, M>;
pub type Mat3x2<T, M> = Matrix<3, 2, T, VecAligned, M>;
pub type Mat3<T, M> = Matrix<3, 3, T, VecAligned, M>;
pub type Mat3x4<T, M> = Matrix<3, 4, T, VecAligned, M>;
pub type Mat4x2<T, M> = Matrix<4, 2, T, VecAligned, M>;
pub type Mat4x3<T, M> = Matrix<4, 3, T, VecAligned, M>;
pub type Mat4<T, M> = Matrix<4, 4, T, VecAligned, M>;

pub type Mat2P<T, M> = Matrix<2, 2, T, VecPacked, M>;
pub type Mat2x3P<T, M> = Matrix<2, 3, T, VecPacked, M>;
pub type Mat2x4P<T, M> = Matrix<2, 4, T, VecPacked, M>;
pub type Mat3x2P<T, M> = Matrix<3, 2, T, VecPacked, M>;
pub type Mat3P<T, M> = Matrix<3, 3, T, VecPacked, M>;
pub type Mat3x4P<T, M> = Matrix<3, 4, T, VecPacked, M>;
pub type Mat4x2P<T, M> = Matrix<4, 2, T, VecPacked, M>;
pub type Mat4x3P<T, M> = Matrix<4, 3, T, VecPacked, M>;
pub type Mat4P<T, M> = Matrix<4, 4, T, VecPacked, M>;

pub fn matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>(
    rows: [Vector<C, T, A>; R],
) -> Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    Matrix::from_rows(rows)
}

pub mod column_major {
    use crate::vector::alignment::*;

    pub type Matrix<const C: usize, const R: usize, T, A> =
        super::Matrix<C, R, T, A, super::ColumnMajor>;

    pub type Mat2<T> = Matrix<2, 2, T, VecAligned>;
    pub type Mat2x3<T> = Matrix<2, 3, T, VecAligned>;
    pub type Mat2x4<T> = Matrix<2, 4, T, VecAligned>;
    pub type Mat3x2<T> = Matrix<3, 2, T, VecAligned>;
    pub type Mat3<T> = Matrix<3, 3, T, VecAligned>;
    pub type Mat3x4<T> = Matrix<3, 4, T, VecAligned>;
    pub type Mat4x2<T> = Matrix<4, 2, T, VecAligned>;
    pub type Mat4x3<T> = Matrix<4, 3, T, VecAligned>;
    pub type Mat4<T> = Matrix<4, 4, T, VecAligned>;

    pub type Mat2P<T> = Matrix<2, 2, T, VecPacked>;
    pub type Mat2x3P<T> = Matrix<2, 3, T, VecPacked>;
    pub type Mat2x4P<T> = Matrix<2, 4, T, VecPacked>;
    pub type Mat3x2P<T> = Matrix<3, 2, T, VecPacked>;
    pub type Mat3P<T> = Matrix<3, 3, T, VecPacked>;
    pub type Mat3x4P<T> = Matrix<3, 4, T, VecPacked>;
    pub type Mat4x2P<T> = Matrix<4, 2, T, VecPacked>;
    pub type Mat4x3P<T> = Matrix<4, 3, T, VecPacked>;
    pub type Mat4P<T> = Matrix<4, 4, T, VecPacked>;
}
pub mod row_major {
    use crate::vector::alignment::*;

    pub type Matrix<const C: usize, const R: usize, T, A> =
        super::Matrix<C, R, T, A, super::RowMajor>;

    pub type Mat2<T> = Matrix<2, 2, T, VecAligned>;
    pub type Mat2x3<T> = Matrix<2, 3, T, VecAligned>;
    pub type Mat2x4<T> = Matrix<2, 4, T, VecAligned>;
    pub type Mat3x2<T> = Matrix<3, 2, T, VecAligned>;
    pub type Mat3<T> = Matrix<3, 3, T, VecAligned>;
    pub type Mat3x4<T> = Matrix<3, 4, T, VecAligned>;
    pub type Mat4x2<T> = Matrix<4, 2, T, VecAligned>;
    pub type Mat4x3<T> = Matrix<4, 3, T, VecAligned>;
    pub type Mat4<T> = Matrix<4, 4, T, VecAligned>;

    pub type Mat2P<T> = Matrix<2, 2, T, VecPacked>;
    pub type Mat2x3P<T> = Matrix<2, 3, T, VecPacked>;
    pub type Mat2x4P<T> = Matrix<2, 4, T, VecPacked>;
    pub type Mat3x2P<T> = Matrix<3, 2, T, VecPacked>;
    pub type Mat3P<T> = Matrix<3, 3, T, VecPacked>;
    pub type Mat3x4P<T> = Matrix<3, 4, T, VecPacked>;
    pub type Mat4x2P<T> = Matrix<4, 2, T, VecPacked>;
    pub type Mat4x3P<T> = Matrix<4, 3, T, VecPacked>;
    pub type Mat4P<T> = Matrix<4, 4, T, VecPacked>;
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    pub fn from_columns(columns: [Vector<R, T, A>; C]) -> Self {
        M::from_columns(columns)
    }
    #[inline(always)]
    pub fn from_rows(rows: [Vector<C, T, A>; R]) -> Self {
        M::from_rows(rows)
    }
    #[inline(always)]
    pub fn from_columns_array(columns: [[T; R]; C]) -> Self {
        M::from_columns_array(columns)
    }
    #[inline(always)]
    pub fn from_rows_array(rows: [[T; C]; R]) -> Self {
        M::from_rows_array(rows)
    }

    #[inline(always)]
    pub fn into_columns(self) -> [Vector<R, T, A>; C] {
        M::into_columns(self)
    }
    #[inline(always)]
    pub fn into_rows(self) -> [Vector<C, T, A>; R] {
        M::into_rows(self)
    }
    #[inline(always)]
    pub fn into_columns_array(self) -> [[T; R]; C] {
        M::into_columns_array(self)
    }
    #[inline(always)]
    pub fn into_rows_array(self) -> [[T; C]; R] {
        M::into_rows_array(self)
    }

    #[inline(always)]
    pub fn get_column(self, index: usize) -> Option<Vector<R, T, A>> {
        M::get_column(self, index)
    }
    #[inline(always)]
    pub fn get_row(self, index: usize) -> Option<Vector<C, T, A>> {
        M::get_row(self, index)
    }
    #[inline(always)]
    pub fn get_column_array(self, index: usize) -> Option<[T; R]> {
        M::get_column_array(self, index)
    }
    #[inline(always)]
    pub fn get_row_array(self, index: usize) -> Option<[T; C]> {
        M::get_row_array(self, index)
    }

    #[inline(always)]
    pub unsafe fn get_column_unchecked(self, index: usize) -> Vector<R, T, A> {
        M::get_column_unchecked(self, index)
    }
    #[inline(always)]
    pub unsafe fn get_row_unchecked(self, index: usize) -> Vector<C, T, A> {
        M::get_row_unchecked(self, index)
    }
    #[inline(always)]
    pub unsafe fn get_column_array_unchecked(self, index: usize) -> [T; R] {
        M::get_column_array_unchecked(self, index)
    }
    #[inline(always)]
    pub unsafe fn get_row_array_unchecked(self, index: usize) -> [T; C] {
        M::get_row_array_unchecked(self, index)
    }
}
