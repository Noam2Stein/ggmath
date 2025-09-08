use cfg_if::cfg_if;

use crate::{Vector, vector::Scalar};

impl Scalar for f32 {
    // Vec2<f32> ops don't benefit from SIMD
    type InnerAlignedVec2 = [Self; 2];

    cfg_if! {
        if #[cfg(any(
            all(target_arch = "x86", target_feature = "sse"),
            all(target_arch = "x86_64", target_feature = "sse"),
            all(target_arch = "arm", target_feature = "neon"),
            all(target_arch = "aarch64", target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        ))] {
            type InnerAlignedVec3 = wide::f32x4;
            type InnerAlignedVec4 = wide::f32x4;
        } else {
            type InnerAlignedVec3 = [Self; 3];
            type InnerAlignedVec4 = [Self; 4];
        }
    }

    const GARBAGE: Self = 0.0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = unsafe { core::mem::zeroed() };
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = unsafe { core::mem::zeroed() };
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = unsafe { core::mem::zeroed() };

    #[inline(always)]
    fn vec_abs_diff<const N: usize, A: crate::VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl crate::VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        crate::Usize<N>: crate::VecLen,
        Self: PartialOrd + core::ops::Sub<Output = Self>,
    {
        (vec - other).abs()
    }
}
