use super::*;

impl<const N: usize, T: Scalar + Add<Output = T>, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn csum(self) -> T {
        T::vector_csum(self)
    }
}
