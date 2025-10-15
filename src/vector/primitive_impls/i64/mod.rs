use crate::{Scalar, SimdBehaviour};

impl Scalar for i64 {}

// TODO: add SIMD optimizations

impl SimdBehaviour<2> for i64 {
    type VectorRepr = [i64; 2];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<3> for i64 {
    type VectorRepr = [i64; 3];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<4> for i64 {
    type VectorRepr = [i64; 4];
}
