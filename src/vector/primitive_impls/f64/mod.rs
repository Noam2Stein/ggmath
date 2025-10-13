use crate::{Scalar, Simd};

// TODO: Implement SIMD optimizations for DVec2.

impl Scalar<2, Simd> for f64 {
    type InnerVectorType = [f64; 2];
}

// TODO: Implement SIMD optimizations for DVec3.

impl Scalar<3, Simd> for f64 {
    type InnerVectorType = [f64; 3];
}

// TODO: Implement SIMD optimizations for DVec4.

impl Scalar<4, Simd> for f64 {
    type InnerVectorType = [f64; 4];
}
