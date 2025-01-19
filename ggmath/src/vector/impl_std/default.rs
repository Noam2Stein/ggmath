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

scalar_defaults_macro! {
    scalar_defaults_vector_default:

    #[inline(always)]
    fn vector_default<const N: usize, A: VecAlignment>() -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: Default,
    {
        Vector::splat(Self::default())
    }
}
