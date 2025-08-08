use std::mem::transmute_copy;

use super::*;

primitive_aliases! { pub U8 => u8 }

#[cfg(feature = "vector")]
impl Scalar for u8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;

    #[inline(always)]
    fn vec3_not(base: Vec3<Self>) -> Option<Vec3<Self>> {
        let base_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&base) };

        let output_vec4 = base_vec4.map(|x| !x);

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_add(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() + rhs_vec4.x(),
            lhs_vec4.y() + rhs_vec4.y(),
            lhs_vec4.z() + rhs_vec4.z(),
            lhs_vec4.w().overflowing_add(rhs_vec4.w()).0,
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_sub(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() - rhs_vec4.x(),
            lhs_vec4.y() - rhs_vec4.y(),
            lhs_vec4.z() - rhs_vec4.z(),
            lhs_vec4.w().overflowing_sub(rhs_vec4.w()).0,
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_mul(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() * rhs_vec4.x(),
            lhs_vec4.y() * rhs_vec4.y(),
            lhs_vec4.z() * rhs_vec4.z(),
            lhs_vec4.w().overflowing_mul(rhs_vec4.w()).0,
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_bitand(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() & rhs_vec4.x(),
            lhs_vec4.y() & rhs_vec4.y(),
            lhs_vec4.z() & rhs_vec4.z(),
            lhs_vec4.w() & rhs_vec4.w(),
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_bitor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() | rhs_vec4.x(),
            lhs_vec4.y() | rhs_vec4.y(),
            lhs_vec4.z() | rhs_vec4.z(),
            lhs_vec4.w() | rhs_vec4.w(),
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }

    #[inline(always)]
    fn vec3_bitxor(lhs: Vec3<Self>, rhs: Vec3<Self>) -> Option<Vec3<Self>> {
        let lhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&lhs) };
        let rhs_vec4 = unsafe { transmute_copy::<Vec3<Self>, Vec4<Self>>(&rhs) };

        let output_vec4 = vec4!(
            lhs_vec4.x() ^ rhs_vec4.x(),
            lhs_vec4.y() ^ rhs_vec4.y(),
            lhs_vec4.z() ^ rhs_vec4.z(),
            lhs_vec4.w() ^ rhs_vec4.w(),
        );

        Some(unsafe { transmute_copy::<Vec4<Self>, Vec3<Self>>(&output_vec4) })
    }
}

// impl for all uint types
repetitive! {
    @for uint in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @uint, A>
        where
            Usize<N>: VecLen,
        {
            /// Returns a vector of the signum of the input vector.
            /// This is equivalent to `if x == 0 { 0 } else { 1 }`.
            #[inline(always)]
            pub fn signumt(self) -> Self {
                self.map(|x| if x == 0 { 0 } else { 1 })
            }
        }
    }
}

// impl for all int types
repetitive! {
    @for int in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @int, A>
        where
            Usize<N>: VecLen,
        {
            /// Vector of all `0` values.
            pub const ZERO: Self = Self::splat(0);
            /// Vector of all `1` values.
            pub const ONE: Self = Self::splat(1);

            /// Returns a vector of boolean values, where each element is `true` if the corresponding element in the input vector is positive, and `false` otherwise.
            /// This is equivalent to `self > 0`.
            #[inline(always)]
            pub fn is_positive(&self) -> Vector<N, bool, A> {
                self.map(|x| x > 0)
            }
            /// Returns a vector of boolean values, where each element is `true` if the corresponding element in the input vector is zero, and `false` otherwise.
            /// This is equivalent to `self == 0`.
            #[inline(always)]
            pub fn is_zero(&self) -> Vector<N, bool, A> {
                self.map(|x| x == 0)
            }

            /// Returns a vector of the minimum elements between the two vectors.
            #[inline(always)]
            pub fn min(self, other: Vector<N, @int, impl VecAlignment>) -> Self {
                self.map_rhs(other, @int::min)
            }
            /// Returns a vector of the maximum elements between the two vectors.
            #[inline(always)]
            pub fn max(self, other: Vector<N, @int, impl VecAlignment>) -> Self {
                self.map_rhs(other, @int::max)
            }

            /// Returns the square of the magnitude of the vector.
            ///
            /// The `mag` method does not exist for ints because it requires a square root.
            #[inline(always)]
            pub fn mag_sq(self) -> @int {
                self.map(|x| x * x).sum()
            }
            /// Returns the square of the magnitude of the vector.
            ///
            /// The `mag` method does not exist for ints because it requires a square root.
            #[inline(always)]
            #[deprecated(note = "Renamed to `mag_sq`")]
            pub fn square_mag(self) -> @int {
                self.mag_sq()
            }
        }

        #[cfg(feature = "aabb")]
        impl AabbScalar for @int {
            #[inline(always)]
            fn aabb_mul_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec * 2
            }

            #[inline(always)]
            fn aabb_div_vector_by_two<const N: usize, A: VecAlignment>(
                vec: Vector<N, Self, A>,
            ) -> Vector<N, Self, A>
            where
                Usize<N>: VecLen,
            {
                vec / 2
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

// impl for all number types
repetitive! {
    @for num in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @num, A>
        where
            Usize<N>: VecLen,
        {
            #[doc = @str["Vector of all `" num "::MIN` values."]]
            pub const MIN: Self = Self::splat(@num::MIN);
            #[doc = @str["Vector of all `" num "::MAX` values."]]
            pub const MAX: Self = Self::splat(@num::MAX);

            /// Returns a vector of the elements clamped between the minimum and maximum vectors.
            #[inline(always)]
            pub fn clamp(
                self,
                min: Vector<N, @num, impl VecAlignment>,
                max: Vector<N, @num, impl VecAlignment>,
            ) -> Self {
                self.min(max).max(min)
            }

            /// Returns a vector of the absolute difference between the elements and the corresponding elements of the other vector.
            /// This is equivalent to `abs(self - rhs)`, not `abs(self) - abs(rhs)`.
            #[inline(always)]
            pub fn abs_diff(self, rhs: Vector<N, @num, impl VecAlignment>) -> Self {
                self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
            }

            @for other_num in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64] {
                @if other_num != num {
                    /// Convert the vector to a vector of the given primitive type.
                    /// This uses the `as` keyword to perform the conversion.
                    #[inline(always)]
                    pub const fn @['as_ other_num](self) -> Vector<N, @other_num, A> {
                        let mut output = Vector::splat(0 as @other_num);

                        let mut i = 0;
                        while i < N {
                            output.as_array_mut()[i] = self.to_array()[i] as @other_num;
                            i += 1;
                        }

                        output
                    }
                }
            }
        }

    }
}
