use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> PartialEq for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.into_array().eq(&other.into_array())
    }
}
