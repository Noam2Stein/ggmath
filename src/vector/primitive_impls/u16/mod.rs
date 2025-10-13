use crate::{Scalar, Simd};

// u16 is only 16 bits, so none of its vectors benefit from SIMD

impl Scalar<2, Simd> for u16 {
    type InnerVectorType = [u16; 2];
}

impl Scalar<3, Simd> for u16 {
    type InnerVectorType = [u16; 3];
}

impl Scalar<4, Simd> for u16 {
    type InnerVectorType = [u16; 4];
}
