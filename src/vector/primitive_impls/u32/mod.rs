use crate::{Scalar, Simd};

// UVec2 is only 64 bits, so it doesn't benefit from SIMD

impl Scalar<2, Simd> for u32 {
    type InnerVectorType = [u32; 2];
}

// TODO: Implement SIMD optimizations for UVec3.

impl Scalar<3, Simd> for u32 {
    type InnerVectorType = [u32; 3];
}

// TODO: Implement SIMD optimizations for UVec4.

impl Scalar<4, Simd> for u32 {
    type InnerVectorType = [u32; 4];
}
