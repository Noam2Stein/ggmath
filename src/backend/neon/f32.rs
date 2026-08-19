use core::{arch::aarch64::*, mem::transmute};

#[allow(unused_imports, reason = "rustc incorrectly thinks this is unused")]
use crate::utils::PrimitiveFloatUtils;
use crate::{
    Aligned, Mask, Mask3A, Mask4A, QuatA, Quaternion, Rot3A, Rotor, Vec3A, Vec4A, Vector,
    backend::{FloatVectorBackend, MaskBackend, QuaternionBackend, RotorBackend, VectorBackend},
    utils::{Repr4, safe_target_feature},
};

// `Self::Inner` follows its requirements.
unsafe impl VectorBackend<3, Aligned> for f32 {
    type Inner = float32x4_t;

    safe_target_feature! {
        #[inline]
        fn vector_eq(vector: &Vec3A<f32>, other: &Vec3A<f32>) -> bool {
            vector.eq_mask(*other).all()
        }

        #[inline]
        fn vector_ne(vector: &Vec3A<f32>, other: &Vec3A<f32>) -> bool {
            !(vector == other)
        }

        #[inline]
        fn vector_neg(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vnegq_f32(vector.0))
        }

        #[inline]
        fn vector_not(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vreinterpretq_f32_u32(vmvnq_u32(vreinterpretq_u32_f32(vector.0))))
        }

        #[inline]
        fn vector_add(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vaddq_f32(vector.0, rhs.0))
        }

        #[inline]
        fn vector_sub(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vsubq_f32(vector.0, rhs.0))
        }

        #[inline]
        fn vector_mul(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vmulq_f32(vector.0, rhs.0))
        }

        #[inline]
        fn vector_div(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vdivq_f32(vector.0, rhs.0))
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
            Vector(vreinterpretq_f32_u32(vandq_u32(
                vreinterpretq_u32_f32(vector.0),
                vreinterpretq_u32_f32(rhs.0))),
            )
        }

        #[inline]
        fn vector_bitor(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vreinterpretq_f32_u32(vorrq_u32(
                vreinterpretq_u32_f32(vector.0),
                vreinterpretq_u32_f32(rhs.0))),
            )
        }

        #[inline]
        fn vector_bitxor(vector: Vec3A<f32>, rhs: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vreinterpretq_f32_u32(veorq_u32(
                vreinterpretq_u32_f32(vector.0),
                vreinterpretq_u32_f32(rhs.0))),
            )
        }

        #[inline]
        fn vector_element_sum(vector: Vec3A<f32>) -> f32 {
            // Add `-0.0` to retain the sign of the left operand. Adding `+0.0`
            // would incorrectly reset the sign when `z` is `-0.0`.
            vaddvq_f32(vsetq_lane_f32(-0.0, vector.0, 3))
        }

        #[inline]
        fn vector_element_product(vector: Vec3A<f32>) -> f32 {
            vector.x * vector.y * vector.z
        }

        #[inline]
        fn vector_eq_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(vceqq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_ne_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            !vector.eq_mask(other)
        }

        #[inline]
        fn vector_lt_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(vcltq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_gt_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(vcgtq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_le_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(vcleq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_ge_mask(vector: Vec3A<f32>, other: Vec3A<f32>) -> Mask3A<f32> {
            Mask(vcgeq_f32(vector.0, other.0))
        }
    }
}

// `Self::Inner` follows its requirements.
unsafe impl VectorBackend<4, Aligned> for f32 {
    type Inner = float32x4_t;

    safe_target_feature! {
        #[inline]
        fn vector_eq(vector: &Vec4A<f32>, other: &Vec4A<f32>) -> bool {
            vector.eq_mask(*other).all()
        }

        #[inline]
        fn vector_ne(vector: &Vec4A<f32>, other: &Vec4A<f32>) -> bool {
            vector.ne_mask(*other).any()
        }

        #[inline]
        fn vector_neg(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vnegq_f32(vector.0))
        }

        #[inline]
        fn vector_not(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vreinterpretq_f32_u32(vmvnq_u32(vreinterpretq_u32_f32(vector.0))))
        }

        #[inline]
        fn vector_add(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vaddq_f32(vector.0, rhs.0))
        }

        #[inline]
        fn vector_sub(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vsubq_f32(vector.0, rhs.0))
        }

        #[inline]
        fn vector_mul(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vmulq_f32(vector.0, rhs.0))
        }

        #[inline]
        fn vector_div(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vdivq_f32(vector.0, rhs.0))
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
            Vector(vreinterpretq_f32_u32(vandq_u32(
                vreinterpretq_u32_f32(vector.0),
                vreinterpretq_u32_f32(rhs.0))),
            )
        }

        #[inline]
        fn vector_bitor(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vreinterpretq_f32_u32(vorrq_u32(
                vreinterpretq_u32_f32(vector.0),
                vreinterpretq_u32_f32(rhs.0))),
            )
        }

        #[inline]
        fn vector_bitxor(vector: Vec4A<f32>, rhs: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vreinterpretq_f32_u32(veorq_u32(
                vreinterpretq_u32_f32(vector.0),
                vreinterpretq_u32_f32(rhs.0))),
            )
        }

        #[inline]
        fn vector_element_sum(vector: Vec4A<f32>) -> f32 {
            // (a + b) + (c + d)
            vaddv_f32(vpadd_f32(vget_low_f32(vector.0), vget_high_f32(vector.0)))
        }

        #[inline]
        fn vector_element_product(vector: Vec4A<f32>) -> f32 {
            let bcda = vextq_f32::<1>(vector.0, vector.0);
            let temp = vmulq_f32(vector.0, bcda);
            vgetq_lane_f32::<0>(temp) * vgetq_lane_f32::<2>(temp)
        }

        #[inline]
        fn vector_eq_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(vceqq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_ne_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            !vector.eq_mask(other)
        }

        #[inline]
        fn vector_lt_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(vcltq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_gt_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(vcgtq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_le_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(vcleq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_ge_mask(vector: Vec4A<f32>, other: Vec4A<f32>) -> Mask4A<f32> {
            Mask(vcgeq_f32(vector.0, other.0))
        }
    }
}

impl RotorBackend<3, Aligned> for f32 {
    #[inline]
    fn rotor_vector_mul(vector: Vec3A<f32>, rhs: Rot3A<f32>) -> Vec3A<f32> {
        let vector_xyzz = vector.xyzz();
        let vector_yxyx = vector.yxyx();
        let rhs_wwwx = rhs.0.wwwx();
        let rhs_yyxx = rhs.0.yyxx();
        let rhs_zzzy = rhs.0.zzzy();
        let rhs_xxxx = rhs.0.xxxx();
        let rhs_yyyy = rhs.0.yyyy();

        let f = rhs_wwwx * vector_xyzz
            + Vec4A::new(1.0, 1.0, -1.0, 1.0)
                * (rhs_zzzy * vector_xyzz - rhs_yyxx * vector_yxyx).wzyx();

        rhs_wwwx.truncate() * f.truncate()
            + Vec4A::new(1.0, 1.0, -1.0, -1.0).truncate() * rhs_zzzy.truncate() * f.wzy()
            + Vec4A::new(1.0, -1.0, 1.0, -1.0).truncate() * rhs_xxxx.truncate() * f.yxw()
            + Vec4A::new(1.0, -1.0, -1.0, 1.0).truncate() * rhs_yyyy.truncate() * f.zwx()
    }

    #[inline]
    fn rotor_mul(rotor: Rot3A<f32>, rhs: Rot3A<f32>) -> Rot3A<f32> {
        // This is the result of simplifying the multiplication of two rotors
        // with our basis bivectors:
        //
        // xy0 * s1 + s0 * xy1 - xz0 * yz1 + yz0 * xz1
        // xz0 * s1 + s0 * xz1 + xy0 * yz1 + yz0 * xy1
        // yz0 * s1 + s0 * yz1 + xz0 * xy1 - xy0 * xz1
        // s0 * s1 - xy0 * xy1 - xz0 * xz1 - yz0 * yz1

        // In raw vector form, this is:
        //
        // x0 * w1 + w0 * x1 - y0 * z1 + z0 * y1
        // y0 * w1 + w0 * y1 + x0 * z1 + z0 * x1
        // z0 * w1 + w0 * z1 + y0 * x1 - x0 * y1
        // w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1

        // Change the addition order to put subtracts at the end:
        //
        // x0 * w1 + w0 * x1 + z0 * y1 - y0 * z1
        // y0 * w1 + w0 * y1 + x0 * z1 + z0 * x1
        // z0 * w1 + w0 * z1 + y0 * x1 - x0 * y1
        // w0 * w1 - x0 * x1 - y0 * y1 - z0 * z1

        // Wrap negation:
        //
        // x0 * w1 + (w0 * x1 + z0 * y1 - y0 * z1)
        // y0 * w1 + (w0 * y1 + x0 * z1 + z0 * x1)
        // z0 * w1 + (w0 * z1 + y0 * x1 - x0 * y1)
        // w0 * w1 - (x0 * x1 + y0 * y1 + z0 * z1)

        // In swizzle notation, this is:
        //
        // a*b.wwww + (+++-)(a.wwwx*b.xyzx + a.zxyy*b.yzxy + (-+-+)a.yzxz*b.zxyz)

        // This implementation uses 7 shuffles and 2 bitxors. I *think* this is
        // the most efficient this operation can be.

        const PPPN: Vec4A<f32> = Vec4A::new(0.0, 0.0, 0.0, -0.0);
        const NPNP: Vec4A<f32> = Vec4A::new(-0.0, 0.0, -0.0, 0.0);

        let coe1 = rotor.0 * rhs.0.wwww();
        let coe2 = rotor.0.wwwx() * rhs.0.xyzx();
        let coe3 = rotor.0.zxyy() * rhs.0.yzxy();
        let coe4 = rotor.0.yzxz() * rhs.0.zxyz();

        let neg_coe4 = Vec4A::<f32>::from_bits(NPNP.to_bits() ^ coe4.to_bits());
        let coe123 = coe2 + coe3 + neg_coe4;
        let neg_coe123 = Vec4A::<f32>::from_bits(PPPN.to_bits() ^ coe123.to_bits());

        Rotor(coe1 + neg_coe123)
    }
}

impl QuaternionBackend<Aligned> for f32 {
    #[inline]
    fn quat_mul(quat: QuatA<f32>, rhs: QuatA<f32>) -> QuatA<f32> {
        const PNPN: Vec4A<f32> = Vec4A::new(0.0, -0.0, 0.0, -0.0);
        const PPNN: Vec4A<f32> = Vec4A::new(0.0, 0.0, -0.0, -0.0);
        const NPPN: Vec4A<f32> = Vec4A::new(-0.0, 0.0, 0.0, -0.0);

        Quaternion(
            quat.0 * rhs.0.wwww()
                + Vec4A::<f32>::from_bits(
                    PNPN.to_bits() ^ (quat.0.wzyx() * rhs.0.xxxx()).to_bits(),
                )
                + Vec4A::<f32>::from_bits(
                    PPNN.to_bits() ^ (quat.0.zwxy() * rhs.0.yyyy()).to_bits(),
                )
                + Vec4A::<f32>::from_bits(
                    NPPN.to_bits() ^ (quat.0.yxwz() * rhs.0.zzzz()).to_bits(),
                ),
        )
    }
}

// `Self::Inner` follows its requirements.
unsafe impl MaskBackend<3, Aligned> for f32 {
    type Inner = uint32x4_t;

    safe_target_feature! {
        #[inline]
        fn mask_from_array(array: [bool; 3]) -> Mask3A<f32> {
            // SAFETY: Both types accept all bit-patterns.
            Mask(unsafe {
                transmute::<Repr4<i32>, uint32x4_t>(Repr4(
                    -(array[0] as i32),
                    -(array[1] as i32),
                    -(array[2] as i32),
                    -(array[2] as i32),
                ))
            })
        }

        #[inline]
        fn mask_splat(value: bool) -> Mask3A<f32> {
            Mask(vdupq_n_u32(-(value as i32) as u32))
        }

        #[inline]
        fn mask_to_array(mask: Mask3A<f32>) -> [bool; 3] {
            [
                vgetq_lane_u32::<0>(mask.0) != 0,
                vgetq_lane_u32::<1>(mask.0) != 0,
                vgetq_lane_u32::<2>(mask.0) != 0,
            ]
        }

        #[inline]
        fn mask_all(mask: Mask3A<f32>) -> bool {
            // SAFETY: Both types accept all bit-patterns.
            const MASK: uint32x4_t = unsafe { transmute::<[u32; 4], uint32x4_t>([0b001, 0b010, 0b100, 0]) };

            let masked = vandq_u32(mask.0, MASK);
            let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
            let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

            bitmask == 0b111
        }

        #[inline]
        fn mask_any(mask: Mask3A<f32>) -> bool {
            // SAFETY: Both types accept all bit-patterns.
            const MASK: uint32x4_t = unsafe { transmute::<[u32; 4], uint32x4_t>([0b001, 0b010, 0b100, 0]) };

            let masked = vandq_u32(mask.0, MASK);
            let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
            let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

            bitmask != 0
        }

        #[inline]
        fn mask_select(mask: Mask3A<f32>, if_true: Vec3A<f32>, if_false: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vbslq_f32(mask.0, if_true.0, if_false.0))
        }

        #[inline]
        fn mask_get(mask: Mask3A<f32>, index: usize) -> bool {
            match index {
                0 => vgetq_lane_u32::<0>(mask.0) != 0,
                1 => vgetq_lane_u32::<1>(mask.0) != 0,
                2 => vgetq_lane_u32::<2>(mask.0) != 0,
                _ => panic!("index out of bounds"),
            }
        }

        #[inline]
        fn mask_set(mask: &mut Mask3A<f32>, index: usize, value: bool) {
            let value = -(value as i32) as u32;
            mask.0 = match index {
                0 => vsetq_lane_u32::<0>(value, mask.0),
                1 => vsetq_lane_u32::<1>(value, mask.0),
                2 => vsetq_lane_u32::<2>(value, mask.0),
                _ => panic!("index out of bounds"),
            };
        }

        #[inline]
        fn mask_eq(mask: &Mask3A<f32>, other: &Mask3A<f32>) -> bool {
            // SAFETY: Both types accept all bit-patterns.
            const MASK: uint32x4_t = unsafe { transmute::<[u32; 4], uint32x4_t>([0b001, 0b010, 0b100, 0]) };

            let masked = vandq_u32(vceqq_u32(mask.0, other.0), MASK);
            let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
            let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

            bitmask == 0b111
        }

        #[inline]
        fn mask_ne(mask: &Mask3A<f32>, other: &Mask3A<f32>) -> bool {
            !(mask == other)
        }

        #[inline]
        fn mask_not(mask: Mask3A<f32>) -> Mask3A<f32> {
            Mask(vmvnq_u32(mask.0))
        }

        #[inline]
        fn mask_bitand(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
            Mask(vandq_u32(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitor(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
            Mask(vorrq_u32(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitxor(mask: Mask3A<f32>, rhs: Mask3A<f32>) -> Mask3A<f32> {
            Mask(veorq_u32(mask.0, rhs.0))
        }
    }
}

// `Self::Inner` follows its requirements.
unsafe impl MaskBackend<4, Aligned> for f32 {
    type Inner = uint32x4_t;

    safe_target_feature! {
        #[inline]
        fn mask_from_array(array: [bool; 4]) -> Mask4A<f32> {
            // SAFETY: Both types accept all bit-patterns.
            Mask(unsafe {
                transmute::<Repr4<i32>, uint32x4_t>(Repr4(
                    -(array[0] as i32),
                    -(array[1] as i32),
                    -(array[2] as i32),
                    -(array[3] as i32),
                ))
            })
        }

        #[inline]
        fn mask_splat(value: bool) -> Mask4A<f32> {
            Mask(vdupq_n_u32(-(value as i32) as u32))
        }

        #[inline]
        fn mask_to_array(mask: Mask4A<f32>) -> [bool; 4] {
            [
                vgetq_lane_u32::<0>(mask.0) != 0,
                vgetq_lane_u32::<1>(mask.0) != 0,
                vgetq_lane_u32::<2>(mask.0) != 0,
                vgetq_lane_u32::<3>(mask.0) != 0,
            ]
        }

        #[inline]
        fn mask_all(mask: Mask4A<f32>) -> bool {
            // SAFETY: Both types accept all bit-patterns.
            const MASK: uint32x4_t = unsafe { transmute::<[u32; 4], uint32x4_t>([0b0001, 0b0010, 0b0100, 0b1000]) };

            let masked = vandq_u32(mask.0, MASK);
            let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
            let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

            bitmask == 0b1111
        }

        #[inline]
        fn mask_any(mask: Mask4A<f32>) -> bool {
            // SAFETY: Both types accept all bit-patterns.
            const MASK: uint32x4_t = unsafe { transmute::<[u32; 4], uint32x4_t>([0b0001, 0b0010, 0b0100, 0b1000]) };

            let masked = vandq_u32(mask.0, MASK);
            let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
            let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

            bitmask != 0
        }

        #[inline]
        fn mask_select(mask: Mask4A<f32>, if_true: Vec4A<f32>, if_false: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vbslq_f32(mask.0, if_true.0, if_false.0))
        }

        #[inline]
        fn mask_get(mask: Mask4A<f32>, index: usize) -> bool {
            match index {
                0 => vgetq_lane_u32::<0>(mask.0) != 0,
                1 => vgetq_lane_u32::<1>(mask.0) != 0,
                2 => vgetq_lane_u32::<2>(mask.0) != 0,
                3 => vgetq_lane_u32::<3>(mask.0) != 0,
                _ => panic!("index out of bounds"),
            }
        }

        #[inline]
        fn mask_set(mask: &mut Mask4A<f32>, index: usize, value: bool) {
            let value = -(value as i32) as u32;
            mask.0 = match index {
                0 => vsetq_lane_u32::<0>(value, mask.0),
                1 => vsetq_lane_u32::<1>(value, mask.0),
                2 => vsetq_lane_u32::<2>(value, mask.0),
                3 => vsetq_lane_u32::<3>(value, mask.0),
                _ => panic!("index out of bounds"),
            };
        }

        #[inline]
        fn mask_eq(mask: &Mask4A<f32>, other: &Mask4A<f32>) -> bool {
            // SAFETY: Both types accept all bit-patterns.
            const MASK: uint32x4_t = unsafe { transmute::<[u32; 4], uint32x4_t>([0b0001, 0b0010, 0b0100, 0b1000]) };

            let masked = vandq_u32(vceqq_u32(mask.0, other.0), MASK);
            let reduce_2 = vorr_u32(vget_low_u32(masked), vget_high_u32(masked));
            let bitmask = vget_lane_u32::<0>(reduce_2) | vget_lane_u32::<1>(reduce_2);

            bitmask == 0b1111
        }

        #[inline]
        fn mask_ne(mask: &Mask4A<f32>, other: &Mask4A<f32>) -> bool {
            !(mask == other)
        }

        #[inline]
        fn mask_not(mask: Mask4A<f32>) -> Mask4A<f32> {
            Mask(vmvnq_u32(mask.0))
        }

        #[inline]
        fn mask_bitand(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
            Mask(vandq_u32(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitor(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
            Mask(vorrq_u32(mask.0, rhs.0))
        }

        #[inline]
        fn mask_bitxor(mask: Mask4A<f32>, rhs: Mask4A<f32>) -> Mask4A<f32> {
            Mask(veorq_u32(mask.0, rhs.0))
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
            Vector(vmaxq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_min(vector: Vec3A<f32>, other: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vminq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_abs(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vabsq_f32(vector.0))
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
            vmaxvq_f32(vsetq_lane_f32::<3>(f32::NEG_INFINITY, vector.0))
        }

        #[inline]
        fn vector_min_element(vector: Vec3A<f32>) -> f32 {
            vminvq_f32(vsetq_lane_f32::<3>(f32::INFINITY, vector.0))
        }

        #[inline(always)]
        fn vector_floor(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vrndmq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_ceil(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vrndpq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_round(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vrndaq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_trunc(vector: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vrndq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_mul_add(vector: Vec3A<f32>, a: Vec3A<f32>, b: Vec3A<f32>) -> Vec3A<f32> {
            Vector(vfmaq_f32(b.0, vector.0, a.0))
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
            Vector(vsqrtq_f32(vector.0))
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
            Vector(vmaxq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_min(vector: Vec4A<f32>, other: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vminq_f32(vector.0, other.0))
        }

        #[inline]
        fn vector_abs(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vabsq_f32(vector.0))
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
            vmaxvq_f32(vector.0)
        }

        #[inline]
        fn vector_min_element(vector: Vec4A<f32>) -> f32 {
            vminvq_f32(vector.0)
        }

        #[inline(always)]
        fn vector_floor(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vrndmq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_ceil(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vrndpq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_round(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vrndaq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_trunc(vector: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vrndq_f32(vector.0))
        }

        #[inline(always)]
        fn vector_mul_add(vector: Vec4A<f32>, a: Vec4A<f32>, b: Vec4A<f32>) -> Vec4A<f32> {
            Vector(vfmaq_f32(b.0, vector.0, a.0))
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
            Vector(vsqrtq_f32(vector.0))
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
    fn rem(vector: float32x4_t, rhs: float32x4_t) -> float32x4_t {
        let result = vsubq_f32(vector, vmulq_f32(vrndq_f32(vdivq_f32(vector, rhs)), rhs));

        let inf_mask = vceqq_f32(vabsq_f32(rhs), vdupq_n_f32(f32::INFINITY));
        let zero_mask = vceqq_f32(rhs, vdupq_n_f32(0.0));
        let result = vbslq_f32(
            vreinterpretq_u32_f32(vnegq_f32(vreinterpretq_f32_u32(inf_mask))),
            vector,
            result,
        );

        vbslq_f32(zero_mask, vdupq_n_f32(f32::NAN), result)
    }

    #[inline]
    fn nan_mask(vector: float32x4_t) -> uint32x4_t {
        vmvnq_u32(vceqq_f32(vector, vector))
    }

    #[inline]
    fn finite_mask(vector: float32x4_t) -> uint32x4_t {
        vcltq_f32(vabsq_f32(vector), vdupq_n_f32(f32::INFINITY))
    }

    #[inline]
    fn sign_positive_mask(vector: float32x4_t) -> uint32x4_t {
        vceqq_u32(
            vreinterpretq_u32_f32(vector),
            vreinterpretq_u32_f32(vabsq_f32(vector)),
        )
    }

    #[inline]
    fn sign_negative_mask(vector: float32x4_t) -> uint32x4_t {
        vmvnq_u32(vceqq_u32(
            vreinterpretq_u32_f32(vector),
            vreinterpretq_u32_f32(vabsq_f32(vector)),
        ))
    }

    #[inline]
    fn signum(vector: float32x4_t) -> float32x4_t {
        vreinterpretq_f32_u32(vorrq_u32(
            vorrq_u32(vdupq_n_u32(1f32.to_bits()), nan_mask(vector)),
            vandq_u32(vreinterpretq_u32_f32(vector), vdupq_n_u32((-0.0f32).to_bits())),
        ))
    }

    #[inline]
    fn copysign(vector: float32x4_t, sign: float32x4_t) -> float32x4_t {
        vbslq_f32(vdupq_n_u32((-0.0f32).to_bits()), sign, vector)
    }
}
