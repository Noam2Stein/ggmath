use super::*;

impl<const N: usize, S: VecStorage, T: Scalar> Clone for Vector<N, S, T>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, S: VecStorage, T: Scalar> Copy for Vector<N, S, T> where
    ScalarCount<N>: VecLen<N>
{
}
