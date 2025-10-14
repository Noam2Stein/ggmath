use crate::Scalar;

// BVec2 is only 16 bits long, so it doesn't benefit from SIMD

impl Scalar<2> for bool {
    type InnerSimdVectorType = [bool; 2];
}

// BVec3 is only 24 bits long, so it doesn't benefit from SIMD

impl Scalar<3> for bool {
    type InnerSimdVectorType = [bool; 3];
}

// BVec4 is only 32 bits long, so it doesn't benefit from SIMD

impl Scalar<4> for bool {
    type InnerSimdVectorType = [bool; 4];
}
