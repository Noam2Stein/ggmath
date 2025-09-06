use cfg_if::cfg_if;

use crate::vector::Scalar;

impl Scalar for usize {
    cfg_if! {
        if #[cfg(target_pointer_width = "16")] {
            type InnerAlignedVec2 = <u16 as Scalar>::InnerAlignedVec2;
            type InnerAlignedVec3 = <u16 as Scalar>::InnerAlignedVec3;
            type InnerAlignedVec4 = <u16 as Scalar>::InnerAlignedVec4;
        } else if #[cfg(target_pointer_width = "32")] {
            type InnerAlignedVec2 = <u32 as Scalar>::InnerAlignedVec2;
            type InnerAlignedVec3 = <u32 as Scalar>::InnerAlignedVec3;
            type InnerAlignedVec4 = <u32 as Scalar>::InnerAlignedVec4;
        } else if #[cfg(target_pointer_width = "64")] {
            type InnerAlignedVec2 = <u64 as Scalar>::InnerAlignedVec2;
            type InnerAlignedVec3 = <u64 as Scalar>::InnerAlignedVec3;
            type InnerAlignedVec4 = <u64 as Scalar>::InnerAlignedVec4;
        } else {
            type InnerAlignedVec2 = [Self; 2];
            type InnerAlignedVec3 = [Self; 3];
            type InnerAlignedVec4 = [Self; 4];
        }
    }

    const GARBAGE: Self = 0;
    const INNER_ALIGNED_VEC2_GARBAGE: Self::InnerAlignedVec2 = unsafe { core::mem::zeroed() };
    const INNER_ALIGNED_VEC3_GARBAGE: Self::InnerAlignedVec3 = unsafe { core::mem::zeroed() };
    const INNER_ALIGNED_VEC4_GARBAGE: Self::InnerAlignedVec4 = unsafe { core::mem::zeroed() };
}
