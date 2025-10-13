use crate::{Scalar, Simd};

// IVec2 is only 64 bits, so it doesn't benefit from SIMD

impl Scalar<2, Simd> for i32 {
    type InnerVectorType = [i32; 2];
}

// TODO: Implement SIMD optimizations for IVec3.

impl Scalar<3, Simd> for i32 {
    type InnerVectorType = [i32; 3];
}

// TODO: Implement SIMD optimizations for IVec4.

impl Scalar<4, Simd> for i32 {
    type InnerVectorType = [i32; 4];
}
