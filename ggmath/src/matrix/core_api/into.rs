use std::array;

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    pub fn into_columns(self) -> [Vector<R, T, A>; C] {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => mat.inner,
            MajorAxisResolvedMatrix::RowMajor(mat) => {
                array::from_fn(|column_index| mat.get_column(column_index).unwrap())
            }
        }
    }
    #[inline(always)]
    pub fn into_columns_array(self) -> [[T; R]; C] {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                mat.inner.map(|column| column.into_array())
            }
            MajorAxisResolvedMatrix::RowMajor(mat) => {
                array::from_fn(|column_index| mat.get_column_array(column_index).unwrap())
            }
        }
    }

    #[inline(always)]
    pub fn into_rows(self) -> [Vector<C, T, A>; R] {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                array::from_fn(|row_index| mat.get_row(row_index).unwrap())
            }
            MajorAxisResolvedMatrix::RowMajor(mat) => mat.inner,
        }
    }
    #[inline(always)]
    pub fn into_rows_array(self) -> [[T; C]; R] {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                array::from_fn(|row_index| mat.get_row_array(row_index).unwrap())
            }
            MajorAxisResolvedMatrix::RowMajor(mat) => mat.inner.map(|row| row.into_array()),
        }
    }
}
