use super::*;

impl<
        const C: usize,
        const R: usize,
        T: ScalarPartialEq<Rhs>,
        A: VecAlignment,
        M: MatrixMajorAxis,
        Rhs: Scalar,
    > PartialEq<Matrix<C, R, Rhs, A, M>> for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    fn eq(&self, other: &Matrix<C, R, Rhs, A, M>) -> bool {
        M::eq(self, other)
    }
}

impl<
        const C: usize,
        const R: usize,
        T: ScalarPartialEq<T> + Eq,
        A: VecAlignment,
        M: MatrixMajorAxis,
    > Eq for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
}
