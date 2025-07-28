use super::*;

// cannot be derived because we want more impls than just PartialEq<Self>.

impl<const N: usize, T: Scalar + PartialEq<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
    PartialEq<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, TRhs, ARhs>) -> bool {
        self.array
            .iter()
            .zip(other.array.iter())
            .all(|(a, b)| *a == *b)
    }
}
