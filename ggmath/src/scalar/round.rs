use super::*;

pub trait ScalarRound: Scalar {
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;

    #[inline(always)]
    fn vector_floor<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.floor())
    }
    #[inline(always)]
    fn vector_ceil<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.ceil())
    }
    #[inline(always)]
    fn vector_round<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.round())
    }
    #[inline(always)]
    fn vector_trunc<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.trunc())
    }
}

impl<const N: usize, T: ScalarRound, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn floor(self) -> Self {
        T::vector_floor(self)
    }
    #[inline(always)]
    pub fn ceil(self) -> Self {
        T::vector_ceil(self)
    }
    #[inline(always)]
    pub fn round(self) -> Self {
        T::vector_round(self)
    }
    #[inline(always)]
    pub fn trunc(self) -> Self {
        T::vector_trunc(self)
    }
}
