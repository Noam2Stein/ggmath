use crate::{Scalar, SimdBehaviour};

#[cfg(target_feature = "sse2")]
mod b128_sse2;

#[cfg(not(target_feature = "sse2"))]
mod b128_fallback;

impl Scalar for u32 {}

// UVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<2> for u32 {
    type VectorRepr = [u32; 2];
}
