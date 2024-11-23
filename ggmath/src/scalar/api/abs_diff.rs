use super::*;

pub trait ScalarAbsDiff<Rhs: Scalar = Self>: Scalar + AbsDiff<Rhs, Output: Scalar> {
    #[inline(always)]
    fn vector_abs_diff<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
        rhs: &Vector<N, Rhs, A>,
    ) -> Vector<N, Self::Output, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_fn(|i| vec[i].abs_diff(&rhs[i]))
    }
}

impl<const N: usize, T: ScalarAbsDiff<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
    AbsDiff<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn abs_diff(&self, rhs: &Vector<N, TRhs, ARhs>) -> Self::Output {
        T::vector_abs_diff(self, rhs)
    }
}
