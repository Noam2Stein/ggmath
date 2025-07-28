use super::*;

primitive_aliases! { pub F => f32 }

impl Scalar for f32 {
    type Vec2Alignment = Align<8>;
    type Vec3Alignment = Align<16>;
    type Vec4Alignment = Align<16>;
}

// impl for all float types
macro_loop! {
    @for float in [f32, f64] {
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
            pub fn is_positive(&self) -> Vector<N, bool, A> {
                self.map(|x| x > 0.0)
            }
            /// Maps the vector to a vector of booleans, where each element is `true` if the element is negative, and `false` otherwise.
            /// This returns FALSE if the element is zero.
            /// To check the binary sign, use `is_bin_negative`.
            pub fn is_negative(&self) -> Vector<N, bool, A> {
                self.map(|x| x < 0.0)
            }
            /// Maps the vector to a vector of booleans, where each element is `true` if the element is zero, and `false` otherwise.
            pub fn is_zero(&self) -> Vector<N, bool, A> {
                self.map(|x| x == 0.0)
            }
            /// Maps the vector to a vector of booleans,
            /// where each element is `true` if the element's binary sign is positive, and `false` otherwise.
            ///
            /// This returns true if the element is `+0.0`.
            /// To check if the element is more than zero, use `is_positive`.
            pub fn is_bin_positive(&self) -> Vector<N, bool, A> {
                self.map(@float::is_sign_positive)
            }
            /// Maps the vector to a vector of booleans,
            /// where each element is `true` if the element's binary sign is negative, and `false` otherwise.
            ///
            /// This returns true if the element is `-0.0`.
            /// To check if the element is less than zero, use `is_negative`.
            pub fn is_bin_negative(&self) -> Vector<N, bool, A> {
                self.map(@float::is_sign_negative)
            }

            /// Maps the vector to a vector of the absolute values of the elements.
            pub fn abs(self) -> Self {
                self.map(@float::abs)
            }
            /// Maps the vector to a vector of the negative absolute values of the elements.
            pub fn neg_abs(self) -> Self {
                self.map(|x| -x.abs())
            }

            /// Maps the vector to a vector of the signum values of the elements.
            /// This returns `0.0` if the element is zero.
            ///
            /// For binary sign signum, use `bin_signum`.
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
            pub fn bin_signum(self) -> Self {
                self.map(@float::signum)
            }

            /// Maps the vector to a vector of the minimum of the elements and the corresponding elements of the other vector.
            pub fn min(self, other: Vector<N, @float, impl VecAlignment>) -> Self {
                self.map_rhs(other, |a, b| if a < b { a } else { b })
            }
            /// Maps the vector to a vector of the maximum of the elements and the corresponding elements of the other vector.
            pub fn max(self, other: Vector<N, @float, impl VecAlignment>) -> Self {
                self.map_rhs(other, |a, b| if a > b { a } else { b })
            }

            /// Maps the vector to a vector of the rounded values of the elements.
            pub fn round(self) -> Self {
                self.map(@float::round)
            }
            /// Maps the vector to a vector of the floor values of the elements.
            pub fn floor(self) -> Self {
                self.map(@float::floor)
            }
            /// Maps the vector to a vector of the ceiling values of the elements.
            pub fn ceil(self) -> Self {
                self.map(@float::ceil)
            }
            /// Maps the vector to a vector of the truncated values of the elements.
            pub fn trunc(self) -> Self {
                self.map(@float::trunc)
            }
            /// Maps the vector to a vector of the "anti truncated" values of the elements.
            /// This ceils positive values and floors negative values, opposite to `trunc`.
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
            pub fn sin(self) -> Self {
                self.map(@float::sin)
            }
            /// Maps the vector to a vector of the cosine values of the elements.
            pub fn cos(self) -> Self {
                self.map(@float::cos)
            }
            /// Maps the vector to a vector of the tangent values of the elements.
            pub fn tan(self) -> Self {
                self.map(@float::tan)
            }
            /// Maps the vector to a vector of the arcsine values of the elements.
            pub fn asin(self) -> Self {
                self.map(@float::asin)
            }
            /// Maps the vector to a vector of the arccosine values of the elements.
            pub fn acos(self) -> Self {
                self.map(@float::acos)
            }
            /// Maps the vector to a vector of the arctangent values of the elements.
            pub fn atan(self) -> Self {
                self.map(@float::atan)
            }
            /// Maps the vector to a vector of the hyperbolic sine values of the elements.
            pub fn sinh(self) -> Self {
                self.map(@float::sinh)
            }
            /// Maps the vector to a vector of the hyperbolic cosine values of the elements.
            pub fn cosh(self) -> Self {
                self.map(@float::cosh)
            }
            /// Maps the vector to a vector of the hyperbolic tangent values of the elements.
            pub fn tanh(self) -> Self {
                self.map(@float::tanh)
            }
            /// Maps the vector to a vector of the hyperbolic arcsine values of the elements.
            pub fn asinh(self) -> Self {
                self.map(@float::asinh)
            }
            /// Maps the vector to a vector of the hyperbolic arccosine values of the elements.
            pub fn acosh(self) -> Self {
                self.map(@float::acosh)
            }
            /// Maps the vector to a vector of the hyperbolic arctangent values of the elements.
            pub fn atanh(self) -> Self {
                self.map(@float::atanh)
            }
        }
    }
}
