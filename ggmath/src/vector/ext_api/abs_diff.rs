use std::ops::Sub;

use super::*;

impl<const N: usize, T: Scalar + PartialOrd + Sub<Output: Scalar>, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn abs_diff(self, rhs: Vector<N, T, impl VecAlignment>) -> Vector<N, T::Output, A> {
        T::vector_abs_diff(self, rhs)
    }
}
