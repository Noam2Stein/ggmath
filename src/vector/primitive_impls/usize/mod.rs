use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for usize
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [usize; N];
}
