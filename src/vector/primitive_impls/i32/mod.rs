use crate::{Scalar, SimdBehaviour};

impl Scalar for i32 {}

// IVec2 is only 64 bits long, so it doesn't benefit from SIMD

impl SimdBehaviour<2> for i32 {
    type VectorRepr = [i32; 2];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<3> for i32 {
    type VectorRepr = [i32; 3];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<4> for i32 {
    type VectorRepr = [i32; 4];
}
