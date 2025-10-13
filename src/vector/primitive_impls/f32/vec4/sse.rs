#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::{
    arch::asm,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::{InnerVectorType, Scalar, Simd, Vector, vec4, vector::primitive_api::f32::ScalarF32};

impl Scalar<4, Simd> for f32 {
    type InnerVectorType = __m128;

    #[inline(always)]
    fn vec_from_array(array: [Self; 4]) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_set_ps(array[3], array[2], array[1], array[0]) })
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_set1_ps(value) })
    }

    #[inline(always)]
    fn vec_as_array(vec: Vector<4, Self, Simd>) -> [Self; 4] {
        *vec.as_array_ref()
    }

    #[inline(always)]
    unsafe fn vec_get_const_vec2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<4, Self, Simd>,
    ) -> Vector<2, Self, Simd>
    where
        Self: Scalar<2, Simd>,
    {
        Vector([vec[X_SRC], vec[Y_SRC]])
    }

    #[inline(always)]
    unsafe fn vec_get_const_vec3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<4, Self, Simd>,
    ) -> Vector<3, Self, Simd>
    where
        Self: Scalar<3, Simd>,
    {
        let result_as_vec4 = vec.get_const_vec4::<X_SRC, Y_SRC, Z_SRC, Z_SRC>();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    unsafe fn vec_get_const_vec4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd>
    where
        Self: Scalar<4, Simd>,
    {
        let result: __m128;
        unsafe {
            asm!("shufps {0}, {0}, {1}", inout(xmm_reg) vec.0 => result, const {
                let x_src_bits = (X_SRC as u32) << 0;
                let y_src_bits = (Y_SRC as u32) << 2;
                let z_src_bits = (Z_SRC as u32) << 4;
                let w_src_bits = (W_SRC as u32) << 6;

                (x_src_bits | y_src_bits | z_src_bits | w_src_bits).cast_signed()
            });
        }

        Vector(result)
    }

    #[inline(always)]
    fn vec_reverse(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec.wzyx()
    }

    // TODO: optimized eq and ne once masks are implemented

    #[inline(always)]
    fn vec_neg(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd>
    where
        Self: Neg<Output = Self>,
    {
        Vector(unsafe { _mm_xor_ps(_mm_set1_ps(-0.0), vec.0) })
    }

    #[inline(always)]
    fn vec_add(vec: Vector<4, Self, Simd>, rhs: Vector<4, Self, Simd>) -> Vector<4, Self, Simd>
    where
        Self: Add<Output = Self>,
    {
        Vector(unsafe { _mm_add_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_sub(vec: Vector<4, Self, Simd>, rhs: Vector<4, Self, Simd>) -> Vector<4, Self, Simd>
    where
        Self: Sub<Output = Self>,
    {
        Vector(unsafe { _mm_sub_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_mul(vec: Vector<4, Self, Simd>, rhs: Vector<4, Self, Simd>) -> Vector<4, Self, Simd>
    where
        Self: Mul<Output = Self>,
    {
        Vector(unsafe { _mm_mul_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_div(vec: Vector<4, Self, Simd>, rhs: Vector<4, Self, Simd>) -> Vector<4, Self, Simd>
    where
        Self: Div<Output = Self>,
    {
        Vector(unsafe { _mm_div_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_rem(vec: Vector<4, Self, Simd>, rhs: Vector<4, Self, Simd>) -> Vector<4, Self, Simd>
    where
        Self: Rem<Output = Self>,
    {
        vec4!(vec.x % rhs.x, vec.y % rhs.y, vec.z % rhs.z, vec.w % rhs.w)
    }
}

impl ScalarF32<4, Simd> for f32 {
    #[inline(always)]
    fn vec_floor(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_floor_ps(vec.0) })
    }

    #[inline(always)]
    fn vec_ceil(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_ceil_ps(vec.0) })
    }

    #[inline(always)]
    fn vec_round(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_round_ps::<_MM_FROUND_TO_NEAREST_INT>(vec.0) })
    }

    #[inline(always)]
    fn vec_trunc(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_round_ps::<_MM_FROUND_TO_ZERO>(vec.0) })
    }

    #[inline(always)]
    fn vec_fract(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec - vec.trunc()
    }

    #[inline(always)]
    fn vec_mul_add(
        vec: Vector<4, Self, Simd>,
        a: Vector<4, Self, Simd>,
        b: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        #[cfg(target_feature = "fma")]
        unsafe {
            Vector(_mm_fmadd_ps(vec.0, a.0, b.0))
        }

        #[cfg(not(target_feature = "fma"))]
        vec4!(
            vec.x.mul_add(a.x, b.x),
            vec.y.mul_add(a.y, b.y),
            vec.z.mul_add(a.z, b.z),
            vec.w.mul_add(a.w, b.w),
        )
    }

    #[inline(always)]
    fn vec_div_euclid(
        vec: Vector<4, Self, Simd>,
        rhs: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        vec4!(
            vec.x.div_euclid(rhs.x),
            vec.y.div_euclid(rhs.y),
            vec.z.div_euclid(rhs.z),
            vec.w.div_euclid(rhs.w),
        )
    }

    #[inline(always)]
    fn vec_rem_euclid(
        vec: Vector<4, Self, Simd>,
        rhs: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        vec4!(
            vec.x.rem_euclid(rhs.x),
            vec.y.rem_euclid(rhs.y),
            vec.z.rem_euclid(rhs.z),
            vec.w.rem_euclid(rhs.w),
        )
    }

    #[inline(always)]
    fn vec_sqrt(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        Vector(unsafe { _mm_sqrt_ps(vec.0) })
    }

    #[inline(always)]
    fn vec_sin(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(vec.x.sin(), vec.y.sin(), vec.z.sin(), vec.w.sin())

        // TODO: optimize this
        /*

        // Copied directly from `wide`, which is "Based on the Agner Fog "vector class library""
        // https://github.com/Lokathor/wide/blob/main/src/f32x4_.rs
        // https://github.com/vectorclass/version2/blob/master/vectormath_trig.h

        const DP1F: Vector<4, f32, Simd> = Vector::const_from_array([0.78515625 * 2.0; 4]);
        const DP2F: Vector<4, f32, Simd> =
            Vector::const_from_array([2.4187564849853515625E-4 * 2.0; 4]);
        const DP3F: Vector<4, f32, Simd> =
            Vector::const_from_array([3.77489497744594108E-8 * 2.0; 4]);

        const P0SINF: Vector<4, f32, Simd> = Vector::const_from_array([-1.6666654611E-1; 4]);
        const P1SINF: Vector<4, f32, Simd> = Vector::const_from_array([8.3321608736E-3; 4]);
        const P2SINF: Vector<4, f32, Simd> = Vector::const_from_array([-1.9515295891E-4; 4]);

        const P0COSF: Vector<4, f32, Simd> = Vector::const_from_array([4.166664568298827E-2; 4]);
        const P1COSF: Vector<4, f32, Simd> = Vector::const_from_array([-1.388731625493765E-3; 4]);
        const P2COSF: Vector<4, f32, Simd> = Vector::const_from_array([2.443315711809948E-5; 4]);

        const TWO_OVER_PI: Vector<4, f32, Simd> =
            Vector::const_from_array([2.0 / core::f32::consts::PI; 4]);

        let xa = vec.abs();

        // Find quadrant
        let y = (xa * TWO_OVER_PI).round();
        let q: i32x4 = y.round_int();

        let x = y.mul_neg_add(DP3F, y.mul_neg_add(DP2F, y.mul_neg_add(DP1F, xa)));

        let x2 = x * x;
        let mut s = polynomial_2!(x2, P0sinf, P1sinf, P2sinf) * (x * x2) + x;
        let mut c = polynomial_2!(x2, P0cosf, P1cosf, P2cosf) * (x2 * x2)
            + f32x4::from(0.5).mul_neg_add(x2, f32x4::from(1.0));

        let swap = !(q & i32x4::from(1)).simd_eq(i32x4::from(0));

        let mut overflow: f32x4 = cast(q.simd_gt(i32x4::from(0x2000000)));
        overflow &= xa.is_finite();
        s = overflow.blend(f32x4::from(0.0), s);
        c = overflow.blend(f32x4::from(1.0), c);

        // calc sin
        let mut sin1 = cast::<_, f32x4>(swap).blend(c, s);
        let sign_sin: i32x4 = (q << 30) ^ cast::<_, i32x4>(self);
        sin1 = sin1.flip_signs(cast(sign_sin));

        // calc cos
        let mut cos1 = cast::<_, f32x4>(swap).blend(s, c);
        let sign_cos: i32x4 = ((q + i32x4::from(1)) & i32x4::from(2)) << 30;
        cos1 ^= cast::<_, f32x4>(sign_cos);

        (sin1, cos1)

        */
    }

    #[inline(always)]
    fn vec_cos(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(vec.x.cos(), vec.y.cos(), vec.z.cos(), vec.w.cos())

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_tan(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(vec.x.tan(), vec.y.tan(), vec.z.tan(), vec.w.tan())

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_asin(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(vec.x.asin(), vec.y.asin(), vec.z.asin(), vec.w.asin())

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_acos(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(vec.x.acos(), vec.y.acos(), vec.z.acos(), vec.w.acos())

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_atan(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(vec.x.atan(), vec.y.atan(), vec.z.atan(), vec.w.atan())

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_recip(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        Vector::ONE / vec
    }

    #[inline(always)]
    fn vec_max(vec: Vector<4, Self, Simd>, other: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        debug_assert!(vec.iter().all(|x| !x.is_nan()));
        debug_assert!(other.iter().all(|x| !x.is_nan()));

        Vector(unsafe { _mm_max_ps(vec.0, other.0) })
    }

    #[inline(always)]
    fn vec_min(vec: Vector<4, Self, Simd>, other: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        debug_assert!(vec.iter().all(|x| !x.is_nan()));
        debug_assert!(other.iter().all(|x| !x.is_nan()));

        Vector(unsafe { _mm_min_ps(vec.0, other.0) })
    }

    #[inline(always)]
    fn vec_midpoint(
        vec: Vector<4, Self, Simd>,
        other: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        vec * vec4!(0.5) + other * vec4!(0.5)

        // TODO: update this once mul by scalar is implemented
    }

    #[inline(always)]
    fn vec_clamp(
        vec: Vector<4, Self, Simd>,
        min: Vector<4, Self, Simd>,
        max: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        debug_assert!(min.zip(max).iter().all(|(min, max)| min <= max));
        debug_assert!(min.iter().all(|x| !x.is_nan()));
        debug_assert!(max.iter().all(|x| !x.is_nan()));

        Vector(unsafe { _mm_min_ps(_mm_max_ps(vec.0, min.0), max.0) })
    }

    #[inline(always)]
    fn vec_abs(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        const NO_SIGN_MASK: Vector<4, f32, Simd> =
            Vector::const_from_array([f32::from_bits(0x7f_ff_ff_ff); 4]);

        Vector(unsafe { _mm_and_ps(vec.0, NO_SIGN_MASK.0) })
    }

    #[inline(always)]
    fn vec_signum(vec: Vector<4, Self, Simd>) -> Vector<4, Self, Simd> {
        vec4!(
            vec.x.signum(),
            vec.y.signum(),
            vec.z.signum(),
            vec.w.signum()
        )

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_copysign(
        vec: Vector<4, Self, Simd>,
        sign: Vector<4, Self, Simd>,
    ) -> Vector<4, Self, Simd> {
        const SIGN_MASK: Vector<4, f32, Simd> = Vector::const_from_array([-0.0; 4]);

        Vector(unsafe {
            _mm_or_ps(
                _mm_and_ps(sign.0, SIGN_MASK.0),
                _mm_andnot_ps(SIGN_MASK.0, vec.0),
            )
        })
    }

    #[inline(always)]
    fn vec_sum(vec: Vector<4, Self, Simd>) -> Self {
        #[cfg(target_feature = "sse3")]
        unsafe {
            let xplusy_zplusw__ = _mm_hadd_ps(vec.0, vec.0);
            let sum___ = _mm_hadd_ps(xplusy_zplusw__, xplusy_zplusw__);

            sum___.x
        }

        #[cfg(not(target_feature = "sse3"))]
        {
            let xy__ = vec;
            let zw__ = vec.zwxy();
            let xplusz_yplusw__ = xy__ + zw__;
            let yplusw_xplusz__ = xplusz_yplusw__.yxwz();
            let sum___ = xplusz_yplusw__ + yplusw_xplusz__;

            sum___.x
        }
    }

    #[inline(always)]
    fn vec_product(vec: Vector<4, Self, Simd>) -> Self {
        let xy__ = vec;
        let zw__ = vec.zwxy();
        let xtimesz_ytimesw__ = xy__ * zw__;
        let ytimesw_xtimesz__ = xtimesz_ytimesw__.yxwz();
        let product___ = xtimesz_ytimesw__ * ytimesw_xtimesz__;

        product___.x
    }
}

// SAFETY: `__m128` contains exactly 4 f32s which guarantees:
// - that `__m128` starts with 4 f32 elements
// - that `__m128` doesn't have more padding than [f32; 4]
unsafe impl InnerVectorType<4, f32> for __m128 {
    const PADDING: Option<Self> = None;
}
