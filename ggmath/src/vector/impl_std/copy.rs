use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            array: self.array,
            alignent: self.alignent,
        }
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where
    MaybeVecLen<N>: VecLen
{
}
