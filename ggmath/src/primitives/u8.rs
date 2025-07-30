use super::*;

primitive_aliases! { pub U8 => u8 }

impl Scalar for u8 {
    type Vec2Alignment = Align<2>;
    type Vec3Alignment = Align<4>;
    type Vec4Alignment = Align<4>;

    const NEG_GARBAGE: Option<fn(Self) -> Self> =
        Some(|a| unsafe { a.checked_neg().unwrap_unchecked() });

    const NOT_GARBAGE: Option<fn(Self) -> Self> = Some(|a| !a);

    const ADD_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_add(b).unwrap_unchecked() });

    const SUB_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_sub(b).unwrap_unchecked() });

    const MUL_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_mul(b).unwrap_unchecked() });

    const DIV_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_div(b).unwrap_unchecked() });

    const REM_GARBAGE: Option<fn(Self, Self) -> Self> =
        Some(|a, b| unsafe { a.checked_rem(b).unwrap_unchecked() });

    const BITAND_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a & b);

    const BITOR_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a | b);

    const BITXOR_GARBAGE: Option<fn(Self, Self) -> Self> = Some(|a, b| a ^ b);
}

// impl for all uint types
macro_loop! {
    @for uint in [u8, u16, u32, u64, u128, usize] {
        impl<const N: usize, A: VecAlignment> Vector<N, @uint, A>
        where
            Usize<N>: VecLen,
        {
            /// Returns a vector of the signum of the input vector.
            /// This is equivalent to `if x == 0 { 0 } else { 1 }`.
            pub fn signumt(self) -> Self {
                self.map(|x| if x == 0 { 0 } else { 1 })
            }
        }
    }
}

// impl for all int types
macro_loop! {
    @for int in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize] {
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
            pub fn is_positive(&self) -> Vector<N, bool, A> {
                self.map(|x| x > 0)
            }
            /// Returns a vector of boolean values, where each element is `true` if the corresponding element in the input vector is zero, and `false` otherwise.
            /// This is equivalent to `self == 0`.
            pub fn is_zero(&self) -> Vector<N, bool, A> {
                self.map(|x| x == 0)
            }

            /// Returns a vector of the minimum elements between the two vectors.
            pub fn min(self, other: Vector<N, @int, impl VecAlignment>) -> Self {
                self.map_rhs(other, @int::min)
            }
            /// Returns a vector of the maximum elements between the two vectors.
            pub fn max(self, other: Vector<N, @int, impl VecAlignment>) -> Self {
                self.map_rhs(other, @int::max)
            }

            /// Returns the square of the magnitude of the vector.
            ///
            /// The `mag` method does not exist for ints because it requires a square root.
            #[inline(always)]
            pub fn square_mag(self) -> @int {
                self.map(|x| x * x).sum()
            }
        }
    }
}

// impl for all number types
macro_loop! {
    @for num in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64] {
        impl<const N: usize, A: VecAlignment> Vector<N, @num, A>
        where
            Usize<N>: VecLen,
        {
            #[doc = @("Vector of all `" + @num + "::MIN` values.")]
            pub const MIN: Self = Self::splat(@num::MIN);
            #[doc = @("Vector of all `" + @num + "::MAX` values.")]
            pub const MAX: Self = Self::splat(@num::MAX);

            /// Returns a vector of the elements clamped between the minimum and maximum vectors.
            pub fn clamp(
                self,
                min: Vector<N, @num, impl VecAlignment>,
                max: Vector<N, @num, impl VecAlignment>,
            ) -> Self {
                self.min(max).max(min)
            }

            /// Returns a vector of the absolute difference between the elements and the corresponding elements of the other vector.
            /// This is equivalent to `abs(self - rhs)`, not `abs(self) - abs(rhs)`.
            pub fn abs_diff(self, rhs: Vector<N, @num, impl VecAlignment>) -> Self {
                self.map_rhs(rhs, |a, b| if a > b { a - b } else { b - a })
            }

            macro_loop! {
                @for other_num in [u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64] {
                    @if @other_num != @num {
                        /// Convert the vector to a vector of the given primitive type.
                        /// This uses the `as` keyword to perform the conversion.
                        pub const fn @[as_ @other_num](self) -> Vector<N, @other_num, A> {
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
}
