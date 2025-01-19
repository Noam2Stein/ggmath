use newnum::Round;

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Round<Output: Scalar>, A: VecAlignment> Round for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn floor(self) -> Self::Output {
        T::vector_floor(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self::Output {
        T::vector_ceil(self)
    }
    #[inline(always)]
    fn round(self) -> Self::Output {
        T::vector_round(self)
    }
    #[inline(always)]
    fn trunc(self) -> Self::Output {
        T::vector_trunc(self)
    }
    #[inline(always)]
    fn atrunc(self) -> Self::Output {
        T::vector_atrunc(self)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_round:

    #[inline(always)]
    fn vector_round<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Round<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::round)
    }

    #[inline(always)]
    fn vector_floor<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Round<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::floor)
    }

    #[inline(always)]
    fn vector_ceil<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Round<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::ceil)
    }

    #[inline(always)]
    fn vector_trunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Round<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::trunc)
    }

    #[inline(always)]
    fn vector_atrunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Round<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::atrunc)
    }
}
