use std::ops::{Index, IndexMut};

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Index<(usize, usize)> for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    type Output = T;

    #[inline(always)]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        match self.resolve_major_axis_ref() {
            MajorAxisResolvedMatrixRef::ColumnMajor(mat) => &mat.inner[index.0][index.1],
            MajorAxisResolvedMatrixRef::RowMajor(mat) => &mat.inner[index.1][index.0],
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    IndexMut<(usize, usize)> for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        match self.resolve_major_axis_mut() {
            MajorAxisResolvedMatrixMut::ColumnMajor(mat) => &mut mat.inner[index.0][index.1],
            MajorAxisResolvedMatrixMut::RowMajor(mat) => &mut mat.inner[index.1][index.0],
        }
    }
}