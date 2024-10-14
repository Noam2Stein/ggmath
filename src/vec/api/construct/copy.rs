use super::*;

impl<const N: usize, T: Scalar, S: VecStorage> Clone for Vector<N, T, S>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: Scalar, S: VecStorage> Copy for Vector<N, T, S> where
    ScalarCount<N>: VecLen<N>
{
}
