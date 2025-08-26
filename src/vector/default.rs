use super::*;

impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn default() -> Self {
        T::vec_default()
    }
}
