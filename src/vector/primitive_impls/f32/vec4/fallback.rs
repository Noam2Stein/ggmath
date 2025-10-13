use crate::{Scalar, Simd, vector::primitive_api::f32::ScalarF32};

impl Scalar<4, Simd> for f32 {
    type InnerVectorType = [f32; 4];
}

impl ScalarF32<4, Simd> for f32 {}
