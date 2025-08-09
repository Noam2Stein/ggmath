use std::mem::transmute_copy;

use super::*;

primitive_aliases! { pub F => f32 }

#[cfg(feature = "vector")]
impl Scalar for f32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;

    #[inline(always)]
    fn vec3_neg(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let base_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&base) };

        let output_vec4 = -base_vec4;

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_add(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = lhs_vec4 + rhs_vec4;

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_sub(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = lhs_vec4 - rhs_vec4;

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_mul(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = lhs_vec4 * rhs_vec4;

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_div(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = lhs_vec4 / rhs_vec4;

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_rem(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = lhs_vec4 % rhs_vec4;

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }
}

// impl for all float types
repetitive! {
    @for float in ['f32, 'f64] {


        #[cfg(feature = "aabb")]
        impl AabbScalar for @float {
            #[inline(always)]
            fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec * 2.0
            }

            #[inline(always)]
            fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec / 2.0
            }

            #[inline(always)]
            fn aabb_vector_abs_diff<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                rhs: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.abs_diff(rhs)
            }

            #[inline(always)]
            fn aabb_vector_min<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.min(other)
            }

            #[inline(always)]
            fn aabb_vector_max<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
                other: Vector<N, Self, impl VecAlignment>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec.max(other)
            }
        }
    }
}
