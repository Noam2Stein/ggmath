#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;
#[cfg(target_arch = "aarch64")]
use core::{arch::aarch64, mem::transmute};

use wide::{f32x4, u32x4};

use crate::{
    Aligned, Mask, Mask3A, Mask4A, Rotor, Rotor3A, Vec3A, Vec4A, Vector,
    backend::{AffineBackend, FloatVectorBackend, MaskBackend, RotorBackend, VectorBackend},
};

// `Self::Inner` follows its requirements.
unsafe impl VectorBackend<3, Aligned> for f32 {
    type Inner = f32x4;

    #[inline]
    fn eq(vector: &Vec3A<f32>, other: &Vec3A<f32>) -> bool {
        vector.eq_mask(*other).all()
    }

    #[inline]
    fn ne(vector: &Vec3A<f32>, other: &Vec3A<f32>) -> bool {
        !(vector == other)
    }

    #[inline]
    fn neg(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(-vector.0)
    }

    #[inline]
    fn not(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(!vector.0)
    }

    #[inline]
    fn add(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 + rhs.0)
    }

    #[inline]
    fn sub(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 - rhs.0)
    }

    #[inline]
    fn mul(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 * rhs.0)
    }

    #[inline]
    fn div(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 / rhs.0)
    }

    #[inline]
    fn rem(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(rem(vector.0, rhs.0))
    }

    #[inline]
    fn shl(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::<f32>::from_bits(vector.to_bits() << rhs.to_bits())
    }

    #[inline]
    fn shr(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::<f32>::from_bits(vector.to_bits() >> rhs.to_bits())
    }

    #[inline]
    fn bitand(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 & rhs.0)
    }

    #[inline]
    fn bitor(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 | rhs.0)
    }

    #[inline]
    fn bitxor(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0 ^ rhs.0)
    }

    #[inline]
    fn element_sum(vector: Vec3A<f32>) -> f32 {
        cfg_select! {
            target_feature = "sse2" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    // Add `-0.0` to retain the sign of the left operand. Adding
                    // `+0.0` would incorrectly reset the sign when `z` is
                    // `-0.0`.
                    let vector = _mm_add_ps(
                        vector,
                        _mm_shuffle_ps(vector, _mm_set1_ps(-0.0), 0b00_11_00_01),
                    );
                    let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
                    _mm_cvtss_f32(vector)
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                let vector = float32x4_t::from(vector.0);
                // SAFETY: These functions only require `neon`.
                unsafe {
                    // Add `-0.0` to retain the sign of the left operand. Adding
                    // `+0.0` would incorrectly reset the sign when `z` is
                    // `-0.0`.
                    vaddvq_f32(vsetq_lane_f32(-0.0, vector.0, 3))
                }
            }
        }
    }

    #[inline]
    fn element_product(vector: Vec3A<f32>) -> f32 {
        cfg_select! {
            target_feature = "sse2" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    let vector = _mm_mul_ps(
                        vector,
                        _mm_shuffle_ps(vector, _mm_set1_ps(1.0), 0b00_11_00_01),
                    );
                    let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
                    _mm_cvtss_f32(vector)
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                vector.x * vector.y * vector.z
            }
        }
    }

    #[inline]
    fn eq_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.simd_eq(other.0))
    }

    #[inline]
    fn ne_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.simd_ne(other.0))
    }

    #[inline]
    fn lt_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.simd_lt(other.0))
    }

    #[inline]
    fn gt_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.simd_gt(other.0))
    }

    #[inline]
    fn le_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.simd_le(other.0))
    }

    #[inline]
    fn ge_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.simd_ge(other.0))
    }
}

// `Self::Inner` follows its requirements.
unsafe impl VectorBackend<4, Aligned> for f32 {
    type Inner = f32x4;

    #[inline]
    fn eq(vector: &Vec4A<f32>, other: &Vec4A<f32>) -> bool {
        vector.0 == other.0
    }

    #[inline]
    fn ne(vector: &Vec4A<f32>, other: &Vec4A<f32>) -> bool {
        !(vector == other)
    }

    #[inline]
    fn neg(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(-vector.0)
    }

    #[inline]
    fn not(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(!vector.0)
    }

    #[inline]
    fn add(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 + rhs.0)
    }

    #[inline]
    fn sub(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 - rhs.0)
    }

    #[inline]
    fn mul(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 * rhs.0)
    }

    #[inline]
    fn div(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 / rhs.0)
    }

    #[inline]
    fn rem(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(rem(vector.0, rhs.0))
    }

    #[inline]
    fn shl(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(f32x4::from_bits(vector.0.to_bits() << rhs.0.to_bits()))
    }

    #[inline]
    fn shr(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(f32x4::from_bits(vector.0.to_bits() >> rhs.0.to_bits()))
    }

    #[inline]
    fn bitand(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 & rhs.0)
    }

    #[inline]
    fn bitor(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 | rhs.0)
    }

    #[inline]
    fn bitxor(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0 ^ rhs.0)
    }

    #[inline]
    fn element_sum(vector: Vec4A<f32>) -> f32 {
        cfg_select! {
            target_feature = "ssse3" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `ssse3`.
                unsafe {
                    let reduce_2 = _mm_hadd_ps(vector, vector);
                    let reduce_1 = _mm_hadd_ps(reduce_2, reduce_2);
                    _mm_cvtss_f32(reduce_1)
                }
            }
            target_feature = "sse2" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_11_00_01));
                    let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
                    _mm_cvtss_f32(vector)
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                let vector = float32x4_t::from(vector.0);
                // SAFETY: These functions only require `neon`.
                unsafe {
                    // (a + b) + (c + d)
                    vaddv_f32(vpadd_f32(vget_low_f32(vector), vget_high_f32(vector)))
                }
            }
        }
    }

    #[inline]
    fn element_product(vector: Vec4A<f32>) -> f32 {
        cfg_select! {
            target_feature = "sse2" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_11_00_01));
                    let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
                    _mm_cvtss_f32(vector)
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                let vector = float32x4_t::from(vector.0);
                // SAFETY: These functions only require `neon`.
                unsafe {
                    let bcda = vextq_f32::<1>(vector, vector);
                    let temp = vmulq_f32(vector, bcda);
                    vgetq_lane_f32::<0>(temp) * vgetq_lane_f32::<2>(temp)
                }
            }
        }
    }

    #[inline]
    fn eq_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.simd_eq(other.0))
    }

    #[inline]
    fn ne_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.simd_ne(other.0))
    }

    #[inline]
    fn lt_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.simd_lt(other.0))
    }

    #[inline]
    fn gt_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.simd_gt(other.0))
    }

    #[inline]
    fn le_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.simd_le(other.0))
    }

    #[inline]
    fn ge_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.simd_ge(other.0))
    }
}

// SAFETY: The first `f32x4` represents the matrix, the first two elements of
// the second `f32x4` represent the vector, and the two remaining elements are
// padding. The padding satisfies the requirements of `Pod`. `Mat2A<f32>` is
// represented by `Vec4A<f32>` which is represented by `f32x4`, so we have the
// same alignment.
unsafe impl AffineBackend<2, Aligned> for f32 {
    type Inner = [f32x4; 2];
}

impl RotorBackend<3, Aligned> for f32 {
    #[inline]
    fn conjugate(rotor: Rotor3A<f32>) -> Rotor3A<f32> {
        const SIGNS: Vec4A<u32> = Vec4A::<f32>::new(-0.0, -0.0, -0.0, 0.0).to_bits();

        Rotor3A::from_raw_vector(Vec4A::<f32>::from_bits(rotor.0.to_bits() ^ SIGNS))
    }

    #[inline]
    fn mul(rotor: Rotor3A<f32>, rhs: Rotor3A<f32>) -> Rotor3A<f32> {
        const PNPN: Vec4A<f32> = Vec4A::new(0.0, -0.0, 0.0, -0.0);
        const PPNN: Vec4A<f32> = Vec4A::new(0.0, 0.0, -0.0, -0.0);
        const NPPN: Vec4A<f32> = Vec4A::new(-0.0, 0.0, 0.0, -0.0);

        Rotor(
            rotor.0 * rhs.0.wwww()
                + Vec4A::<f32>::from_bits(
                    PNPN.to_bits() ^ (rotor.0.wzyx() * rhs.0.xxxx()).to_bits(),
                )
                + Vec4A::<f32>::from_bits(
                    PPNN.to_bits() ^ (rotor.0.zwxy() * rhs.0.yyyy()).to_bits(),
                )
                + Vec4A::<f32>::from_bits(
                    NPPN.to_bits() ^ (rotor.0.yxwz() * rhs.0.zzzz()).to_bits(),
                ),
        )
    }
}

// `Self::Inner` follows its requirements.
unsafe impl MaskBackend<3, Aligned> for f32 {
    type Inner = f32x4;

    #[inline]
    fn from_array(array: [bool; 3]) -> Mask3A<f32> {
        Mask(f32x4::from_bits(-u32x4::new([
            array[0] as u32,
            array[1] as u32,
            array[2] as u32,
            array[2] as u32,
        ])))
    }

    #[inline]
    fn splat(value: bool) -> Mask3A<f32> {
        Mask(f32x4::from_bits(u32x4::splat(
            (-(value as i32)).cast_unsigned(),
        )))
    }

    #[inline]
    fn to_array(mask: Mask3A<f32>) -> [bool; 3] {
        let bitmask = mask.0.to_bitmask();
        [
            bitmask & 0b001 != 0,
            bitmask & 0b010 != 0,
            bitmask & 0b100 != 0,
        ]
    }

    #[inline]
    fn all(mask: Mask3A<f32>) -> bool {
        cfg_select! {
            target_feature = "sse2" => {
                let mask = __m128::from(mask.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    _mm_movemask_ps(mask) & 0b111 == 0b111
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                // SAFETY: Both types accept all bit-patterns.
                const MASK: uint32x4_t = unsafe {
                    transmute::<[u32; 4], uint32x4_t>([0b001, 0b010, 0b100, 0])
                };

                let mask = uint32x4_t::from(mask.0.to_bits());
                // SAFETY: These functions only require `neon`.
                unsafe {
                    let masked = vandq_u32(mask, MASK);
                    let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
                    let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

                    bitmask == 0b111
                }
            }
        }
    }

    #[inline]
    fn any(mask: Mask3A<f32>) -> bool {
        cfg_select! {
            target_feature = "sse2" => {
                let mask = __m128::from(mask.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    _mm_movemask_ps(mask) & 0b111 != 0
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                // SAFETY: Both types accept all bit-patterns.
                const MASK: uint32x4_t = unsafe {
                    transmute::<[u32; 4], uint32x4_t>([0b001, 0b010, 0b100, 0])
                };

                let mask = uint32x4_t::from(mask.0.to_bits());
                // SAFETY: These functions only require `neon`.
                unsafe {
                    let masked = vandq_u32(mask, MASK);
                    let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
                    let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

                    bitmask != 0
                }
            }
        }
    }

    #[inline]
    fn select(mask: Mask3A<f32>, if_true: Vec3A<f32>, if_false: Vec3A<f32>) -> Vec3A<f32> {
        Vector(mask.0.select(if_true.0, if_false.0))
    }

    #[inline]
    fn get(mask: Mask3A<f32>, index: usize) -> bool {
        match index {
            0 => mask.0.to_bitmask() & 0b001 != 0,
            1 => mask.0.to_bitmask() & 0b010 != 0,
            2 => mask.0.to_bitmask() & 0b100 != 0,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn set(mask: &mut Mask3A<f32>, index: usize, value: bool) {
        if index < 3 {
            // SAFETY: `*mut f32x4` is valid as `*mut i32` for 4 values. Adding
            // `index` is valid because it was just checked to be less then 3,
            // and the result is a pointer to a valid `i32`.
            let slot = unsafe {
                core::ptr::from_mut::<f32x4>(&mut mask.0)
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
    fn eq(mask: &Mask3A<f32>, other: &Mask3A<f32>) -> bool {
        cfg_select! {
            target_feature = "sse2" => {
                let mask = __m128::from(mask.0);
                let other = __m128::from(other.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    _mm_movemask_ps(mask) & 0b111 == _mm_movemask_ps(other) & 0b111
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                // SAFETY: Both types accept all bit-patterns.
                const MASK: uint32x4_t = unsafe {
                    transmute::<[u32; 4], uint32x4_t>([0b001, 0b010, 0b100, 0])
                };

                let mask = uint32x4_t::from(mask.0.to_bits());
                let other = uint32x4_t::from(other.0.to_bits());
                // SAFETY: These functions only require `neon`.
                unsafe {
                    let masked = vandq_u32(vceqq_u32(mask, other), MASK);
                    let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
                    let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

                    bitmask == 0b111
                }
            }
        }
    }

    #[inline]
    fn ne(mask: &Mask3A<f32>, other: &Mask3A<f32>) -> bool {
        !(mask == other)
    }

    #[inline]
    fn not(mask: Mask3A<f32>) -> Mask3A<f32> {
        Mask(!mask.0)
    }

    #[inline]
    fn bitand(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
        Mask(mask.0 & rhs.0)
    }

    #[inline]
    fn bitor(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
        Mask(mask.0 | rhs.0)
    }

    #[inline]
    fn bitxor(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
        Mask(mask.0 ^ rhs.0)
    }
}

// `Self::Inner` follows its requirements.
unsafe impl MaskBackend<4, Aligned> for f32 {
    type Inner = f32x4;

    #[inline]
    fn from_array(array: [bool; 4]) -> Mask4A<f32> {
        Mask(f32x4::from_bits(-u32x4::new([
            array[0] as u32,
            array[1] as u32,
            array[2] as u32,
            array[3] as u32,
        ])))
    }

    #[inline]
    fn splat(value: bool) -> Mask4A<f32> {
        Mask(f32x4::from_bits(u32x4::splat(
            (-(value as i32)).cast_unsigned(),
        )))
    }

    #[inline]
    fn to_array(mask: Mask4A<f32>) -> [bool; 4] {
        let bitmask = mask.0.to_bitmask();
        [
            bitmask & 0b0001 != 0,
            bitmask & 0b0010 != 0,
            bitmask & 0b0100 != 0,
            bitmask & 0b1000 != 0,
        ]
    }

    #[inline]
    fn all(mask: Mask4A<f32>) -> bool {
        mask.0.all()
    }

    #[inline]
    fn any(mask: Mask4A<f32>) -> bool {
        mask.0.any()
    }

    #[inline]
    fn select(mask: Mask4A<f32>, if_true: Vec4A<f32>, if_false: Vec4A<f32>) -> Vec4A<f32> {
        Vector(mask.0.select(if_true.0, if_false.0))
    }

    #[inline]
    fn get(mask: Mask4A<f32>, index: usize) -> bool {
        match index {
            0 => mask.0.to_bitmask() & 0b0001 != 0,
            1 => mask.0.to_bitmask() & 0b0010 != 0,
            2 => mask.0.to_bitmask() & 0b0100 != 0,
            3 => mask.0.to_bitmask() & 0b1000 != 0,
            _ => panic!("index out of bounds"),
        }
    }

    #[inline]
    fn set(mask: &mut Mask4A<f32>, index: usize, value: bool) {
        if index < 4 {
            // SAFETY: `*mut f32x4` is valid as `*mut i32` for 4 values. Adding
            // `index` is valid because it was just checked to be less then 4,
            // and the result is a pointer to a valid `i32`.
            let slot = unsafe {
                core::ptr::from_mut::<f32x4>(&mut mask.0)
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
    fn eq(mask: &Mask4A<f32>, other: &Mask4A<f32>) -> bool {
        mask.0 == other.0
    }

    #[inline]
    fn ne(mask: &Mask4A<f32>, other: &Mask4A<f32>) -> bool {
        !(mask == other)
    }

    #[inline]
    fn not(mask: Mask4A<f32>) -> Mask4A<f32> {
        Mask(!mask.0)
    }

    #[inline]
    fn bitand(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
        Mask(mask.0 & rhs.0)
    }

    #[inline]
    fn bitor(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
        Mask(mask.0 | rhs.0)
    }

    #[inline]
    fn bitxor(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
        Mask(mask.0 ^ rhs.0)
    }
}

impl FloatVectorBackend<3, Aligned> for f32 {
    #[inline]
    fn nan_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.is_nan())
    }

    #[inline]
    fn finite_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.is_finite())
    }

    #[inline]
    fn sign_positive_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.is_sign_positive())
    }

    #[inline]
    fn sign_negative_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
        Mask(vector.0.is_sign_negative())
    }

    #[inline]
    fn max(vector: Vec3A<f32>, other: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.fast_max(other.0))
    }

    #[inline]
    fn min(vector: Vec3A<f32>, other: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.fast_min(other.0))
    }

    #[inline]
    fn abs(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.abs())
    }

    #[inline]
    fn signum(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.signum())
    }

    #[inline]
    fn copysign(vector: Vec3A<f32>, sign: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.copysign(sign.0))
    }

    #[inline]
    fn max_element(vector: Vec3A<f32>) -> f32 {
        cfg_select! {
            target_feature = "sse2" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    let reduce_2 = _mm_max_ps(
                        vector,
                        _mm_shuffle_ps(vector, vector, 0b00_00_10_10),
                    );
                    let reduce_1 = _mm_max_ps(
                        reduce_2,
                        _mm_shuffle_ps(reduce_2, reduce_2, 0b00_00_00_01),
                    );
                    _mm_cvtss_f32(reduce_1)
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                let vector = float32x4_t::from(vector.0);
                // SAFETY: These functions only require `neon`.
                unsafe {
                    vmaxvq_f32(vsetq_lane_f32::<3>(f32::NEG_INFINITY, vector))
                }
            }
        }
    }

    #[inline]
    fn min_element(vector: Vec3A<f32>) -> f32 {
        cfg_select! {
            target_feature = "sse2" => {
                let vector = __m128::from(vector.0);
                // SAFETY: These functions only require `sse2`.
                unsafe {
                    let reduce_2 = _mm_min_ps(
                        vector,
                        _mm_shuffle_ps(vector, vector, 0b00_00_10_10),
                    );
                    let reduce_1 = _mm_min_ps(
                        reduce_2,
                        _mm_shuffle_ps(reduce_2, reduce_2, 0b00_00_00_01),
                    );
                    _mm_cvtss_f32(reduce_1)
                }
            }
            all(target_arch = "aarch64", target_feature = "neon") => {
                let vector = float32x4_t::from(vector.0);
                // SAFETY: These functions only require `neon`.
                unsafe {
                    vminvq_f32(vsetq_lane_f32::<3>(f32::INFINITY, vector))
                }
            }
        }
    }

    #[inline(always)]
    fn floor(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.floor())
    }

    #[inline(always)]
    fn ceil(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.ceil())
    }

    #[inline(always)]
    fn round(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.round())
    }

    #[inline(always)]
    fn trunc(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.trunc())
    }

    #[inline(always)]
    fn mul_add(vector: Vec3A<f32>, a: Vec3A<f32>, b: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(
            vector.x.mul_add(a.x, b.x),
            vector.y.mul_add(a.y, b.y),
            vector.z.mul_add(a.z, b.z),
        )
    }

    #[inline(always)]
    fn div_euclid(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(
            vector.x.div_euclid(rhs.x),
            vector.y.div_euclid(rhs.y),
            vector.z.div_euclid(rhs.z),
        )
    }

    #[inline(always)]
    fn rem_euclid(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(
            vector.x.rem_euclid(rhs.x),
            vector.y.rem_euclid(rhs.y),
            vector.z.rem_euclid(rhs.z),
        )
    }

    #[inline(always)]
    fn powf(vector: Vec3A<f32>, n: f32) -> Vec3A<f32> {
        Vec3A::new(vector.x.powf(n), vector.y.powf(n), vector.z.powf(n))
    }

    #[inline(always)]
    fn sqrt(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vector(vector.0.sqrt())
    }

    #[inline(always)]
    fn exp(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.exp(), vector.y.exp(), vector.z.exp())
    }

    #[inline(always)]
    fn exp2(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.exp2(), vector.y.exp2(), vector.z.exp2())
    }

    #[inline(always)]
    fn ln(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.ln(), vector.y.ln(), vector.z.ln())
    }

    #[inline(always)]
    fn log2(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.log2(), vector.y.log2(), vector.z.log2())
    }

    #[inline(always)]
    fn sin(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.sin(), vector.y.sin(), vector.z.sin())
    }

    #[inline(always)]
    fn cos(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.cos(), vector.y.cos(), vector.z.cos())
    }

    #[inline(always)]
    fn tan(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.tan(), vector.y.tan(), vector.z.tan())
    }

    #[inline(always)]
    fn asin(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.asin(), vector.y.asin(), vector.z.asin())
    }

    #[inline(always)]
    fn acos(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.acos(), vector.y.acos(), vector.z.acos())
    }

    #[inline(always)]
    fn atan(vector: Vec3A<f32>) -> Vec3A<f32> {
        Vec3A::new(vector.x.atan(), vector.y.atan(), vector.z.atan())
    }

    #[inline(always)]
    fn sin_cos(vector: Vec3A<f32>) -> (Vec3A<f32>, Vec3A<f32>) {
        let x_sin_cos = vector.x.sin_cos();
        let y_sin_cos = vector.y.sin_cos();
        let z_sin_cos = vector.z.sin_cos();
        (
            Vec3A::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0),
            Vec3A::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1),
        )
    }
}

impl FloatVectorBackend<4, Aligned> for f32 {
    #[inline]
    fn nan_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.is_nan())
    }

    #[inline]
    fn finite_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.is_finite())
    }

    #[inline]
    fn sign_positive_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.is_sign_positive())
    }

    #[inline]
    fn sign_negative_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
        Mask(vector.0.is_sign_negative())
    }

    #[inline]
    fn max(vector: Vec4A<f32>, other: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.fast_max(other.0))
    }

    #[inline]
    fn min(vector: Vec4A<f32>, other: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.fast_min(other.0))
    }

    #[inline]
    fn abs(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.abs())
    }

    #[inline]
    fn signum(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.signum())
    }

    #[inline]
    fn copysign(vector: Vec4A<f32>, sign: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.copysign(sign.0))
    }

    #[inline]
    fn max_element(vector: Vec4A<f32>) -> f32 {
        // vector.0.fast_reduce_max()
        todo!("wait for https://github.com/Lokathor/wide/pull/331")
    }

    #[inline]
    fn min_element(vector: Vec4A<f32>) -> f32 {
        // vector.0.fast_reduce_min()
        todo!("wait for https://github.com/Lokathor/wide/pull/331")
    }

    #[inline(always)]
    fn floor(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.floor())
    }

    #[inline(always)]
    fn ceil(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.ceil())
    }

    #[inline(always)]
    fn round(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.round())
    }

    #[inline(always)]
    fn trunc(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.trunc())
    }

    #[inline(always)]
    fn mul_add(vector: Vec4A<f32>, a: Vec4A<f32>, b: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.mul_add(a.x, b.x),
            vector.y.mul_add(a.y, b.y),
            vector.z.mul_add(a.z, b.z),
            vector.w.mul_add(a.w, b.w),
        )
    }

    #[inline(always)]
    fn div_euclid(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.div_euclid(rhs.x),
            vector.y.div_euclid(rhs.y),
            vector.z.div_euclid(rhs.z),
            vector.w.div_euclid(rhs.w),
        )
    }

    #[inline(always)]
    fn rem_euclid(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.rem_euclid(rhs.x),
            vector.y.rem_euclid(rhs.y),
            vector.z.rem_euclid(rhs.z),
            vector.w.rem_euclid(rhs.w),
        )
    }

    #[inline(always)]
    fn powf(vector: Vec4A<f32>, n: f32) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.powf(n),
            vector.y.powf(n),
            vector.z.powf(n),
            vector.w.powf(n),
        )
    }

    #[inline(always)]
    fn sqrt(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vector(vector.0.sqrt())
    }

    #[inline(always)]
    fn exp(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.exp(),
            vector.y.exp(),
            vector.z.exp(),
            vector.w.exp(),
        )
    }

    #[inline(always)]
    fn exp2(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.exp2(),
            vector.y.exp2(),
            vector.z.exp2(),
            vector.w.exp2(),
        )
    }

    #[inline(always)]
    fn ln(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(vector.x.ln(), vector.y.ln(), vector.z.ln(), vector.w.ln())
    }

    #[inline(always)]
    fn log2(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.log2(),
            vector.y.log2(),
            vector.z.log2(),
            vector.w.log2(),
        )
    }

    #[inline(always)]
    fn sin(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.sin(),
            vector.y.sin(),
            vector.z.sin(),
            vector.w.sin(),
        )
    }

    #[inline(always)]
    fn cos(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.cos(),
            vector.y.cos(),
            vector.z.cos(),
            vector.w.cos(),
        )
    }

    #[inline(always)]
    fn tan(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.tan(),
            vector.y.tan(),
            vector.z.tan(),
            vector.w.tan(),
        )
    }

    #[inline(always)]
    fn asin(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.asin(),
            vector.y.asin(),
            vector.z.asin(),
            vector.w.asin(),
        )
    }

    #[inline(always)]
    fn acos(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.acos(),
            vector.y.acos(),
            vector.z.acos(),
            vector.w.acos(),
        )
    }

    #[inline(always)]
    fn atan(vector: Vec4A<f32>) -> Vec4A<f32> {
        Vec4A::new(
            vector.x.atan(),
            vector.y.atan(),
            vector.z.atan(),
            vector.w.atan(),
        )
    }

    #[inline(always)]
    fn sin_cos(vector: Vec4A<f32>) -> (Vec4A<f32>, Vec4A<f32>) {
        let x_sin_cos = vector.x.sin_cos();
        let y_sin_cos = vector.y.sin_cos();
        let z_sin_cos = vector.z.sin_cos();
        let w_sin_cos = vector.w.sin_cos();
        (
            Vec4A::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0, w_sin_cos.0),
            Vec4A::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1, w_sin_cos.1),
        )
    }
}

#[inline]
fn rem(vector: f32x4, rhs: f32x4) -> f32x4 {
    let result = vector - (vector / rhs).trunc() * rhs;

    let is_infinite = rhs.abs().simd_eq(f32x4::INFINITY);
    let is_zero = rhs.simd_eq(f32x4::ZERO);

    let result = (is_infinite | -0.0).bitselect(vector, result);
    is_zero.select(f32x4::NAN, result)
}
