use crate::{Scalar, Simd};

// u8 is only 8 bits, so none of its vectors benefit from SIMD

impl Scalar<2, Simd> for u8 {
    type InnerVectorType = [u8; 2];
}

impl Scalar<3, Simd> for u8 {
    type InnerVectorType = [u8; 3];
}

impl Scalar<4, Simd> for u8 {
    type InnerVectorType = [u8; 4];
}
