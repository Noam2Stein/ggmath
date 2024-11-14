use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Clone for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Copy for Vector<N, T, A> where
    ScalarCount<N>: VecLen<N>
{
}
