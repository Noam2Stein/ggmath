use std::ops::Neg;

use super::*;

impl<const N: usize, T: Scalar + Num + Zero, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn zero() -> Self {
        Self::splat(num!(0))
    }
}
impl<const N: usize, T: Scalar + Num + Positive, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn one() -> Self {
        Self::splat(num!(1))
    }
}
impl<const N: usize, T: Scalar + Num + Negative + Neg<Output = T>, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn neg_one() -> Self {
        Self::splat(-num!(1))
    }
}
