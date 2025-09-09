use cfg_if::cfg_if;

use crate::vector::Scalar;

impl Scalar for i64 {
    // SSE alone doesn't provide integer instructions so SSE2 is required
    cfg_if! {
        if #[cfg(i64x2_simd)] {
            type InnerAlignedVec2 = wide::i64x2;
        } else {
            type InnerAlignedVec2 = [Self; 2];
        }
    }

    // ARM and WASM don't support 256-bit SIMD
    // AVX alone doesn't provide integer instructions so AVX2 is required
    cfg_if! {
        if #[cfg(i64x4_simd)] {
            type InnerAlignedVec3 = wide::i64x4;
            type InnerAlignedVec4 = wide::i64x4;
        } else {
            type InnerAlignedVec3 = [Self; 3];
            type InnerAlignedVec4 = [Self; 4];
        }
    }

    const GARBAGE: Self = 0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = unsafe { core::mem::zeroed() };
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = unsafe { core::mem::zeroed() };
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = unsafe { core::mem::zeroed() };
}
