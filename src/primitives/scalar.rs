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
