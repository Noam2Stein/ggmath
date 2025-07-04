use super::*;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Clone for Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Copy for Rectangle<N, T, A, R> where
    MaybeVecLen<N>: VecLen
{
}
