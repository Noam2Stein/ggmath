use newnum::{AHyper, ATrig, Hyper, Trig};

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Trig<Output: Scalar>, A: VecAlignment> Trig for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sin(self) -> Self::Output {
        T::vector_sin(self)
    }

    #[inline(always)]
    fn cos(self) -> Self::Output {
        T::vector_cos(self)
    }

    #[inline(always)]
    fn tan(self) -> Self::Output {
        T::vector_tan(self)
    }

    #[inline(always)]
    fn cot(self) -> Self::Output {
        T::vector_cot(self)
    }
}

impl<const N: usize, T: Scalar + ATrig<Output: Scalar>, A: VecAlignment> ATrig for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn asin(self) -> Self::Output {
        T::vector_asin(self)
    }

    #[inline(always)]
    fn acos(self) -> Self::Output {
        T::vector_acos(self)
    }

    #[inline(always)]
    fn atan(self) -> Self::Output {
        T::vector_atan(self)
    }

    #[inline(always)]
    fn acot(self) -> Self::Output {
        T::vector_acot(self)
    }
}

impl<const N: usize, T: Scalar + Hyper<Output: Scalar>, A: VecAlignment> Hyper for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sinh(self) -> Self::Output {
        T::vector_sinh(self)
    }

    #[inline(always)]
    fn cosh(self) -> Self::Output {
        T::vector_cosh(self)
    }

    #[inline(always)]
    fn tanh(self) -> Self::Output {
        T::vector_tanh(self)
    }

    #[inline(always)]
    fn coth(self) -> Self::Output {
        T::vector_coth(self)
    }
}

impl<const N: usize, T: Scalar + AHyper<Output: Scalar>, A: VecAlignment> AHyper for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn asinh(self) -> Self::Output {
        T::vector_asinh(self)
    }

    #[inline(always)]
    fn acosh(self) -> Self::Output {
        T::vector_acosh(self)
    }

    #[inline(always)]
    fn atanh(self) -> Self::Output {
        T::vector_atanh(self)
    }

    #[inline(always)]
    fn acoth(self) -> Self::Output {
        T::vector_acoth(self)
    }
}

scalar_defaults_macro! {
    scalar_defaults_vector_trig:

    #[inline(always)]
    fn vector_sin<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::sin)
    }

    #[inline(always)]
    fn vector_cos<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::cos)
    }

    #[inline(always)]
    fn vector_tan<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::tan)
    }

    #[inline(always)]
    fn vector_cot<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Trig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Trig::cot)
    }

    // Vector: ATrig

    #[inline(always)]
    fn vector_asin<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::asin)
    }

    #[inline(always)]
    fn vector_acos<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::acos)
    }

    #[inline(always)]
    fn vector_atan<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::atan)
    }

    #[inline(always)]
    fn vector_acot<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrig<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrig::acot)
    }

    // Vector: Hyper

    #[inline(always)]
    fn vector_sinh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Hyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Hyper::sinh)
    }

    #[inline(always)]
    fn vector_cosh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Hyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Hyper::cosh)
    }

    #[inline(always)]
    fn vector_tanh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Hyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Hyper::tanh)
    }

    #[inline(always)]
    fn vector_coth<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: Hyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(Hyper::coth)
    }

    // Vector: AHyper

    #[inline(always)]
    fn vector_asinh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: AHyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(AHyper::asinh)
    }

    #[inline(always)]
    fn vector_acosh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: AHyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(AHyper::acosh)
    }

    #[inline(always)]
    fn vector_atanh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: AHyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(AHyper::atanh)
    }

    #[inline(always)]
    fn vector_acoth<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: AHyper<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(AHyper::acoth)
    }
}
