use crate::SimdBehaviour;

impl SimdBehaviour<3> for i32 {
    type VectorRepr = [i32; 3];
}

impl SimdBehaviour<4> for i32 {
    type VectorRepr = [i32; 4];
}
