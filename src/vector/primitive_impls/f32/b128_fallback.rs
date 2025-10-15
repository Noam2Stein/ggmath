use crate::{Simd, SimdBehaviour, vector::primitive_api::F32VectorApi};

impl SimdBehaviour<3> for f32 {
    type VectorRepr = [f32; 3];
}
impl SimdBehaviour<4> for f32 {
    type VectorRepr = [f32; 4];
}

impl F32VectorApi<3, Simd> for f32 {}
impl F32VectorApi<4, Simd> for f32 {}
