use std::mem::transmute_copy;

use super::*;

primitive_aliases! { pub B => bool }

#[cfg(feature = "vector")]
impl Scalar for bool {
    // A vector of bools can act like a uint when performing bitwise operations.
    // This means that the alignment of a vector of bools should be the alignment of the corresponding uint type.
    type Vec2Alignment = Align<{ align_of::<u16>() }>;
    type Vec3Alignment = Align<{ align_of::<u32>() }>;
    type Vec4Alignment = Align<{ align_of::<u32>() }>;

    #[inline(always)]
    fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
        let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

        let output_int = lhs_int & rhs_int;

        Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
    }

    #[inline(always)]
    fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
        let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

        let output_int = lhs_int | rhs_int;

        Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
    }

    #[inline(always)]
    fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
        let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

        let output_int = lhs_int ^ rhs_int;

        Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
    }
}

const _: () = assert!(size_of::<bool>() == 1);
const _: () = assert!(align_of::<bool>() == 1);
