use std::ops::{Add, Mul};

use super::*;

impl<const N: usize, T: Scalar + Add<Output = T> + Mul<Output = T>, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn dot(self, other: Vector<N, T, impl VecAlignment>) -> T {
        T::vector_dot(self, other)
    }
}
