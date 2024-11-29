use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    /// Creates a vector where all elements have the same given value.
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        T::vector_splat(value)
    }
}
