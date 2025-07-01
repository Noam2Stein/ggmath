use super::*;

impl<const N: usize, T: Scalar + Default, A: VecAlignment> Default for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    fn default() -> Self {
        Vector::splat(T::default())
    }
}
