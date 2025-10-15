use crate::{Scalar, SimdBehaviour, SupportedVecLen, VecLen};

impl<T: Scalar> Scalar for Option<T> {}

impl<const N: usize, T: Scalar> SimdBehaviour<N> for Option<T>
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [Option<T>; N];
}
