use std::ops::Sub;

use super::*;

impl<
    const N: usize,
    T: Scalar + AbsDiff<TRhs, Output: Scalar>,
    A: VecAlignment,
    TRhs: Scalar + Sub<T, Output = T::Output>,
> AbsDiff<Vector<N, TRhs, A>> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn abs_diff(self, rhs: Vector<N, TRhs, A>) -> Vector<N, T::Output, A> {
        T::vector_abs_diff(self, rhs)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_abs_diff:

    #[inline(always)]
    fn vector_abs_diff<const N: usize, A: VecAlignment, TRhs: Scalar>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, TRhs, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: AbsDiff<TRhs, Output: Scalar>,
        TRhs: Sub<Self, Output = Self::Output>,
    {
        vec.map_rhs(rhs, AbsDiff::abs_diff)
    }
}
