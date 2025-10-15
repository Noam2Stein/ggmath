use crate::{Scalar, Simd, SimdBehaviour, vector::primitive_api::F64VectorApi};

impl Scalar for f64 {}

// TODO: add SIMD optimizations

impl SimdBehaviour<2> for f64 {
    type VectorRepr = [f64; 2];
}

impl F64VectorApi<2, Simd> for f64 {}

// TODO: add SIMD optimizations

impl SimdBehaviour<3> for f64 {
    type VectorRepr = [f64; 3];
}

impl F64VectorApi<3, Simd> for f64 {}

// TODO: add SIMD optimizations

impl SimdBehaviour<4> for f64 {
    type VectorRepr = [f64; 4];
}

impl F64VectorApi<4, Simd> for f64 {}
