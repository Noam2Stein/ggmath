use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Not},
    panic::{RefUnwindSafe, UnwindSafe},
};

use assert_impl_trait::assert_impl;
use ggmath::{Alignment, Length, Mask, Scalar, SupportedLength};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Mask<N, T, A>: Send,
        Mask<N, T, A>: Sync,
        Mask<N, T, A>: Unpin,
        Mask<N, T, A>: UnwindSafe,
        Mask<N, T, A>: RefUnwindSafe,
        Mask<N, T, A>: Debug,
        Mask<N, T, A>: Clone,
        Mask<N, T, A>: Copy,
        Mask<N, T, A>: PartialEq,
        Mask<N, T, A>: Eq,
        Mask<N, T, A>: Hash,
        Mask<N, T, A>: Default,
        Mask<N, T, A>: Display,
        Mask<N, T, A>: IntoIterator<Item = bool>,
        Mask<N, T, A>: Not,
        Mask<N, T, A>: BitAnd,
        Mask<N, T, A>: BitAnd<bool>,
        Mask<N, T, A>: BitAndAssign,
        Mask<N, T, A>: BitAndAssign<bool>,
        Mask<N, T, A>: BitOr,
        Mask<N, T, A>: BitOr<bool>,
        Mask<N, T, A>: BitOrAssign,
        Mask<N, T, A>: BitOrAssign<bool>,
        Mask<N, T, A>: BitXor,
        Mask<N, T, A>: BitXor<bool>,
        Mask<N, T, A>: BitXorAssign,
        Mask<N, T, A>: BitXorAssign<bool>,

        for<'a> where T: 'a {
            Mask<N, T, A>: 'a,
            &'a Mask<N, T, A>: IntoIterator<Item = bool>,
        }
    }
);
