use std::hash::Hash;

use super::*;

impl<const C: usize, const R: usize, T: Scalar + Hash, A: VecAlignment, M: MatMajorAxis> Hash
    for Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(mat) => mat.inner.hash(state),
            ResolvedMatrix::RowMajor(mat) => mat.inner.hash(state),
        }
    }
}
