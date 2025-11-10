use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for i64
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [i64; N];
}
