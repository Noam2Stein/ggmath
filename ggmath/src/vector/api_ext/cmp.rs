use super::*;

impl<const N: usize, T: Scalar + PartialOrd, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn cmin(self) -> T {
        T::vector_cmin(self)
    }
    pub fn cmax(self) -> T {
        T::vector_cmax(self)
    }

    pub fn min(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_min(self, other)
    }
    pub fn max(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_max(self, other)
    }

    pub fn clamp(
        self,
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        T::vector_clamp(self, min, max)
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
        ScalarCount<N>: VecLen,
    {
        vec.fold(|a, b| if a < b { a } else { b })
    }
    #[inline(always)]
    fn vector_cmax<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Self
    where
        Self: PartialOrd,
        ScalarCount<N>: VecLen,
    {
        vec.fold(|a, b| if a > b { a } else { b })
    }

    fn vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        vec.map_rhs(other, |a, b| if a < b { a } else { b })
    }
    fn vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
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
        ScalarCount<N>: VecLen,
        Self: PartialOrd,
    {
        vec.min(max).max(min)
    }
);
