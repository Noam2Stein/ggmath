#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    Aligned, Mask, Mask3, Mask4, PrimitiveFloatBackend, ScalarBackend, Unaligned, Vec3, Vec4,
    Vector, utils::safe_arch,
};

impl ScalarBackend<2, Aligned> for f32 {}

impl ScalarBackend<3, Aligned> for f32 {
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
}

impl ScalarBackend<4, Aligned> for f32 {
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
}

impl ScalarBackend<2, Unaligned> for f32 {}
impl ScalarBackend<3, Unaligned> for f32 {}
impl ScalarBackend<4, Unaligned> for f32 {}

impl PrimitiveFloatBackend<2, Aligned> for f32 {}

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

        #[cfg(backend)]
        #[inline(always)]
        fn vector_floor(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(floor(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_ceil(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(ceil(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_round(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(round(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_trunc(vector: Vec3<f32>) -> Vec3<f32> {
            Vector(trunc(vector.0))
        }

        #[cfg(backend)]
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

        #[cfg(backend)]
        #[inline(always)]
        fn vector_floor(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(floor(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_ceil(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(ceil(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_round(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(round(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_trunc(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(trunc(vector.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vector_sqrt(vector: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_sqrt_ps(vector.0))
        }
    }
}

impl PrimitiveFloatBackend<2, Unaligned> for f32 {}
impl PrimitiveFloatBackend<3, Unaligned> for f32 {}
impl PrimitiveFloatBackend<4, Unaligned> for f32 {}

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

#[cfg(backend)]
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

#[cfg(backend)]
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

#[cfg(backend)]
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
