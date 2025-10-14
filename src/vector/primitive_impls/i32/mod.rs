use crate::Scalar;

// IVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for i32 {
    type InnerSimdVectorType = [i32; 2];
}

// TODO: Implement SIMD optimizations for IVec3.

impl Scalar<3> for i32 {
    type InnerSimdVectorType = [i32; 3];
}

// TODO: Implement SIMD optimizations for IVec4.

impl Scalar<4> for i32 {
    type InnerSimdVectorType = [i32; 4];
}
