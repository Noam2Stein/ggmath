#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use core::{
    mem::transmute,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{ElementOfVector, Simd, Vector};

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

    // TODO: optimized rem once trunc is implemented
}
