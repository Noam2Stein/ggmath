use crate::{
    Aligned, Alignment, EqTest, FloatExt, Unaligned,
    backend::{AffineBackend, DefaultBackend, MaskBackend, RotorBackend, VectorBackend},
};

/// A trait for elements of vectors.
///
/// This requires [`Copy`].
///
/// Due to type system limitations, this trait cannot be implemented directly.
/// Instead implement the [`CustomElement`] trait:
///
/// ```
/// use ggmath::{Alignment, CustomElement, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl CustomElement for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
#[expect(private_bounds)]
pub trait Element:
    Copy
    + VectorBackend<2, Aligned>
    + VectorBackend<3, Aligned>
    + VectorBackend<4, Aligned>
    + VectorBackend<2, Unaligned>
    + VectorBackend<3, Unaligned>
    + VectorBackend<4, Unaligned>
    + AffineBackend<2, Aligned>
    + AffineBackend<3, Aligned>
    + AffineBackend<4, Aligned>
    + AffineBackend<2, Unaligned>
    + AffineBackend<3, Unaligned>
    + AffineBackend<4, Unaligned>
    + RotorBackend<3, Aligned>
    + RotorBackend<3, Unaligned>
    + MaskBackend<2, Aligned>
    + MaskBackend<3, Aligned>
    + MaskBackend<4, Aligned>
    + MaskBackend<2, Unaligned>
    + MaskBackend<3, Unaligned>
    + MaskBackend<4, Unaligned>
{
}

/// A trait for types with a `0` value.
///
/// This is used for generic functions that require `0`, like
/// [`Matrix::from_diagonal`].
///
/// [`Matrix::from_diagonal`]: crate::Matrix::from_diagonal
pub trait Zero {
    /// `0`.
    const ZERO: Self;
}

/// A trait for types with a `1` value.
///
/// This is used for generic functions that require `1`, like
/// [`Affine::from_translation`].
///
/// [`Affine::from_translation`]: crate::Affine::from_translation
pub trait One {
    /// `1`.
    const ONE: Self;
}

/// A trait for types with a `-1` value.
///
/// Currently this is used by vector constants, like [`Vector::NEG_ONE`].
///
/// [`Vector::NEG_ONE`]: crate::Vector::NEG_ONE
pub trait NegOne {
    /// `-1`.
    const NEG_ONE: Self;
}

/// A trait to implement [`Element`] for downstream types.
///
/// Due to type system limitations, the [`Element`] trait cannot be implemented
/// directly. Instead implement this trait:
///
/// ```
/// use ggmath::{Alignment, CustomElement, Vec2};
///
/// #[derive(Debug, Clone, Copy)]
/// struct Foo(i32);
///
/// impl CustomElement for Foo {}
///
/// // `Foo` can then be stored inside vectors.
/// println!("{:?}", Vec2::new(Foo(1), Foo(2)));
/// ```
pub trait CustomElement: Copy {}

#[diagnostic::do_not_recommend]
impl<T> Element for T where T: CustomElement {}

#[diagnostic::do_not_recommend]
impl<T, const N: usize, A: Alignment> DefaultBackend<N, A> for T where T: CustomElement {}

macro_rules! float_impl {
    ($T:ident) => {
        impl Element for $T {}

        impl Zero for $T {
            const ZERO: Self = 0.0;
        }

        impl One for $T {
            const ONE: Self = 1.0;
        }

        impl NegOne for $T {
            const NEG_ONE: Self = -1.0;
        }

        impl EqTest for $T {
            #[inline]
            fn eq_test(&self, other: &Self) -> bool {
                self.abs_diff_eq(*other, 2e-4)
            }
        }
    };
}
float_impl!(f32);
float_impl!(f64);

macro_rules! integer_impl {
    ($T:ident) => {
        impl Element for $T {}

        impl Zero for $T {
            const ZERO: Self = 0;
        }

        impl One for $T {
            const ONE: Self = 1;
        }

        impl EqTest for $T {
            #[inline]
            fn eq_test(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}
integer_impl!(i8);
integer_impl!(i16);
integer_impl!(i32);
integer_impl!(i64);
integer_impl!(i128);
integer_impl!(isize);
integer_impl!(u8);
integer_impl!(u16);
integer_impl!(u32);
integer_impl!(u64);
integer_impl!(u128);
integer_impl!(usize);

macro_rules! signed_impl {
    ($T:ident) => {
        impl NegOne for $T {
            const NEG_ONE: Self = -1;
        }
    };
}
signed_impl!(i8);
signed_impl!(i16);
signed_impl!(i32);
signed_impl!(i64);
signed_impl!(i128);
signed_impl!(isize);

impl Element for bool {}

#[cfg(feature = "fixed")]
mod fixed_impl {
    use fixed::{
        FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
        FixedU128,
        types::extra::{
            IsLessOrEqual, True, U6, U7, U14, U15, U30, U31, U62, U63, U126, U127, Unsigned,
        },
    };

    use crate::{CustomElement, EqTest, NegOne, One, Zero};

    macro_rules! fixed_impl {
        ($Fixed:ident) => {
            impl<Frac> CustomElement for $Fixed<Frac> {}

            impl<Frac> Zero for $Fixed<Frac> {
                const ZERO: Self = Self::ZERO;
            }

            impl<Frac> EqTest for $Fixed<Frac>
            where
                Frac: Unsigned,
            {
                #[inline]
                fn eq_test(&self, other: &Self) -> bool {
                    *self == *other
                }
            }
        };
    }
    fixed_impl!(FixedI8);
    fixed_impl!(FixedI16);
    fixed_impl!(FixedI32);
    fixed_impl!(FixedI64);
    fixed_impl!(FixedI128);
    fixed_impl!(FixedU8);
    fixed_impl!(FixedU16);
    fixed_impl!(FixedU32);
    fixed_impl!(FixedU64);
    fixed_impl!(FixedU128);

    macro_rules! fixed_signed_impl {
        ($Fixed:ident, $UBitsM1:ident, $UBitsM2:ident) => {
            impl<Frac> One for $Fixed<Frac>
            where
                Frac: IsLessOrEqual<$UBitsM2, Output = True> + Unsigned,
            {
                const ONE: Self = Self::ONE;
            }

            impl<Frac> NegOne for $Fixed<Frac>
            where
                Frac: IsLessOrEqual<$UBitsM1, Output = True> + Unsigned,
            {
                const NEG_ONE: Self = Self::NEG_ONE;
            }
        };
    }
    fixed_signed_impl!(FixedI8, U7, U6);
    fixed_signed_impl!(FixedI16, U15, U14);
    fixed_signed_impl!(FixedI32, U31, U30);
    fixed_signed_impl!(FixedI64, U63, U62);
    fixed_signed_impl!(FixedI128, U127, U126);

    macro_rules! fixed_unsigned_impl {
        ($Fixed:ident, $UBitsM1:ident) => {
            impl<Frac> One for $Fixed<Frac>
            where
                Frac: IsLessOrEqual<$UBitsM1, Output = True> + Unsigned,
            {
                const ONE: Self = Self::ONE;
            }
        };
    }
    fixed_unsigned_impl!(FixedU8, U7);
    fixed_unsigned_impl!(FixedU16, U15);
    fixed_unsigned_impl!(FixedU32, U31);
    fixed_unsigned_impl!(FixedU64, U63);
    fixed_unsigned_impl!(FixedU128, U127);
}

#[cfg(feature = "half")]
mod half_impl {
    use half::{bf16, f16};

    use crate::{CustomElement, NegOne, One, Zero};

    macro_rules! half_impl {
        ($T:ident) => {
            impl CustomElement for $T {}

            impl Zero for $T {
                const ZERO: Self = Self::ZERO;
            }

            impl One for $T {
                const ONE: Self = Self::ONE;
            }

            impl NegOne for $T {
                const NEG_ONE: Self = Self::NEG_ONE;
            }
        };
    }
    half_impl!(f16);
    half_impl!(bf16);
}

#[cfg(feature = "wide")]
mod wide_impl {
    use wide::{
        f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i8x64, i16x8, i16x16, i16x32,
        i32x4, i32x8, i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u8x64, u16x8, u16x16, u16x32,
        u32x4, u32x8, u32x16, u64x2, u64x4, u64x8,
    };

    use crate::{CustomElement, EqTest, NegOne, One, Zero};

    macro_rules! wide_impl {
        ($T:ident, $N:literal, $Simd:ident) => {
            impl CustomElement for $Simd {}

            impl EqTest for $Simd {
                #[inline(always)]
                fn eq_test(&self, _other: &Self) -> bool {
                    true
                }
            }
        };
    }
    wide_impl!(f32, 4, f32x4);
    wide_impl!(f32, 8, f32x8);
    wide_impl!(f32, 16, f32x16);
    wide_impl!(f64, 2, f64x2);
    wide_impl!(f64, 4, f64x4);
    wide_impl!(f64, 8, f64x8);
    wide_impl!(i8, 16, i8x16);
    wide_impl!(i8, 32, i8x32);
    wide_impl!(i8, 64, i8x64);
    wide_impl!(i16, 8, i16x8);
    wide_impl!(i16, 16, i16x16);
    wide_impl!(i16, 32, i16x32);
    wide_impl!(i32, 4, i32x4);
    wide_impl!(i32, 8, i32x8);
    wide_impl!(i32, 16, i32x16);
    wide_impl!(i64, 2, i64x2);
    wide_impl!(i64, 4, i64x4);
    wide_impl!(i64, 8, i64x8);
    wide_impl!(u8, 16, u8x16);
    wide_impl!(u8, 32, u8x32);
    wide_impl!(u8, 64, u8x64);
    wide_impl!(u16, 8, u16x8);
    wide_impl!(u16, 16, u16x16);
    wide_impl!(u16, 32, u16x32);
    wide_impl!(u32, 4, u32x4);
    wide_impl!(u32, 8, u32x8);
    wide_impl!(u32, 16, u32x16);
    wide_impl!(u64, 2, u64x2);
    wide_impl!(u64, 4, u64x4);
    wide_impl!(u64, 8, u64x8);

    macro_rules! wide_float_impl {
        ($T:ident, $N:literal, $Simd:ident) => {
            impl Zero for $Simd {
                const ZERO: Self = Self::ZERO;
            }

            impl One for $Simd {
                const ONE: Self = Self::ONE;
            }

            impl NegOne for $Simd {
                const NEG_ONE: Self = Self::splat(-1.0);
            }
        };
    }
    wide_float_impl!(f32, 4, f32x4);
    wide_float_impl!(f32, 8, f32x8);
    wide_float_impl!(f32, 16, f32x16);
    wide_float_impl!(f64, 2, f64x2);
    wide_float_impl!(f64, 4, f64x4);
    wide_float_impl!(f64, 8, f64x8);

    macro_rules! wide_integer_impl {
        ($T:ident, $N:literal, $Simd:ident) => {
            impl Zero for $Simd {
                const ZERO: Self = Self::ZERO;
            }

            impl One for $Simd {
                const ONE: Self = Self::ONE;
            }
        };
    }
    wide_integer_impl!(i8, 16, i8x16);
    wide_integer_impl!(i8, 32, i8x32);
    wide_integer_impl!(i8, 64, i8x64);
    wide_integer_impl!(i16, 8, i16x8);
    wide_integer_impl!(i16, 16, i16x16);
    wide_integer_impl!(i16, 32, i16x32);
    wide_integer_impl!(i32, 4, i32x4);
    wide_integer_impl!(i32, 8, i32x8);
    wide_integer_impl!(i32, 16, i32x16);
    wide_integer_impl!(i64, 2, i64x2);
    wide_integer_impl!(i64, 4, i64x4);
    wide_integer_impl!(i64, 8, i64x8);
    wide_integer_impl!(u8, 16, u8x16);
    wide_integer_impl!(u8, 32, u8x32);
    wide_integer_impl!(u8, 64, u8x64);
    wide_integer_impl!(u16, 8, u16x8);
    wide_integer_impl!(u16, 16, u16x16);
    wide_integer_impl!(u16, 32, u16x32);
    wide_integer_impl!(u32, 4, u32x4);
    wide_integer_impl!(u32, 8, u32x8);
    wide_integer_impl!(u32, 16, u32x16);
    wide_integer_impl!(u64, 2, u64x2);
    wide_integer_impl!(u64, 4, u64x4);
    wide_integer_impl!(u64, 8, u64x8);

    macro_rules! wide_signed_impl {
        ($T:ident, $N:literal, $Simd:ident) => {
            impl NegOne for $Simd {
                const NEG_ONE: Self = Self::splat(-1);
            }
        };
    }
    wide_signed_impl!(i8, 16, i8x16);
    wide_signed_impl!(i8, 32, i8x32);
    wide_signed_impl!(i8, 64, i8x64);
    wide_signed_impl!(i16, 8, i16x8);
    wide_signed_impl!(i16, 16, i16x16);
    wide_signed_impl!(i16, 32, i16x32);
    wide_signed_impl!(i32, 4, i32x4);
    wide_signed_impl!(i32, 8, i32x8);
    wide_signed_impl!(i32, 16, i32x16);
    wide_signed_impl!(i64, 2, i64x2);
    wide_signed_impl!(i64, 4, i64x4);
    wide_signed_impl!(i64, 8, i64x8);
}
