use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for i32
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [i32; N];
}
