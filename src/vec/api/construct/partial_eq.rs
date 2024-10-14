use super::*;

impl<const N: usize, T: Scalar, S: VecStorage> PartialEq for Vector<N, T, S>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
