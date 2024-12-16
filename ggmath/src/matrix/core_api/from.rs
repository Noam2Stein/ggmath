use std::array;

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    pub fn from_columns(columns: [Vector<R, T, A>; C]) -> Self {
        Self::from_resolved_major_axis_fns(
            || Matrix { inner: columns },
            || {
                Matrix::from_rows_fn(|row_index| {
                    Vector::from_fn(|column_index| columns[column_index][row_index])
                })
            },
        )
    }
    #[inline(always)]
    pub fn from_columns_array(columns: [[T; R]; C]) -> Self {
        Self::from_resolved_major_axis_fns(
            || Matrix {
                inner: columns.map(|column| Vector::from_array(column)),
            },
            || {
                Matrix::from_rows_fn(|row_index| {
                    Vector::from_fn(|column_index| columns[column_index][row_index])
                })
            },
        )
    }
    #[inline(always)]
    pub fn from_columns_fn(f: impl FnMut(usize) -> Vector<R, T, A>) -> Self {
        Self::from_columns(array::from_fn(f))
    }
    #[inline(always)]
    pub fn from_columns_array_fn(f: impl FnMut(usize) -> [T; R]) -> Self {
        Self::from_columns_array(array::from_fn(f))
    }

    #[inline(always)]
    pub fn from_rows(rows: [Vector<C, T, A>; R]) -> Self {
        Self::from_resolved_major_axis_fns(
            || {
                Matrix::from_columns_fn(|column_index| {
                    Vector::from_fn(|row_index| rows[row_index][column_index])
                })
            },
            || Matrix { inner: rows },
        )
    }
    #[inline(always)]
    pub fn from_rows_array(rows: [[T; C]; R]) -> Self {
        Self::from_resolved_major_axis_fns(
            || {
                Matrix::from_columns_fn(|column_index| {
                    Vector::from_fn(|row_index| rows[row_index][column_index])
                })
            },
            || Matrix {
                inner: rows.map(|row| Vector::from_array(row)),
            },
        )
    }
    #[inline(always)]
    pub fn from_rows_fn(f: impl FnMut(usize) -> Vector<C, T, A>) -> Self {
        Self::from_rows(array::from_fn(f))
    }
    #[inline(always)]
    pub fn from_rows_array_fn(f: impl FnMut(usize) -> [T; C]) -> Self {
        Self::from_rows_array(array::from_fn(f))
    }
}
