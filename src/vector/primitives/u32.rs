use crate::vector::Scalar;

impl Scalar for u32 {
    // Vec2<u32> ops don't benefit from SIMD
    type InnerAlignedVec2 = [Self; 2];

    // SSE alone doesn't provide integer instructions so SSE2 is required
    cfg_if::cfg_if! {
        if #[cfg(any(
            all(target_arch = "x86", target_feature = "sse2"),
            all(target_arch = "x86_64", target_feature = "sse2"),
            all(target_arch = "arm", target_feature = "neon"),
            all(target_arch = "aarch64", target_feature = "neon"),
            all(target_arch = "wasm32", target_feature = "simd128"),
        ))] {
            type InnerAlignedVec3 = super::Aligned128<[Self; 4]>;
            type InnerAlignedVec4 = super::Aligned128<[Self; 4]>;
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
