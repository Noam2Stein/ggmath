use crate::{Scalar, Simd};

// bool is only 8 bits, so none of its vectors benefit from SIMD

impl Scalar<2, Simd> for bool {
    type InnerVectorType = [bool; 2];
}

impl Scalar<3, Simd> for bool {
    type InnerVectorType = [bool; 3];
}

impl Scalar<4, Simd> for bool {
    type InnerVectorType = [bool; 4];
}
