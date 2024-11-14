use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> PartialEq for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
