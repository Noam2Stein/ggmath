#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[allow(unused_imports, reason = "rustc incorrectly thinks this is unused")]
use crate::utils::PrimitiveFloatUtils;
use crate::{
    Aligned, Mask, Mask3A, Mask4A, Rotor3A, Vec3A, Vec4A, Vector,
    backend::{AffineBackend, FloatVectorBackend, MaskBackend, RotorBackend, VectorBackend},
    utils::safe_target_feature,
};

// `Self::Inner` follows its requirements.
unsafe impl VectorBackend<3, Aligned> for f32 {
    type Inner = __m128;

    safe_target_feature! {
        #[inline]
        fn vector_eq(vector: &Vec3A<f32>, other: &Vec3A<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vector.0, other.0)) as u32 & 0b111 == 0b111
        }

        #[inline]
        fn vector_ne(vector: &Vec3A<f32>, other: &Vec3A<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vector.0, other.0)) as u32 & 0b111 != 0
        }

        #[inline]
        fn vector_neg(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(neg(vector.0))
        }

        #[inline]
        fn vector_not(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_xor_ps(vector.0, _mm_set1_ps(f32::from_bits(!0))))
        }

        #[inline]
        fn vector_add(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_add_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_sub(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_sub_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_mul(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_mul_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_div(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_div_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_rem(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(rem(vector.0, rhs.0))
        }

        #[inline]
        fn vector_shl(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::<f32>::from_bits(vector.to_bits() << rhs.to_bits())
        }

        #[inline]
        fn vector_shr(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::<f32>::from_bits(vector.to_bits() >> rhs.to_bits())
        }

        #[inline]
        fn vector_bitand(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_and_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_bitor(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_or_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_bitxor(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_xor_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_element_sum(vector: Vec3A<f32>) -> f32 {
            let vector = vector.0;
            // Add `-0.0` to retain the sign of the left operand. Adding `+0.0`
            // would incorrectly reset the sign when `z` is `-0.0`.
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, _mm_set1_ps(-0.0), 0b00_11_00_01));
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_element_product(vector: Vec3A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, _mm_set1_ps(1.0), 0b00_11_00_01));
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_eq_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(_mm_cmpeq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ne_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(_mm_cmpneq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_lt_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(_mm_cmplt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_gt_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(_mm_cmpgt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_le_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(_mm_cmple_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ge_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(_mm_cmpge_ps(vector.0, other.0))
        }
    }
}

// `Self::Inner` follows its requirements.
unsafe impl VectorBackend<4, Aligned> for f32 {
    type Inner = __m128;

    safe_target_feature! {
        #[inline]
        fn vector_eq(vector: &Vec4A<f32>, other: &Vec4A<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpeq_ps(vector.0, other.0)) as u32 == 0xf
        }

        #[inline]
        fn vector_ne(vector: &Vec4A<f32>, other: &Vec4A<f32>) -> bool {
            _mm_movemask_ps(_mm_cmpneq_ps(vector.0, other.0)) as u32 != 0
        }

        #[inline]
        fn vector_neg(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(neg(vector.0))
        }

        #[inline]
        fn vector_not(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_xor_ps(vector.0, _mm_set1_ps(f32::from_bits(!0))))
        }

        #[inline]
        fn vector_add(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_add_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_sub(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_sub_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_mul(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_mul_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_div(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_div_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_rem(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(rem(vector.0, rhs.0))
        }

        #[inline]
        fn vector_shl(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::<f32>::from_bits(vector.to_bits() << rhs.to_bits())
        }

        #[inline]
        fn vector_shr(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::<f32>::from_bits(vector.to_bits() >> rhs.to_bits())
        }

        #[inline]
        fn vector_bitand(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_and_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_bitor(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_or_ps(vector.0, rhs.0))
        }

        #[inline]
        fn vector_bitxor(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_xor_ps(vector.0, rhs.0))
        }

        #[cfg(not(target_feature = "ssse3"))]
        #[inline]
        fn vector_element_sum(vector: Vec4A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_11_00_01));
            let vector = _mm_add_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[cfg(target_feature = "ssse3")]
        #[inline]
        fn vector_element_sum(vector: Vec4A<f32>) -> f32 {
            let reduce_2 = _mm_hadd_ps(vector.0, vector.0);
            let reduce_1 = _mm_hadd_ps(reduce_2, reduce_2);
            _mm_cvtss_f32(reduce_1)
        }

        #[inline]
        fn vector_element_product(vector: Vec4A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_11_00_01));
            let vector = _mm_mul_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_10));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_eq_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(_mm_cmpeq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ne_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(_mm_cmpneq_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_lt_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(_mm_cmplt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_gt_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(_mm_cmpgt_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_le_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(_mm_cmple_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_ge_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(_mm_cmpge_ps(vector.0, other.0))
        }
    }
}

// SAFETY: The first `__m128` represents the matrix, the first two elements of
// the second `__m128` represent the vector, and the two remaining elements are
// padding. The padding satisfies the requirements of `Pod`. `Mat2A<f32>` is
// represented by `Vec4A<f32>` which is represented by `__m128`, so we have the
// same alignment.
unsafe impl AffineBackend<2, Aligned> for f32 {
    type Inner = [__m128; 2];
}

impl RotorBackend<3, Aligned> for f32 {
    #[inline]
    fn rotor_vector_mul(_vector: Vec3A<f32>, _rhs: Rotor3A<f32>) -> Vec3A<f32> {
        todo!()
    }

    #[inline]
    fn rotor_mul(_rotor: Rotor3A<f32>, _rhs: Rotor3A<f32>) -> Rotor3A<f32> {
        todo!()
    }
}

// `Self::Inner` follows its requirements.
unsafe impl MaskBackend<3, Aligned> for f32 {
    type Inner = __m128;

    safe_target_feature! {
        #[inline]
        fn mask_from_array(array: [bool; 3]) -> Mask3A<f32> {
            Mask(_mm_castsi128_ps(_mm_set_epi32(
                -(array[2] as i32),
                -(array[2] as i32),
                -(array[1] as i32),
                -(array[0] as i32),
            )))
        }

        #[inline]
        fn mask_splat(value: bool) -> Mask3A<f32> {
            Mask(_mm_castsi128_ps(_mm_set1_epi32(-(value as i32))))
        }

        #[inline]
        fn mask_to_array(mask: Mask3A<f32>) -> [bool; 3] {
            let bits = _mm_movemask_ps(mask.0);
            [bits & 0x1 != 0, bits & 0x2 != 0, bits & 0x4 != 0]
        }

        #[inline]
        fn mask_all(mask: Mask3A<f32>) -> bool {
            _mm_movemask_ps(mask.0) & 0x7 == 0x7
        }

        #[inline]
        fn mask_any(mask: Mask3A<f32>) -> bool {
            _mm_movemask_ps(mask.0) & 0x7 != 0
        }

        #[inline]
        fn mask_select(mask: Mask3A<f32>, if_true: Vec3A<f32>, if_false: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            ))
        }

        #[inline]
        fn mask_get(mask: Mask3A<f32>, index: usize) -> bool {
            match index {
                0 => _mm_movemask_ps(mask.0) & 0x1 != 0,
                1 => _mm_movemask_ps(mask.0) & 0x2 != 0,
                2 => _mm_movemask_ps(mask.0) & 0x4 != 0,
                _ => panic!("index out of bounds"),
            }
        }

        #[inline]
        fn mask_set(mask: &mut Mask3A<f32>, index: usize, value: bool) {
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
        fn mask_eq(mask: &Mask3A<f32>, other: &Mask3A<f32>) -> bool {
            _mm_movemask_ps(mask.0) & 0x7 == _mm_movemask_ps(other.0) & 0x7
        }

        #[inline]
        fn mask_ne(mask: &Mask3A<f32>, other: &Mask3A<f32>) -> bool {
            !(mask == other)
        }

        #[inline]
        fn mask_not(mask: Mask3A<f32>) -> Mask3A<f32> {
            Mask(_mm_xor_ps(mask.0, _mm_set1_ps(f32::from_bits(!0))))
        }

        #[inline]
        fn mask_bitand(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
            Mask(_mm_and_ps(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitor(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
            Mask(_mm_or_ps(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitxor(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
            Mask(_mm_xor_ps(mask.0, rhs.0))
        }
    }
}

// `Self::Inner` follows its requirements.
unsafe impl MaskBackend<4, Aligned> for f32 {
    type Inner = __m128;

    safe_target_feature! {
        #[inline]
        fn mask_from_array(array: [bool; 4]) -> Mask4A<f32> {
            Mask(_mm_castsi128_ps(_mm_set_epi32(
                -(array[3] as i32),
                -(array[2] as i32),
                -(array[1] as i32),
                -(array[0] as i32),
            )))
        }

        #[inline]
        fn mask_splat(value: bool) -> Mask4A<f32> {
            Mask(_mm_castsi128_ps(_mm_set1_epi32(-(value as i32))))
        }

        #[inline]
        fn mask_to_array(mask: Mask4A<f32>) -> [bool; 4] {
            let bits = _mm_movemask_ps(mask.0);
            [
                bits & 0x1 != 0,
                bits & 0x2 != 0,
                bits & 0x4 != 0,
                bits & 0x8 != 0,
            ]
        }

        #[inline]
        fn mask_all(mask: Mask4A<f32>) -> bool {
            _mm_movemask_ps(mask.0) == 0xf
        }

        #[inline]
        fn mask_any(mask: Mask4A<f32>) -> bool {
            _mm_movemask_ps(mask.0) != 0
        }

        #[inline]
        fn mask_select(mask: Mask4A<f32>, if_true: Vec4A<f32>, if_false: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_or_ps(
                _mm_andnot_ps(mask.0, if_false.0),
                _mm_and_ps(if_true.0, mask.0),
            ))
        }

        #[inline]
        fn mask_get(mask: Mask4A<f32>, index: usize) -> bool {
            match index {
                0 => _mm_movemask_ps(mask.0) & 0x1 != 0,
                1 => _mm_movemask_ps(mask.0) & 0x2 != 0,
                2 => _mm_movemask_ps(mask.0) & 0x4 != 0,
                3 => _mm_movemask_ps(mask.0) & 0x8 != 0,
                _ => panic!("index out of bounds"),
            }
        }

        #[inline]
        fn mask_set(mask: &mut Mask4A<f32>, index: usize, value: bool) {
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
        fn mask_eq(mask: &Mask4A<f32>, other: &Mask4A<f32>) -> bool {
            _mm_movemask_ps(mask.0) == _mm_movemask_ps(other.0)
        }

        #[inline]
        fn mask_ne(mask: &Mask4A<f32>, other: &Mask4A<f32>) -> bool {
            !(mask == other)
        }

        #[inline]
        fn mask_not(mask: Mask4A<f32>) -> Mask4A<f32> {
            Mask(_mm_xor_ps(mask.0, _mm_set1_ps(f32::from_bits(!0))))
        }

        #[inline]
        fn mask_bitand(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
            Mask(_mm_and_ps(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitor(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
            Mask(_mm_or_ps(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitxor(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
            Mask(_mm_xor_ps(mask.0, rhs.0))
        }
    }
}

impl FloatVectorBackend<3, Aligned> for f32 {
    safe_target_feature! {
        #[inline]
        fn vector_nan_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
            Mask(nan_mask(vector.0))
        }

        #[inline]
        fn vector_finite_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
            Mask(finite_mask(vector.0))
        }

        #[inline]
        fn vector_sign_positive_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
            Mask(sign_positive_mask(vector.0))
        }

        #[inline]
        fn vector_sign_negative_mask(vector: Vec3A<f32>) -> Mask3A<f32> {
            Mask(sign_negative_mask(vector.0))
        }

        #[inline]
        fn vector_max(vector: Vec3A<f32>, other: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_max_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_min(vector: Vec3A<f32>, other: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_min_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_abs(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(abs(vector.0))
        }

        #[inline]
        fn vector_signum(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(signum(vector.0))
        }

        #[inline]
        fn vector_copysign(vector: Vec3A<f32>, sign: Vec3A<f32>) -> Vec3A<f32> {
            Vector(copysign(vector.0, sign.0))
        }

        #[inline]
        fn vector_max_element(vector: Vec3A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_10_10));
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_min_element(vector: Vec3A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b01_01_10_10));
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline(always)]
        fn vector_floor(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(floor(vector.0))
        }

        #[inline(always)]
        fn vector_ceil(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(ceil(vector.0))
        }

        #[inline(always)]
        fn vector_round(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(round(vector.0))
        }

        #[inline(always)]
        fn vector_trunc(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(trunc(vector.0))
        }

        #[inline(always)]
        fn vector_mul_add(vector: Vec3A<f32>, a: Vec3A<f32>, b: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(
                vector.x.mul_add(a.x, b.x),
                vector.y.mul_add(a.y, b.y),
                vector.z.mul_add(a.z, b.z),
            )
        }

        #[inline(always)]
        fn vector_div_euclid(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(
                vector.x.div_euclid(rhs.x),
                vector.y.div_euclid(rhs.y),
                vector.z.div_euclid(rhs.z),
            )
        }

        #[inline(always)]
        fn vector_rem_euclid(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(
                vector.x.rem_euclid(rhs.x),
                vector.y.rem_euclid(rhs.y),
                vector.z.rem_euclid(rhs.z),
            )
        }

        #[inline(always)]
        fn vector_powf(vector: Vec3A<f32>, n: f32) -> Vec3A<f32> {
            Vec3A::new(vector.x.powf(n), vector.y.powf(n), vector.z.powf(n))
        }

        #[inline(always)]
        fn vector_sqrt(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(_mm_sqrt_ps(vector.0))
        }

        #[inline(always)]
        fn vector_exp(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.exp(), vector.y.exp(), vector.z.exp())
        }

        #[inline(always)]
        fn vector_exp2(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.exp2(), vector.y.exp2(), vector.z.exp2())
        }

        #[inline(always)]
        fn vector_ln(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.ln(), vector.y.ln(), vector.z.ln())
        }

        #[inline(always)]
        fn vector_log2(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.log2(), vector.y.log2(), vector.z.log2())
        }

        #[inline(always)]
        fn vector_sin(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.sin(), vector.y.sin(), vector.z.sin())
        }

        #[inline(always)]
        fn vector_cos(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.cos(), vector.y.cos(), vector.z.cos())
        }

        #[inline(always)]
        fn vector_tan(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.tan(), vector.y.tan(), vector.z.tan())
        }

        #[inline(always)]
        fn vector_asin(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.asin(), vector.y.asin(), vector.z.asin())
        }

        #[inline(always)]
        fn vector_acos(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.acos(), vector.y.acos(), vector.z.acos())
        }

        #[inline(always)]
        fn vector_atan(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vec3A::new(vector.x.atan(), vector.y.atan(), vector.z.atan())
        }

        #[inline(always)]
        fn vector_sin_cos(vector: Vec3A<f32>) -> (Vec3A<f32>, Vec3A<f32>) {
            let x_sin_cos = vector.x.sin_cos();
            let y_sin_cos = vector.y.sin_cos();
            let z_sin_cos = vector.z.sin_cos();
            (
                Vec3A::new(x_sin_cos.0, y_sin_cos.0, z_sin_cos.0),
                Vec3A::new(x_sin_cos.1, y_sin_cos.1, z_sin_cos.1),
            )
        }
    }
}

impl FloatVectorBackend<4, Aligned> for f32 {
    safe_target_feature! {
        #[inline]
        fn vector_nan_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
            Mask(nan_mask(vector.0))
        }

        #[inline]
        fn vector_finite_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
            Mask(finite_mask(vector.0))
        }

        #[inline]
        fn vector_sign_positive_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
            Mask(sign_positive_mask(vector.0))
        }

        #[inline]
        fn vector_sign_negative_mask(vector: Vec4A<f32>) -> Mask4A<f32> {
            Mask(sign_negative_mask(vector.0))
        }

        #[inline]
        fn vector_max(vector: Vec4A<f32>, other: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_max_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_min(vector: Vec4A<f32>, other: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_min_ps(vector.0, other.0))
        }

        #[inline]
        fn vector_abs(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(abs(vector.0))
        }

        #[inline]
        fn vector_signum(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(signum(vector.0))
        }

        #[inline]
        fn vector_copysign(vector: Vec4A<f32>, sign: Vec4A<f32>) -> Vec4A<f32> {
            Vector(copysign(vector.0, sign.0))
        }

        #[inline]
        fn vector_max_element(vector: Vec4A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_11_10));
            let vector = _mm_max_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline]
        fn vector_min_element(vector: Vec4A<f32>) -> f32 {
            let vector = vector.0;
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_11_10));
            let vector = _mm_min_ps(vector, _mm_shuffle_ps(vector, vector, 0b00_00_00_01));
            _mm_cvtss_f32(vector)
        }

        #[inline(always)]
        fn vector_floor(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(floor(vector.0))
        }

        #[inline(always)]
        fn vector_ceil(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(ceil(vector.0))
        }

        #[inline(always)]
        fn vector_round(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(round(vector.0))
        }

        #[inline(always)]
        fn vector_trunc(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(trunc(vector.0))
        }

        #[inline(always)]
        fn vector_mul_add(vector: Vec4A<f32>, a: Vec4A<f32>, b: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(
                vector.x.mul_add(a.x, b.x),
                vector.y.mul_add(a.y, b.y),
                vector.z.mul_add(a.z, b.z),
                vector.w.mul_add(a.w, b.w),
            )
        }

        #[inline(always)]
        fn vector_div_euclid(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(
                vector.x.div_euclid(rhs.x),
                vector.y.div_euclid(rhs.y),
                vector.z.div_euclid(rhs.z),
                vector.w.div_euclid(rhs.w),
            )
        }

        #[inline(always)]
        fn vector_rem_euclid(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(
                vector.x.rem_euclid(rhs.x),
                vector.y.rem_euclid(rhs.y),
                vector.z.rem_euclid(rhs.z),
                vector.w.rem_euclid(rhs.w),
            )
        }

        #[inline(always)]
        fn vector_powf(vector: Vec4A<f32>, n: f32) -> Vec4A<f32> {
            Vec4A::new(vector.x.powf(n), vector.y.powf(n), vector.z.powf(n), vector.w.powf(n))
        }

        #[inline(always)]
        fn vector_sqrt(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(_mm_sqrt_ps(vector.0))
        }

        #[inline(always)]
        fn vector_exp(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.exp(), vector.y.exp(), vector.z.exp(), vector.w.exp())
        }

        #[inline(always)]
        fn vector_exp2(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.exp2(), vector.y.exp2(), vector.z.exp2(), vector.w.exp2())
        }

        #[inline(always)]
        fn vector_ln(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.ln(), vector.y.ln(), vector.z.ln(), vector.w.ln())
        }

        #[inline(always)]
        fn vector_log2(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.log2(), vector.y.log2(), vector.z.log2(), vector.w.log2())
        }

        #[inline(always)]
        fn vector_sin(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.sin(), vector.y.sin(), vector.z.sin(), vector.w.sin())
        }

        #[inline(always)]
        fn vector_cos(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.cos(), vector.y.cos(), vector.z.cos(), vector.w.cos())
        }

        #[inline(always)]
        fn vector_tan(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.tan(), vector.y.tan(), vector.z.tan(), vector.w.tan())
        }

        #[inline(always)]
        fn vector_asin(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.asin(), vector.y.asin(), vector.z.asin(), vector.w.asin())
        }

        #[inline(always)]
        fn vector_acos(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.acos(), vector.y.acos(), vector.z.acos(), vector.w.acos())
        }

        #[inline(always)]
        fn vector_atan(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vec4A::new(vector.x.atan(), vector.y.atan(), vector.z.atan(), vector.w.atan())
        }

        #[inline(always)]
        fn vector_sin_cos(vector: Vec4A<f32>) -> (Vec4A<f32>, Vec4A<f32>) {
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
}

safe_target_feature! {
    #[inline]
    fn neg(vector: __m128) -> __m128 {
        _mm_xor_ps(vector, _mm_set1_ps(-0.0))
    }

    #[inline]
    fn rem(vector: __m128, rhs: __m128) -> __m128 {
        let result = _mm_sub_ps(vector, _mm_mul_ps(trunc(_mm_div_ps(vector, rhs)), rhs));

        let inf_mask = _mm_cmpeq_ps(abs(rhs), _mm_set1_ps(f32::INFINITY));
        let zero_mask = _mm_cmpeq_ps(rhs, _mm_set1_ps(0.0));
        let result = select(_mm_or_ps(inf_mask, _mm_set1_ps(-0.0)), vector, result);

        select(zero_mask, _mm_set1_ps(f32::NAN), result)
    }

    #[inline]
    fn nan_mask(vector: __m128) -> __m128 {
        _mm_cmpneq_ps(vector, vector)
    }

    #[inline]
    fn finite_mask(vector: __m128) -> __m128 {
        _mm_cmplt_ps(abs(vector), _mm_set1_ps(f32::INFINITY))
    }

    #[inline]
    fn sign_positive_mask(vector: __m128) -> __m128 {
        _mm_castsi128_ps(_mm_cmpeq_epi32(
            _mm_castps_si128(vector),
            _mm_castps_si128(abs(vector)),
        ))
    }

    #[inline]
    fn sign_negative_mask(vector: __m128) -> __m128 {
        _mm_castsi128_ps(_mm_cmpeq_epi32(
            _mm_castps_si128(vector),
            _mm_castps_si128(_mm_or_ps(_mm_set1_ps(-0.0), vector)),
        ))
    }

    #[inline]
    fn abs(vector: __m128) -> __m128 {
        _mm_and_ps(_mm_set1_ps(f32::from_bits(0x7fffffff)), vector)
    }

    #[inline]
    fn signum(vector: __m128) -> __m128 {
        let result = _mm_or_ps(_mm_set1_ps(1.0), _mm_and_ps(vector, _mm_set1_ps(-0.0)));
        let nan_mask = _mm_cmpneq_ps(vector, vector);

        select(nan_mask, vector, result)
    }

    #[inline]
    fn copysign(vector: __m128, sign: __m128) -> __m128 {
        select(_mm_set1_ps(-0.0), sign, vector)
    }

    #[inline]
    fn select(mask: __m128, if_true: __m128, if_false: __m128) -> __m128 {
        _mm_or_ps(_mm_and_ps(mask, if_true), _mm_andnot_ps(mask, if_false))
    }

    #[cfg(not(target_feature = "sse4.1"))]
    #[inline]
    fn floor(vector: __m128) -> __m128 {
        let trunc = _mm_cvtepi32_ps(_mm_cvttps_epi32(vector));
        let greater_mask = _mm_cmpgt_ps(trunc, vector);
        // 0 -> 0, 0xffffffff -> -1.0f
        let offset = _mm_cvtepi32_ps(_mm_castps_si128(greater_mask));
        let result = _mm_add_ps(trunc, offset);

        // Handle large values, inf, and NaN
        let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
            _mm_castps_si128(abs(vector)),
            _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
        ));

        select(abs(bounds_mask), result, vector)
    }

    #[cfg(target_feature = "sse4.1")]
    #[inline]
    fn floor(vector: __m128) -> __m128 {
        _mm_floor_ps(vector)
    }

    #[cfg(not(target_feature = "sse4.1"))]
    #[inline]
    fn ceil(vector: __m128) -> __m128 {
        let trunc = _mm_cvtepi32_ps(_mm_cvttps_epi32(vector));
        let less_mask = _mm_cmplt_ps(trunc, vector);
        // 0 -> 0, 0xffffffff -> -1.0f
        let neg_offset = _mm_cvtepi32_ps(_mm_castps_si128(less_mask));
        let result = _mm_sub_ps(trunc, neg_offset);

        // Handle large values, inf, and NaN
        let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
            _mm_castps_si128(abs(vector)),
            _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
        ));

        select(abs(bounds_mask), result, vector)
    }

    #[cfg(target_feature = "sse4.1")]
    #[inline]
    fn ceil(vector: __m128) -> __m128 {
        _mm_ceil_ps(vector)
    }

    #[cfg(not(target_feature = "sse4.1"))]
    #[inline]
    fn round(vector: __m128) -> __m128 {
        let vector_abs = abs(vector);
        let result_abs = _mm_cvtepi32_ps(_mm_cvttps_epi32(_mm_add_ps(vector_abs, _mm_set1_ps(0.5))));

        // The addition breaks for `0.5.next_down()` which incorrectly rounds to
        // `1.0`. This resets `result` to `0.0`.
        let result_abs = _mm_and_ps(
            result_abs,
            _mm_cmpneq_ps(vector_abs, _mm_set1_ps(0.5_f32.next_down())),
        );

        // Large value, infinity and NaN need special handling.
        let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
            _mm_castps_si128(vector_abs),
            _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
        ));

        // `abs` keeps the original sign.
        select(abs(bounds_mask), result_abs, vector)
    }

    #[cfg(target_feature = "sse4.1")]
    #[inline]
    fn round(vector: __m128) -> __m128 {
        let vector_abs = abs(vector);
        let result_abs = _mm_round_ps::<_MM_FROUND_TO_ZERO>(_mm_add_ps(vector_abs, _mm_set1_ps(0.5)));

        // The addition breaks for `0.5.next_down()` which incorrectly rounds to
        // `1.0`. This resets `result` to `0.0`.
        let result_abs = _mm_and_ps(
            result_abs,
            _mm_cmpneq_ps(vector_abs, _mm_set1_ps(0.5_f32.next_down())),
        );

        // Large value, infinity and NaN need special handling.
        let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
            _mm_castps_si128(vector_abs),
            _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
        ));

        // `abs` keeps the original sign.
        select(abs(bounds_mask), result_abs, vector)
    }

    #[cfg(not(target_feature = "sse4.1"))]
    #[inline]
    fn trunc(vector: __m128) -> __m128 {
        let result = _mm_cvtepi32_ps(_mm_cvttps_epi32(vector));

        // Large value, infinity, and NaN need special handling.
        let bounds_mask = _mm_castsi128_ps(_mm_cmplt_epi32(
            _mm_castps_si128(abs(vector)),
            _mm_set1_epi32(8388608.0_f32.to_bits() as i32),
        ));

        select(abs(bounds_mask), result, vector)
    }

    #[cfg(target_feature = "sse4.1")]
    #[inline]
    fn trunc(vector: __m128) -> __m128 {
        _mm_round_ps::<_MM_FROUND_TO_ZERO>(vector)
    }
}
