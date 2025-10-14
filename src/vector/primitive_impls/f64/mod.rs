use crate::{Scalar, Simd, vector::primitive_api::f64::F64Api};

// TODO: Implement SIMD optimizations for DVec2.

impl Scalar<2> for f64 {
    type InnerSimdVectorType = [f64; 2];
}

impl F64Api<2, Simd> for f64 {}

// TODO: Implement SIMD optimizations for DVec3.

impl Scalar<3> for f64 {
    type InnerSimdVectorType = [f64; 3];
}

impl F64Api<3, Simd> for f64 {}

// TODO: Implement SIMD optimizations for DVec4.

impl Scalar<4> for f64 {
    type InnerSimdVectorType = [f64; 4];
}

impl F64Api<4, Simd> for f64 {}
