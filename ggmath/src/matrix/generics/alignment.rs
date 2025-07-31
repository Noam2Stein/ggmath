use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Aligns the matrix to [`VecAligned`] vectors.
    #[inline(always)]
    pub fn align(self) -> Matrix<C, R, T, VecAligned, M> {
        self.to_storage()
    }

    /// Unaligns the matrix to [`VecPacked`] vectors.
    ///
    /// This is always a zero cost operation.
    #[inline(always)]
    pub fn unalign(self) -> Matrix<C, R, T, VecAligned, M> {
        self.to_storage()
    }
}
