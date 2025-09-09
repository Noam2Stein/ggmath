use cfg_if::cfg_if;

use crate::{Vector, vector::Scalar};

impl Scalar for f64 {
    // SSE alone doesn't provide f64x2 instructions so SSE2 is required
    cfg_if! {
        if #[cfg(f64x2_simd)] {
            type InnerAlignedVec2 = wide::f64x2;
        } else {
            type InnerAlignedVec2 = [Self; 2];
        }
    }

    // ARM and WASM don't support 256-bit SIMD
    cfg_if! {
        if #[cfg(f64x4_simd)] {
            type InnerAlignedVec3 = wide::f64x4;
            type InnerAlignedVec4 = wide::f64x4;
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
