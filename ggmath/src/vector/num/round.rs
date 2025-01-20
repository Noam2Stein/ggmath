use newnum::{IRound, Round};

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

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

impl<const N: usize, T: Scalar + IRound<Whole: Scalar>, A: VecAlignment> IRound for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn ifloor(self) -> Vector<N, T::Whole, A> {
        T::vector_ifloor(self)
    }

    #[inline(always)]
    fn iceil(self) -> Vector<N, T::Whole, A> {
        T::vector_iceil(self)
    }

    #[inline(always)]
    fn iround(self) -> Vector<N, T::Whole, A> {
        T::vector_iround(self)
    }

    #[inline(always)]
    fn itrunc(self) -> Vector<N, T::Whole, A> {
        T::vector_itrunc(self)
    }

    #[inline(always)]
    fn iatrunc(self) -> Vector<N, T::Whole, A> {
        T::vector_iatrunc(self)
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

    // IRound

    #[inline(always)]
    fn vector_iround<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: IRound<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(IRound::iround)
    }

    #[inline(always)]
    fn vector_ifloor<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: IRound<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(IRound::ifloor)
    }

    #[inline(always)]
    fn vector_iceil<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: IRound<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(IRound::iceil)
    }

    #[inline(always)]
    fn vector_itrunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: IRound<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(IRound::itrunc)
    }

    #[inline(always)]
    fn vector_iatrunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: IRound<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(IRound::iatrunc)
    }
}
