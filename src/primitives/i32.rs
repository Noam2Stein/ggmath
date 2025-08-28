use crate::vector::Scalar;

impl Scalar for i32 {
    type InnerAlignedVec2 = [Self; 2];
    type InnerAlignedVec3 = [Self; 3];
    type InnerAlignedVec4 = [Self; 4];

    const GARBAGE: Self = 0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = [0, 0];
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = [0, 0, 0];
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = [0, 0, 0, 0];
}
