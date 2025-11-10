use crate::{ScalarBackend, SupportedVecLen, VecLen, vector::primitive_api::f64_api::FloatBackend};

impl<const N: usize> ScalarBackend<N> for f64
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [f64; N];
}

impl<const N: usize> FloatBackend<N> for f64 where VecLen<N>: SupportedVecLen {}
