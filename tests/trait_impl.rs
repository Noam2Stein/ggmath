use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
        DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl,
        ShlAssign, Shr, ShrAssign, Sub, SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use assert_impl_trait::assert_impl;
#[cfg(feature = "bytemuck")]
use bytemuck::{AnyBitPattern, NoUninit, Pod, Zeroable};
#[cfg(feature = "fixed")]
use fixed::{
    FixedI8, FixedI16, FixedI32, FixedI64, FixedI128, FixedU8, FixedU16, FixedU32, FixedU64,
    FixedU128,
};
#[cfg(feature = "fixp")]
use fixp::FixedPoint;
use ggmath::{
    Affine, Alignment, Length, Mask, Matrix, Quaternion, Scalar, SupportedLength, Vector,
    constants::{Infinity, Max, Min, Nan, NegInfinity, NegOne, One, Zero},
};
#[cfg(feature = "mint")]
use mint::IntoMint;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wide")]
use wide::{
    f32x4, f32x8, f32x16, f64x2, f64x4, f64x8, i8x16, i8x32, i16x8, i16x16, i16x32, i32x4, i32x8,
    i32x16, i64x2, i64x4, i64x8, u8x16, u8x32, u16x8, u16x16, u16x32, u32x4, u32x8, u32x16, u64x2,
    u64x4, u64x8,
};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        f32: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        i8: Scalar + Zero + One + NegOne + Min + Max,
        i16: Scalar + Zero + One + NegOne + Min + Max,
        i32: Scalar + Zero + One + NegOne + Min + Max,
        i64: Scalar + Zero + One + NegOne + Min + Max,
        i128: Scalar + Zero + One + NegOne + Min + Max,
        isize: Scalar + Zero + One + NegOne + Min + Max,
        u8: Scalar + Zero + One + Min + Max,
        u16: Scalar + Zero + One + Min + Max,
        u32: Scalar + Zero + One + Min + Max,
        u64: Scalar + Zero + One + Min + Max,
        u128: Scalar + Zero + One + Min + Max,
        usize: Scalar + Zero + One + Min + Max,

        Mask<N, T, A>: Debug,
        where T: Debug {
            Vector<N, T, A>: Debug,
            Matrix<N, T, A>: Debug,
            Quaternion<T, A>: Debug,
            Affine<N, T, A>: Debug,
        }

        Mask<N, T, A>: Display,
        where T: Display {
            Vector<N, T, A>: Display,
            Matrix<N, T, A>: Display,
            Quaternion<T, A>: Display,
            Affine<N, T, A>: Display,
        }

        Vector<N, T, A>: Clone,
        Matrix<N, T, A>: Clone,
        Quaternion<T, A>: Clone,
        Affine<N, T, A>: Clone,
        Mask<N, T, A>: Clone,

        Vector<N, T, A>: Copy,
        Matrix<N, T, A>: Copy,
        Quaternion<T, A>: Copy,
        Affine<N, T, A>: Copy,
        Mask<N, T, A>: Copy,

        Mask<N, T, A>: PartialEq,
        where T: PartialEq {
            Vector<N, T, A>: PartialEq,
            Matrix<N, T, A>: PartialEq,
            Quaternion<T, A>: PartialEq,
            Affine<N, T, A>: PartialEq,
        }

        Mask<N, T, A>: Eq,
        where T: Eq {
            Vector<N, T, A>: Eq,
            Matrix<N, T, A>: Eq,
            Quaternion<T, A>: Eq,
            Affine<N, T, A>: Eq,
        }

        Mask<N, T, A>: Hash,
        where T: Hash {
            Vector<N, T, A>: Hash,
            Matrix<N, T, A>: Hash,
            Quaternion<T, A>: Hash,
            Affine<N, T, A>: Hash,
        }

        Mask<N, T, A>: Default,
        where T: Default {
            Vector<N, T, A>: Default,
        }
        where T: Zero + One {
            Matrix<N, T, A>: Default,
            Quaternion<T, A>: Default,
            Affine<N, T, A>: Default,
        }

        Vector<N, T, A>: IntoIterator<Item = T>,
        Mask<N, T, A>: IntoIterator<Item = bool>,
        for<'a> where T: 'a {
            &'a Vector<N, T, A>: IntoIterator<Item = T>,
            &'a Mask<N, T, A>: IntoIterator<Item = bool>,
            &'a mut Vector<N, T, A>: IntoIterator<Item = &'a mut T>,
        }

        Vector<N, T, A>: Index<usize, Output = T>,

        Vector<N, T, A>: IndexMut<usize>,

        Vector<2, T, A>: Deref,
        Vector<3, T, A>: Deref,
        Vector<4, T, A>: Deref,
        Matrix<2, T, A>: Deref,
        Matrix<3, T, A>: Deref,
        Matrix<4, T, A>: Deref,
        Quaternion<T, A>: Deref,
        Affine<N, T, A>: Deref,

        Vector<2, T, A>: DerefMut,
        Vector<3, T, A>: DerefMut,
        Vector<4, T, A>: DerefMut,
        Matrix<2, T, A>: DerefMut,
        Matrix<3, T, A>: DerefMut,
        Matrix<4, T, A>: DerefMut,
        Quaternion<T, A>: DerefMut,
        Affine<N, T, A>: DerefMut,

        where T: Neg<Output = T> {
            Vector<N, T, A>: Neg<Output = Vector<N, T, A>>,
            Matrix<N, T, A>: Neg<Output = Matrix<N, T, A>>,
            Quaternion<T, A>: Neg<Output = Quaternion<T, A>>,
            for<'a> where T: 'a {
                &'a Vector<N, T, A>: Neg<Output = Vector<N, T, A>>,
                &'a Matrix<N, T, A>: Neg<Output = Matrix<N, T, A>>,
            }
        }

        Mask<N, T, A>: Not<Output = Mask<N, T, A>>,
        for<'a> where T: 'a {
            &'a Mask<N, T, A>: Not<Output = Mask<N, T, A>>,
        }
        where T: Not<Output = T> {
            Vector<N, T, A>: Not<Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                &'a Vector<N, T, A>: Not<Output = Vector<N, T, A>>,
            }
        }

        where T: Add<Output = T> {
            Vector<N, T, A>: Add<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Add<T, Output = Vector<N, T, A>>,
            Matrix<N, T, A>: Add<Output = Matrix<N, T, A>>,
            Quaternion<T, A>: Add<Output = Quaternion<T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Add<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Add<&'a T, Output = Vector<N, T, A>>,
                Matrix<N, T, A>: Add<&'a Matrix<N, T, A>, Output = Matrix<N, T, A>>,
                &'a Vector<N, T, A>: Add<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Add<T, Output = Vector<N, T, A>>,
                &'a Matrix<N, T, A>: Add<Matrix<N, T, A>, Output = Matrix<N, T, A>>,
                &'a Vector<N, T, A>: Add<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Add<&'a T, Output = Vector<N, T, A>>,
                &'a Matrix<N, T, A>: Add<&'a Matrix<N, T, A>, Output = Matrix<N, T, A>>,
            }
        }

        where T: Add<Output = T> {
            Vector<N, T, A>: AddAssign,
            Vector<N, T, A>: AddAssign<T>,
            Matrix<N, T, A>: AddAssign,
            Quaternion<T, A>: AddAssign,
            for<'a> where T: 'a {
                Vector<N, T, A>: AddAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: AddAssign<&'a T>,
                Matrix<N, T, A>: AddAssign<&'a Matrix<N, T, A>>,
            }
        }

        where T: Sub<Output = T> {
            Vector<N, T, A>: Sub<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Sub<T, Output = Vector<N, T, A>>,
            Matrix<N, T, A>: Sub<Output = Matrix<N, T, A>>,
            Quaternion<T, A>: Sub<Output = Quaternion<T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Sub<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Sub<&'a T, Output = Vector<N, T, A>>,
                Matrix<N, T, A>: Sub<&'a Matrix<N, T, A>, Output = Matrix<N, T, A>>,
                &'a Vector<N, T, A>: Sub<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Sub<T, Output = Vector<N, T, A>>,
                &'a Matrix<N, T, A>: Sub<Matrix<N, T, A>, Output = Matrix<N, T, A>>,
                &'a Vector<N, T, A>: Sub<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Sub<&'a T, Output = Vector<N, T, A>>,
                &'a Matrix<N, T, A>: Sub<&'a Matrix<N, T, A>, Output = Matrix<N, T, A>>,
            }
        }

        where T: Sub<Output = T> {
            Vector<N, T, A>: SubAssign,
            Vector<N, T, A>: SubAssign<T>,
            Matrix<N, T, A>: SubAssign,
            Quaternion<T, A>: SubAssign,
            for<'a> where T: 'a {
                Vector<N, T, A>: SubAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: SubAssign<&'a T>,
                Matrix<N, T, A>: SubAssign<&'a Matrix<N, T, A>>,
            }
        }

        where T: Mul<Output = T> {
            Vector<N, T, A>: Mul<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Mul<T, Output = Vector<N, T, A>>,
            Quaternion<T, A>: Mul<T, Output = Quaternion<T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Mul<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Mul<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Mul<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Mul<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Mul<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Mul<&'a T, Output = Vector<N, T, A>>,
            }
        }

        where T: Mul<Output = T> {
            Vector<N, T, A>: MulAssign,
            Vector<N, T, A>: MulAssign<T>,
            Quaternion<T, A>: MulAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: MulAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: MulAssign<&'a T>,
            }
        }

        where T: Div<Output = T> {
            Vector<N, T, A>: Div<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Div<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Div<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Div<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Div<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Div<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Div<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Div<&'a T, Output = Vector<N, T, A>>,
            }
        }

        where T: Div<Output = T> {
            Vector<N, T, A>: DivAssign,
            Vector<N, T, A>: DivAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: DivAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: DivAssign<&'a T>,
            }
        }

        where T: Rem<Output = T> {
            Vector<N, T, A>: Rem<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Rem<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Rem<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Rem<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Rem<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Rem<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Rem<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Rem<&'a T, Output = Vector<N, T, A>>,
            }
        }

        where T: Rem<Output = T> {
            Vector<N, T, A>: RemAssign,
            Vector<N, T, A>: RemAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: RemAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: RemAssign<&'a T>,
            }
        }

        where T: Shl<Output = T> {
            Vector<N, T, A>: Shl<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Shl<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Shl<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Shl<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shl<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shl<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shl<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shl<&'a T, Output = Vector<N, T, A>>,
            }
        }

        where T: Shl<Output = T> {
            Vector<N, T, A>: ShlAssign,
            Vector<N, T, A>: ShlAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: ShlAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: ShlAssign<&'a T>,
            }
        }

        where T: Shr<Output = T> {
            Vector<N, T, A>: Shr<Output = Vector<N, T, A>>,
            Vector<N, T, A>: Shr<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: Shr<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: Shr<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shr<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shr<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shr<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: Shr<&'a T, Output = Vector<N, T, A>>,
            }
        }

        where T: Shr<Output = T> {
            Vector<N, T, A>: ShrAssign,
            Vector<N, T, A>: ShrAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: ShrAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: ShrAssign<&'a T>,
            }
        }

        Mask<N, T, A>: BitAnd<Output = Mask<N, T, A>>,
        Mask<N, T, A>: BitAnd<bool, Output = Mask<N, T, A>>,
        for<'a> where T: 'a {
            Mask<N, T, A>: BitAnd<&'a Mask<N, T, A>, Output = Mask<N, T, A>>,
            Mask<N, T, A>: BitAnd<&'a bool, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitAnd<Mask<N, T, A>, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitAnd<bool, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitAnd<&'a Mask<N, T, A>, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitAnd<&'a bool, Output = Mask<N, T, A>>,
        }
        where T: BitAnd<Output = T> {
            Vector<N, T, A>: BitAnd<Output = Vector<N, T, A>>,
            Vector<N, T, A>: BitAnd<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: BitAnd<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: BitAnd<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitAnd<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitAnd<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitAnd<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitAnd<&'a T, Output = Vector<N, T, A>>,
            }
        }

        Mask<N, T, A>: BitAndAssign,
        Mask<N, T, A>: BitAndAssign<bool>,
        for<'a> where T: 'a {
            Mask<N, T, A>: BitAndAssign<&'a Mask<N, T, A>>,
            Mask<N, T, A>: BitAndAssign<&'a bool>,
        }
        where T: BitAnd<Output = T> {
            Vector<N, T, A>: BitAndAssign,
            Vector<N, T, A>: BitAndAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: BitAndAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: BitAndAssign<&'a T>,
            }
        }

        Mask<N, T, A>: BitOr<Output = Mask<N, T, A>>,
        Mask<N, T, A>: BitOr<bool, Output = Mask<N, T, A>>,
        for<'a> where T: 'a {
            Mask<N, T, A>: BitOr<&'a Mask<N, T, A>, Output = Mask<N, T, A>>,
            Mask<N, T, A>: BitOr<&'a bool, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitOr<Mask<N, T, A>, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitOr<bool, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitOr<&'a Mask<N, T, A>, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitOr<&'a bool, Output = Mask<N, T, A>>,
        }
        where T: BitOr<Output = T> {
            Vector<N, T, A>: BitOr<Output = Vector<N, T, A>>,
            Vector<N, T, A>: BitOr<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: BitOr<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: BitOr<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitOr<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitOr<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitOr<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitOr<&'a T, Output = Vector<N, T, A>>,
            }
        }

        Mask<N, T, A>: BitOrAssign,
        Mask<N, T, A>: BitOrAssign<bool>,
        for<'a> where T: 'a {
            Mask<N, T, A>: BitOrAssign<&'a Mask<N, T, A>>,
            Mask<N, T, A>: BitOrAssign<&'a bool>,
        }
        where T: BitOr<Output = T> {
            Vector<N, T, A>: BitOrAssign,
            Vector<N, T, A>: BitOrAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: BitOrAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: BitOrAssign<&'a T>,
            }
        }

        Mask<N, T, A>: BitXor<Output = Mask<N, T, A>>,
        Mask<N, T, A>: BitXor<bool, Output = Mask<N, T, A>>,
        for<'a> where T: 'a {
            Mask<N, T, A>: BitXor<&'a Mask<N, T, A>, Output = Mask<N, T, A>>,
            Mask<N, T, A>: BitXor<&'a bool, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitXor<Mask<N, T, A>, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitXor<bool, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitXor<&'a Mask<N, T, A>, Output = Mask<N, T, A>>,
            &'a Mask<N, T, A>: BitXor<&'a bool, Output = Mask<N, T, A>>,
        }
        where T: BitXor<Output = T> {
            Vector<N, T, A>: BitXor<Output = Vector<N, T, A>>,
            Vector<N, T, A>: BitXor<T, Output = Vector<N, T, A>>,
            for<'a> where T: 'a {
                Vector<N, T, A>: BitXor<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                Vector<N, T, A>: BitXor<&'a T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitXor<Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitXor<T, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitXor<&'a Vector<N, T, A>, Output = Vector<N, T, A>>,
                &'a Vector<N, T, A>: BitXor<&'a T, Output = Vector<N, T, A>>,
            }
        }

        Mask<N, T, A>: BitXorAssign,
        Mask<N, T, A>: BitXorAssign<bool>,
        for<'a> where T: 'a {
            Mask<N, T, A>: BitXorAssign<&'a Mask<N, T, A>>,
            Mask<N, T, A>: BitXorAssign<&'a bool>,
        }
        where T: BitXor<Output = T> {
            Vector<N, T, A>: BitXorAssign,
            Vector<N, T, A>: BitXorAssign<T>,
            for<'a> where T: 'a {
                Vector<N, T, A>: BitXorAssign<&'a Vector<N, T, A>>,
                Vector<N, T, A>: BitXorAssign<&'a T>,
            }
        }

        for<'a> where T: 'a {
            Vector<N, T, A>: 'a,
            Matrix<N, T, A>: 'a,
            Quaternion<T, A>: 'a,
            Affine<N, T, A>: 'a,
            Mask<N, T, A>: 'a,
        }

        Mask<N, T, A>: Send,
        where T: Send {
            Vector<N, T, A>: Send,
            Matrix<N, T, A>: Send,
            Quaternion<T, A>: Send,
            Affine<N, T, A>: Send,
        }

        Mask<N, T, A>: Sync,
        where T: Sync {
            Vector<N, T, A>: Sync,
            Matrix<N, T, A>: Sync,
            Quaternion<T, A>: Sync,
            Affine<N, T, A>: Sync,
        }

        Mask<N, T, A>: Unpin,
        where T: Unpin {
            Vector<N, T, A>: Unpin,
            Matrix<N, T, A>: Unpin,
            Quaternion<T, A>: Unpin,
            Affine<N, T, A>: Unpin,
        }

        Mask<N, T, A>: UnwindSafe,
        where T: UnwindSafe {
            Vector<N, T, A>: UnwindSafe,
            Matrix<N, T, A>: UnwindSafe,
            Quaternion<T, A>: UnwindSafe,
            Affine<N, T, A>: UnwindSafe,
        }

        Mask<N, T, A>: RefUnwindSafe,
        where T: RefUnwindSafe {
            Vector<N, T, A>: RefUnwindSafe,
            Matrix<N, T, A>: RefUnwindSafe,
            Quaternion<T, A>: RefUnwindSafe,
            Affine<N, T, A>: RefUnwindSafe,
        }
    }
);

#[cfg(feature = "bytemuck")]
assert_impl!(
    for<const N: usize, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        for<T: Scalar + Pod> {
            Vector<N, T, A>: Pod,
        }
        for<T: Scalar + Zeroable> {
            Vector<N, T, A>: Zeroable,
        }
        Vector<N, f32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, f64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, i128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, isize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, u128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, usize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Vector<N, bool, A>: Zeroable,

        for<T: Scalar + Pod> {
            Matrix<N, T, A>: Pod,
        }
        for<T: Scalar + Zeroable> {
            Matrix<N, T, A>: Zeroable,
        }
        Matrix<N, f32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, f64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, i8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, i16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, i32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, i64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, i128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, isize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, u8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, u16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, u32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, u64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, u128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, usize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Matrix<N, bool, A>: Zeroable,

        for<T: Scalar + Pod> {
            Quaternion<T, A>: Pod,
        }
        for<T: Scalar + Zeroable> {
            Quaternion<T, A>: Zeroable,
        }
        Quaternion<f32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<f64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<i128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<isize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<u128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<usize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Quaternion<bool, A>: Zeroable,

        for<T: Scalar + Pod> {
            Affine<N, T, A>: Pod,
        }
        for<T: Scalar + Zeroable> {
            Affine<N, T, A>: Zeroable,
        }
        Affine<N, f32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, f64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, i8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, i16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, i32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, i64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, i128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, isize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, u8, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, u16, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, u32, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, u64, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, u128, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, usize, A>: AnyBitPattern + NoUninit + Pod + Zeroable,
        Affine<N, bool, A>: Zeroable,

        for<T: Scalar + 'static> {
            Mask<N, T, A>: NoUninit,
        }
        for<T: Scalar> {
            Mask<N, T, A>: Zeroable,
        }
    }
);

#[cfg(feature = "fixed")]
assert_impl!(
    for<const N: usize, Frac, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        FixedI8<Frac>: Scalar + Zero + Min + Max,
        FixedI16<Frac>: Scalar + Zero + Min + Max,
        FixedI32<Frac>: Scalar + Zero + Min + Max,
        FixedI64<Frac>: Scalar + Zero + Min + Max,
        FixedI128<Frac>: Scalar + Zero + Min + Max,
        FixedU8<Frac>: Scalar + Zero + Min + Max,
        FixedU16<Frac>: Scalar + Zero + Min + Max,
        FixedU32<Frac>: Scalar + Zero + Min + Max,
        FixedU64<Frac>: Scalar + Zero + Min + Max,
        FixedU128<Frac>: Scalar + Zero + Min + Max,
    }
);

#[cfg(feature = "fixp")]
assert_impl!(
    for<const N: usize, const FRAC_BITS: usize, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        FixedPoint<i8, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<i16, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<i32, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<i64, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<i128, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<isize, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<u8, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<u16, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<u32, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<u64, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<u128, FRAC_BITS>: Scalar + Zero + Min + Max,
        FixedPoint<usize, FRAC_BITS>: Scalar + Zero + Min + Max,
    }
);

#[cfg(feature = "mint")]
assert_impl!(
    for<T, A: Alignment>
    where
        T: Scalar,
    {
        Vector<2, T, A>: IntoMint<MintType = mint::Vector2<T>>,
        Vector<2, T, A>: From<mint::Point2<T>>,
        Vector<2, T, A>: Into<mint::Point2<T>>,
        Vector<2, T, A>: From<mint::Vector2<T>>,
        Vector<2, T, A>: Into<mint::Vector2<T>>,

        Vector<3, T, A>: IntoMint<MintType = mint::Vector3<T>>,
        Vector<3, T, A>: From<mint::Point3<T>>,
        Vector<3, T, A>: Into<mint::Point3<T>>,
        Vector<3, T, A>: From<mint::Vector3<T>>,
        Vector<3, T, A>: Into<mint::Vector3<T>>,

        Vector<4, T, A>: IntoMint<MintType = mint::Vector4<T>>,
        Vector<4, T, A>: From<mint::Vector4<T>>,
        Vector<4, T, A>: Into<mint::Vector4<T>>,

        Matrix<2, T, A>: IntoMint<MintType = mint::ColumnMatrix2<T>>,
        Matrix<2, T, A>: From<mint::ColumnMatrix2<T>>,
        Matrix<2, T, A>: Into<mint::ColumnMatrix2<T>>,
        Matrix<2, T, A>: From<mint::RowMatrix2<T>>,
        Matrix<2, T, A>: Into<mint::RowMatrix2<T>>,

        Matrix<3, T, A>: IntoMint<MintType = mint::ColumnMatrix3<T>>,
        Matrix<3, T, A>: From<mint::ColumnMatrix3<T>>,
        Matrix<3, T, A>: Into<mint::ColumnMatrix3<T>>,
        Matrix<3, T, A>: From<mint::RowMatrix3<T>>,
        Matrix<3, T, A>: Into<mint::RowMatrix3<T>>,

        Matrix<4, T, A>: IntoMint<MintType = mint::ColumnMatrix4<T>>,
        Matrix<4, T, A>: From<mint::ColumnMatrix4<T>>,
        Matrix<4, T, A>: Into<mint::ColumnMatrix4<T>>,
        Matrix<4, T, A>: From<mint::RowMatrix4<T>>,
        Matrix<4, T, A>: Into<mint::RowMatrix4<T>>,

        Quaternion<T, A>: IntoMint<MintType = mint::Quaternion<T>>,
        Quaternion<T, A>: From<mint::Quaternion<T>>,
        Quaternion<T, A>: Into<mint::Quaternion<T>>,

        Mask<2, T, A>: IntoMint<MintType = mint::Vector2<bool>>,
        Mask<2, T, A>: From<mint::Vector2<bool>>,
        Mask<2, T, A>: Into<mint::Vector2<bool>>,

        Mask<3, T, A>: IntoMint<MintType = mint::Vector3<bool>>,
        Mask<3, T, A>: From<mint::Vector3<bool>>,
        Mask<3, T, A>: Into<mint::Vector3<bool>>,

        Mask<4, T, A>: IntoMint<MintType = mint::Vector4<bool>>,
        Mask<4, T, A>: From<mint::Vector4<bool>>,
        Mask<4, T, A>: Into<mint::Vector4<bool>>,
    }
);

#[cfg(feature = "serde")]
assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Mask<N, T, A>: Serialize,
        where T: Serialize {
            Vector<N, T, A>: Serialize,
            Matrix<N, T, A>: Serialize,
            Quaternion<T, A>: Serialize,
            Affine<N, T, A>: Serialize,
        }

        for<'de> {
            Mask<N, T, A>: Deserialize<'de>,
            where T: Deserialize<'de> {
                Vector<N, T, A>: Deserialize<'de>,
                Matrix<N, T, A>: Deserialize<'de>,
                Quaternion<T, A>: Deserialize<'de>,
                Affine<N, T, A>: Deserialize<'de>,
            }
        }
    }
);

#[cfg(feature = "wide")]
assert_impl!(
    for<const N: usize, Frac, A: Alignment>
    where
        Length<N>: SupportedLength,
    {
        f32x4: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f32x8: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f32x16: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64x2: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64x4: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        f64x8: Scalar + Zero + One + NegOne + Min + Max + Nan + Infinity + NegInfinity,
        i8x16: Scalar + Zero + One + NegOne + Min + Max,
        i8x32: Scalar + Zero + One + NegOne + Min + Max,
        i16x8: Scalar + Zero + One + NegOne + Min + Max,
        i16x16: Scalar + Zero + One + NegOne + Min + Max,
        i16x32: Scalar + Zero + One + NegOne + Min + Max,
        i32x4: Scalar + Zero + One + NegOne + Min + Max,
        i32x8: Scalar + Zero + One + NegOne + Min + Max,
        i32x16: Scalar + Zero + One + NegOne + Min + Max,
        i64x2: Scalar + Zero + One + NegOne + Min + Max,
        i64x4: Scalar + Zero + One + NegOne + Min + Max,
        i64x8: Scalar + Zero + One + NegOne + Min + Max,
        u8x16: Scalar + Zero + One + Min + Max,
        u8x32: Scalar + Zero + One + Min + Max,
        u16x8: Scalar + Zero + One + Min + Max,
        u16x16: Scalar + Zero + One + Min + Max,
        u16x32: Scalar + Zero + One + Min + Max,
        u32x4: Scalar + Zero + One + Min + Max,
        u32x8: Scalar + Zero + One + Min + Max,
        u32x16: Scalar + Zero + One + Min + Max,
        u64x2: Scalar + Zero + One + Min + Max,
        u64x4: Scalar + Zero + One + Min + Max,
        u64x8: Scalar + Zero + One + Min + Max,
    }
);
