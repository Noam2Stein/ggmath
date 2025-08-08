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
        #[cfg(feature = "vector")]
        impl<const N: usize, A: VecAlignment> Vector<N, @float, A>
        where
            Usize<N>: VecLen,
        {
            /// Vector of all `0.0` values.
            pub const ZERO: Self = Self::splat(0.0);
            /// Vector of all `1.0` values.
            pub const ONE: Self = Self::splat(1.0);
            /// Vector of all `-1.0` values.
            pub const NEG_ONE: Self = Self::splat(-1.0);

            /// Vector of all `NaN` values.
            pub const NAN: Self = Self::splat(@float::NAN);
            /// Vector of all `Infinity` values.
            pub const INFINITY: Self = Self::splat(@float::INFINITY);
            /// Vector of all `-Infinity` values.
            pub const NEG_INFINITY: Self = Self::splat(@float::NEG_INFINITY);

            /// Maps the vector to a vector of booleans, where each element is `true` if the element is positive, and `false` otherwise.
            /// This returns FALSE if the element is zero.
            /// To check the binary sign, use `is_bin_positive`.
            #[inline(always)]
            pub fn is_positive(&self) -> Vector<N, bool, A> {
                self.map(|x| x > 0.0)
            }
            /// Maps the vector to a vector of booleans, where each element is `true` if the element is negative, and `false` otherwise.
            /// This returns FALSE if the element is zero.
            /// To check the binary sign, use `is_bin_negative`.
            #[inline(always)]
            pub fn is_negative(&self) -> Vector<N, bool, A> {
                self.map(|x| x < 0.0)
            }
            /// Maps the vector to a vector of booleans, where each element is `true` if the element is zero, and `false` otherwise.
            #[inline(always)]
            pub fn is_zero(&self) -> Vector<N, bool, A> {
                self.map(|x| x == 0.0)
            }
            /// Maps the vector to a vector of booleans,
            /// where each element is `true` if the element's binary sign is positive, and `false` otherwise.
            ///
            /// This returns true if the element is `+0.0`.
            /// To check if the element is more than zero, use `is_positive`.
            #[inline(always)]
            pub fn is_bin_positive(&self) -> Vector<N, bool, A> {
                self.map(@float::is_sign_positive)
            }
            /// Maps the vector to a vector of booleans,
            /// where each element is `true` if the element's binary sign is negative, and `false` otherwise.
            ///
            /// This returns true if the element is `-0.0`.
            /// To check if the element is less than zero, use `is_negative`.
            #[inline(always)]
            pub fn is_bin_negative(&self) -> Vector<N, bool, A> {
                self.map(@float::is_sign_negative)
            }

            /// Maps the vector to a vector of the absolute values of the elements.
            #[inline(always)]
            pub fn abs(self) -> Self {
                self.map(@float::abs)
            }
            /// Maps the vector to a vector of the negative absolute values of the elements.
            #[inline(always)]
            pub fn neg_abs(self) -> Self {
                self.map(|x| -x.abs())
            }

            /// Maps the vector to a vector of the signum values of the elements.
            /// This returns `0.0` if the element is zero.
            ///
            /// For binary sign signum, use `bin_signum`.
            #[inline(always)]
            pub fn signumt(self) -> Self {
                self.map(|x| {
                    if x > 0.0 {
                        1.0
                    } else if x < 0.0 {
                        -1.0
                    } else {
                        0.0
                    }
                })
            }
            /// Maps the vector to a vector of the signum values of the elements.
            /// This returns `1.0` for `+0.0` and `-1.0` for `-0.0`.
            ///
            /// For signum that returns `0.0` for zero, use `signumt`.
            #[inline(always)]
            pub fn bin_signum(self) -> Self {
                self.map(@float::signum)
            }

            /// Maps the vector to a vector of the minimum of the elements and the corresponding elements of the other vector.
            #[inline(always)]
            pub fn min(self, other: Vector<N, @float, impl VecAlignment>) -> Self {
                self.map_rhs(other, |a, b| if a < b { a } else { b })
            }
            /// Maps the vector to a vector of the maximum of the elements and the corresponding elements of the other vector.
            #[inline(always)]
            pub fn max(self, other: Vector<N, @float, impl VecAlignment>) -> Self {
                self.map_rhs(other, |a, b| if a > b { a } else { b })
            }

            /// Maps the vector to a vector of the rounded values of the elements.
            #[inline(always)]
            pub fn round(self) -> Self {
                self.map(@float::round)
            }
            /// Maps the vector to a vector of the floor values of the elements.
            #[inline(always)]
            pub fn floor(self) -> Self {
                self.map(@float::floor)
            }
            /// Maps the vector to a vector of the ceiling values of the elements.
            #[inline(always)]
            pub fn ceil(self) -> Self {
                self.map(@float::ceil)
            }
            /// Maps the vector to a vector of the truncated values of the elements.
            #[inline(always)]
            pub fn trunc(self) -> Self {
                self.map(@float::trunc)
            }
            /// Maps the vector to a vector of the "anti truncated" values of the elements.
            /// This ceils positive values and floors negative values, opposite to `trunc`.
            #[inline(always)]
            pub fn atrunc(self) -> Self {
                self.map(|x| {
                    if x.is_sign_positive() {
                        x.ceil()
                    } else {
                        x.floor()
                    }
                })
            }

            /// Maps the vector to a vector of the sine values of the elements.
            #[inline(always)]
            pub fn sin(self) -> Self {
                self.map(@float::sin)
            }
            /// Maps the vector to a vector of the cosine values of the elements.
            #[inline(always)]
            pub fn cos(self) -> Self {
                self.map(@float::cos)
            }
            /// Maps the vector to a vector of the tangent values of the elements.
            #[inline(always)]
            pub fn tan(self) -> Self {
                self.map(@float::tan)
            }
            /// Maps the vector to a vector of the arcsine values of the elements.
            #[inline(always)]
            pub fn asin(self) -> Self {
                self.map(@float::asin)
            }
            /// Maps the vector to a vector of the arccosine values of the elements.
            #[inline(always)]
            pub fn acos(self) -> Self {
                self.map(@float::acos)
            }
            /// Maps the vector to a vector of the arctangent values of the elements.
            #[inline(always)]
            pub fn atan(self) -> Self {
                self.map(@float::atan)
            }
            /// Maps the vector to a vector of the hyperbolic sine values of the elements.
            #[inline(always)]
            pub fn sinh(self) -> Self {
                self.map(@float::sinh)
            }
            /// Maps the vector to a vector of the hyperbolic cosine values of the elements.
            #[inline(always)]
            pub fn cosh(self) -> Self {
                self.map(@float::cosh)
            }
            /// Maps the vector to a vector of the hyperbolic tangent values of the elements.
            #[inline(always)]
            pub fn tanh(self) -> Self {
                self.map(@float::tanh)
            }
            /// Maps the vector to a vector of the hyperbolic arcsine values of the elements.
            #[inline(always)]
            pub fn asinh(self) -> Self {
                self.map(@float::asinh)
            }
            /// Maps the vector to a vector of the hyperbolic arccosine values of the elements.
            #[inline(always)]
            pub fn acosh(self) -> Self {
                self.map(@float::acosh)
            }
            /// Maps the vector to a vector of the hyperbolic arctangent values of the elements.
            #[inline(always)]
            pub fn atanh(self) -> Self {
                self.map(@float::atanh)
            }

            /// Returns the magnitude of the vector.
            #[inline(always)]
            pub fn mag(self) -> @float {
                self.map(|x| x * x).sum().sqrt()
            }
            /// Returns the square of the magnitude of the vector.
            ///
            /// This is faster than `mag` because it doesn't require performing a square root.
            /// So when doing things like comparing magnitudes, use this instead of `mag`.
            #[inline(always)]
            pub fn mag_sq(self) -> @float {
                self.map(|x| x * x).sum()
            }
            /// Returns the square of the magnitude of the vector.
            ///
            /// This is faster than `mag` because it doesn't require performing a square root.
            /// So when doing things like comparing magnitudes, use this instead of `mag`.
            #[inline(always)]
            #[deprecated(note = "Renamed to `mag_sq`")]
            pub fn square_mag(self) -> @float {
                self.mag_sq()
            }

            /// Returns a vector with the same direction as the original, but with a magnitude of 1.
            ///
            /// Calling this on a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn normalize(self) -> Self {
                self / self.mag()
            }

            /// Returns a vector with the same direction as the original, but with the given magnitude.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn with_mag(self, mag: @float) -> Self {
                self / self.mag() * mag
            }
            /// Returns a vector with the same direction as the original, but with the given square magnitude.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn with_mag_sq(self, mag_sq: @float) -> Self {
                self.with_mag(mag_sq.sqrt())
            }
            /// Returns a vector with the same direction as the original, but with the given square magnitude.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[deprecated(note = "Renamed to `with_mag_sq`")]
            pub fn with_square_mag(self, square_mag: @float) -> Self {
                self.with_mag(square_mag.sqrt())
            }

            /// Returns a vector with the same direction as the original, but with the magnitude at least the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn with_min_mag(self, min_mag: @float) -> Self {
                if self.mag_sq() < min_mag * min_mag {
                    self.with_mag(min_mag)
                } else {
                    self
                }
            }
            /// Returns a vector with the same direction as the original, but with the magnitude at most the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn with_max_mag(self, max_mag: @float) -> Self {
                if self.mag_sq() > max_mag * max_mag {
                    self.with_mag(max_mag)
                } else {
                    self
                }
            }
            /// Returns a vector with the same direction as the original, but with the magnitude clamped between the given values.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn clamp_mag(self, min_mag: @float, max_mag: @float) -> Self {
                if self.mag_sq() < min_mag * min_mag {
                    self.with_mag(min_mag)
                } else if self.mag_sq() > max_mag * max_mag {
                    self.with_mag(max_mag)
                } else {
                    self
                }
            }

            /// Returns a vector with the same direction as the original, but with the square magnitude at least the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn with_min_mag_sq(self, min_mag_sq: @float) -> Self {
                if self.mag_sq() < min_mag_sq {
                    self.with_mag_sq(min_mag_sq)
                } else {
                    self
                }
            }
            /// Returns a vector with the same direction as the original, but with the square magnitude at most the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn with_max_mag_sq(self, max_mag_sq: @float) -> Self {
                if self.mag_sq() > max_mag_sq {
                    self.with_mag_sq(max_mag_sq)
                } else {
                    self
                }
            }
            /// Returns a vector with the same direction as the original, but with the square magnitude clamped between the given values.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            pub fn clamp_mag_sq(self, min_mag_sq: @float, max_mag_sq: @float) -> Self {
                self.with_min_mag_sq(min_mag_sq).with_max_mag_sq(max_mag_sq)
            }
            /// Returns a vector with the same direction as the original, but with the square magnitude at least the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[deprecated(note = "Renamed to `with_min_mag_sq`")]
            pub fn with_min_square_mag(self, min_square_mag: @float) -> Self {
                self.with_min_mag_sq(min_square_mag)
            }
            /// Returns a vector with the same direction as the original, but with the square magnitude at most the given value.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[deprecated(note = "Renamed to `with_max_mag_sq`")]
            pub fn with_max_square_mag(self, max_square_mag: @float) -> Self {
                self.with_max_mag_sq(max_square_mag)
            }
            /// Returns a vector with the same direction as the original, but with the square magnitude clamped between the given values.
            ///
            /// Calling this with a zero vector will result in a NaN vector.
            #[inline(always)]
            #[deprecated(note = "Renamed to `clamp_mag_sq`")]
            pub fn clamp_square_mag(self, min_square_mag: @float, max_square_mag: @float) -> Self {
                self.clamp_mag_sq(min_square_mag, max_square_mag)
            }

            /// Performs linear interpolation between `self` and `other` based on the value `t`.
            ///
            /// When `t` is `0.0`, the result will be equal to `self`.
            /// When `t` is `1.0`, the result will be equal to `other`.
            /// `t` is clamped to the range `[0, 1]`.
            ///
            /// For unclamped interpolation, use `lerp_unclamped`.
            #[inline(always)]
            pub fn lerp(self, other: Vector<N, @float, impl VecAlignment>, t: @float) -> Self {
                self.lerp_unclamped(other, t.clamp(0.0, 1.0))
            }
            /// Performs linear interpolation between `self` and `other` based on the value `t`.
            ///
            /// When `t` is `0.0`, the result will be equal to `self`.
            /// When `t` is `1.0`, the result will be equal to `other`.
            /// When `t` is outside of range `[0, 1]`, the result is linearly extrapolated.
            #[inline(always)]
            pub fn lerp_unclamped(self, other: Vector<N, @float, impl VecAlignment>, t: @float) -> Self {
                self * (1.0 - t) + other * t
            }
        }

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
