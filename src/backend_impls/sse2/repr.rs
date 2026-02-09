#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::{
    Aligned, Alignment, Length, Mask, MaskBackend, Scalar, ScalarRepr, SupportedLength, Vector,
    utils::{Repr2, Repr3, Repr4},
};

// SAFETY: Look at the safety note for each associated type.
unsafe impl ScalarRepr for i32 {
    // SAFETY: Select chooses `ReprN` from `Repr2`, `Repr3`, and
    // `Repr4`. Each type is guaranteed to be a simple struct equivalent
    // to `[T; N]`. The vector is made out of consecutive values of `T`,
    // the vector is guaranteed to have the size and alignment of
    // `[T; N]`, and two scalars that share `$T` must have the same size
    // thus the vectors have the same size and element positions too. `__m128`
    // is made out of consecutive elements of `T` because `T` must be 4-bytes
    // long and have no uninitialized bytes. `Vector<3, T, Aligned>` has the
    // size of `[T; 4]` and its padding element has initialized bytes.
    // `Vector<4, T, Aligned>` has the size of `[T; 4]`. `__m128` is consistent
    // across scalars with the same `Repr`.
    type VectorRepr<const N: usize, T, A: Alignment>
        = <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<Repr2<T>, __m128, __m128>,
        <Length<N> as SupportedLength>::Select<Repr2<T>, Repr3<T>, Repr4<T>>,
    >
    where
        Length<N>: SupportedLength,
        T: Scalar;

    // SAFETY: `__m128` is identical to the representation of `Vec4<T>`.
    // `Repr3<__m128>` is three times `Vec3<T>`. `Repr4<__m128>` is four times
    // `Vec4<T>`. `Repr4<T>` is identical to the representation of `Vec4U<T>`.
    // `Repr3<Repr3<T>>` is three times `Vec3U<T>`. `Repr4<Repr4<T>>` is four
    // times `Vec4U<T>`.
    type MatrixRepr<const N: usize, T, A: Alignment>
        = <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<__m128, Repr3<__m128>, Repr4<__m128>>,
        <Length<N> as SupportedLength>::Select<Repr4<T>, Repr3<Repr3<T>>, Repr4<Repr4<T>>>,
    >
    where
        Length<N>: SupportedLength,
        T: Scalar;

    // SAFETY: For the `bool` types, select chooses `ReprN` from `Repr2`, `Repr3`, and
    // `Repr4`. Each type is guaranteed to be a simple struct equivalent
    // to `[bool; N]`. `[bool; N]` has no uninitialized bytes, and is
    // zeroable. `__m128` has no uninitialized bytes, and is zeroable. Masks of
    // `$T` have the same representation no matter their `T` type.
    type MaskRepr<const N: usize, A: Alignment>
        = <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<Repr2<bool>, __m128, __m128>,
        <Length<N> as SupportedLength>::Select<Repr2<bool>, Repr3<bool>, Repr4<bool>>,
    >
    where
        Length<N>: SupportedLength;
}

impl MaskBackend<3, Aligned> for i32 {
    #[inline]
    fn mask_from_array<T>(array: [bool; 3]) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_splat<T>(value: bool) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The two intrinsics are part of SSE2.
        unsafe { Mask(_mm_castsi128_ps(_mm_set1_epi32(-(value as i32)))) }
    }

    #[inline]
    fn mask_from_fn<T, F>((mut f,): (F,)) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool,
    {
        Mask::from_array([f(0), f(1), f(2)])
    }

    #[inline]
    fn mask_to_array<T>(mask: Mask<3, T, Aligned>) -> [bool; 3]
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        let bits = unsafe { _mm_movemask_ps(mask.0) };
        [bits & 0x1 != 0, bits & 0x2 != 0, bits & 0x4 != 0]
    }

    #[inline]
    fn mask_all<T>(mask: Mask<3, T, Aligned>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) & 0x7 == 0x7 }
    }

    #[inline]
    fn mask_any<T>(mask: Mask<3, T, Aligned>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) & 0x7 != 0 }
    }

    #[inline]
    fn mask_select<T>(
        mask: Mask<3, T, Aligned>,
        if_true: Vector<3, T, Aligned>,
        if_false: Vector<3, T, Aligned>,
    ) -> Vector<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The three intrinsics are part of SSE.
        Vector(unsafe {
            _mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            )
        })
    }

    #[inline]
    fn mask_get<T>(mask: Mask<3, T, Aligned>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_set<T>(mask: &mut Mask<3, T, Aligned>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_eq<T>(mask: &Mask<3, T, Aligned>, other: &Mask<3, T, Aligned>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) & 0x7 == _mm_movemask_ps(other.0) & 0x7 }
    }

    #[inline]
    fn mask_not<T>(mask: Mask<3, T, Aligned>) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The two intrinsics are part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, _mm_set1_ps(f32::from_bits(!0))) })
    }

    #[inline]
    fn mask_bitand<T>(mask: Mask<3, T, Aligned>, rhs: Mask<3, T, Aligned>) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_and_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitor<T>(mask: Mask<3, T, Aligned>, rhs: Mask<3, T, Aligned>) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_or_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitxor<T>(mask: Mask<3, T, Aligned>, rhs: Mask<3, T, Aligned>) -> Mask<3, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, rhs.0) })
    }
}

impl MaskBackend<4, Aligned> for i32 {
    #[inline]
    fn mask_from_array<T>(array: [bool; 4]) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_splat<T>(value: bool) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The two intrinsics are part of SSE2.
        unsafe { Mask(_mm_castsi128_ps(_mm_set1_epi32(-(value as i32)))) }
    }

    #[inline]
    fn mask_from_fn<T, F>((mut f,): (F,)) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
        F: FnMut(usize) -> bool,
    {
        Mask::from_array([f(0), f(1), f(2), f(3)])
    }

    #[inline]
    fn mask_to_array<T>(mask: Mask<4, T, Aligned>) -> [bool; 4]
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_all<T>(mask: Mask<4, T, Aligned>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) == 0xf }
    }

    #[inline]
    fn mask_any<T>(mask: Mask<4, T, Aligned>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) != 0 }
    }

    #[inline]
    fn mask_select<T>(
        mask: Mask<4, T, Aligned>,
        if_true: Vector<4, T, Aligned>,
        if_false: Vector<4, T, Aligned>,
    ) -> Vector<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The three intrinsics are part of SSE.
        Vector(unsafe {
            _mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            )
        })
    }

    #[inline]
    fn mask_get<T>(mask: Mask<4, T, Aligned>, index: usize) -> bool
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_set<T>(mask: &mut Mask<4, T, Aligned>, index: usize, value: bool)
    where
        T: Scalar<Repr = Self>,
    {
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
    fn mask_eq<T>(mask: &Mask<4, T, Aligned>, other: &Mask<4, T, Aligned>) -> bool
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        unsafe { _mm_movemask_ps(mask.0) == _mm_movemask_ps(other.0) }
    }

    #[inline]
    fn mask_not<T>(mask: Mask<4, T, Aligned>) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The two intrinsics are part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, _mm_set1_ps(f32::from_bits(!0))) })
    }

    #[inline]
    fn mask_bitand<T>(mask: Mask<4, T, Aligned>, rhs: Mask<4, T, Aligned>) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_and_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitor<T>(mask: Mask<4, T, Aligned>, rhs: Mask<4, T, Aligned>) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_or_ps(mask.0, rhs.0) })
    }

    #[inline]
    fn mask_bitxor<T>(mask: Mask<4, T, Aligned>, rhs: Mask<4, T, Aligned>) -> Mask<4, T, Aligned>
    where
        T: Scalar<Repr = Self>,
    {
        // SAFETY: The intrinsic is part of SSE.
        Mask(unsafe { _mm_xor_ps(mask.0, rhs.0) })
    }
}
