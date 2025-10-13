use crate::{Scalar, Simd};

// i16 is only 16 bits, so none of its vectors benefit from SIMD

impl Scalar<2, Simd> for i16 {
    type InnerVectorType = [i16; 2];
}

impl Scalar<3, Simd> for i16 {
    type InnerVectorType = [i16; 3];
}

impl Scalar<4, Simd> for i16 {
    type InnerVectorType = [i16; 4];
}
