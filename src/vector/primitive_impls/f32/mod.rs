use crate::{
    Aligned, Length, ScalarBackend, SupportedLength, Unaligned,
    vector::primitive_apis::f32::FloatBackend,
};

#[cfg(not(any(target_feature = "sse")))]
mod v128_fallback;
#[cfg(target_feature = "sse")]
mod v128_sse;

impl<const N: usize> ScalarBackend<N, Unaligned> for f32
where
    Length<N>: SupportedLength,
{
    type VectorRepr = [f32; N];
}

impl ScalarBackend<2, Aligned> for f32 {
    type VectorRepr = [f32; 2];
}

impl<const N: usize> FloatBackend<N, Unaligned> for f32 where Length<N>: SupportedLength {}

impl FloatBackend<2, Aligned> for f32 {}
