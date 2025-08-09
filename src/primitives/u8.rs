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
            /// Returns a vector of the absolute difference between the elements and the corresponding elements of the other vector.
            /// This is equivalent to `abs(self - rhs)`, not `abs(self) - abs(rhs)`.
            #[inline(always)]
            pub const fn abs_diff(self, rhs: Vector<N, @uint, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0 as @uint);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) > rhs.index(i) {
                        self.index(i) - rhs.index(i)
                    } else {
                        rhs.index(i) - self.index(i)
                    };
                    i += 1;
                }

                output
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

            /// Returns a vector of minimum values between the two vector's elements.
            ///
            /// Basically `[self.x.min(other.x), self.y.min(other.y), ...]`.
            #[inline(always)]
            pub const fn min(self, other: Vector<N, @int, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) < other.index(i) {
                        self.index(i)
                    } else {
                        other.index(i)
                    };
                    i += 1;
                }

                output
            }

            /// Returns a vector of maximum values between the two vector's elements.
            ///
            /// Basically `[self.x.max(other.x), self.y.max(other.y), ...]`.
            #[inline(always)]
            pub const fn max(self, other: Vector<N, @int, impl VecAlignment>) -> Self {
                let mut output = Vector::splat(0);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) > other.index(i) {
                        self.index(i)
                    } else {
                        other.index(i)
                    };
                    i += 1;
                }

                output
            }

            /// Returns the square of the magnitude of the vector.
            ///
            /// The `mag` method does not exist for ints because it could be a non whole number.
            #[inline(always)]
            pub const fn mag_sq(self) -> @int {
                let mut output = 0;

                let mut i = 0;
                while i < N {
                    output += self.index(i) * self.index(i);
                    i += 1;
                }

                output
            }

            // Deprecated

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
            pub const fn clamp(
                self,
                min: Vector<N, @num, impl VecAlignment>,
                max: Vector<N, @num, impl VecAlignment>,
            ) -> Self {
                self.min(max).max(min)
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

        impl Zero for @num {
            const ZERO: Self = 0 as @num;
        }
        impl One for @num {
            const ONE: Self = 1 as @num;
        }
    }
}

repetitive! {
    @for [prim, prim_is_int, prim_is_float, prim_is_signed] in [
        ['u8, true, false, false],
        ['u16, true, false, false],
        ['u32, true, false, false],
        ['u64, true, false, false],
        ['u128, true, false, false],
        ['usize, true, false, false],
        ['i8, true, false, true],
        ['i16, true, false, true],
        ['i32, true, false, true],
        ['i64, true, false, true],
        ['i128, true, false, true],
        ['isize, true, false, true],
        ['f32, false, true, true],
        ['f64, false, true, true],
    ] {
        @let prim_is_sint = prim_is_int && prim_is_signed;
        @let prim_is_uint = prim_is_int && !prim_is_signed;

        // Sign

        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @prim, A>
        where
            Usize<N>: VecLen,
        {
            /// Returns a vector of bools, where each element is `true` is the input element is positive.
            ///
            /// Basically `[self.x > 0, self.y > 0, ...]`.
            #[inline(always)]
            pub const fn positive_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) > 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is zero.
            ///
            /// Basically `[self.x == 0, self.y == 0, ...]`.
            #[inline(always)]
            pub const fn zero_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) == 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is negative.
            ///
            /// Basically `[self.x < 0, self.y < 0, ...]`.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            pub const fn negative_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) < 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element is non-negative.
            ///
            /// Basically `[self.x >= 0, self.y >= 0, ...]`.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            pub const fn bin_positive_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i) >= 0 as @prim;
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element's binary sign is positive.
            ///
            /// This returns true if the element is `+0.0`, and false if the element is `-0.0`.
            ///
            /// Basically `[self.x.is_sign_positive(), self.y.is_sign_positive(), ...]`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn bin_positive_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).is_sign_positive();
                    i += 1;
                }

                output
            }

            /// Returns a vector of bools, where each element is `true` is the input element's binary sign is negative.
            ///
            /// This returns true if the element is `-0.0`, and false if the element is `+0.0`.
            ///
            /// Basically `[self.x.is_sign_negative(), self.y.is_sign_negative(), ...]`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn bin_negative_mask(self) -> Vector<N, bool, A> {
                let mut output = Vector::splat(false);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).is_sign_negative();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the absolute values of the elements.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            pub const fn abs(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).abs();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `0` if the number is `0`,
            /// - `1` if the number is positive.
            #[inline(always)]
            #[cfg(@prim_is_uint)]
            pub const fn signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) == 0 { 0 } else { 1 };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `0` if the number is `0`,
            /// - `1` if the number is positive,
            /// - `-1` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            pub const fn signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).signum();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `+0.0` if the number is `+0.0`,
            /// - `-0.0` if the number is `-0.0`,
            /// - `1.0` if the number is positive,
            /// - `-1.0` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn tri_signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i).is_sign_positive() {
                        if self.index(i) == 0.0 {
                            0.0
                        } else {
                            1.0
                        }
                    } else {
                        if self.index(i) == 0.0 {
                            -0.0
                        } else {
                            -1.0
                        }
                    };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `1` if the number is positive or zero,
            /// - `-1` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            pub const fn bin_signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = if self.index(i) >= 0 { 1 } else { -1 };
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `1.0` if the number is positive or `+0.0`,
            /// - `-1.0` if the number is negative or `-0.0`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            pub const fn bin_signum(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = self.index(i).signum();
                    i += 1;
                }

                output
            }

            // Deprecated

            /// Returns a vector of bools, where each element is `true` is the input element is positive.
            ///
            /// Basically `[self.x > 0, self.y > 0, ...]`.
            #[inline(always)]
            #[deprecated(note = "Renamed to `positive_mask`")]
            pub const fn is_positive(self) -> Vector<N, bool, A> {
                self.positive_mask()
            }

            /// Returns a vector of bools, where each element is `true` is the input element is zero.
            ///
            /// Basically `[self.x == 0, self.y == 0, ...]`.
            #[inline(always)]
            #[deprecated(note = "Renamed to `zero_mask`")]
            pub const fn is_zero(self) -> Vector<N, bool, A> {
                self.zero_mask()
            }

            /// Returns a vector of bools, where each element is `true` is the input element is negative.
            ///
            /// Basically `[self.x < 0, self.y < 0, ...]`.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            #[deprecated(note = "Renamed to `negative_mask`")]
            pub const fn is_negative(self) -> Vector<N, bool, A> {
                self.negative_mask()
            }

            /// Returns a vector of bools, where each element is `true` is the input element is non-negative.
            ///
            /// Basically `[self.x >= 0, self.y >= 0, ...]`.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            #[deprecated(note = "Renamed to `bin_positive_mask`")]
            pub const fn is_bin_positive(self) -> Vector<N, bool, A> {
                self.bin_positive_mask()
            }

            /// Returns a vector of bools, where each element is `true` is the input element's binary sign is positive.
            ///
            /// This returns true if the element is `+0.0`, and false if the element is `-0.0`.
            ///
            /// Basically `[self.x.is_sign_positive(), self.y.is_sign_positive(), ...]`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `bin_positive_mask`")]
            pub const fn is_bin_positive(self) -> Vector<N, bool, A> {
                self.bin_positive_mask()
            }

            /// Returns a vector of bools, where each element is `true` is the input element's binary sign is negative.
            ///
            /// This returns true if the element is `-0.0`, and false if the element is `+0.0`.
            ///
            /// Basically `[self.x.is_sign_negative(), self.y.is_sign_negative(), ...]`.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `bin_negative_mask`")]
            pub const fn is_bin_negative(self) -> Vector<N, bool, A> {
                self.bin_negative_mask()
            }

            /// Returns a vector of the negative absolute values of the elements.
            #[inline(always)]
            #[cfg(@prim_is_signed)]
            #[deprecated(note = "Use `-self.abs()` instead")]
            pub const fn neg_abs(self) -> Self {
                let mut output = Vector::splat(0 as @prim);

                let mut i = 0;
                while i < N {
                    *output.index_mut(i) = -self.index(i).abs();
                    i += 1;
                }

                output
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `0` if the number is `0`,
            /// - `1` if the number is positive.
            #[inline(always)]
            #[cfg(@prim_is_uint)]
            #[deprecated(note = "Renamed to `signum`")]
            pub const fn signumt(self) -> Self {
                self.signum()
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `0` if the number is `0`,
            /// - `1` if the number is positive,
            /// - `-1` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_sint)]
            #[deprecated(note = "Renamed to `signum`")]
            pub const fn signumt(self) -> Self {
                self.signum()
            }

            /// Returns a vector of the signum of the elements.
            ///
            /// - `+0.0` if the number is `+0.0`,
            /// - `-0.0` if the number is `-0.0`,
            /// - `1.0` if the number is positive,
            /// - `-1.0` if the number is negative.
            #[inline(always)]
            #[cfg(@prim_is_float)]
            #[deprecated(note = "Renamed to `tri_signum`")]
            pub const fn signumt(self) -> Self {
                self.tri_signum()
            }
        }
    }
}
