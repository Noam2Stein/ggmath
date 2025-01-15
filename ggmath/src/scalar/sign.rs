use super::*;

pub trait ScalarSigned: Scalar {
    fn is_positive(self) -> bool;
    fn is_negative(self) -> bool;
    fn signum(self) -> Self;
    fn abs(self) -> Self;

    #[inline(always)]
    fn vector_c_are_positive<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, bool, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.is_positive())
    }
    #[inline(always)]
    fn vector_c_are_negative<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, bool, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.is_negative())
    }

    #[inline(always)]
    fn vector_is_positive<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> bool
    where
        ScalarCount<N>: VecLen,
    {
        vec.iter().all(|c| c.is_positive())
    }
    #[inline(always)]
    fn vector_is_negative<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> bool
    where
        ScalarCount<N>: VecLen,
    {
        vec.iter().all(|c| c.is_negative())
    }

    #[inline(always)]
    fn vector_signum<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.signum())
    }

    #[inline(always)]
    fn vector_abs<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.map(|c| c.abs())
    }
}

impl<const N: usize, T: ScalarSigned, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn c_are_positive(self) -> Vector<N, bool, A> {
        T::vector_c_are_positive(self)
    }
    #[inline(always)]
    pub fn c_are_negative(self) -> Vector<N, bool, A> {
        T::vector_c_are_negative(self)
    }

    #[inline(always)]
    pub fn is_positive(self) -> bool {
        T::vector_is_positive(self)
    }
    #[inline(always)]
    pub fn is_negative(self) -> bool {
        T::vector_is_negative(self)
    }

    #[inline(always)]
    pub fn signum(self) -> Self {
        T::vector_signum(self)
    }
    #[inline(always)]
    pub fn abs(self) -> Self {
        T::vector_abs(self)
    }
}
