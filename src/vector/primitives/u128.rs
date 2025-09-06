use cfg_if::cfg_if;

use crate::vector::Scalar;

impl Scalar for u128 {
    cfg_if! {
        if #[cfg(any(
            all(target_arch = "x86", target_feature = "avx2"),
            all(target_arch = "x86_64", target_feature = "avx2"),
        ))] {
            type InnerAlignedVec2 = super::Aligned256<[Self; 2]>;
        } else {
            type InnerAlignedVec2 = [Self; 2];
        }
    }

    cfg_if! {
        if #[cfg(any(
            all(target_arch = "x86", target_feature = "avx512f"),
            all(target_arch = "x86_64", target_feature = "avx512f"),
        ))] {
            type InnerAlignedVec3 = super::Aligned512<[Self; 4]>;
            type InnerAlignedVec4 = super::Aligned512<[Self; 4]>;
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
