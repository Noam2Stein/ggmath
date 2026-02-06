use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
        ShrAssign, Sub, SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use assert_impl_trait::assert_impl;
use ggmath::{Alignment, Length, Scalar, SupportedLength, Vector};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Vector<N, T, A>: Clone,
        Vector<N, T, A>: Copy,
        Vector<N, T, A>: Index<usize>,
        Vector<N, T, A>: IndexMut<usize>,
        Vector<N, T, A>: IntoIterator<Item = T>,
        for<'a> where T: 'a {
            &'a Vector<N, T, A>: IntoIterator<Item = T>,
            &'a mut Vector<N, T, A>: IntoIterator<Item = &'a mut T>,
        }

        for<'a> where T: 'a {
            Vector<N, T, A>: 'a,
        }
        where T: Send {
            Vector<N, T, A>: Send,
        }
        where T: Sync {
            Vector<N, T, A>: Sync,
        }
        where T: Unpin {
            Vector<N, T, A>: Unpin,
        }
        where T: UnwindSafe {
            Vector<N, T, A>: UnwindSafe,
        }
        where T: RefUnwindSafe {
            Vector<N, T, A>: RefUnwindSafe,
        }
        where T: Debug {
            Vector<N, T, A>: Debug,
        }
        where T: Display {
            Vector<N, T, A>: Display,
        }
        where T: PartialEq {
            Vector<N, T, A>: PartialEq,
        }
        where T: Eq {
            Vector<N, T, A>: Eq,
        }
        where T: Hash {
            Vector<N, T, A>: Hash,
        }
        where T: Default {
            Vector<N, T, A>: Default,
        }

        where T: Neg<Output = T> {
            Vector<N, T, A>: Neg,
        }
        where T: Not<Output = T> {
            Vector<N, T, A>: Not,
        }

        where T: Add<Output = T> {
            Vector<N, T, A>: Add,
            Vector<N, T, A>: Add<T>,
            Vector<N, T, A>: AddAssign,
            Vector<N, T, A>: AddAssign<T>,
        }
        where T: Sub<Output = T> {
            Vector<N, T, A>: Sub,
            Vector<N, T, A>: Sub<T>,
            Vector<N, T, A>: SubAssign,
            Vector<N, T, A>: SubAssign<T>,
        }
        where T: Mul<Output = T> {
            Vector<N, T, A>: Mul,
            Vector<N, T, A>: Mul<T>,
            Vector<N, T, A>: MulAssign,
            Vector<N, T, A>: MulAssign<T>,
        }
        where T: Div<Output = T> {
            Vector<N, T, A>: Div,
            Vector<N, T, A>: Div<T>,
            Vector<N, T, A>: DivAssign,
            Vector<N, T, A>: DivAssign<T>,
        }
        where T: Rem<Output = T> {
            Vector<N, T, A>: Rem,
            Vector<N, T, A>: Rem<T>,
            Vector<N, T, A>: RemAssign,
            Vector<N, T, A>: RemAssign<T>,
        }
        where T: Shl<Output = T> {
            Vector<N, T, A>: Shl,
            Vector<N, T, A>: Shl<T>,
            Vector<N, T, A>: ShlAssign,
            Vector<N, T, A>: ShlAssign<T>,
        }
        where T: Shr<Output = T> {
            Vector<N, T, A>: Shr,
            Vector<N, T, A>: Shr<T>,
            Vector<N, T, A>: ShrAssign,
            Vector<N, T, A>: ShrAssign<T>,
        }
        where T: BitAnd<Output = T> {
            Vector<N, T, A>: BitAnd,
            Vector<N, T, A>: BitAnd<T>,
            Vector<N, T, A>: BitAndAssign,
            Vector<N, T, A>: BitAndAssign<T>,
        }
        where T: BitOr<Output = T> {
            Vector<N, T, A>: BitOr,
            Vector<N, T, A>: BitOr<T>,
            Vector<N, T, A>: BitOrAssign,
            Vector<N, T, A>: BitOrAssign<T>,
        }
        where T: BitXor<Output = T> {
            Vector<N, T, A>: BitXor,
            Vector<N, T, A>: BitXor<T>,
            Vector<N, T, A>: BitXorAssign,
            Vector<N, T, A>: BitXorAssign<T>,
        }
    }
);
