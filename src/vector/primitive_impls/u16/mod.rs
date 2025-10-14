use crate::Scalar;

// U16Vec2 is only 32 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for u16 {
    type InnerSimdVectorType = [u16; 2];
}

// U16Vec3 is only 48 bits long, so it doesn't benefit from SIMD

impl Scalar<3> for u16 {
    type InnerSimdVectorType = [u16; 3];
}

// U16Vec4 is only 64 bits long, so it doesn't benefit from SIMD

impl Scalar<4> for u16 {
    type InnerSimdVectorType = [u16; 4];
}
