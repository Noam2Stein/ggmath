use crate::{ScalarBackend, SupportedVecLen, VecLen, vector::primitive_api::f32_api::FloatBackend};

impl<const N: usize> ScalarBackend<N> for f32
where
    VecLen<N>: SupportedVecLen,
{
    type VectorRepr = [f32; N];
}

impl<const N: usize> FloatBackend<N> for f32 where VecLen<N>: SupportedVecLen {}
