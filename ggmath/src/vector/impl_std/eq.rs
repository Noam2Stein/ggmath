use super::*;

impl<
        const N: usize,
        T: Scalar + PartialEq<TRhs>,
        A: VecAlignment,
        TRhs: Scalar,
        ARhs: VecAlignment,
    > PartialEq<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, TRhs, ARhs>) -> bool {
        T::vector_eq(&self, other)
    }
}

impl<const N: usize, T: Scalar + Eq, A: VecAlignment> Eq for Vector<N, T, A> where
    ScalarCount<N>: VecLen
{
}

scalar_defaults_macro! {
    scalar_defaults_vector_eq:

    #[inline(always)]
    fn vector_eq<const N: usize, TRhs: Scalar>(
        vec: &Vector<N, Self, impl VecAlignment>,
        other: &Vector<N, TRhs, impl VecAlignment>,
    ) -> bool
    where
        ScalarCount<N>: VecLen,
        Self: PartialEq<TRhs>,
    {
        (0..N).all(|i| vec[i].eq(&other[i]))
    }
}
