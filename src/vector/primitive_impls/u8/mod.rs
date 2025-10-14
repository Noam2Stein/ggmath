use crate::Scalar;

// U8Vec2 is only 16 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for u8 {
    type InnerSimdVectorType = [u8; 2];
}

// U8Vec3 is only 24 bits long, so it doesn't benefit from SIMD

impl Scalar<3> for u8 {
    type InnerSimdVectorType = [u8; 3];
}

// U8Vec4 is only 32 bits long, so it doesn't benefit from SIMD

impl Scalar<4> for u8 {
    type InnerSimdVectorType = [u8; 4];
}
