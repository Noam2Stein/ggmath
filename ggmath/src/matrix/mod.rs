use crate::{
    construct::*,
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub mod major_axis;
use major_axis::*;

mod api;
mod impl_std;
// Due to probably a bug, the compiler doesn't pick up 'api::builder' from the 'api::*' import, so it has to be explicitly imported.
#[allow(unused_imports)]
pub use api::{builder, *};
#[allow(unused_imports)]
pub use impl_std::*;

pub struct Matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    inner: M::InnerMatrix<C, R, T, A>,
}

pub type Mat2C<T> = Matrix<2, 2, T, VecAligned, ColumnMajor>;
pub type Mat2x3C<T> = Matrix<2, 3, T, VecAligned, ColumnMajor>;
pub type Mat2x4C<T> = Matrix<2, 4, T, VecAligned, ColumnMajor>;
pub type Mat3x2C<T> = Matrix<3, 2, T, VecAligned, ColumnMajor>;
pub type Mat3C<T> = Matrix<3, 3, T, VecAligned, ColumnMajor>;
pub type Mat3x4C<T> = Matrix<3, 4, T, VecAligned, ColumnMajor>;
pub type Mat4x2C<T> = Matrix<4, 2, T, VecAligned, ColumnMajor>;
pub type Mat4x3C<T> = Matrix<4, 3, T, VecAligned, ColumnMajor>;
pub type Mat4C<T> = Matrix<4, 4, T, VecAligned, ColumnMajor>;

pub type Mat2CP<T> = Matrix<2, 2, T, VecPacked, ColumnMajor>;
pub type Mat2x3CP<T> = Matrix<2, 3, T, VecPacked, ColumnMajor>;
pub type Mat2x4CP<T> = Matrix<2, 4, T, VecPacked, ColumnMajor>;
pub type Mat3x2CP<T> = Matrix<3, 2, T, VecPacked, ColumnMajor>;
pub type Mat3CP<T> = Matrix<3, 3, T, VecPacked, ColumnMajor>;
pub type Mat3x4CP<T> = Matrix<3, 4, T, VecPacked, ColumnMajor>;
pub type Mat4x2CP<T> = Matrix<4, 2, T, VecPacked, ColumnMajor>;
pub type Mat4x3CP<T> = Matrix<4, 3, T, VecPacked, ColumnMajor>;
pub type Mat4CP<T> = Matrix<4, 4, T, VecPacked, ColumnMajor>;

pub type Mat2R<T> = Matrix<2, 2, T, VecAligned, RowMajor>;
pub type Mat2x3R<T> = Matrix<2, 3, T, VecAligned, RowMajor>;
pub type Mat2x4R<T> = Matrix<2, 4, T, VecAligned, RowMajor>;
pub type Mat3x2R<T> = Matrix<3, 2, T, VecAligned, RowMajor>;
pub type Mat3R<T> = Matrix<3, 3, T, VecAligned, RowMajor>;
pub type Mat3x4R<T> = Matrix<3, 4, T, VecAligned, RowMajor>;
pub type Mat4x2R<T> = Matrix<4, 2, T, VecAligned, RowMajor>;
pub type Mat4x3R<T> = Matrix<4, 3, T, VecAligned, RowMajor>;
pub type Mat4R<T> = Matrix<4, 4, T, VecAligned, RowMajor>;

pub type Mat2RP<T> = Matrix<2, 2, T, VecPacked, RowMajor>;
pub type Mat2x3RP<T> = Matrix<2, 3, T, VecPacked, RowMajor>;
pub type Mat2x4RP<T> = Matrix<2, 4, T, VecPacked, RowMajor>;
pub type Mat3x2RP<T> = Matrix<3, 2, T, VecPacked, RowMajor>;
pub type Mat3RP<T> = Matrix<3, 3, T, VecPacked, RowMajor>;
pub type Mat3x4RP<T> = Matrix<3, 4, T, VecPacked, RowMajor>;
pub type Mat4x2RP<T> = Matrix<4, 2, T, VecPacked, RowMajor>;
pub type Mat4x3RP<T> = Matrix<4, 3, T, VecPacked, RowMajor>;
pub type Mat4RP<T> = Matrix<4, 4, T, VecPacked, RowMajor>;

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
    pub fn index_column(self, index: usize) -> Vector<R, T, A> {
        self.get_column(index).unwrap()
    }
    #[inline(always)]
    pub fn index_row(self, index: usize) -> Vector<C, T, A> {
        self.get_row(index).unwrap()
    }
    #[inline(always)]
    pub fn index_column_array(self, index: usize) -> [T; R] {
        self.get_column_array(index).unwrap()
    }
    #[inline(always)]
    pub fn index_row_array(self, index: usize) -> [T; C] {
        self.get_row_array(index).unwrap()
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

    #[inline(always)]
    pub fn into_aligned(self) -> Matrix<C, R, T, VecAligned, M> {
        self.into_alignment()
    }
    #[inline(always)]
    pub fn into_packed(self) -> Matrix<C, R, T, VecAligned, M> {
        self.into_alignment()
    }
    #[inline(always)]
    pub fn into_alignment<AOutput: VecAlignment>(self) -> Matrix<C, R, T, AOutput, M> {
        M::into_alignment(self)
    }
    #[inline(always)]
    pub fn into_column_major(self) -> Matrix<C, R, T, A, ColumnMajor> {
        M::into_column_major(self)
    }
    #[inline(always)]
    pub fn into_row_major(self) -> Matrix<C, R, T, A, RowMajor> {
        M::into_row_major(self)
    }
    #[inline(always)]
    pub fn into_major_axis<MOutput: MatrixMajorAxis>(self) -> Matrix<C, R, T, A, MOutput> {
        MOutput::from_major_axis(self)
    }
    #[inline(always)]
    pub fn into_repr<AOutput: VecAlignment, MOutput: MatrixMajorAxis>(
        self,
    ) -> Matrix<C, R, T, AOutput, MOutput> {
        self.into_alignment().into_major_axis()
    }
}
