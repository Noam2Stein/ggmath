use std::mem::transmute_copy;

use super::*;

primitive_aliases! { pub U64 => u64 }

#[cfg(feature = "vector")]
impl Scalar for u64 {
    type Vec2Alignment = Align<16>;
    type Vec3Alignment = Align<32>;
    type Vec4Alignment = Align<32>;

    #[inline(always)]
    fn vec3_not(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let base_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&base) };

        let output_vec4 = base_vec4.map(|x| !x);

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_add(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() + rhs_vec4.x(),
            lhs_vec4.y() + rhs_vec4.y(),
            lhs_vec4.z() + rhs_vec4.z(),
            lhs_vec4.w().overflowing_add(rhs_vec4.w()).0,
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_sub(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() - rhs_vec4.x(),
            lhs_vec4.y() - rhs_vec4.y(),
            lhs_vec4.z() - rhs_vec4.z(),
            lhs_vec4.w().overflowing_sub(rhs_vec4.w()).0,
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_mul(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() * rhs_vec4.x(),
            lhs_vec4.y() * rhs_vec4.y(),
            lhs_vec4.z() * rhs_vec4.z(),
            lhs_vec4.w().overflowing_mul(rhs_vec4.w()).0,
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() & rhs_vec4.x(),
            lhs_vec4.y() & rhs_vec4.y(),
            lhs_vec4.z() & rhs_vec4.z(),
            lhs_vec4.w() & rhs_vec4.w(),
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() | rhs_vec4.x(),
            lhs_vec4.y() | rhs_vec4.y(),
            lhs_vec4.z() | rhs_vec4.z(),
            lhs_vec4.w() | rhs_vec4.w(),
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() ^ rhs_vec4.x(),
            lhs_vec4.y() ^ rhs_vec4.y(),
            lhs_vec4.z() ^ rhs_vec4.z(),
            lhs_vec4.w() ^ rhs_vec4.w(),
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }
}

// u64 methods are defined using `macro_loop` in other files
