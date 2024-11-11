use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> PartialEq
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
