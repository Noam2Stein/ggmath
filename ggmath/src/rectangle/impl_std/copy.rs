use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Clone for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Copy for Rectangle<N, T, A, R> where
    ScalarCount<N>: VecLen<N>
{
}
