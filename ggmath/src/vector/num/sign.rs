use newnum::{Negative, Positive, Sign, Zero};

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Sign<Bool = bool>, A: VecAlignment> Sign for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Bool = Vector<N, bool, A>;

    #[inline(always)]
    fn is_negative(&self) -> Self::Bool {
        self.map_ref(Sign::is_negative)
    }
    #[inline(always)]
    fn is_positive(&self) -> Self::Bool {
        self.map_ref(Sign::is_positive)
    }
    #[inline(always)]
    fn is_zero(&self) -> Self::Bool {
        self.map_ref(Sign::is_zero)
    }

    #[inline(always)]
    fn signum(self) -> Self {
        self.map(Sign::signum)
    }
}

impl<const N: usize, T: Scalar + Positive<Bool = bool>, A: VecAlignment> Positive
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn abs(self) -> Self {
        self.map(Positive::abs)
    }
}

impl<const N: usize, T: Scalar + Negative<Bool = bool>, A: VecAlignment> Negative
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn neg_abs(self) -> Self {
        self.map(Negative::neg_abs)
    }
}

impl<const N: usize, T: Scalar + Zero<Bool = bool>, A: VecAlignment> Zero for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn zero() -> Self {
        Vector::splat(T::zero())
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_sign:


}
