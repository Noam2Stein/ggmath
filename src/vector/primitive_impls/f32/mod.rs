use crate::{Scalar, Simd, SimdBehaviour, vector::primitive_api::F32VectorApi};

#[cfg(target_feature = "sse")]
mod b128_sse;

#[cfg(not(target_feature = "sse"))]
mod b128_fallback;

impl Scalar for f32 {}

// FVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<2> for f32 {
    type VectorRepr = [f32; 2];
}

impl F32VectorApi<2, Simd> for f32 {}
