use std::ops::Mul;

use super::*;

pub trait ScalarDot<Rhs: Scalar = Self>: ScalarMul<Rhs, Output: ScalarCSum> {
    #[inline(always)]
    fn vector_dot<const N: usize>(
        vec: Vector<N, Self, impl VecAlignment>,
        other: Vector<N, Rhs, impl VecAlignment>,
    ) -> <Self as Mul<Rhs>>::Output
    where
        ScalarCount<N>: VecLen,
    {
        (vec * other).csum()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn dot<TRhs: Scalar>(self, other: Vector<N, TRhs, impl VecAlignment>) -> T::Output
    where
        T: ScalarDot<TRhs>,
    {
        T::vector_dot(self, other)
    }
}
