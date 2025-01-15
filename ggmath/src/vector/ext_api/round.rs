use newnum::Round;

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector};

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
    fn atrunc(self) -> Self {
        T::vector_atrunc(self)
    }
}
