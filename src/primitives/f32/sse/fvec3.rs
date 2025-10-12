#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::{
    mem::transmute,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use crate::*;

// SAFETY:
// InnerVectorType - __m128 starts with 3 f32s, which makes it valid.
// VECTOR_PADDING - it is `Some` which is always sound.
unsafe impl ElementOfVector<3, Simd> for f32 {
    type InnerVectorType = __m128;

    const VECTOR_PADDING: Option<Vector<3, Self, Simd>> = Some(unsafe { transmute([0.0f32; 4]) });

    #[inline(always)]
    fn vec_from_array(array: [Self; 3]) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_set_ps(array[2], array[2], array[1], array[0]) })
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_set1_ps(value) })
    }

    #[inline(always)]
    fn vec_as_array(vec: Vector<3, Self, Simd>) -> [Self; 3] {
        *vec.as_array_ref()
    }

    #[inline(always)]
    unsafe fn vec_get_const_vec2<const X_SRC: usize, const Y_SRC: usize>(
        vec: Vector<3, Self, Simd>,
    ) -> Vector<2, Self, Simd>
    where
        Self: ElementOfVector<2, Simd>,
    {
        Vector([vec[X_SRC], vec[Y_SRC]])
    }

    #[inline(always)]
    unsafe fn vec_get_const_vec3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd>
    where
        Self: ElementOfVector<3, Simd>,
    {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);

        vec_as_vec4.get_const_vec3::<X_SRC, Y_SRC, Z_SRC>()
    }

    #[inline(always)]
    unsafe fn vec_get_const_vec4<
        const X_SRC: usize,
        const Y_SRC: usize,
        const Z_SRC: usize,
        const W_SRC: usize,
    >(
        vec: Vector<3, Self, Simd>,
    ) -> Vector<4, Self, Simd>
    where
        Self: ElementOfVector<4, Simd>,
    {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);

        vec_as_vec4.get_const_vec4::<X_SRC, Y_SRC, Z_SRC, W_SRC>()
    }

    #[inline(always)]
    fn vec_reverse(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        vec.zyx()
    }

    // TODO: optimized eq and ne once masks are implemented

    #[inline(always)]
    fn vec_neg(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd>
    where
        Self: Neg<Output = Self>,
    {
        Vector(unsafe { _mm_xor_ps(_mm_set1_ps(-0.0), vec.0) })
    }

    #[inline(always)]
    fn vec_add(vec: Vector<3, Self, Simd>, rhs: Vector<3, Self, Simd>) -> Vector<3, Self, Simd>
    where
        Self: Add<Output = Self>,
    {
        Vector(unsafe { _mm_add_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_sub(vec: Vector<3, Self, Simd>, rhs: Vector<3, Self, Simd>) -> Vector<3, Self, Simd>
    where
        Self: Sub<Output = Self>,
    {
        Vector(unsafe { _mm_sub_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_mul(vec: Vector<3, Self, Simd>, rhs: Vector<3, Self, Simd>) -> Vector<3, Self, Simd>
    where
        Self: Mul<Output = Self>,
    {
        Vector(unsafe { _mm_mul_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_div(vec: Vector<3, Self, Simd>, rhs: Vector<3, Self, Simd>) -> Vector<3, Self, Simd>
    where
        Self: Div<Output = Self>,
    {
        Vector(unsafe { _mm_div_ps(vec.0, rhs.0) })
    }

    #[inline(always)]
    fn vec_rem(vec: Vector<3, Self, Simd>, rhs: Vector<3, Self, Simd>) -> Vector<3, Self, Simd>
    where
        Self: Rem<Output = Self>,
    {
        vec3!(vec.x % rhs.x, vec.y % rhs.y, vec.z % rhs.z)
    }
}

impl FloatElementOfVector<3, Simd> for f32 {
    #[inline(always)]
    fn vec_floor(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_floor_ps(vec.0) })
    }

    #[inline(always)]
    fn vec_ceil(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_ceil_ps(vec.0) })
    }

    #[inline(always)]
    fn vec_round(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_round_ps::<_MM_FROUND_TO_NEAREST_INT>(vec.0) })
    }

    #[inline(always)]
    fn vec_trunc(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_round_ps::<_MM_FROUND_TO_ZERO>(vec.0) })
    }

    #[inline(always)]
    fn vec_fract(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        vec - vec.trunc()
    }

    #[inline(always)]
    fn vec_mul_add(
        vec: Vector<3, Self, Simd>,
        a: Vector<3, Self, Simd>,
        b: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        #[cfg(target_feature = "fma")]
        unsafe {
            Vector(_mm_fmadd_ps(vec.0, a.0, b.0))
        }

        #[cfg(not(target_feature = "fma"))]
        vec3!(
            vec.x.mul_add(a.x, b.x),
            vec.y.mul_add(a.y, b.y),
            vec.z.mul_add(a.z, b.z),
        )
    }

    #[inline(always)]
    fn vec_div_euclid(
        vec: Vector<3, Self, Simd>,
        rhs: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        vec3!(
            vec.x.div_euclid(rhs.x),
            vec.y.div_euclid(rhs.y),
            vec.z.div_euclid(rhs.z),
        )
    }

    #[inline(always)]
    fn vec_rem_euclid(
        vec: Vector<3, Self, Simd>,
        rhs: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        vec3!(
            vec.x.rem_euclid(rhs.x),
            vec.y.rem_euclid(rhs.y),
            vec.z.rem_euclid(rhs.z),
        )
    }

    #[inline(always)]
    fn vec_sqrt(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        Vector(unsafe { _mm_sqrt_ps(vec.0) })
    }

    #[inline(always)]
    fn vec_sin(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.sin();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_cos(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.cos();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_tan(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.tan();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_asin(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.asin();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_acos(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.acos();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_atan(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.atan();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_recip(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        Vector::ONE / vec
    }

    #[inline(always)]
    fn vec_max(vec: Vector<3, Self, Simd>, other: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        debug_assert!(vec.iter().all(|x| !x.is_nan()));
        debug_assert!(other.iter().all(|x| !x.is_nan()));

        Vector(unsafe { _mm_max_ps(vec.0, other.0) })
    }

    #[inline(always)]
    fn vec_min(vec: Vector<3, Self, Simd>, other: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        debug_assert!(vec.iter().all(|x| !x.is_nan()));
        debug_assert!(other.iter().all(|x| !x.is_nan()));

        Vector(unsafe { _mm_min_ps(vec.0, other.0) })
    }

    #[inline(always)]
    fn vec_midpoint(
        vec: Vector<3, Self, Simd>,
        other: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        vec * vec3!(0.5) + other * vec3!(0.5)

        // TODO: update this once mul by scalar is implemented
    }

    #[inline(always)]
    fn vec_clamp(
        vec: Vector<3, Self, Simd>,
        min: Vector<3, Self, Simd>,
        max: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        debug_assert!(min.zip(max).iter().all(|(min, max)| min <= max));
        debug_assert!(min.iter().all(|x| !x.is_nan()));
        debug_assert!(max.iter().all(|x| !x.is_nan()));

        Vector(unsafe { _mm_min_ps(_mm_max_ps(vec.0, min.0), max.0) })
    }

    #[inline(always)]
    fn vec_abs(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let result_as_vec4 = vec_as_vec4.abs();

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_signum(vec: Vector<3, Self, Simd>) -> Vector<3, Self, Simd> {
        vec3!(vec.x.signum(), vec.y.signum(), vec.z.signum())

        // TODO: optimize this
    }

    #[inline(always)]
    fn vec_copysign(
        vec: Vector<3, Self, Simd>,
        sign: Vector<3, Self, Simd>,
    ) -> Vector<3, Self, Simd> {
        let vec_as_vec4 = Vector::<4, Self, Simd>(vec.0);
        let sign_as_vec4 = Vector::<4, Self, Simd>(sign.0);
        let result_as_vec4 = vec_as_vec4.copysign(sign_as_vec4);

        Vector(result_as_vec4.0)
    }

    #[inline(always)]
    fn vec_sum(vec: Vector<3, Self, Simd>) -> Self {
        #[cfg(target_feature = "sse3")]
        unsafe {
            const NO_W_MASK: Vector<4, Self, Simd> = Vector::const_from_array([
                f32::from_bits(0xff_ff_ff_ff),
                f32::from_bits(0xff_ff_ff_ff),
                f32::from_bits(0xff_ff_ff_ff),
                f32::from_bits(0x00_00_00_00),
            ]);

            let xyz0 = _mm_and_ps(vec.0, NO_W_MASK.0);
            let xplusy_zplus0__ = _mm_hadd_ps(xyz0, xyz0);
            let sum___ = _mm_hadd_ps(xplusy_zplus0__, xplusy_zplus0__);

            sum___.x
        }

        #[cfg(not(target_feature = "sse3"))]
        {
            vec.x + vec.y + vec.z
        }
    }

    #[inline(always)]
    fn vec_product(vec: Vector<3, Self, Simd>) -> Self {
        vec.x * vec.y * vec.z
    }
}
