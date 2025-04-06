use newnum::*;

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector, scalar_defaults_macro};

impl<const N: usize, T: Scalar + Sign<BoolMapped = bool>, A: VecAlignment> Sign for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type BoolMapped = Vector<N, T::BoolMapped, A>;

    #[inline(always)]
    fn is_positive(&self) -> Self::BoolMapped {
        T::vector_is_positive(self)
    }

    #[inline(always)]
    fn is_negative(&self) -> Self::BoolMapped {
        T::vector_is_negative(self)
    }

    #[inline(always)]
    fn is_zero(&self) -> Self::BoolMapped {
        T::vector_is_zero(self)
    }

    #[inline(always)]
    fn is_sign_positive(&self) -> Self::BoolMapped {
        T::vector_is_sign_positive(self)
    }

    #[inline(always)]
    fn is_sign_negative(&self) -> Self::BoolMapped {
        T::vector_is_sign_negative(self)
    }
}

impl<const N: usize, T: Scalar + Positive<BoolMapped = bool>, A: VecAlignment> Positive
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn abs(self) -> Self {
        T::vector_abs(self)
    }
}

impl<const N: usize, T: Scalar + Negative<BoolMapped = bool>, A: VecAlignment> Negative
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn neg_abs(self) -> Self {
        T::vector_neg_abs(self)
    }
}

impl<const N: usize, T: Scalar + Zero<BoolMapped = bool>, A: VecAlignment> Zero for Vector<N, T, A> where
    ScalarCount<N>: VecLen
{
}

impl<const N: usize, T: Scalar + Signum<BoolMapped = bool> + Zero, A: VecAlignment> Signum
    for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    fn signum(self) -> Self
    where
        Self: Zero,
    {
        T::vector_signum(self)
    }

    fn signumf(self) -> Self {
        T::vector_signumf(self)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_sign:

    #[inline(always)]
    fn vector_is_positive<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map_ref(Sign::is_negative)
    }

    #[inline(always)]
    fn vector_is_negative<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map_ref(Sign::is_negative)
    }

    #[inline(always)]
    fn vector_is_zero<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map_ref(Sign::is_zero)
    }

    #[inline(always)]
    fn vector_is_sign_positive<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map_ref(Sign::is_sign_positive)
    }

    #[inline(always)]
    fn vector_is_sign_negative<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map_ref(Sign::is_sign_negative)
    }

    #[inline(always)]
    fn vector_abs<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Positive<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Positive::abs)
    }

    #[inline(always)]
    fn vector_neg_abs<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Negative<BoolMapped: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Negative::neg_abs)
    }

    #[inline(always)]
    fn vector_signum<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Signum<BoolMapped = bool> + Zero,
        ScalarCount<N>: VecLen,
    {
        vec.map(Signum::signum)
    }

    #[inline(always)]
    fn vector_signumf<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Signum<BoolMapped = bool>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Signum::signumf)
    }
}
