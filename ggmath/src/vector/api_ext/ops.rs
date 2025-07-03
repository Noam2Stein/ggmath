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
        self.fold(|a, b| a + b)
    }

    #[inline(always)]
    pub fn dot(self, other: Vector<N, T, impl VecAlignment>) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.map_rhs(other, |a, b| a * b).sum()
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        (self.zxy() * other - self * other.zxy()).zxy()
    }
}
