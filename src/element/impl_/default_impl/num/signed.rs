use super::*;

impl<T: ElementDefaultImpl + NumElement + Signed> SignedElement for T {
    #[inline(always)]
    fn neg_one() -> Self {
        -Self::one()
    }
    fn abs(self) -> Self {
        <Self as Signed>::abs(&self)
    }
    fn signum(self) -> Self {
        <Self as Signed>::signum(&self)
    }
    fn is_positive(self) -> bool {
        <Self as Signed>::is_positive(&self)
    }
    fn is_negative(self) -> bool {
        <Self as Signed>::is_negative(&self)
    }
}
impl<const N: usize, T: ElementDefaultImpl + NumElement + Signed> SignedElementVecFns<N> for T
where
    MaybeVecNum<N>: VecNum,
{
    fn neg_one() -> VecN<N, Self> {
        -Self::one()
    }
    fn abs(value: VecN<N, Self>) -> VecN<N, Self> {
        value.map(|c| c.abs())
    }
    fn signum(value: VecN<N, Self>) -> VecN<N, Self> {
        value.map(|c| c.signum())
    }
    fn are_positive(value: VecN<N, Self>) -> VecN<N, bool> {
        value.map(|c| c.is_positive())
    }
    fn are_negative(value: VecN<N, Self>) -> VecN<N, bool> {
        value.map(|c| c.is_negative())
    }
}
