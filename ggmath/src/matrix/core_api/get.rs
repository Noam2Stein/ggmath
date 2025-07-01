use std::array;

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    #[inline(always)]
    pub fn get_cell(self, index: (usize, usize)) -> Option<T> {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => mat
                .get_column(index.0)
                .map_or(None, |column| column.get(index.1)),
            MajorAxisResolvedMatrix::RowMajor(mat) => {
                mat.get_row(index.1).map_or(None, |row| row.get(index.0))
            }
        }
    }

    #[inline(always)]
    pub fn get_column(self, index: usize) -> Option<Vector<R, T, A>> {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => mat.inner.get(index).map(|column| *column),
            MajorAxisResolvedMatrix::RowMajor(mat) => {
                if index >= C {
                    None
                } else {
                    Some(Vector::from_fn(|row_index| mat.inner[row_index][index]))
                }
            }
        }
    }
    #[inline(always)]
    pub fn get_column_array(self, index: usize) -> Option<[T; R]> {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                mat.inner.get(index).map(|column| column.into_array())
            }
            MajorAxisResolvedMatrix::RowMajor(mat) => {
                if index >= C {
                    None
                } else {
                    Some(array::from_fn(|row_index| mat.inner[row_index][index]))
                }
            }
        }
    }

    #[inline(always)]
    pub fn get_row(self, index: usize) -> Option<Vector<C, T, A>> {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                if index >= R {
                    None
                } else {
                    Some(Vector::from_fn(|column_index| {
                        mat.inner[column_index][index]
                    }))
                }
            }
            MajorAxisResolvedMatrix::RowMajor(mat) => mat.inner.get(index).map(|row| *row),
        }
    }
    #[inline(always)]
    pub fn get_row_array(self, index: usize) -> Option<[T; C]> {
        match self.resolve_major_axis() {
            MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                if index >= R {
                    None
                } else {
                    Some(array::from_fn(|column_index| {
                        mat.inner[column_index][index]
                    }))
                }
            }
            MajorAxisResolvedMatrix::RowMajor(mat) => {
                mat.inner.get(index).map(|row| row.into_array())
            }
        }
    }

    #[inline(always)]
    pub unsafe fn get_cell_unchecked(self, index: (usize, usize)) -> T {
        unsafe {
            match self.resolve_major_axis() {
                MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                    mat.inner.get_unchecked(index.0).get_unchecked(index.1)
                }
                MajorAxisResolvedMatrix::RowMajor(mat) => {
                    mat.inner.get_unchecked(index.1).get_unchecked(index.0)
                }
            }
        }
    }

    #[inline(always)]
    pub unsafe fn get_column_unchecked(self, index: usize) -> Vector<R, T, A> {
        unsafe {
            match self.resolve_major_axis() {
                MajorAxisResolvedMatrix::ColumnMajor(mat) => *mat.inner.get_unchecked(index),
                MajorAxisResolvedMatrix::RowMajor(mat) => {
                    Vector::from_fn(|row_index| mat.inner[row_index].get_unchecked(index))
                }
            }
        }
    }
    #[inline(always)]
    pub unsafe fn get_column_array_unchecked(self, index: usize) -> [T; R] {
        unsafe {
            match self.resolve_major_axis() {
                MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                    mat.inner.get_unchecked(index).into_array()
                }
                MajorAxisResolvedMatrix::RowMajor(mat) => {
                    array::from_fn(|row_index| mat.inner[row_index].get_unchecked(index))
                }
            }
        }
    }

    #[inline(always)]
    pub unsafe fn get_row_unchecked(self, index: usize) -> Vector<C, T, A> {
        unsafe {
            match self.resolve_major_axis() {
                MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                    Vector::from_fn(|column_index| mat.inner[column_index].get_unchecked(index))
                }
                MajorAxisResolvedMatrix::RowMajor(mat) => *mat.inner.get_unchecked(index),
            }
        }
    }
    #[inline(always)]
    pub unsafe fn get_row_array_unchecked(self, index: usize) -> [T; C] {
        unsafe {
            match self.resolve_major_axis() {
                MajorAxisResolvedMatrix::ColumnMajor(mat) => {
                    array::from_fn(|column_index| mat.inner[column_index].get_unchecked(index))
                }
                MajorAxisResolvedMatrix::RowMajor(mat) => {
                    mat.inner.get_unchecked(index).into_array()
                }
            }
        }
    }
}
