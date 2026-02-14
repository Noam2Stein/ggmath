use std::panic::{RefUnwindSafe, UnwindSafe};

use assert_impl_trait::assert_impl;
use ggmath::{Affine, Alignment, Length, Scalar, SupportedLength};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Affine<N, T, A>: Clone,
        Affine<N, T, A>: Copy,

        for<'a> where T: 'a {
            Affine<N, T, A>: 'a,
        }
        where T: Send {
            Affine<N, T, A>: Send,
        }
        where T: Sync {
            Affine<N, T, A>: Sync,
        }
        where T: Unpin {
            Affine<N, T, A>: Unpin,
        }
        where T: UnwindSafe {
            Affine<N, T, A>: UnwindSafe,
        }
        where T: RefUnwindSafe {
            Affine<N, T, A>: RefUnwindSafe,
        }
    }
);
