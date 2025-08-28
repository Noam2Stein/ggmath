use crate::vector::Scalar;

impl Scalar for bool {
    type InnerAlignedVec2 = [Self; 2];
    type InnerAlignedVec3 = [Self; 3];
    type InnerAlignedVec4 = [Self; 4];

    const GARBAGE: Self = false;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = [false, false];
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = [false, false, false];
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = [false, false, false, false];
}
