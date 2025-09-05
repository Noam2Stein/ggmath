use crate::vector::Scalar;

impl Scalar for f64 {
    type InnerAlignedVec2 = glam::DVec2;
    type InnerAlignedVec3 = glam::DVec3;
    type InnerAlignedVec4 = glam::DVec4;

    const GARBAGE: Self = 0.0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = glam::DVec2::ZERO;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = glam::DVec3::ZERO;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = glam::DVec4::ZERO;
}
