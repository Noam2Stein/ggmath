use super::*;

impl<const N: usize, T: Scalar, S: VecStorage> PartialEq for Vector<N, T, S>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.into_array().eq(&other.into_array())
    }
}
