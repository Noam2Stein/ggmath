use super::*;

impl<
    const C: usize,
    const R: usize,
    T: Scalar + PartialEq<TRhs>,
    A: VecAlignment,
    M: MatrixMajorAxis,
    TRhs: Scalar,
    ARhs: VecAlignment,
    MRhs: MatrixMajorAxis,
> PartialEq<Matrix<C, R, TRhs, ARhs, MRhs>> for Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Matrix<C, R, TRhs, ARhs, MRhs>) -> bool {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(mat) => mat.inner.eq(&other.to_c_major().inner),
            ResolvedMatrix::RowMajor(mat) => mat.inner.eq(&other.to_r_major().inner),
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar + Eq, A: VecAlignment, M: MatrixMajorAxis> Eq
    for Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
}
