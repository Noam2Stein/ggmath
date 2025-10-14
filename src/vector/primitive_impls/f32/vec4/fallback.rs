use crate::{Scalar, Simd, vector::primitive_api::f32::F32Api};

impl Scalar<4> for f32 {
    type InnerVectorType = [f32; 4];
}

impl F32Api<4, Simd> for f32 {}
