use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        T::vector_splat(value)
    }
}
