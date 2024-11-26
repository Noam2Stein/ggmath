use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where
    ScalarCount<N>: VecLen
{
}
