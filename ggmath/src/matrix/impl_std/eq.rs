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
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Matrix<C, R, TRhs, ARhs, MRhs>) -> bool {
        match self.resolve() {
            ResolvedMatrix::ColumnMajor(mat) => mat
                .inner
                .iter()
                .zip(other.to_c_major().inner.iter())
                .all(|(a, b)| *a == *b),

            ResolvedMatrix::RowMajor(mat) => mat
                .inner
                .iter()
                .zip(other.to_r_major().inner.iter())
                .all(|(a, b)| a == b),
        }
    }
}

impl<const C: usize, const R: usize, T: Scalar + Eq, A: VecAlignment, M: MatrixMajorAxis> Eq
    for Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
}
