use crate::{Vector, vector::Scalar};

impl Scalar for f64 {
    type InnerAlignedVec2 = glam::DVec2;
    type InnerAlignedVec3 = glam::DVec3;
    type InnerAlignedVec4 = glam::DVec4;

    const GARBAGE: Self = 0.0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = glam::DVec2::ZERO;
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = glam::DVec3::ZERO;
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = glam::DVec4::ZERO;

    fn vec_abs_diff<const N: usize, A: crate::VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl crate::VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd + std::ops::Sub<Output = Self>,
    {
        (vec - other).abs()
    }
}
