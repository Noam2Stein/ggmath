use std::ops::*;

use super::*;

repetitive! {
    // Unary Ops

    @for [op_trait, op_fn] in [
        ['Neg, 'neg],
        ['Not, 'not],
    ] {
        impl<const N: usize, T: Scalar + @op_trait<Output: Scalar>, A: VecAlignment> @op_trait
            for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            #[inline(always)]
            fn @op_fn(self) -> Vector<N, <T as @op_trait>::Output, A> {
                T::@['vec_ op_fn](self)
            }
        }
    }

    // Binary Ops, Assign Ops

    @for [op_trait, op_fn] in [
        ['Add, 'add],
        ['Sub, 'sub],
        ['Mul, 'mul],
        ['Div, 'div],
        ['Rem, 'rem],
        ['BitAnd, 'bitand],
        ['BitOr, 'bitor],
        ['BitXor, 'bitxor],
        ['Shl, 'shl],
        ['Shr, 'shr],
    ] {
        // Binary Ops

        impl<
            const N: usize,
            T: Scalar + @op_trait<T2, Output: Scalar>,
            A: VecAlignment,
            T2: Scalar,
            A2: VecAlignment,
        > @op_trait<Vector<N, T2, A2>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = Vector<N, T::Output, A>;

            #[inline(always)]
            fn @op_fn(self, rhs: Vector<N, T2, A2>) -> Vector<N, <T as @op_trait<T2>>::Output, A> {
                T::@['vec_ op_fn](self, rhs)
            }
        }

        // Assign Ops

        @let op_assign_trait = @[op_trait 'Assign];
        @let op_assign_fn = @[op_fn '_assign];

        impl<const N: usize, T: Scalar + @op_assign_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        @op_assign_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn @op_assign_fn(&mut self, rhs: Vector<N, TRhs, ARhs>) {
                for i in 0..N {
                    self[i].@op_assign_fn(rhs[i]);
                }
            }
        }
    }

    // Scalar Ops

    @for [op_trait, op_fn] in [
        ['Mul, 'mul],
        ['Div, 'div],
        ['Rem, 'rem],
    ] {
        impl<
            const N: usize,
            T: Scalar + @op_trait<Rhs, Output: Scalar>,
            A: VecAlignment,
            Rhs: Scalar,
        > @op_trait<Rhs> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            type Output = <Self as @op_trait<Vector<N, Rhs, A>>>::Output;

            #[inline(always)]
            fn @op_fn(self, rhs: Rhs) -> Self::Output {
                self.@op_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }

        @let op_assign_trait = @[op_trait 'Assign];
        @let op_assign_fn = @[op_fn '_assign];

        impl<const N: usize, T: Scalar + @op_assign_trait<Rhs>, A: VecAlignment, Rhs: Scalar>
            @op_assign_trait<Rhs> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {
            #[inline(always)]
            fn @op_assign_fn(&mut self, rhs: Rhs) {
                self.@op_assign_fn(Vector::<N, Rhs, A>::splat(rhs))
            }
        }
    }
}

// Operator Dependent Functions

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns the sum of all components.
    ///
    /// This uses the `Add` trait to add up the components.
    #[inline(always)]
    pub fn sum(self) -> T
    where
        T: Add<Output = T>,
    {
        self.fold(|a, b| a + b)
    }

    /// Returns the dot product of two vectors.
    ///
    /// This uses the precise trait bounds of `Add` and `Mul` traits to calculate the dot product.
    #[inline(always)]
    pub fn dot(self, other: Vector<N, T, impl VecAlignment>) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        (self * other).sum()
    }

    /// Returns a vector of the absolute difference between the elements and the corresponding elements of the other vector.
    /// Basically `abs(self - other)`.
    #[inline(always)]
    pub fn abs_diff(self, other: Vector<N, T, impl VecAlignment>) -> Vector<N, T, A>
    where
        T: PartialOrd + Sub<Output = T>,
    {
        T::vec_abs_diff(self, other)
    }
}

impl<T: Scalar, A: VecAlignment> Vector<3, T, A> {
    /// Returns the cross product of two vectors.
    ///
    /// This uses the precise trait bounds of `Mul` and `Sub` traits to calculate the cross product.
    #[inline(always)]
    pub fn cross(self, other: Self) -> Self
    where
        T: Mul<Output = T> + Sub<Output = T>,
    {
        (self.zxy() * other - self * other.zxy()).zxy()
    }
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    /// Returns the vector with the padding set to the given value, if there is padding.
    ///
    /// This is not useful for most scenarios because the padding value is usually garbage and is not meant to be read.
    /// This is used to test edge cases for `Vec3` SIMD operators that could break upon padding overflow.
    #[inline(always)]
    pub const fn with_padding(mut self, padding: T) -> Self {
        let mut i = N;
        while i < size_of::<Self>() / size_of::<T>() {
            unsafe {
                *self.as_mut_ptr().add(i).as_mut().unwrap_unchecked() = padding;
            };

            i += 1;
        }

        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    repetitive! {
        @for
            N in 2..=4,
            T in ['u8, 'u16, 'u32, 'u64, 'u128, 'usize, 'i8, 'i16, 'i32, 'i64, 'i128, 'isize, 'f32, 'f64, 'bool],
            A in ['VecAligned, 'VecPacked],
            A2 in ['VecAligned, 'VecPacked],
        {
            @let Vec = @{ Vector::<@N, @T, @A> };
            @let Vec2 = @{ Vector::<@N, @T, @A2> };

            @let t = match T {
                'u8 => 'u8,
                'u16 => 'u16,
                'u32 => 'u,
                'u64 => 'u64,
                'u128 => 'u128,
                'usize => 'usize,
                'i8 => 'i8,
                'i16 => 'i16,
                'i32 => 'i,
                'i64 => 'i64,
                'i128 => 'i128,
                'isize => 'isize,
                'f32 => 'f,
                'f64 => 'd,
                'bool => 'b,
            };

            @let a = match A {
                'VecAligned => 'a,
                'VecPacked => 'p,
            };
            @let a2 = match A2 {
                'VecAligned => 'a,
                'VecPacked => 'p,
            };

            @let tvecna = @[t 'vec N a];

            @let t_is_int = match T {
                'u8 => true,
                'u16 => true,
                'u32 => true,
                'u64 => true,
                'u128 => true,
                'usize => true,
                'i8 => true,
                'i16 => true,
                'i32 => true,
                'i64 => true,
                'i128 => true,
                'isize => true,
                'f32 => false,
                'f64 => false,
                'bool => false,
            };

            @let t_is_sint = match T {
                'u8 => false,
                'u16 => false,
                'u32 => false,
                'u64 => false,
                'u128 => false,
                'usize => false,
                'i8 => true,
                'i16 => true,
                'i32 => true,
                'i64 => true,
                'i128 => true,
                'isize => true,
                'f32 => false,
                'f64 => false,
                'bool => false,
            };

            @let t_is_float = match T {
                'u8 => false,
                'u16 => false,
                'u32 => false,
                'u64 => false,
                'u128 => false,
                'usize => false,
                'i8 => false,
                'i16 => false,
                'i32 => false,
                'i64 => false,
                'i128 => false,
                'isize => false,
                'f32 => true,
                'f64 => true,
                'bool => false,
            };

            #[test]
            fn @['test_ tvecna '_ a2 '_ops]() {
                @if T == 'bool {
                    assert_eq!(
                        !@Vec::splat(false).with_y(true),
                        @Vec::splat(true).with_y(false),
                        "not operator",
                    );

                    assert_eq!(
                        @Vec::splat(false).with_y(true) & @Vec::splat(true),
                        @Vec::splat(false).with_y(true),
                        "and operator",
                    );
                    assert_eq!(
                        @Vec::splat(true).with_x(false) & @Vec::splat(false).with_y(true),
                        @Vec::splat(false).with_y(true),
                        "and operator",
                    );

                    assert_eq!(
                        @Vec::splat(false) | @Vec::splat(true).with_y(false),
                        @Vec::splat(true).with_y(false),
                        "or operator",
                    );
                    assert_eq!(
                        @Vec::splat(false).with_y(true) | @Vec::splat(true).with_x(false).with_y(false),
                        @Vec::splat(true).with_x(false),
                        "or operator",
                    );

                    assert_eq!(
                        @Vec::splat(false).with_y(true) ^ @Vec::splat(true).with_x(false).with_y(false),
                        @Vec::splat(true).with_x(false),
                        "xor operator",
                    );
                }

                @if t_is_int {
                    assert_eq!(
                        @Vec::splat(1).with_y(2).with_padding(@T::MAX) + @Vec::splat(2).with_padding(1),
                        @Vec::splat(3).with_y(4),
                        "add operator",
                    );
                    assert_eq!(
                        @Vec::splat(2).with_y(3).with_padding(@T::MIN) - @Vec::splat(1).with_padding(1),
                        @Vec::splat(1).with_y(2),
                        "sub operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4).with_padding(@T::MAX) * @Vec::splat(4).with_padding(2),
                        @Vec::splat(12).with_y(16),
                        "mul operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4).with_padding(7) / @Vec::splat(2).with_padding(0),
                        @Vec::splat(1).with_y(2),
                        "div operator (padding is zero)",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4).with_padding(7) % @Vec::splat(2).with_padding(0),
                        @Vec::splat(1).with_y(0),
                        "rem operator (padding is zero)",
                    );

                    assert_eq!(
                        !@Vec::splat(3).with_y(4),
                        @Vec::splat(!3).with_y(!4),
                        "not operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4) & @Vec::splat(2).with_y(0),
                        @Vec::splat(3 & 2).with_y(4 & 0),
                        "and operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4) | @Vec::splat(2).with_y(0),
                        @Vec::splat(3 | 2).with_y(4 | 0),
                        "or operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4) ^ @Vec::splat(2).with_y(0),
                        @Vec::splat(3 ^ 2).with_y(4 ^ 0),
                        "xor operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4) << @Vec::splat(2).with_y(0),
                        @Vec::splat(3 << 2).with_y(4 << 0),
                        "shl operator",
                    );
                    assert_eq!(
                        @Vec::splat(3).with_y(4) >> @Vec::splat(2).with_y(0),
                        @Vec::splat(3 >> 2).with_y(4 >> 0),
                        "shr operator",
                    );

                    @match N {
                        2 => {
                            assert_eq!(
                                @Vec::from_array([1, 2]).abs_diff(@Vec::from_array([3, 4])),
                                @Vec::from_array([2, 2]),
                                "abs diff",
                            );
                            assert_eq!(
                                @Vec::from_array([1, 0]).abs_diff(@Vec::from_array([3, @T::MAX])),
                                @Vec::from_array([2, @T::MAX]),
                                "abs diff",
                            );

                            assert_eq!(
                                @Vec::from_array([1, 2]).sum(),
                                1 + 2,
                                "sum",
                            );

                            assert_eq!(
                                @Vec::from_array([1, 2]).dot(@Vec::from_array([3, 4])),
                                1 * 3 + 2 * 4,
                                "dot product",
                            );
                            assert_eq!(
                                @Vec::from_array([@T::MAX, 2]).dot(@Vec::from_array([0, 4])),
                                2 * 4,
                                "dot product",
                            );
                        }
                        3 => {
                            assert_eq!(
                                @Vec::from_array([1, 2, 3]).abs_diff(@Vec::from_array([3, 4, 5])),
                                @Vec::from_array([2, 2, 2]),
                                "abs diff",
                            );
                            assert_eq!(
                                @Vec::from_array([1, 0, 3]).abs_diff(@Vec::from_array([3, @T::MAX, 5])),
                                @Vec::from_array([2, @T::MAX, 2]),
                                "abs diff",
                            );

                            assert_eq!(
                                @Vec::from_array([1, 2, 3]).sum(),
                                1 + 2 + 3,
                                "sum",
                            );

                            assert_eq!(
                                @Vec::from_array([1, 2, 3]).dot(@Vec::from_array([3, 4, 5])),
                                1 * 3 + 2 * 4 + 3 * 5,
                                "dot product",
                            );
                            assert_eq!(
                                @Vec::from_array([@T::MAX, 2, 3]).dot(@Vec::from_array([0, 4, 5])),
                                2 * 4 + 3 * 5,
                                "dot product",
                            );
                        }
                        4 => {
                            assert_eq!(
                                @Vec::from_array([1, 2, 3, 4]).abs_diff(@Vec::from_array([3, 4, 5, 6])),
                                @Vec::from_array([2, 2, 2, 2]),
                                "abs diff",
                            );
                            assert_eq!(
                                @Vec::from_array([1, 0, 3, 4]).abs_diff(@Vec::from_array([3, @T::MAX, 5, 6])),
                                @Vec::from_array([2, @T::MAX, 2, 2]),
                                "abs diff",
                            );

                            assert_eq!(
                                @Vec::from_array([1, 2, 3, 4]).sum(),
                                1 + 2 + 3 + 4,
                                "sum",
                            );

                            assert_eq!(
                                @Vec::from_array([1, 2, 3, 4]).dot(@Vec::from_array([3, 4, 5, 6])),
                                1 * 3 + 2 * 4 + 3 * 5 + 4 * 6,
                                "dot product",
                            );
                            assert_eq!(
                                @Vec::from_array([@T::MAX, 2, 3, 4]).dot(@Vec::from_array([0, 4, 5, 6])),
                                2 * 4 + 3 * 5 + 4 * 6,
                                "dot product",
                            );
                        }
                    }

                    @if t_is_sint {
                        assert_eq!(
                            -@Vec::splat(3).with_y(4).with_padding(@T::MIN),
                            @Vec::splat(-3).with_y(-4),
                            "neg operator (padding overflows)",
                        );

                        assert_eq!(
                            @Vec::splat(3).with_y(4).with_padding(@T::MIN) / @Vec::splat(2).with_padding(-1),
                            @Vec::splat(1).with_y(2),
                            "div operator (padding overflows)",
                        );
                        assert_eq!(
                            @Vec::splat(3).with_y(4).with_padding(@T::MIN) % @Vec::splat(2).with_padding(-1),
                            @Vec::splat(1).with_y(0),
                            "rem operator (padding overflows)",
                        );
                    }
                }

                @if t_is_float {
                    assert_eq!(
                        -@Vec::splat(3.0).with_y(4.0),
                        @Vec::splat(-3.0).with_y(-4.0),
                        "neg operator",
                    );
                    assert_eq!(
                        @Vec::splat(1.0).with_y(2.0).with_padding(@T::MAX) + @Vec::splat(2.0).with_padding(@T::NAN),
                        @Vec::splat(3.0).with_y(4.0),
                        "add operator (padding is nan)",
                    );
                    assert_eq!(
                        @Vec::splat(2.0).with_y(3.0).with_padding(@T::MIN) - @Vec::splat(1.0).with_padding(@T::NAN),
                        @Vec::splat(1.0).with_y(2.0),
                        "sub operator (padding is nan)",
                    );
                    assert_eq!(
                        @Vec::splat(3.0).with_y(4.0).with_padding(@T::MAX) * @Vec::splat(4.0).with_padding(@T::NAN),
                        @Vec::splat(12.0).with_y(16.0),
                        "mul operator (padding is nan)",
                    );
                    assert_eq!(
                        @Vec::splat(3.0).with_y(4.0).with_padding(7.0) / @Vec::splat(2.0).with_padding(0.0),
                        @Vec::splat(1.5).with_y(2.0),
                        "div operator (padding is nan)",
                    );
                    assert_eq!(
                        @Vec::splat(3.0).with_y(4.0).with_padding(7.0) % @Vec::splat(2.0).with_padding(@T::NAN),
                        @Vec::splat(1.0).with_y(0.0),
                        "rem operator (padding is nan)",
                    );
                }
            }

            @if t_is_int {
                #[test]
                #[should_panic]
                fn @['test_ tvecna '_ a2 '_add_panic]() {
                    let _ = @Vec::splat(1).with_y(@T::MAX) + @Vec::splat(3);
                }

                #[test]
                #[should_panic]
                fn @['test_ tvecna '_ a2 '_sub_panic]() {
                    let _ = @Vec::splat(4).with_y(@T::MIN) - @Vec::splat(3);
                }

                #[test]
                #[should_panic]
                fn @['test_ tvecna '_ a2 '_mul_panic]() {
                    let _ = @Vec::splat(4).with_y(@T::MAX) * @Vec::splat(3);
                }

                #[test]
                #[should_panic]
                fn @['test_ tvecna '_ a2 '_div_zero_panic]() {
                    let _ = @Vec::splat(4) / @Vec::splat(3).with_y(0);
                }

                #[test]
                #[should_panic]
                fn @['test_ tvecna '_ a2 '_rem_zero_panic]() {
                    let _ = @Vec::splat(4) % @Vec::splat(3).with_y(0);
                }

                @if t_is_sint {
                    #[test]
                    #[should_panic]
                    fn @['test_ tvecna '_ a2 '_div_neg_panic]() {
                        let _ = @Vec::splat(4).with_y(@T::MIN) / @Vec::splat(-1);
                    }

                    #[test]
                    #[should_panic]
                    fn @['test_ tvecna '_ a2 '_div_panic]() {
                        let _ = @Vec::splat(4).with_y(@T::MIN) / @Vec::splat(-1);
                    }

                    #[test]
                    #[should_panic]
                    fn @['test_ tvecna '_ a2 '_rem_panic]() {
                        let _ = @Vec::splat(4).with_y(@T::MIN) % @Vec::splat(-1);
                    }
                }
            }
        }
    }
}
