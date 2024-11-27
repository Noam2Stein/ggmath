use super::*;

impl<const N: usize, T: Scalar + PartialOrd, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn cmin(self) -> T {
        T::vector_cmin(self)
    }

    #[inline(always)]
    pub fn cmax(self) -> T {
        T::vector_cmax(self)
    }

    #[inline(always)]
    pub fn min(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_min(self, other)
    }

    #[inline(always)]
    pub fn max(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_max(self, other)
    }

    #[inline(always)]
    pub fn clamp(
        self,
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        T::vector_clamp(self, min, max)
    }
}
