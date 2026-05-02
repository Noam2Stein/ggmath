#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    Aligned, Backend, Mask, Mask3, Mask4, PrimitiveFloatBackend, Vec3, Vec4, Vector,
    utils::safe_arch,
};

// SAFETY: All associated types uphold requirements.
unsafe impl Backend<3, Aligned> for f32 {
    type Vector = __m128;
    type Mask = __m128;

    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[inline]
        fn vector_eq(vector: &Vec3<f32>, other: &Vec3<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vector.0, other.0)) as u32 & 0b111 == 0b111
        }

        #[inline]
        fn vector_ne(vector: &Vec3<f32>, other: &Vec3<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vector.0, other.0)) as u32 & 0b111 != 0
        }

        #[inline]
        fn vector_neg(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(neg(vector.0))
        }

        #[inline]
        fn vector_add(vector: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_add_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_sub(vector: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_sub_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_mul(vector: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_mul_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_div(vector: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_div_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_rem(vector: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(rem(vector.0, rhs.0))
        }

        #[inline]
        fn vector_element_sum(vector: Vec3<f32>) -> f32 {
            let vector = vector.0;
            // Add `-0.0` to retain the sign of the left operand. Adding `+0.0`
            // would incorrectly reset the sign when `z` is `-0.0`.
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, _mm_set1_ps(-0.0), 0b00_11_00_01));
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_element_product(vector: Vec3<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, _mm_set1_ps(1.0), 0b00_11_00_01));
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_eq_mask(vector: Vec3<f32>, other: Vec3<f32>) -> Mask3<f32> {
            Mask(_mm_cmpeq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ne_mask(vector: Vec3<f32>, other: Vec3<f32>) -> Mask3<f32> {
            Mask(_mm_cmpneq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_lt_mask(vector: Vec3<f32>, other: Vec3<f32>) -> Mask3<f32> {
            Mask(_mm_cmplt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_gt_mask(vector: Vec3<f32>, other: Vec3<f32>) -> Mask3<f32> {
            Mask(_mm_cmpgt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_le_mask(vector: Vec3<f32>, other: Vec3<f32>) -> Mask3<f32> {
            Mask(_mm_cmple_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ge_mask(vector: Vec3<f32>, other: Vec3<f32>) -> Mask3<f32> {
            Mask(_mm_cmpge_ps(vector.0, other.0))
        }
    }

    #[inline]
    fn mask_from_array(array: [bool; 3]) -> Mask3<f32> {
        // SAFETY: The two intrinsics are part of SSE2.
        unsafe {
            Mask(_mm_castsi128_ps(_mm_set_epi32(
                -(array[2] as i32),
                -(array[2] as i32),
                -(array[1] as i32),
                -(array[0] as i32),
            )))
        }
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask3<f32> {
        // SAFETY: The two intrinsics are part of SSE2.
        unsafe { Mask(_mm_castsi128_ps(_mm_set1_epi32(-(value as i32)))) }
    }

    #[inline]
    fn mask_from_fn<F>(mut f: F) -> Mask3<f32>
    where
        F: FnMut(usize) -> bool,
    {
        Mask::from_array([f(0), f(1), f(2)])
    }

    #[inline]
    fn mask_to_array(mask: Mask3<f32>) -> [bool; 3] {
        // SAFETY: The intrinsic is part of SSE.
        let bits = unsafe { _mm_movemask_ps(mask.0) };
        [bits & 0x1 != 0, bits & 0x2 != 0, bits & 0x4 != 0]
    }

    #[inline]
    fn mask_all(mask: Mask3<f32>) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) & 0x7 == 0x7 }
    }

    #[inline]
    fn mask_any(mask: Mask3<f32>) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) & 0x7 != 0 }
    }

    #[inline]
    fn mask_select(mask: Mask3<f32>, if_true: Vec3<f32>, if_false: Vec3<f32>) -> Vec3<f32> {
        // SAFETY: The three intrinsics are part of SSE.
        Vector(unsafe {
            _mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            )
        })
    }

    #[inline]
    fn mask_get(mask: Mask3<f32>, index: usize) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe {
            match index {
                0 => _mm_movemask_ps(mask.0) & 0x1 != 0,
                1 => _mm_movemask_ps(mask.0) & 0x2 != 0,
                2 => _mm_movemask_ps(mask.0) & 0x4 != 0,
                _ => panic!("index out of bounds"),
            }
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask3<f32>, index: usize, value: bool) {
        if index < 3 {
            // SAFETY: `*mut __m128` is valid as `*mut i32` for 4 values. Adding
            // `index` is valid because it was just checked to be less then 3,
            // and the result is a pointer to a valid `i32`.
            let slot = unsafe {
                core::ptr::from_mut::<__m128>(&mut mask.0)
                    .cast::<i32>()
                    .add(index)
                    .as_mut()
                    .unwrap_unchecked()
            };

            *slot = -(value as i32);
        } else {
            panic!("index out of bounds")
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask3<f32>, other: &Mask3<f32>) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) & 0x7 == _mm_movemask_ps(other.0) & 0x7 }
    }

    #[inline]
    fn mask_not(mask: Mask3<f32>) -> Mask3<f32> {
        // SAFETY: The two intrinsics are part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, _mm_set1_ps(f32::from_bits(!0))) })
    }

    #[inline]
    fn mask_bitand(mask: Mask3<f32>, rhs: Mask3<f32>) -> Mask3<f32> {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_and_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitor(mask: Mask3<f32>, rhs: Mask3<f32>) -> Mask3<f32> {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_or_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitxor(mask: Mask3<f32>, rhs: Mask3<f32>) -> Mask3<f32> {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, rhs.0) })
    }
}

// SAFETY: All associated types uphold requirements.
unsafe impl Backend<4, Aligned> for f32 {
    type Vector = __m128;
    type Mask = __m128;

    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[inline]
        fn vector_eq(vector: &Vec4<f32>, other: &Vec4<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vector.0, other.0)) as u32 == 0xf
        }

        #[inline]
        fn vector_ne(vector: &Vec4<f32>, other: &Vec4<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vector.0, other.0)) as u32 != 0
        }

        #[inline]
        fn vector_neg(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(neg(vector.0))
        }

        #[inline]
        fn vector_add(vector: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_add_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_sub(vector: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_sub_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_mul(vector: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_mul_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_div(vector: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_div_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_rem(vector: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(rem(vector.0, rhs.0))
        }

        #[inline]
        fn vector_element_sum(vector: Vec4<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_11_00_01));
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_element_product(vector: Vec4<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_11_00_01));
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_eq_mask(vector: Vec4<f32>, other: Vec4<f32>) -> Mask4<f32> {
            Mask(_mm_cmpeq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ne_mask(vector: Vec4<f32>, other: Vec4<f32>) -> Mask4<f32> {
            Mask(_mm_cmpneq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_lt_mask(vector: Vec4<f32>, other: Vec4<f32>) -> Mask4<f32> {
            Mask(_mm_cmplt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_gt_mask(vector: Vec4<f32>, other: Vec4<f32>) -> Mask4<f32> {
            Mask(_mm_cmpgt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_le_mask(vector: Vec4<f32>, other: Vec4<f32>) -> Mask4<f32> {
            Mask(_mm_cmple_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ge_mask(vector: Vec4<f32>, other: Vec4<f32>) -> Mask4<f32> {
            Mask(_mm_cmpge_ps(vector.0, other.0))
        }
    }

    #[inline]
    fn mask_from_array(array: [bool; 4]) -> Mask4<f32> {
        // SAFETY: The two intrinsics are part of SSE2.
        unsafe {
            Mask(_mm_castsi128_ps(_mm_set_epi32(
                -(array[3] as i32),
                -(array[2] as i32),
                -(array[1] as i32),
                -(array[0] as i32),
            )))
        }
    }

    #[inline]
    fn mask_splat(value: bool) -> Mask4<f32> {
        // SAFETY: The two intrinsics are part of SSE2.
        unsafe { Mask(_mm_castsi128_ps(_mm_set1_epi32(-(value as i32)))) }
    }

    #[inline]
    fn mask_from_fn<F>(mut f: F) -> Mask4<f32>
    where
        F: FnMut(usize) -> bool,
    {
        Mask::from_array([f(0), f(1), f(2), f(3)])
    }

    #[inline]
    fn mask_to_array(mask: Mask4<f32>) -> [bool; 4] {
        // SAFETY: The intrinsic is part of SSE.
        let bits = unsafe { _mm_movemask_ps(mask.0) };
        [
            bits & 0x1 != 0,
            bits & 0x2 != 0,
            bits & 0x4 != 0,
            bits & 0x8 != 0,
        ]
    }

    #[inline]
    fn mask_all(mask: Mask4<f32>) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) == 0xf }
    }

    #[inline]
    fn mask_any(mask: Mask4<f32>) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) != 0 }
    }

    #[inline]
    fn mask_select(mask: Mask4<f32>, if_true: Vec4<f32>, if_false: Vec4<f32>) -> Vec4<f32> {
        // SAFETY: The three intrinsics are part of SSE.
        Vector(unsafe {
            _mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            )
        })
    }

    #[inline]
    fn mask_get(mask: Mask4<f32>, index: usize) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe {
            match index {
                0 => _mm_movemask_ps(mask.0) & 0x1 != 0,
                1 => _mm_movemask_ps(mask.0) & 0x2 != 0,
                2 => _mm_movemask_ps(mask.0) & 0x4 != 0,
                3 => _mm_movemask_ps(mask.0) & 0x8 != 0,
                _ => panic!("index out of bounds"),
            }
        }
    }

    #[inline]
    fn mask_set(mask: &mut Mask4<f32>, index: usize, value: bool) {
        if index < 4 {
            // SAFETY: `*mut __m128` is valid as `*mut i32` for 4 values. Adding
            // `index` is valid because it was just checked to be less then 4,
            // and the result is a pointer to a valid `i32`.
            let slot = unsafe {
                core::ptr::from_mut::<__m128>(&mut mask.0)
                    .cast::<i32>()
                    .add(index)
                    .as_mut()
                    .unwrap_unchecked()
            };

            *slot = -(value as i32);
        } else {
            panic!("index out of bounds")
        }
    }

    #[inline]
    fn mask_eq(mask: &Mask4<f32>, other: &Mask4<f32>) -> bool {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) == _mm_movemask_ps(other.0) }
    }

    #[inline]
    fn mask_not(mask: Mask4<f32>) -> Mask4<f32> {
        // SAFETY: The two intrinsics are part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, _mm_set1_ps(f32::from_bits(!0))) })
    }

    #[inline]
    fn mask_bitand(mask: Mask4<f32>, rhs: Mask4<f32>) -> Mask4<f32> {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_and_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitor(mask: Mask4<f32>, rhs: Mask4<f32>) -> Mask4<f32> {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_or_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitxor(mask: Mask4<f32>, rhs: Mask4<f32>) -> Mask4<f32> {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, rhs.0) })
    }
}

impl PrimitiveFloatBackend<3, Aligned> for f32 {
    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[inline]
        fn vector_nan_mask(vector: Vec3<f32>) -> Mask3<f32> {
            Mask(nan_mask(vector.0))
        }

        #[inline]
        fn vector_finite_mask(vector: Vec3<f32>) -> Mask3<f32> {
            Mask(finite_mask(vector.0))
        }

        #[inline]
        fn vector_sign_positive_mask(vector: Vec3<f32>) -> Mask3<f32> {
            Mask(sign_positive_mask(vector.0))
        }

        #[inline]
        fn vector_sign_negative_mask(vector: Vec3<f32>) -> Mask3<f32> {
            Mask(sign_negative_mask(vector.0))
        }

        #[inline]
        fn vector_max(vector: Vec3<f32>, other: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_max_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_min(vector: Vec3<f32>, other: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_min_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_abs(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(abs(vector.0))
        }

        #[inline]
        fn vector_signum(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(signum(vector.0))
        }

        #[inline]
        fn vector_copysign(vector: Vec3<f32>, sign: Vec3<f32>) -> Vec3<f32> {
            Vector(copysign(vector.0, sign.0))
        }

        #[inline]
        fn vector_max_element(vector: Vec3<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_10_10));
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_min_element(vector: Vec3<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b01_01_10_10));
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline(always)]
        fn vector_floor(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(floor(vector.0))
        }

        #[inline(always)]
        fn vector_ceil(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(ceil(vector.0))
        }

        #[inline(always)]
        fn vector_round(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(round(vector.0))
        }

        #[inline(always)]
        fn vector_trunc(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(trunc(vector.0))
        }

        #[inline(always)]
        fn vector_sqrt(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_sqrt_ps(vector.0))
        }
    }
}

impl PrimitiveFloatBackend<4, Aligned> for f32 {
    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[inline]
        fn vector_nan_mask(vector: Vec4<f32>) -> Mask4<f32> {
            Mask(nan_mask(vector.0))
        }

        #[inline]
        fn vector_finite_mask(vector: Vec4<f32>) -> Mask4<f32> {
            Mask(finite_mask(vector.0))
        }

        #[inline]
        fn vector_sign_positive_mask(vector: Vec4<f32>) -> Mask4<f32> {
            Mask(sign_positive_mask(vector.0))
        }

        #[inline]
        fn vector_sign_negative_mask(vector: Vec4<f32>) -> Mask4<f32> {
            Mask(sign_negative_mask(vector.0))
        }

        #[inline]
        fn vector_max(vector: Vec4<f32>, other: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_max_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_min(vector: Vec4<f32>, other: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_min_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_abs(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(abs(vector.0))
        }

        #[inline]
        fn vector_signum(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(signum(vector.0))
        }

        #[inline]
        fn vector_copysign(vector: Vec4<f32>, sign: Vec4<f32>) -> Vec4<f32> {
            Vector(copysign(vector.0, sign.0))
        }

        #[inline]
        fn vector_max_element(vector: Vec4<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_11_10));
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_min_element(vector: Vec4<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_11_10));
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline(always)]
        fn vector_floor(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(floor(vector.0))
        }

        #[inline(always)]
        fn vector_ceil(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(ceil(vector.0))
        }

        #[inline(always)]
        fn vector_round(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(round(vector.0))
        }

        #[inline(always)]
        fn vector_trunc(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(trunc(vector.0))
        }

        #[inline(always)]
        fn vector_sqrt(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_sqrt_ps(vector.0))
        }
    }
}

#[inline]
#[target_feature(enable = "sse2")]
fn neg(vector: __m128) -> __m128 {
    _mm_xor_ps(vector, _mm_set1_ps(-0.0))
}

#[inline]
#[target_feature(enable = "sse2")]
fn rem(vector: __m128, rhs: __m128) -> __m128 {
    let result = _mm_sub_ps(vector, _mm_mul_ps(trunc(_mm_div_ps(vector, rhs)), rhs));

    let inf_mask = _mm_cmpeq_ps(abs(rhs), _mm_set1_ps(f32::INFINITY));
    let zero_mask = _mm_cmpeq_ps(rhs, _mm_set1_ps(0.0));
    let result = select(_mm_or_ps(inf_mask, _mm_set1_ps(-0.0)), vector, result);

    select(zero_mask, _mm_set1_ps(f32::NAN), result)
}

#[inline]
#[target_feature(enable = "sse2")]
fn nan_mask(vector: __m128) -> __m128 {
    _mm_cmpneq_ps(vector, vector)
}

#[inline]
#[target_feature(enable = "sse2")]
fn finite_mask(vector: __m128) -> __m128 {
    _mm_cmplt_ps(abs(vector), _mm_set1_ps(f32::INFINITY))
}

#[inline]
#[target_feature(enable = "sse2")]
fn sign_positive_mask(vector: __m128) -> __m128 {
    _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_castps_si128(vector),
        _mm_castps_si128(abs(vector)),
    ))
}

#[inline]
#[target_feature(enable = "sse2")]
fn sign_negative_mask(vector: __m128) -> __m128 {
    _mm_castsi128_ps(_mm_cmpeq_epi32(
        _mm_castps_si128(vector),
        _mm_castps_si128(_mm_or_ps(_mm_set1_ps(-0.0), vector)),
    ))
}

#[inline]
#[target_feature(enable = "sse2")]
fn abs(vector: __m128) -> __m128 {
    _mm_and_ps(_mm_set1_ps(f32::from_bits(0x7fffffff)), vector)
}

#[inline]
#[target_feature(enable = "sse2")]
fn signum(vector: __m128) -> __m128 {
    let result = _mm_or_ps(_mm_set1_ps(1.0), _mm_and_ps(vector, _mm_set1_ps(-0.0)));
    let nan_mask = _mm_cmpneq_ps(vector, vector);

    select(nan_mask, vector, result)
}

#[inline]
#[target_feature(enable = "sse2")]
fn copysign(vector: __m128, sign: __m128) -> __m128 {
    select(_mm_set1_ps(-0.0), sign, vector)
}

#[inline]
#[target_feature(enable = "sse2")]
fn select(mask: __m128, if_true: __m128, if_false: __m128) -> __m128 {
    _mm_or_ps(_mm_and_ps(mask, if_true), _mm_andnot_ps(mask, if_false))
}

#[inline]
#[target_feature(enable = "sse2")]
fn floor(v: __m128) -> __m128 {
    let trunc_v = _mm_cvtepi32_ps(_mm_cvttps_epi32(v));
    let greater_mask = _mm_cmpgt_ps(trunc_v, v);
    // 0 -> 0, 0xffffffff -> -1.0f
    let offset = _mm_cvtepi32_ps(_mm_castps_si128(greater_mask));
    let result = _mm_add_ps(trunc_v, offset);

    // Handle large values, inf, and NaN
    let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
        _mm_castps_si128(abs(v)),
        _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
    ));

    select(
        _mm_and_ps(bounds_mask, _mm_set1_ps(f32::from_bits(0x7fffffff))),
        result,
        v,
    )
}

#[inline]
#[target_feature(enable = "sse2")]
fn ceil(v: __m128) -> __m128 {
    let trunc_v = _mm_cvtepi32_ps(_mm_cvttps_epi32(v));
    let less_mask = _mm_cmplt_ps(trunc_v, v);
    // 0 -> 0, 0xffffffff -> -1.0f
    let neg_offset = _mm_cvtepi32_ps(_mm_castps_si128(less_mask));
    let result = _mm_sub_ps(trunc_v, neg_offset);

    // Handle large values, inf, and NaN
    let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
        _mm_castps_si128(abs(v)),
        _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
    ));

    select(
        _mm_and_ps(bounds_mask, _mm_set1_ps(f32::from_bits(0x7fffffff))),
        result,
        v,
    )
}

#[inline]
#[target_feature(enable = "sse2")]
fn round(vector: __m128) -> __m128 {
    let magic_val = copysign(_mm_set1_ps(8388608.0), vector);
    let result = _mm_sub_ps(_mm_add_ps(vector, magic_val), magic_val);

    let in_bounds_mask = _mm_cmple_ps(abs(vector), _mm_set1_ps(8388608.0));

    select(abs(in_bounds_mask), result, vector)
}

#[inline]
#[target_feature(enable = "sse2")]
fn trunc(vector: __m128) -> __m128 {
    let result = _mm_cvtepi32_ps(_mm_cvttps_epi32(vector));

    // Large value, infinity, and NaN need special handling.
    let in_bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
        _mm_castps_si128(abs(vector)),
        _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
    ));

    select(
        _mm_and_ps(in_bounds_mask, _mm_set1_ps(f32::from_bits(0x7fffffff))),
        result,
        vector,
    )
}
