use newnum::{ATrig, ATrigH, Trig, TrigH};

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Trig<Output: Scalar>, A: VecAlignment> Trig for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sin(self) -> Self::Output {
        self.map(Trig::sin)
    }
    #[inline(always)]
    fn cos(self) -> Self::Output {
        self.map(Trig::cos)
    }
    #[inline(always)]
    fn tan(self) -> Self::Output {
        self.map(Trig::tan)
    }
    #[inline(always)]
    fn cot(self) -> Self::Output {
        self.map(Trig::cot)
    }
}

impl<const N: usize, T: Scalar + ATrig<Output: Scalar>, A: VecAlignment> ATrig for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn asin(self) -> Self::Output {
        self.map(ATrig::asin)
    }
    #[inline(always)]
    fn acos(self) -> Self::Output {
        self.map(ATrig::acos)
    }
    #[inline(always)]
    fn atan(self) -> Self::Output {
        self.map(ATrig::atan)
    }
    #[inline(always)]
    fn acot(self) -> Self::Output {
        self.map(ATrig::acot)
    }
}

impl<const N: usize, T: Scalar + TrigH<Output: Scalar>, A: VecAlignment> TrigH for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn sinh(self) -> Self::Output {
        self.map(TrigH::sinh)
    }
    #[inline(always)]
    fn cosh(self) -> Self::Output {
        self.map(TrigH::cosh)
    }
    #[inline(always)]
    fn tanh(self) -> Self::Output {
        self.map(TrigH::tanh)
    }
    #[inline(always)]
    fn coth(self) -> Self::Output {
        self.map(TrigH::coth)
    }
}

impl<const N: usize, T: Scalar + ATrigH<Output: Scalar>, A: VecAlignment> ATrigH for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn asinh(self) -> Self::Output {
        self.map(ATrigH::asinh)
    }
    #[inline(always)]
    fn acosh(self) -> Self::Output {
        self.map(ATrigH::acosh)
    }
    #[inline(always)]
    fn atanh(self) -> Self::Output {
        self.map(ATrigH::atanh)
    }
    #[inline(always)]
    fn acoth(self) -> Self::Output {
        self.map(ATrigH::acoth)
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

    // Vector: TrigH

    #[inline(always)]
    fn vector_sinh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::sinh)
    }

    #[inline(always)]
    fn vector_cosh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::cosh)
    }

    #[inline(always)]
    fn vector_tanh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::tanh)
    }

    #[inline(always)]
    fn vector_coth<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: TrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(TrigH::coth)
    }

    // Vector: ATrigH

    #[inline(always)]
    fn vector_asinh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::asinh)
    }

    #[inline(always)]
    fn vector_acosh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::acosh)
    }

    #[inline(always)]
    fn vector_atanh<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::atanh)
    }

    #[inline(always)]
    fn vector_acoth<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
    ) -> Vector<N, Self::Output, A>
    where
        Self: ATrigH<Output: Scalar>,
        ScalarCount<N>: VecLen,
    {
        vec.map(ATrigH::acoth)
    }
}
