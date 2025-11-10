use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for u64
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [u64; N];
}
