use std::mem::transmute_copy;
#[cfg(feature = "vector")]
use std::{cmp::Ordering, mem::MaybeUninit};

use super::*;

repetitive! {
    @for [prim, prim_is_int, prim_is_float, prim_is_signed] in [
        ['u8, true, false, false],
        ['u16, true, false, false],
        ['u32, true, false, false],
        ['u64, true, false, false],
        ['u128, true, false, false],
        ['usize, true, false, false],
        ['i8, true, false, true],
        ['i16, true, false, true],
        ['i32, true, false, true],
        ['i64, true, false, true],
        ['i128, true, false, true],
        ['isize, true, false, true],
        ['f32, false, true, true],
        ['f64, false, true, true],
        ['bool, false, false, false],
    ] {
        @let prim_is_num = prim_is_int || prim_is_float;
        @let prim_is_sint = prim_is_int && prim_is_signed;
        @let prim_is_uint = prim_is_int && !prim_is_signed;
        @let prim_has_bitops = prim_is_int || prim == 'bool;

        @let supported_alignment = @{{
            let _align = 8;

            // X86

            #[cfg(target_feature = "sse")]
            #[cfg(@(prim == 'f32))]
            let _align = 16;

            #[cfg(target_feature = "sse2")]
            let _align = 16;

            #[cfg(target_feature = "avx")]
            #[cfg(@(prim == 'f64))]
            let _align = 32;

            #[cfg(target_feature = "avx2")]
            let _align = 32;

            // ARM

            #[cfg(target_feature = "neon")]
            #[cfg(@(prim == 'f32 || prim_is_int))]
            let _align = 16;

            #[cfg(target_arch = "aarch64")]
            #[cfg(target_feature = "neon")]
            let _align = 32;

            // WASM

            #[cfg(target_feature = "simd128")]
            let _align = 16;

            _align
        }};

        impl Scalar for @prim {
            type Vec2Alignment = Align<{
                let size = size_of::<@prim>() * 2;
                let wanted_alignment = size.next_power_of_two();
                let supported_alignment = @supported_alignment;

                if wanted_alignment > supported_alignment {
                    supported_alignment
                } else {
                    wanted_alignment
                }
            }>;

            type Vec3Alignment = Self::Vec4Alignment;

            type Vec4Alignment = Align<{
                let size = size_of::<@prim>() * 4;
                let wanted_alignment = size.next_power_of_two();
                let supported_alignment = @supported_alignment;

                if wanted_alignment > supported_alignment {
                    supported_alignment
                } else {
                    wanted_alignment
                }
            }>;

            #[inline(always)]
            #[cfg(@prim_is_signed)]
            fn vec3_neg(base: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_float {
                    let base_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&base) };

                    let output_vec4 = -base_vec4;

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {
                    let base_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&base) };

                    let output_vec4 = vec4!(
                        -base_vec4.x(),
                        -base_vec4.y(),
                        -base_vec4.z(),
                        base_vec4.w().overflowing_neg().0,
                    );

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                }
            }

            #[inline(always)]
            #[cfg(@prim_is_int)]
            fn vec3_not(base: Vec3<Self>) -> Option<Vec3<Self>> {
                let base_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&base) };

                let output_vec4 = !base_vec4;

                Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
            }

            #[inline(always)]
            #[cfg(@prim_is_num)]
            fn vec3_add(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_float {
                    let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                    let output_vec4 = lhs_vec4 + rhs_vec4;

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {
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
            }

            #[inline(always)]
            #[cfg(@prim_is_num)]
            fn vec3_sub(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_float {
                    let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                    let output_vec4 = lhs_vec4 - rhs_vec4;

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {
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
            }

            #[inline(always)]
            #[cfg(@prim_is_num)]
            fn vec3_mul(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_float {
                    let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                    let output_vec4 = lhs_vec4 * rhs_vec4;

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {
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
            }

            #[inline(always)]
            #[cfg(@prim_is_float)]
            fn vec3_div(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                let output_vec4 = lhs_vec4 / rhs_vec4;

                Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
            }

            #[inline(always)]
            #[cfg(@prim_is_float)]
            fn vec3_rem(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                let output_vec4 = lhs_vec4 % rhs_vec4;

                Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
            }

            #[inline(always)]
            #[cfg(@prim_has_bitops)]
            fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_int {
                    let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                    let output_vec4 = vec4!(
                        lhs_vec4.x() & rhs_vec4.x(),
                        lhs_vec4.y() & rhs_vec4.y(),
                        lhs_vec4.z() & rhs_vec4.z(),
                        lhs_vec4.w() & rhs_vec4.w(),
                    );

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {
                    let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
                    let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

                    let output_int = lhs_int & rhs_int;

                    Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
                }
            }

            #[inline(always)]
            #[cfg(@prim_has_bitops)]
            fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_int {
                    let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                    let output_vec4 = vec4!(
                        lhs_vec4.x() | rhs_vec4.x(),
                        lhs_vec4.y() | rhs_vec4.y(),
                        lhs_vec4.z() | rhs_vec4.z(),
                        lhs_vec4.w() | rhs_vec4.w(),
                    );

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {
                    let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
                    let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

                    let output_int = lhs_int | rhs_int;

                    Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
                }
            }

            #[inline(always)]
            #[cfg(@prim_has_bitops)]
            fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
                @if prim_is_int {
                    let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
                    let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

                    let output_vec4 = vec4!(
                        lhs_vec4.x() ^ rhs_vec4.x(),
                        lhs_vec4.y() ^ rhs_vec4.y(),
                        lhs_vec4.z() ^ rhs_vec4.z(),
                        lhs_vec4.w() ^ rhs_vec4.w(),
                    );

                    Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
                } else {

                    let lhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&lhs) };
                    let rhs_int = unsafe { transmute_copy::<Vec3<Self>, u32>(&rhs) };

                    let output_int = lhs_int ^ rhs_int;

                    Some(unsafe { transmute_copy::<u32, Vec3<Self>>(&output_int) })
                }
            }
        }
    }
}

#[cfg(feature = "vector")]
impl Scalar for Ordering {
    type Vec2Alignment = Align<1>;
    type Vec3Alignment = Align<1>;
    type Vec4Alignment = Align<1>;
}

#[cfg(feature = "vector")]
impl<T: Scalar> Scalar for MaybeUninit<T> {
    type Vec2Alignment = T::Vec2Alignment;
    type Vec3Alignment = T::Vec3Alignment;
    type Vec4Alignment = T::Vec4Alignment;
}
