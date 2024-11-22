use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Clone
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Copy
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
}
