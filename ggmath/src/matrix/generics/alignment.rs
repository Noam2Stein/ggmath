use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    pub fn align(self) -> Matrix<C, R, T, VecAligned, M> {
        self.to_storage()
    }
    #[inline(always)]
    pub fn unalign(self) -> Matrix<C, R, T, VecAligned, M> {
        self.to_storage()
    }
}
