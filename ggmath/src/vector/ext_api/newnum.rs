use newnum::{
    ATrig, ATrigH, AbsDiff, Cbrt, Negative, Positive, Round, Sign, Sqrt, Trig, TrigH, Zero,
};

use super::{Scalar, ScalarCount, VecAlignment, VecLen, Vector};

impl<const N: usize, T: Scalar + Round<Output: Scalar>, A: VecAlignment> Round for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;

    #[inline(always)]
    fn floor(self) -> Self::Output {
        T::vector_floor(self)
    }
    #[inline(always)]
    fn ceil(self) -> Self::Output {
        T::vector_ceil(self)
    }
    #[inline(always)]
    fn round(self) -> Self::Output {
        T::vector_round(self)
    }
    #[inline(always)]
    fn trunc(self) -> Self::Output {
        T::vector_trunc(self)
    }
    #[inline(always)]
    fn atrunc(self) -> Self::Output {
        T::vector_atrunc(self)
    }
}

impl<const N: usize, T: Scalar + Sqrt<Output: Scalar>, A: VecAlignment> Sqrt for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;
    #[inline(always)]
    fn sqrt(self) -> Self::Output {
        self.map(Sqrt::sqrt)
    }
}
impl<const N: usize, T: Scalar + Cbrt<Output: Scalar>, A: VecAlignment> Cbrt for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    type Output = Vector<N, T::Output, A>;
    #[inline(always)]
    fn cbrt(self) -> Self::Output {
        self.map(Cbrt::cbrt)
    }
}

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
    fn zero() -> Self {
        Vector::splat(T::zero())
    }
}
