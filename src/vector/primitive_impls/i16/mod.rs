use crate::Scalar;

// I16Vec2 is only 32 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for i16 {
    type InnerSimdVectorType = [i16; 2];
}

// I16Vec3 is only 48 bits long, so it doesn't benefit from SIMD

impl Scalar<3> for i16 {
    type InnerSimdVectorType = [i16; 3];
}

// I16Vec4 is only 64 bits long, so it doesn't benefit from SIMD

impl Scalar<4> for i16 {
    type InnerSimdVectorType = [i16; 4];
}
