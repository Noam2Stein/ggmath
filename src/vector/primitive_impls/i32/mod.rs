use crate::{Scalar, SimdBehaviour};

#[cfg(target_feature = "sse2")]
mod b128_sse2;

#[cfg(not(target_feature = "sse2"))]
mod b128_fallback;

impl Scalar for i32 {}

// IVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<2> for i32 {
    type VectorRepr = [i32; 2];
}
