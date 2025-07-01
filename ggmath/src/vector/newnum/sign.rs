use super::*;

impl<const N: usize, T: Scalar + Sign<BoolMapped = bool>, A: VecAlignment> Sign for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    type BoolMapped = Vector<N, T::BoolMapped, A>;

    fn is_positive(&self) -> Self::BoolMapped {
        T::vector_is_positive(self)
    }
    fn is_negative(&self) -> Self::BoolMapped {
        T::vector_is_negative(self)
    }

    fn is_zero(&self) -> Self::BoolMapped {
        T::vector_is_zero(self)
    }

    fn is_bin_positive(&self) -> Self::BoolMapped {
        T::vector_is_sign_positive(self)
    }
    fn is_bin_negative(&self) -> Self::BoolMapped {
        T::vector_is_sign_negative(self)
    }
}

impl<const N: usize, T: Scalar + Positive<BoolMapped = bool>, A: VecAlignment> Positive
    for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn abs(self) -> Self {
        T::vector_abs(self)
    }
}

impl<const N: usize, T: Scalar + Negative<BoolMapped = bool>, A: VecAlignment> Negative
    for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn neg_abs(self) -> Self {
        T::vector_neg_abs(self)
    }
}

impl<const N: usize, T: Scalar + Zero<BoolMapped = bool>, A: VecAlignment> Zero for Vector<N, T, A> where
    MaybeVecLen<N>: VecLen
{
}

impl<const N: usize, T: Scalar + Signum<BoolMapped = bool> + Zero, A: VecAlignment> Signum
    for Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    fn signumt(self) -> Self
    where
        Self: Zero,
    {
        T::vector_signumt(self)
    }

    fn bin_signum(self) -> Self {
        T::vector_bin_signum(self)
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
        MaybeVecLen<N>: VecLen,
    {
        vec.map_ref(Sign::is_negative)
    }

    #[inline(always)]
    fn vector_is_negative<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map_ref(Sign::is_negative)
    }

    #[inline(always)]
    fn vector_is_zero<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map_ref(Sign::is_zero)
    }

    #[inline(always)]
    fn vector_is_sign_positive<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map_ref(Sign::is_bin_positive)
    }

    #[inline(always)]
    fn vector_is_sign_negative<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
    ) -> Vector<N, Self::BoolMapped, A>
    where
        Self: Sign<BoolMapped: Scalar>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map_ref(Sign::is_bin_negative)
    }

    #[inline(always)]
    fn vector_abs<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Positive<BoolMapped: Scalar>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map(Positive::abs)
    }

    #[inline(always)]
    fn vector_neg_abs<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Negative<BoolMapped: Scalar>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map(Negative::neg_abs)
    }

    #[inline(always)]
    fn vector_signumt<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Signum<BoolMapped = bool> + Zero,
        MaybeVecLen<N>: VecLen,
    {
        vec.map(Signum::signumt)
    }

    #[inline(always)]
    fn vector_bin_signum<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self, A>
    where
        Self: Signum<BoolMapped = bool>,
        MaybeVecLen<N>: VecLen,
    {
        vec.map(Signum::bin_signum)
    }
}
