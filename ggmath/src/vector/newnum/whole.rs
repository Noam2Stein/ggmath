use super::*;

impl<const N: usize, T: Scalar + WholeEquivalent<Whole: Scalar>, A: VecAlignment> WholeEquivalent
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Whole = Vector<N, T::Whole, A>;

    fn ifloor(self) -> Vector<N, T::Whole, A> {
        T::vector_ifloor(self)
    }
    fn iceil(self) -> Vector<N, T::Whole, A> {
        T::vector_iceil(self)
    }
    fn iround(self) -> Vector<N, T::Whole, A> {
        T::vector_iround(self)
    }
    fn itrunc(self) -> Vector<N, T::Whole, A> {
        T::vector_itrunc(self)
    }
    fn iatrunc(self) -> Vector<N, T::Whole, A> {
        T::vector_iatrunc(self)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_whole_equivalent:

    #[inline(always)]
    fn vector_iround<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: WholeEquivalent<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(WholeEquivalent::iround)
    }

    #[inline(always)]
    fn vector_ifloor<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: WholeEquivalent<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(WholeEquivalent::ifloor)
    }

    #[inline(always)]
    fn vector_iceil<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: WholeEquivalent<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(WholeEquivalent::iceil)
    }

    #[inline(always)]
    fn vector_itrunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: WholeEquivalent<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(WholeEquivalent::itrunc)
    }

    #[inline(always)]
    fn vector_iatrunc<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Whole, A>
    where
        Self: WholeEquivalent<Whole: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(WholeEquivalent::iatrunc)
    }
}
