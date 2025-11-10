use crate::{ScalarBackend, SupportedVecLen, VecLen};

impl<const N: usize> ScalarBackend<N> for isize
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [isize; N];
}
