use crate::{Scalar, Simd, vector::primitive_api::f32::F32Api};

mod vec3;
mod vec4;

// TODO: Determine if FVec2 benefits from alignment.

impl Scalar<2> for f32 {
    type InnerSimdVectorType = [f32; 2];
}

impl F32Api<2, Simd> for f32 {}
