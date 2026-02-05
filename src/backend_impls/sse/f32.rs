#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    Aligned, F32VectorBackend, ScalarBackend, Unaligned, Vec3, Vec4, Vector, utils::safe_arch,
};

impl ScalarBackend<2, Aligned> for f32 {}

impl ScalarBackend<3, Aligned> for f32 {
    safe_arch! {
        #![target_feature(enable = "sse")]

        #[inline]
        fn vec_eq(vec: &Vec3<f32>, other: &Vec3<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vec.0, other.0)) as u32 & 0b111 == 0b111
        }

        #[inline]
        fn vec_ne(vec: &Vec3<f32>, other: &Vec3<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vec.0, other.0)) as u32 & 0b111 != 0
        }

        #[inline]
        fn vec_neg(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(neg(vec.0))
        }

        #[inline]
        fn vec_add(vec: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_add_ps(vec.0, rhs.0))
        }

        #[inline]
        fn vec_sub(vec: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_sub_ps(vec.0, rhs.0))
        }

        #[inline]
        fn vec_mul(vec: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_mul_ps(vec.0, rhs.0))
        }

        #[inline]
        fn vec_div(vec: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_div_ps(vec.0, rhs.0))
        }
    }

    #[cfg(target_feature = "sse2")]
    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[inline]
        fn vec_rem(vec: Vec3<f32>, rhs: Vec3<f32>) -> Vec3<f32> {
            Vector(rem(vec.0, rhs.0))
        }
    }
}

impl ScalarBackend<4, Aligned> for f32 {
    safe_arch! {
        #![target_feature(enable = "sse")]

        #[inline]
        fn vec_eq(vec: &Vec4<f32>, other: &Vec4<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vec.0, other.0)) as u32 == 0xf
        }

        #[inline]
        fn vec_ne(vec: &Vec4<f32>, other: &Vec4<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vec.0, other.0)) as u32 != 0
        }

        #[inline]
        fn vec_neg(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(neg(vec.0))
        }

        #[inline]
        fn vec_add(vec: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_add_ps(vec.0, rhs.0))
        }

        #[inline]
        fn vec_sub(vec: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_sub_ps(vec.0, rhs.0))
        }

        #[inline]
        fn vec_mul(vec: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_mul_ps(vec.0, rhs.0))
        }

        #[inline]
        fn vec_div(vec: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_div_ps(vec.0, rhs.0))
        }
    }

    #[cfg(target_feature = "sse2")]
    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[inline]
        fn vec_rem(vec: Vec4<f32>, rhs: Vec4<f32>) -> Vec4<f32> {
            Vector(rem(vec.0, rhs.0))
        }
    }
}

impl ScalarBackend<2, Unaligned> for f32 {}
impl ScalarBackend<3, Unaligned> for f32 {}
impl ScalarBackend<4, Unaligned> for f32 {}

impl F32VectorBackend<2, Aligned> for f32 {}

impl F32VectorBackend<3, Aligned> for f32 {
    safe_arch! {
        #![target_feature(enable = "sse")]

        #[inline]
        fn vec_is_nan(vec: Vec3<f32>) -> bool {
            _mm_movemask_ps(nan_mask(vec.0)) as u32 & 0b111 != 0
        }

        #[inline]
        fn vec_is_finite(vec: Vec3<f32>) -> bool {
            _mm_movemask_ps(finite_mask(vec.0)) as u32 & 0b111 == 0b111
        }

        #[inline]
        fn vec_max(vec: Vec3<f32>, other: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_max_ps(vec.0, other.0))
        }

        #[inline]
        fn vec_min(vec: Vec3<f32>, other: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_min_ps(vec.0, other.0))
        }

        #[inline]
        fn vec_abs(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(abs(vec.0))
        }

        #[inline]
        fn vec_signum(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(signum(vec.0))
        }

        #[inline]
        fn vec_copysign(vec: Vec3<f32>, sign: Vec3<f32>) -> Vec3<f32> {
            Vector(copysign(vec.0, sign.0))
        }

        #[inline]
        fn vec_element_sum(vec: Vec3<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_add_ps(vec, _mm_shuffle_ps(vec, _mm_set1_ps(0.0), 0b00_11_00_01));
            let vec = _mm_add_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_10));
            _mm_cvtss_f32(vec)
        }

        #[inline]
        fn vec_element_product(vec: Vec3<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_mul_ps(vec, _mm_shuffle_ps(vec, _mm_set1_ps(1.0), 0b00_11_00_01));
            let vec = _mm_mul_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_10));
            _mm_cvtss_f32(vec)
        }

        #[inline]
        fn vec_max_element(vec: Vec3<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_max_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_10_10));
            let vec = _mm_max_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_01));
            _mm_cvtss_f32(vec)
        }

        #[inline]
        fn vec_min_element(vec: Vec3<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_min_ps(vec, _mm_shuffle_ps(vec, vec, 0b01_01_10_10));
            let vec = _mm_min_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_01));
            _mm_cvtss_f32(vec)
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_round(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(round(vec.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_sqrt(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(_mm_sqrt_ps(vec.0))
        }
    }

    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[cfg(backend)]
        #[inline(always)]
        fn vec_floor(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(floor(vec.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_ceil(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(ceil(vec.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_trunc(vec: Vec3<f32>) -> Vec3<f32> {
            Vector(trunc(vec.0))
        }
    }
}

impl F32VectorBackend<4, Aligned> for f32 {
    safe_arch! {
        #![target_feature(enable = "sse")]

        #[inline]
        fn vec_is_nan(vec: Vec4<f32>) -> bool {
            _mm_movemask_ps(nan_mask(vec.0)) as u32 != 0
        }

        #[inline]
        fn vec_is_finite(vec: Vec4<f32>) -> bool {
            _mm_movemask_ps(finite_mask(vec.0)) as u32 == 0xf
        }

        #[inline]
        fn vec_max(vec: Vec4<f32>, other: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_max_ps(vec.0, other.0))
        }

        #[inline]
        fn vec_min(vec: Vec4<f32>, other: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_min_ps(vec.0, other.0))
        }

        #[inline]
        fn vec_abs(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(abs(vec.0))
        }

        #[inline]
        fn vec_signum(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(signum(vec.0))
        }

        #[inline]
        fn vec_copysign(vec: Vec4<f32>, sign: Vec4<f32>) -> Vec4<f32> {
            Vector(copysign(vec.0, sign.0))
        }

        #[inline]
        fn vec_element_sum(vec: Vec4<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_add_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_11_00_01));
            let vec = _mm_add_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_10));
            _mm_cvtss_f32(vec)
        }

        #[inline]
        fn vec_element_product(vec: Vec4<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_mul_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_11_00_01));
            let vec = _mm_mul_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_10));
            _mm_cvtss_f32(vec)
        }

        #[inline]
        fn vec_max_element(vec: Vec4<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_max_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_11_10));
            let vec = _mm_max_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_01));
            _mm_cvtss_f32(vec)
        }

        #[inline]
        fn vec_min_element(vec: Vec4<f32>) -> f32 {
            let vec = vec.0;
            let vec = _mm_min_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_11_10));
            let vec = _mm_min_ps(vec, _mm_shuffle_ps(vec, vec, 0b00_00_00_01));
            _mm_cvtss_f32(vec)
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_round(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(round(vec.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_sqrt(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(_mm_sqrt_ps(vec.0))
        }
    }

    safe_arch! {
        #![target_feature(enable = "sse2")]

        #[cfg(backend)]
        #[inline(always)]
        fn vec_floor(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(floor(vec.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_ceil(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(ceil(vec.0))
        }

        #[cfg(backend)]
        #[inline(always)]
        fn vec_trunc(vec: Vec4<f32>) -> Vec4<f32> {
            Vector(trunc(vec.0))
        }
    }
}

impl F32VectorBackend<2, Unaligned> for f32 {}
impl F32VectorBackend<3, Unaligned> for f32 {}
impl F32VectorBackend<4, Unaligned> for f32 {}

#[inline]
#[target_feature(enable = "sse")]
fn neg(vec: __m128) -> __m128 {
    _mm_xor_ps(vec, _mm_set1_ps(-0.0))
}

#[inline]
#[target_feature(enable = "sse2")]
fn rem(vec: __m128, rhs: __m128) -> __m128 {
    let result = _mm_sub_ps(vec, _mm_mul_ps(trunc(_mm_div_ps(vec, rhs)), rhs));

    let inf_mask = _mm_cmpeq_ps(abs(rhs), _mm_set1_ps(f32::INFINITY));
    let zero_mask = _mm_cmpeq_ps(rhs, _mm_set1_ps(0.0));
    let result = select(_mm_or_ps(inf_mask, _mm_set1_ps(-0.0)), vec, result);

    select(zero_mask, _mm_set1_ps(f32::NAN), result)
}

#[inline]
#[target_feature(enable = "sse")]
fn nan_mask(vec: __m128) -> __m128 {
    _mm_cmpneq_ps(vec, vec)
}

#[inline]
#[target_feature(enable = "sse")]
fn finite_mask(vec: __m128) -> __m128 {
    _mm_cmplt_ps(abs(vec), _mm_set1_ps(f32::INFINITY))
}

#[inline]
#[target_feature(enable = "sse")]
fn abs(vec: __m128) -> __m128 {
    _mm_and_ps(_mm_set1_ps(f32::from_bits(0x7fffffff)), vec)
}

#[inline]
#[target_feature(enable = "sse")]
fn signum(vec: __m128) -> __m128 {
    let result = _mm_or_ps(_mm_set1_ps(1.0), _mm_and_ps(vec, _mm_set1_ps(-0.0)));
    let nan_mask = _mm_cmpneq_ps(vec, vec);

    select(nan_mask, vec, result)
}

#[inline]
#[target_feature(enable = "sse")]
fn copysign(vec: __m128, sign: __m128) -> __m128 {
    select(_mm_set1_ps(-0.0), sign, vec)
}

#[inline]
#[target_feature(enable = "sse")]
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
#[target_feature(enable = "sse")]
fn round(vec: __m128) -> __m128 {
    let magic_val = copysign(_mm_set1_ps(8388608.0), vec);
    let result = _mm_sub_ps(_mm_add_ps(vec, magic_val), magic_val);

    let in_bounds_mask = _mm_cmple_ps(abs(vec), _mm_set1_ps(8388608.0));

    select(abs(in_bounds_mask), result, vec)
}

#[inline]
#[target_feature(enable = "sse2")]
fn trunc(vec: __m128) -> __m128 {
    let result = _mm_cvtepi32_ps(_mm_cvttps_epi32(vec));

    // Large value, infinity, and NaN need special handling.
    let in_bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
        _mm_castps_si128(abs(vec)),
        _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
    ));

    select(
        _mm_and_ps(in_bounds_mask, _mm_set1_ps(f32::from_bits(0x7fffffff))),
        result,
        vec,
    )
}
