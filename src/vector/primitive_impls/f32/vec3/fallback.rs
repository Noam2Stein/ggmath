use crate::{Scalar, Simd, vector::primitive_api::f32::ScalarF32};

impl Scalar<3, Simd> for f32 {
    type InnerVectorType = [f32; 3];
}

impl ScalarF32<3, Simd> for f32 {}
