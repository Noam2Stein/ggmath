use super::*;

impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn default() -> Self {
        T::vector_default()
    }
}
