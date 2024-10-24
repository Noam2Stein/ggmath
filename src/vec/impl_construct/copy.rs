use super::*;

impl<const N: usize, T: Scalar, S: VecAlignment> Clone for Vector<N, T, S>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: Scalar, S: VecAlignment> Copy for Vector<N, T, S> where
    ScalarCount<N>: VecLen<N>
{
}
