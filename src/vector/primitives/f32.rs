use crate::vector::Scalar;

impl Scalar for f32 {
    type InnerAlignedVec2 = glam::Vec2;
    type InnerAlignedVec3 = glam::Vec3A;
    type InnerAlignedVec4 = glam::Vec4;

    const GARBAGE: Self = 0.0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = glam::Vec2::ZERO;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = glam::Vec3A::ZERO;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = glam::Vec4::ZERO;
}
