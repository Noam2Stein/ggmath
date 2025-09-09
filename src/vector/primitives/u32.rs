use crate::vector::Scalar;

impl Scalar for u32 {
    // Vec2<u32> ops don't benefit from SIMD
    type InnerAlignedVec2 = [Self; 2];

    // SSE alone doesn't provide integer instructions so SSE2 is required
    cfg_if::cfg_if! {
        if #[cfg(i32x4_simd)] {
            type InnerAlignedVec3 = wide::u32x4;
            type InnerAlignedVec4 = wide::u32x4;
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
