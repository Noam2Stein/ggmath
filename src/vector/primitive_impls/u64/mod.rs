use crate::{Scalar, SimdBehaviour};

impl Scalar for u64 {}

// TODO: add SIMD optimizations

impl SimdBehaviour<2> for u64 {
    type VectorRepr = [u64; 2];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<3> for u64 {
    type VectorRepr = [u64; 3];
}

// TODO: add SIMD optimizations

impl SimdBehaviour<4> for u64 {
    type VectorRepr = [u64; 4];
}
