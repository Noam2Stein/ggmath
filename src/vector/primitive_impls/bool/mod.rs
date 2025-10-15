use crate::{Scalar, SimdBehaviour};

impl Scalar for bool {}

// BVec2 is only 16 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<2> for bool {
    type VectorRepr = [bool; 2];
}

// BVec3 is only 24 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<3> for bool {
    type VectorRepr = [bool; 3];
}

// BVec4 is only 32 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<4> for bool {
    type VectorRepr = [bool; 4];
}
