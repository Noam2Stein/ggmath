#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::arch::asm;

use crate::{
    Simd, SimdBehaviour, Vec2, Vec3, Vec4, Vector,
    vector::{SoundVectorRepr, primitive_api::F32VectorApi, vec3, vec4},
};

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

impl SimdBehaviour<4> for f32 {
    type VectorRepr = __m128;

    #[inline(always)]
    fn vec_from_array(array: [Self; 4]) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_set_ps(array[3], array[2], array[1], array[0]) })
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_set1_ps(value) })
    }

    #[inline(always)]
    unsafe fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(vec: Vec4<Self>) -> Vec2<Self> {
        Vector::from_repr([vec[X_SRC], vec[Y_SRC]])
    }

    #[inline(always)]
    unsafe fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vec4<Self>,
    ) -> Vec3<Self> {
        let result_as_vec4 = vec.swizzle4::<X_SRC, Y_SRC, Z_SRC, Z_SRC>();

        Vector::from_repr(result_as_vec4.repr())
    }

    #[inline(always)]
    unsafe fn vec_swizzle4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vec4<Self>,
    ) -> Vec4<Self> {
        let result: __m128;
        unsafe {
            asm!("shufps {0}, {0}, {1}", inout(xmm_reg) vec.repr() => result, const {
                let x_src_bits = (X_SRC as u32) << 0;
                let y_src_bits = (Y_SRC as u32) << 2;
                let z_src_bits = (Z_SRC as u32) << 4;
                let w_src_bits = (W_SRC as u32) << 6;

                (x_src_bits | y_src_bits | z_src_bits | w_src_bits).cast_signed()
            });
        }

        Vector::from_repr(result)
    }

    #[inline(always)]
    fn vec_neg(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_xor_ps(_mm_set1_ps(-0.0), vec.repr()) })
    }

    #[inline(always)]
    fn vec_add(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_add_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_sub(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_sub_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_mul(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_mul_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_div(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_div_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_rem(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        vec - (vec / rhs).trunc() * rhs
    }
}

impl F32VectorApi<4, Simd> for f32 {
    #[inline(always)]
    fn vec_floor(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_floor_ps(vec.repr()) })
    }

    #[inline(always)]
    fn vec_ceil(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_ceil_ps(vec.repr()) })
    }

    #[inline(always)]
    fn vec_round(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_round_ps::<_MM_FROUND_TO_NEAREST_INT>(vec.repr()) })
    }

    #[inline(always)]
    fn vec_trunc(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_round_ps::<_MM_FROUND_TO_ZERO>(vec.repr()) })
    }

    #[inline(always)]
    fn vec_fract(vec: Vec4<Self>) -> Vec4<Self> {
        vec - vec.trunc()
    }

    #[inline(always)]
    fn vec_mul_add(vec: Vec4<Self>, a: Vec4<Self>, b: Vec4<Self>) -> Vec4<Self> {
        #[cfg(target_feature = "fma")]
        {
            Vector::from_repr(unsafe { _mm_fmadd_ps(vec.repr(), a.repr(), b.repr()) })
        }

        #[cfg(not(target_feature = "fma"))]
        {
            vec4!(
                vec.x.mul_add(a.x, b.x),
                vec.y.mul_add(a.y, b.y),
                vec.z.mul_add(a.z, b.z),
                vec.w.mul_add(a.w, b.w),
            )
        }
    }

    // TODO: determine if div_euclid can be optimized

    // TODO: determine if rem_euclid can be optimized

    #[inline(always)]
    fn vec_sqrt(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_sqrt_ps(vec.repr()) })
    }

    #[inline(always)]
    fn vec_sin(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(vec.x.sin(), vec.y.sin(), vec.z.sin(), vec.w.sin(),)
    }

    #[inline(always)]
    fn vec_cos(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(vec.x.cos(), vec.y.cos(), vec.z.cos(), vec.w.cos(),)
    }

    #[inline(always)]
    fn vec_tan(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(vec.x.tan(), vec.y.tan(), vec.z.tan(), vec.w.tan(),)
    }

    #[inline(always)]
    fn vec_asin(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(vec.x.asin(), vec.y.asin(), vec.z.asin(), vec.w.asin(),)
    }

    #[inline(always)]
    fn vec_acos(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(vec.x.acos(), vec.y.acos(), vec.z.acos(), vec.w.acos(),)
    }

    #[inline(always)]
    fn vec_atan(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(vec.x.atan(), vec.y.atan(), vec.z.atan(), vec.w.atan(),)
    }

    #[inline(always)]
    fn vec_recip(vec: Vec4<Self>) -> Vec4<Self> {
        vec4!(1.0) / vec
    }

    #[inline(always)]
    fn vec_max(vec: Vec4<Self>, other: Vec4<Self>) -> Vec4<Self> {
        debug_assert!(!vec.iter().any(f32::is_nan));
        debug_assert!(!other.iter().any(f32::is_nan));

        Vector::from_repr(unsafe { _mm_max_ps(vec.repr(), other.repr()) })
    }

    #[inline(always)]
    fn vec_min(vec: Vec4<Self>, other: Vec4<Self>) -> Vec4<Self> {
        debug_assert!(!vec.iter().any(f32::is_nan));
        debug_assert!(!other.iter().any(f32::is_nan));

        Vector::from_repr(unsafe { _mm_min_ps(vec.repr(), other.repr()) })
    }

    #[inline(always)]
    fn vec_midpoint(vec: Vec4<Self>, other: Vec4<Self>) -> Vec4<Self> {
        (vec + other) / 2.0
    }

    #[inline(always)]
    fn vec_clamp(vec: Vec4<Self>, min: Vec4<Self>, max: Vec4<Self>) -> Vec4<Self> {
        debug_assert!(!vec.iter().any(f32::is_nan));
        debug_assert!(!min.iter().any(f32::is_nan));
        debug_assert!(!max.iter().any(f32::is_nan));
        debug_assert!(min.iter().zip(max).all(|(min, max)| min <= max));

        vec.max(min).min(max)
    }

    #[inline(always)]
    fn vec_abs(vec: Vec4<Self>) -> Vec4<Self> {
        const NO_SIGN_MASK: Vec4<f32> =
            Vector::const_from_array([f32::from_bits(0x7f_ff_ff_ff); 4]);

        Vector::from_repr(unsafe { _mm_and_ps(vec.repr(), NO_SIGN_MASK.repr()) })
    }

    #[inline(always)]
    fn vec_signum(vec: Vec4<Self>) -> Vec4<Self> {
        // TODO: optimize
        vec4!(
            vec.x.signum(),
            vec.y.signum(),
            vec.z.signum(),
            vec.w.signum(),
        )
    }

    #[inline(always)]
    fn vec_copysign(vec: Vec4<Self>, sign: Vec4<Self>) -> Vec4<Self> {
        const SIGN_MASK: Vec4<f32> = Vector::const_from_array([-0.0; 4]);

        Vector::from_repr(unsafe {
            _mm_or_ps(
                _mm_and_ps(sign.repr(), SIGN_MASK.repr()),
                _mm_andnot_ps(SIGN_MASK.repr(), vec.repr()),
            )
        })
    }

    #[inline(always)]
    fn vec_sum(vec: Vec4<Self>) -> Self {
        #[cfg(target_feature = "sse3")]
        unsafe {
            let xplusy_zplusw__ = _mm_hadd_ps(vec.repr(), vec.repr());
            let sum___ = _mm_hadd_ps(xplusy_zplusw__, xplusy_zplusw__);

            _mm_cvtss_f32(sum___)
        }

        #[cfg(not(target_feature = "sse3"))]
        {
            let xy__ = vec;
            let zw__ = vec.swizzle4::<2, 3, 0, 1>();
            let xplusz_yplusw__ = xy__ + zw__;
            let yplusw_xplusz__ = xplusz_yplusw__.swizzle4::<1, 0, 3, 2>();
            let sum___ = xplusz_yplusw__ + yplusw_xplusz__;

            sum___.x
        }
    }

    #[inline(always)]
    fn vec_product(vec: Vec4<Self>) -> Self {
        let xy__ = vec;
        let zw__ = vec.swizzle4::<2, 3, 0, 1>();
        let xtimesz_ytimesw__ = xy__ * zw__;
        let ytimesw_xtimesz__ = xtimesz_ytimesw__.swizzle4::<1, 0, 3, 2>();
        let product___ = xtimesz_ytimesw__ * ytimesw_xtimesz__;

        product___.x
    }
}

// SAFETY: __m128 contains exactly 4 f32s
unsafe impl SoundVectorRepr<4, f32> for __m128 {}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

impl SimdBehaviour<3> for f32 {
    type VectorRepr = __m128;

    #[inline(always)]
    fn vec_from_array(array: [Self; 3]) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_set_ps(array[2], array[2], array[1], array[0]) })
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_set1_ps(value) })
    }

    #[inline(always)]
    unsafe fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(vec: Vec3<Self>) -> Vec2<Self> {
        Vector::from_repr([vec[X_SRC], vec[Y_SRC]])
    }

    #[inline(always)]
    unsafe fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vec3<Self>,
    ) -> Vec3<Self> {
        let vec_as_vec4 = Vec4::from_repr(vec.repr());

        vec_as_vec4.swizzle3::<X_SRC, Y_SRC, Z_SRC>()
    }

    #[inline(always)]
    unsafe fn vec_swizzle4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vec3<Self>,
    ) -> Vec4<Self> {
        let vec_as_vec4 = Vec4::from_repr(vec.repr());

        vec_as_vec4.swizzle4::<X_SRC, Y_SRC, Z_SRC, W_SRC>()
    }

    #[inline(always)]
    fn vec_neg(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_xor_ps(_mm_set1_ps(-0.0), vec.repr()) })
    }

    #[inline(always)]
    fn vec_add(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_add_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_sub(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_sub_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_mul(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_mul_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_div(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_div_ps(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_rem(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        vec - (vec / rhs).trunc() * rhs
    }
}

impl F32VectorApi<3, Simd> for f32 {
    #[inline(always)]
    fn vec_floor(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_floor_ps(vec.repr()) })
    }

    #[inline(always)]
    fn vec_ceil(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_ceil_ps(vec.repr()) })
    }

    #[inline(always)]
    fn vec_round(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_round_ps::<_MM_FROUND_TO_NEAREST_INT>(vec.repr()) })
    }

    #[inline(always)]
    fn vec_trunc(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_round_ps::<_MM_FROUND_TO_ZERO>(vec.repr()) })
    }

    #[inline(always)]
    fn vec_fract(vec: Vec3<Self>) -> Vec3<Self> {
        vec - vec.trunc()
    }

    #[inline(always)]
    fn vec_mul_add(vec: Vec3<Self>, a: Vec3<Self>, b: Vec3<Self>) -> Vec3<Self> {
        #[cfg(target_feature = "fma")]
        {
            Vector::from_repr(unsafe { _mm_fmadd_ps(vec.repr(), a.repr(), b.repr()) })
        }

        #[cfg(not(target_feature = "fma"))]
        {
            vec3!(
                vec.x.mul_add(a.x, b.x),
                vec.y.mul_add(a.y, b.y),
                vec.z.mul_add(a.z, b.z),
            )
        }
    }

    // TODO: determine if div_euclid can be optimized

    // TODO: determine if rem_euclid can be optimized

    #[inline(always)]
    fn vec_sqrt(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_sqrt_ps(vec.repr()) })
    }

    #[inline(always)]
    fn vec_sin(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.sin(), vec.y.sin(), vec.z.sin(),)
    }

    #[inline(always)]
    fn vec_cos(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.cos(), vec.y.cos(), vec.z.cos(),)
    }

    #[inline(always)]
    fn vec_tan(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.tan(), vec.y.tan(), vec.z.tan(),)
    }

    #[inline(always)]
    fn vec_asin(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.asin(), vec.y.asin(), vec.z.asin(),)
    }

    #[inline(always)]
    fn vec_acos(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.acos(), vec.y.acos(), vec.z.acos(),)
    }

    #[inline(always)]
    fn vec_atan(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.atan(), vec.y.atan(), vec.z.atan(),)
    }

    #[inline(always)]
    fn vec_recip(vec: Vec3<Self>) -> Vec3<Self> {
        vec3!(1.0) / vec
    }

    #[inline(always)]
    fn vec_max(vec: Vec3<Self>, other: Vec3<Self>) -> Vec3<Self> {
        debug_assert!(!vec.iter().any(f32::is_nan));
        debug_assert!(!other.iter().any(f32::is_nan));

        Vector::from_repr(unsafe { _mm_max_ps(vec.repr(), other.repr()) })
    }

    #[inline(always)]
    fn vec_min(vec: Vec3<Self>, other: Vec3<Self>) -> Vec3<Self> {
        debug_assert!(!vec.iter().any(f32::is_nan));
        debug_assert!(!other.iter().any(f32::is_nan));

        Vector::from_repr(unsafe { _mm_min_ps(vec.repr(), other.repr()) })
    }

    #[inline(always)]
    fn vec_midpoint(vec: Vec3<Self>, other: Vec3<Self>) -> Vec3<Self> {
        (vec + other) / 2.0
    }

    #[inline(always)]
    fn vec_clamp(vec: Vec3<Self>, min: Vec3<Self>, max: Vec3<Self>) -> Vec3<Self> {
        debug_assert!(!vec.iter().any(f32::is_nan));
        debug_assert!(!min.iter().any(f32::is_nan));
        debug_assert!(!max.iter().any(f32::is_nan));
        debug_assert!(min.iter().zip(max).all(|(min, max)| min <= max));

        vec.max(min).min(max)
    }

    #[inline(always)]
    fn vec_abs(vec: Vec3<Self>) -> Vec3<Self> {
        const NO_SIGN_MASK: Vec4<f32> =
            Vector::const_from_array([f32::from_bits(0x7f_ff_ff_ff); 4]);

        Vector::from_repr(unsafe { _mm_and_ps(vec.repr(), NO_SIGN_MASK.repr()) })
    }

    #[inline(always)]
    fn vec_signum(vec: Vec3<Self>) -> Vec3<Self> {
        // TODO: optimize
        vec3!(vec.x.signum(), vec.y.signum(), vec.z.signum(),)
    }

    #[inline(always)]
    fn vec_copysign(vec: Vec3<Self>, sign: Vec3<Self>) -> Vec3<Self> {
        const SIGN_MASK: Vec4<f32> = Vector::const_from_array([-0.0; 4]);

        Vector::from_repr(unsafe {
            _mm_or_ps(
                _mm_and_ps(sign.repr(), SIGN_MASK.repr()),
                _mm_andnot_ps(SIGN_MASK.repr(), vec.repr()),
            )
        })
    }

    #[inline(always)]
    fn vec_sum(vec: Vec3<Self>) -> Self {
        #[cfg(target_feature = "sse3")]
        unsafe {
            const NO_W_MASK: Vec4<f32> = Vector::const_from_array([
                f32::from_bits(0xff_ff_ff_ff),
                f32::from_bits(0xff_ff_ff_ff),
                f32::from_bits(0xff_ff_ff_ff),
                f32::from_bits(0x00_00_00_00),
            ]);

            let xyz0 = _mm_and_ps(vec.repr(), NO_W_MASK.repr());
            let xplusy_zplus0__ = _mm_hadd_ps(xyz0, xyz0);
            let sum___ = _mm_hadd_ps(xplusy_zplus0__, xplusy_zplus0__);

            _mm_cvtss_f32(sum___)
        }

        #[cfg(not(target_feature = "sse3"))]
        {
            vec.x + vec.y + vec.z
        }
    }

    #[inline(always)]
    fn vec_product(vec: Vec3<Self>) -> Self {
        vec.x * vec.y * vec.z
    }
}

// SAFETY: __m128 contains exactly 4 f32s, so it does begin with 3 f32s
unsafe impl SoundVectorRepr<3, f32> for __m128 {}
