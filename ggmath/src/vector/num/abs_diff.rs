use newnum::AbsDiff;

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

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

scalar_defaults_macro! {
    scalar_defaults_vector_abs_diff:

    #[inline(always)]
    fn vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        rhs: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
        Self: AbsDiff<Output: Scalar>,
    {
        Vector::from_fn(|i| vec[i].abs_diff(rhs[i]))
    }
}
