use newnum::AbsDiff;

use super::*;

impl<const N: usize, T: Scalar + AbsDiff<Output: Scalar>, A: VecAlignment> AbsDiff
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn abs_diff(self, rhs: Self) -> Vector<N, T::Output, A> {
        T::vector_abs_diff(self, rhs)
    }
}
