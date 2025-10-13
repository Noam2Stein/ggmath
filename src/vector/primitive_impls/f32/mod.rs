use crate::{Scalar, Simd, vector::primitive_api::f32::ScalarF32};

mod vec3;
mod vec4;

// TODO: Determine if FVec2 benefits from alignment.

impl Scalar<2, Simd> for f32 {
    type InnerVectorType = [f32; 2];
}

impl ScalarF32<2, Simd> for f32 {}
