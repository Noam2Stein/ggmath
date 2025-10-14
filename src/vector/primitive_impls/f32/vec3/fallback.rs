use crate::{Scalar, Simd, vector::primitive_api::f32::F32Api};

impl Scalar<3> for f32 {
    type InnerVectorType = [f32; 3];
}

impl F32Api<3, Simd> for f32 {}
