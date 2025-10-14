use crate::Scalar;

// UVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for u32 {
    type InnerSimdVectorType = [u32; 2];
}

// TODO: Implement SIMD optimizations for UVec3.

impl Scalar<3> for u32 {
    type InnerSimdVectorType = [u32; 3];
}

// TODO: Implement SIMD optimizations for UVec4.

impl Scalar<4> for u32 {
    type InnerSimdVectorType = [u32; 4];
}
