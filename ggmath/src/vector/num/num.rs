use newnum::{
    ATrig, ATrigH, AbsDiff, Cbrt, Negative, Positive, Round, Sign, Sqrt, Trig, TrigH, Zero,
};

use super::{scalar_defaults_macro, Scalar, ScalarCount, VecAlignment, VecLen, Vector};

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
