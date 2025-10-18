use crate::SimdBehaviour;

impl SimdBehaviour<3> for u32 {
    type VectorRepr = [u32; 3];
}

impl SimdBehaviour<4> for u32 {
    type VectorRepr = [u32; 4];
}
