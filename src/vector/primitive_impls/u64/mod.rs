use crate::Scalar;

// TODO: Implement SIMD optimizations for U64Vec2.

impl Scalar<2> for u64 {
    type InnerSimdVectorType = [u64; 2];
}

// TODO: Implement SIMD optimizations for U64Vec3.

impl Scalar<3> for u64 {
    type InnerSimdVectorType = [u64; 3];
}

// TODO: Implement SIMD optimizations for U64Vec4.

impl Scalar<4> for u64 {
    type InnerSimdVectorType = [u64; 4];
}
