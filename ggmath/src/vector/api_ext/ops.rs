use std::ops::*;

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    #[inline(always)]
    pub fn sum(self) -> T
    where
        T: Add<Output = T>,
    {
        T::vector_sum(self)
    }

    #[inline(always)]
    pub fn dot(self, other: Vector<N, T, impl VecAlignment>) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        T::vector_dot(self, other)
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        T::vector_cross(self, other)
    }
}

scalar_defaults_macro!(
    scalar_defaults_vector_ext_ops:

    #[inline(always)]
    fn vector_sum<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Self
    where
        MaybeVecLen<N>: VecLen,
        Self: Add<Output = Self>,
    {
        vec.fold(|a, b| a + b)
    }

    #[inline(always)]
    fn vector_dot<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        MaybeVecLen<N>: VecLen,
        Self: Add<Output = Self> + Mul<Output = Self>,
    {
        vec.map_rhs(other, |a, b| a * b).sum()
    }

    #[inline(always)]
    fn vector_cross<A: VecAlignment>(
        vec: Vector<3, Self, A>,
        other: Vector<3, Self, impl VecAlignment>,
    ) -> Vector<3, Self, A>
    where
        Self: Mul<Output = Self> + Sub<Output = Self>,
    {
        (vec.zxy() * other - vec * other.zxy()).zxy()
    }
);
