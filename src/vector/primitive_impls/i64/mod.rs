use crate::Scalar;

// TODO: Implement SIMD optimizations for I64Vec2.

impl Scalar<2> for i64 {
    type InnerSimdVectorType = [i64; 2];
}

// TODO: Implement SIMD optimizations for I64Vec3.

impl Scalar<3> for i64 {
    type InnerSimdVectorType = [i64; 3];
}

// TODO: Implement SIMD optimizations for I64Vec4.

impl Scalar<4> for i64 {
    type InnerSimdVectorType = [i64; 4];
}
