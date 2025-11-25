use crate::{Aligned, ScalarBackend, vector::primitive_apis::f32::FloatBackend};

impl ScalarBackend<3, Aligned> for f32 {
    type VectorRepr = [f32; 3];
}

impl ScalarBackend<4, Aligned> for f32 {
    type VectorRepr = [f32; 4];
}

impl FloatBackend<3, Aligned> for f32 {}

impl FloatBackend<4, Aligned> for f32 {}
