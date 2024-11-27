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
