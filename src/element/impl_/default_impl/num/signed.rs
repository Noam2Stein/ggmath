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
impl<T: ElementDefaultImpl + NumElement + Signed, const N: usize, V: VecN<T, N>>
    SignedElementVecFns<N, V> for T
{
    fn abs(value: V) -> V {
        value.map(|c| c.abs())
    }
}
