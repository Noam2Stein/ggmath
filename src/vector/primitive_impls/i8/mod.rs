use crate::{Scalar, Simd};

// i8 is only 8 bits, so none of its vectors benefit from SIMD

impl Scalar<2, Simd> for i8 {
    type InnerVectorType = [i8; 2];
}

impl Scalar<3, Simd> for i8 {
    type InnerVectorType = [i8; 3];
}

impl Scalar<4, Simd> for i8 {
    type InnerVectorType = [i8; 4];
}
