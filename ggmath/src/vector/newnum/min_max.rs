use super::*;

impl<const N: usize, T: Scalar + PartialOrd, A: VecAlignment> MinMax for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn min(self, other: Self) -> Self {
        T::vector_min(self, other)
    }
    fn max(self, other: Self) -> Self {
        T::vector_max(self, other)
    }
    fn clamp(self, min: Self, max: Self) -> Self {
        T::vector_clamp(self, min, max)
    }
}

impl<const N: usize, T: Scalar + PartialOrd, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    pub fn cmin(self) -> T {
        T::vector_cmin(self)
    }
    pub fn cmax(self) -> T {
        T::vector_cmax(self)
    }
}

scalar_defaults_macro!(
    scalar_defaults_vector_ext_cmp:

    #[inline(always)]
    fn vector_cmin<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Self
    where
        Self: PartialOrd,
        MaybeVecLen<N>: VecLen,
    {
        vec.fold(|a, b| if a < b { a } else { b })
    }
    #[inline(always)]
    fn vector_cmax<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Self
    where
        Self: PartialOrd,
        MaybeVecLen<N>: VecLen,
    {
        vec.fold(|a, b| if a > b { a } else { b })
    }

    fn vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
        Self: PartialOrd,
    {
        vec.map_rhs(other, |a, b| if a < b { a } else { b })
    }
    fn vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
        Self: PartialOrd,
    {
        vec.map_rhs(other, |a, b| if a > b { a } else { b })
    }

    fn vector_clamp<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        min: Vector<N, Self, impl VecAlignment>,
        max: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        MaybeVecLen<N>: VecLen,
        Self: PartialOrd,
    {
        Self::vector_max(Self::vector_min(vec, max), min)
    }
);
