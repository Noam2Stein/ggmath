use super::*;

impl<const N: usize, T: Scalar + Round, A: VecAlignment> Round for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn floor(self) -> Self {
        T::vector_floor(self)
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        T::vector_ceil(self)
    }

    #[inline(always)]
    fn round(self) -> Self {
        T::vector_round(self)
    }

    #[inline(always)]
    fn trunc(self) -> Self {
        T::vector_trunc(self)
    }

    #[inline(always)]
    fn atrunc(self) -> Self {
        T::vector_atrunc(self)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_round:

    // Round

    #[inline(always)]
    fn vector_round<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Round,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::round)
    }

    #[inline(always)]
    fn vector_floor<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Round,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::floor)
    }

    #[inline(always)]
    fn vector_ceil<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Round,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::ceil)
    }

    #[inline(always)]
    fn vector_trunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Round,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::trunc)
    }

    #[inline(always)]
    fn vector_atrunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Round,
        ScalarCount<N>: VecLen,
    {
        vec.map(Round::atrunc)
    }
}
