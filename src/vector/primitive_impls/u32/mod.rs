use crate::{Scalar, SimdBehaviour};

impl Scalar for u32 {}

// UVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<2> for u32 {
    type VectorRepr = [u32; 2];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<3> for u32 {
    type VectorRepr = [u32; 3];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<4> for u32 {
    type VectorRepr = [u32; 4];
}
