use crate::Scalar;

// I8Vec2 is only 16 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for i8 {
    type InnerSimdVectorType = [i8; 2];
}

// I8Vec3 is only 24 bits long, so it doesn't benefit from SIMD

impl Scalar<3> for i8 {
    type InnerSimdVectorType = [i8; 3];
}

// I8Vec4 is only 32 bits long, so it doesn't benefit from SIMD

impl Scalar<4> for i8 {
    type InnerSimdVectorType = [i8; 4];
}
