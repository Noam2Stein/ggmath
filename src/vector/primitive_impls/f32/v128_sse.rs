#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    Vec3A, Vec4A,
    vector::{
        Aligned, ScalarBackend, SoundVectorRepr, Vector, primitive_apis::f32::FloatBackend,
        primitive_impls::safe_arch,
    },
};

impl ScalarBackend<3, Aligned> for f32 {
    type VectorRepr = __m128;

    safe_arch! {
        for "sse":

        #[inline(always)]
        fn vec_eq(vec: Vec3A, other: Vec3A) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vec.repr(), other.repr())) as u32 & 0b111 == 0b111
        }

        #[inline(always)]
        fn vec_ne(vec: Vec3A, other: Vec3A) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vec.repr(), other.repr())) as u32 & 0b111 != 0
        }

        #[inline(always)]
        fn vec_neg(vec: Vec3A) -> Vec3A {
            Vector::from_repr(neg(vec.repr()))
        }

        #[inline(always)]
        fn vec_add(vec: Vec3A, rhs: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_add_ps(vec.repr(), rhs.repr()))
        }

        #[inline(always)]
        fn vec_sub(vec: Vec3A, rhs: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_sub_ps(vec.repr(), rhs.repr()))
        }

        #[inline(always)]
        fn vec_mul(vec: Vec3A, rhs: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_mul_ps(vec.repr(), rhs.repr()))
        }

        #[inline(always)]
        fn vec_div(vec: Vec3A, rhs: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_div_ps(vec.repr(), rhs.repr()))
        }
    }

    #[cfg(target_feature = "sse2")]
    safe_arch! {
        for "sse2":

        #[inline(always)]
        fn vec_rem(vec: Vec3A, rhs: Vec3A) -> Vec3A {
            Vector::from_repr(rem(vec.repr(), rhs.repr()))
        }
    }
}

impl ScalarBackend<4, Aligned> for f32 {
    type VectorRepr = __m128;

    safe_arch! {
        for "sse":

        #[inline(always)]
        fn vec_eq(vec: Vec4A, other: Vec4A) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vec.repr(), other.repr())) as u32 == 0xf
        }

        #[inline(always)]
        fn vec_ne(vec: Vec4A, other: Vec4A) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vec.repr(), other.repr())) as u32 != 0
        }

        #[inline(always)]
        fn vec_neg(vec: Vec4A) -> Vec4A {
            Vector::from_repr(neg(vec.repr()))
        }

        #[inline(always)]
        fn vec_add(vec: Vec4A, rhs: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_add_ps(vec.repr(), rhs.repr()))
        }

        #[inline(always)]
        fn vec_sub(vec: Vec4A, rhs: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_sub_ps(vec.repr(), rhs.repr()))
        }

        #[inline(always)]
        fn vec_mul(vec: Vec4A, rhs: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_mul_ps(vec.repr(), rhs.repr()))
        }

        #[inline(always)]
        fn vec_div(vec: Vec4A, rhs: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_div_ps(vec.repr(), rhs.repr()))
        }
    }

    #[cfg(target_feature = "sse2")]
    safe_arch! {
        for "sse2":

        #[inline(always)]
        fn vec_rem(vec: Vec4A, rhs: Vec4A) -> Vec4A {
            Vector::from_repr(rem(vec.repr(), rhs.repr()))
        }
    }
}

impl FloatBackend<3, Aligned> for f32 {
    safe_arch! {
        for "sse":

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_round(vec: Vec3A) -> Vec3A {
            Vector::from_repr(round(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_sqrt(vec: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_sqrt_ps(vec.repr()))
        }

        #[inline(always)]
        fn vec_recip(vec: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_div_ps(_mm_set1_ps(1.0), vec.repr()))
        }

        #[inline(always)]
        fn vec_max(vec: Vec3A, other: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_max_ps(vec.repr(), other.repr()))
        }

        #[inline(always)]
        fn vec_min(vec: Vec3A, other: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_min_ps(vec.repr(), other.repr()))
        }

        #[inline(always)]
        fn vec_clamp(vec: Vec3A, min: Vec3A, max: Vec3A) -> Vec3A {
            Vector::from_repr(_mm_min_ps(_mm_max_ps(vec.repr(), min.repr()), max.repr()))
        }

        #[inline(always)]
        fn vec_abs(vec: Vec3A) -> Vec3A {
            Vector::from_repr(abs(vec.repr()))
        }

        #[inline(always)]
        fn vec_signum(vec: Vec3A) -> Vec3A {
            Vector::from_repr(signum(vec.repr()))
        }

        #[inline(always)]
        fn vec_copysign(vec: Vec3A, sign: Vec3A) -> Vec3A {
            Vector::from_repr(copysign(vec.repr(), sign.repr()))
        }
    }

    #[cfg(target_feature = "sse2")]
    safe_arch! {
        for "sse2":

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_floor(vec: Vec3A) -> Vec3A {
            Vector::from_repr(floor(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_ceil(vec: Vec3A) -> Vec3A {
            Vector::from_repr(ceil(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_trunc(vec: Vec3A) -> Vec3A {
            Vector::from_repr(trunc(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_fract(vec: Vec3A) -> Vec3A {
            Vector::from_repr(fract(vec.repr()))
        }
    }
}

impl FloatBackend<4, Aligned> for f32 {
    safe_arch! {
        for "sse":

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_round(vec: Vec4A) -> Vec4A {
            Vector::from_repr(round(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_sqrt(vec: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_sqrt_ps(vec.repr()))
        }

        #[inline(always)]
        fn vec_recip(vec: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_div_ps(_mm_set1_ps(1.0), vec.repr()))
        }

        #[inline(always)]
        fn vec_max(vec: Vec4A, other: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_max_ps(vec.repr(), other.repr()))
        }

        #[inline(always)]
        fn vec_min(vec: Vec4A, other: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_min_ps(vec.repr(), other.repr()))
        }

        #[inline(always)]
        fn vec_clamp(vec: Vec4A, min: Vec4A, max: Vec4A) -> Vec4A {
            Vector::from_repr(_mm_min_ps(_mm_max_ps(vec.repr(), min.repr()), max.repr()))
        }

        #[inline(always)]
        fn vec_abs(vec: Vec4A) -> Vec4A {
            Vector::from_repr(abs(vec.repr()))
        }

        #[inline(always)]
        fn vec_signum(vec: Vec4A) -> Vec4A {
            Vector::from_repr(signum(vec.repr()))
        }

        #[inline(always)]
        fn vec_copysign(vec: Vec4A, sign: Vec4A) -> Vec4A {
            Vector::from_repr(copysign(vec.repr(), sign.repr()))
        }
    }

    #[cfg(target_feature = "sse2")]
    safe_arch! {
        for "sse2":

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_floor(vec: Vec4A) -> Vec4A {
            Vector::from_repr(floor(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_ceil(vec: Vec4A) -> Vec4A {
            Vector::from_repr(ceil(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_trunc(vec: Vec4A) -> Vec4A {
            Vector::from_repr(trunc(vec.repr()))
        }

        #[cfg(feature = "std")]
        #[inline(always)]
        fn vec_fract(vec: Vec4A) -> Vec4A {
            Vector::from_repr(fract(vec.repr()))
        }
    }
}

unsafe impl SoundVectorRepr<3, f32> for __m128 {
    type ActualRepr = Self;
}
unsafe impl SoundVectorRepr<4, f32> for __m128 {
    type ActualRepr = Self;
}

#[inline]
#[target_feature(enable = "sse")]
fn neg(v: __m128) -> __m128 {
    _mm_xor_ps(v, _mm_set1_ps(-0.0))
}

#[inline]
#[target_feature(enable = "sse2")]
fn rem(a: __m128, b: __m128) -> __m128 {
    let result = _mm_sub_ps(a, _mm_mul_ps(trunc(_mm_div_ps(a, b)), b));

    let inf_mask = _mm_cmpeq_ps(abs(b), _mm_set1_ps(f32::INFINITY));
    let result = select(_mm_or_ps(inf_mask, _mm_set1_ps(-0.0)), a, result);

    let zero_mask = _mm_cmpeq_ps(b, _mm_set1_ps(0.0));
    select(zero_mask, _mm_set1_ps(f32::NAN), result)
}

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
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

#[cfg(feature = "std")]
#[inline]
#[target_feature(enable = "sse")]
fn round(v: __m128) -> __m128 {
    let magic_val = copysign(_mm_set1_ps(8388608.0), v);
    let result = _mm_sub_ps(_mm_add_ps(v, magic_val), magic_val);

    let bounds_mask = _mm_cmple_ps(abs(v), _mm_set1_ps(8388608.0));

    select(
        _mm_and_ps(bounds_mask, _mm_set1_ps(f32::from_bits(0x7fffffff))),
        result,
        v,
    )
}

#[inline]
#[target_feature(enable = "sse2")]
fn trunc(v: __m128) -> __m128 {
    let result = _mm_cvtepi32_ps(_mm_cvttps_epi32(v));

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

#[cfg(feature = "std")]
#[inline]
#[target_feature(enable = "sse2")]
fn fract(v: __m128) -> __m128 {
    _mm_sub_ps(v, trunc(v))
}

#[inline]
#[target_feature(enable = "sse")]
fn abs(v: __m128) -> __m128 {
    _mm_and_ps(_mm_set1_ps(f32::from_bits(0x7fffffff)), v)
}

#[inline]
#[target_feature(enable = "sse")]
fn signum(v: __m128) -> __m128 {
    let result = _mm_or_ps(_mm_set1_ps(1.0), _mm_and_ps(v, _mm_set1_ps(-0.0)));
    let nan_mask = _mm_cmpneq_ps(v, v);

    select(nan_mask, v, result)
}

#[inline]
#[target_feature(enable = "sse")]
fn copysign(v: __m128, sign: __m128) -> __m128 {
    select(_mm_set1_ps(-0.0), sign, v)
}

#[inline]
#[target_feature(enable = "sse")]
fn select(mask: __m128, if_true: __m128, if_false: __m128) -> __m128 {
    _mm_or_ps(_mm_and_ps(mask, if_true), _mm_andnot_ps(mask, if_false))
}
