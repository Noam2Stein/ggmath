use crate::{Scalar, Simd};

// TODO: Determine if char vectors benefit from SIMD.

impl Scalar<2, Simd> for char {
    type InnerVectorType = [char; 2];
}

impl Scalar<3, Simd> for char {
    type InnerVectorType = [char; 3];
}

impl Scalar<4, Simd> for char {
    type InnerVectorType = [char; 4];
}
