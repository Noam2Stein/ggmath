use_core_arch_x86! {
    __m128i,
    _mm_add_epi32,
    _mm_and_si128,
    _mm_or_si128,
    _mm_set1_epi32,
    _mm_set_epi32,
    _mm_sll_epi32,
    _mm_sub_epi32,
    _mm_sra_epi32,
    _mm_xor_si128,
}

use core::arch::asm;

use crate::{
    SimdBehaviour, Vec2, Vec3, Vec4, Vector,
    vector::{SoundVectorRepr, primitive_impls::use_core_arch_x86, vec2, vec3, vec4},
};

// SAFETY: __m128i can contain exactly 4 i32s
unsafe impl SoundVectorRepr<4, u32> for __m128i {}

// SAFETY: __m128i can contain exactly 4 i32s, so it does begin with 3 i32s
unsafe impl SoundVectorRepr<3, u32> for __m128i {}

////////////////////////////////////////////////////////////////////////////////
// Vector4
////////////////////////////////////////////////////////////////////////////////

impl SimdBehaviour<4> for u32 {
    type VectorRepr = __m128i;

    #[inline(always)]
    fn vec_from_array(array: [Self; 4]) -> Vec4<Self> {
        Vector::from_repr(unsafe {
            _mm_set_epi32(
                array[3].cast_signed(),
                array[2].cast_signed(),
                array[1].cast_signed(),
                array[0].cast_signed(),
            )
        })
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_set1_epi32(value.cast_signed()) })
    }

    #[inline(always)]
    unsafe fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(vec: Vec4<Self>) -> Vec2<Self> {
        vec2!(vec[X_SRC], vec[Y_SRC])
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
        let result: __m128i;
        // SAFETY: pshufd is part of sse2, so it is safe to use here.
        unsafe {
            asm!("pshufd {0}, {0}, {1}", inout(xmm_reg) vec.repr() => result, const {
                let x_src_bits = (X_SRC as u32) << 0;
                let y_src_bits = (Y_SRC as u32) << 2;
                let z_src_bits = (Z_SRC as u32) << 4;
                let w_src_bits = (W_SRC as u32) << 6;

                (x_src_bits | y_src_bits | z_src_bits | w_src_bits).cast_signed()
            });
        }

        Vector::from_repr(result)
    }

    // TODO: optimize eq and ne once masks are implemented

    #[inline(always)]
    fn vec_not(vec: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_xor_si128(vec.repr(), vec4!(-1).repr()) })
    }

    #[inline(always)]
    fn vec_add(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        if cfg!(debug_assertions) {
            vec4!(vec.x + rhs.x, vec.y + rhs.y, vec.z + rhs.z, vec.w + rhs.w)
        } else {
            Vector::from_repr(unsafe { _mm_add_epi32(vec.repr(), rhs.repr()) })
        }
    }

    #[inline(always)]
    fn vec_sub(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        if cfg!(debug_assertions) {
            vec4!(vec.x - rhs.x, vec.y - rhs.y, vec.z - rhs.z, vec.w - rhs.w)
        } else {
            Vector::from_repr(unsafe { _mm_sub_epi32(vec.repr(), rhs.repr()) })
        }
    }

    #[inline(always)]
    fn vec_mul(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        // TODO: determine if this can be optimized

        vec4!(vec.x * rhs.x, vec.y * rhs.y, vec.z * rhs.z, vec.w * rhs.w)
    }

    #[inline(always)]
    fn vec_div(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        vec4!(vec.x / rhs.x, vec.y / rhs.y, vec.z / rhs.z, vec.w / rhs.w)
    }

    #[inline(always)]
    fn vec_rem(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        vec4!(vec.x % rhs.x, vec.y % rhs.y, vec.z % rhs.z, vec.w % rhs.w)
    }

    #[inline(always)]
    fn vec_shl(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_sll_epi32(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_shr(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_sra_epi32(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_bitand(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_and_si128(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_bitor(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_or_si128(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_bitxor(vec: Vec4<Self>, rhs: Vec4<Self>) -> Vec4<Self> {
        Vector::from_repr(unsafe { _mm_xor_si128(vec.repr(), rhs.repr()) })
    }
}

////////////////////////////////////////////////////////////////////////////////
// Vector3
////////////////////////////////////////////////////////////////////////////////

impl SimdBehaviour<3> for u32 {
    type VectorRepr = __m128i;

    #[inline(always)]
    fn vec_from_array(array: [Self; 3]) -> Vec3<Self> {
        Vector::from_repr(unsafe {
            _mm_set_epi32(
                array[2].cast_signed(),
                array[2].cast_signed(),
                array[1].cast_signed(),
                array[0].cast_signed(),
            )
        })
    }

    #[inline(always)]
    fn vec_splat(value: Self) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_set1_epi32(value.cast_signed()) })
    }

    #[inline(always)]
    unsafe fn vec_swizzle2<const X_SRC: usize, const Y_SRC: usize>(vec: Vec3<Self>) -> Vec2<Self> {
        vec2!(vec[X_SRC], vec[Y_SRC])
    }

    #[inline(always)]
    unsafe fn vec_swizzle3<const X_SRC: usize, const Y_SRC: usize, const Z_SRC: usize>(
        vec: Vec3<Self>,
    ) -> Vec3<Self> {
        let vec_as_vec4 = Vec4::<Self>::from_repr(vec.repr());
        let result_as_vec4 = vec_as_vec4.swizzle4::<X_SRC, Y_SRC, Z_SRC, Z_SRC>();

        Vector::from_repr(result_as_vec4.repr())
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
        let vec_as_vec4 = Vec4::<Self>::from_repr(vec.repr());

        vec_as_vec4.swizzle4::<X_SRC, Y_SRC, Z_SRC, W_SRC>()
    }

    // TODO: optimize eq and ne once masks are implemented

    #[inline(always)]
    fn vec_not(vec: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_xor_si128(vec.repr(), vec3!(-1).repr()) })
    }

    #[inline(always)]
    fn vec_add(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        if cfg!(debug_assertions) {
            vec3!(vec.x + rhs.x, vec.y + rhs.y, vec.z + rhs.z)
        } else {
            Vector::from_repr(unsafe { _mm_add_epi32(vec.repr(), rhs.repr()) })
        }
    }

    #[inline(always)]
    fn vec_sub(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        if cfg!(debug_assertions) {
            vec3!(vec.x - rhs.x, vec.y - rhs.y, vec.z - rhs.z)
        } else {
            Vector::from_repr(unsafe { _mm_sub_epi32(vec.repr(), rhs.repr()) })
        }
    }

    #[inline(always)]
    fn vec_mul(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        // TODO: determine if this can be optimized

        vec3!(vec.x * rhs.x, vec.y * rhs.y, vec.z * rhs.z)
    }

    #[inline(always)]
    fn vec_div(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        vec3!(vec.x / rhs.x, vec.y / rhs.y, vec.z / rhs.z)
    }

    #[inline(always)]
    fn vec_rem(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        vec3!(vec.x % rhs.x, vec.y % rhs.y, vec.z % rhs.z)
    }

    #[inline(always)]
    fn vec_shl(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_sll_epi32(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_shr(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_sra_epi32(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_bitand(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_and_si128(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_bitor(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_or_si128(vec.repr(), rhs.repr()) })
    }

    #[inline(always)]
    fn vec_bitxor(vec: Vec3<Self>, rhs: Vec3<Self>) -> Vec3<Self> {
        Vector::from_repr(unsafe { _mm_xor_si128(vec.repr(), rhs.repr()) })
    }
}
