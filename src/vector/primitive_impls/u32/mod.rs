use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for u32
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [u32; N];
}
