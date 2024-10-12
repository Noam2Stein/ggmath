use super::*;

impl<const N: usize, S: VecStorage, T: Scalar> PartialEq for Vector<N, S, T>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
