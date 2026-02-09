use std::panic::{RefUnwindSafe, UnwindSafe};

use assert_impl_trait::assert_impl;
use ggmath::{Alignment, Length, Matrix, Scalar, SupportedLength};

assert_impl!(
    for<const N: usize, T, A: Alignment>
    where
        Length<N>: SupportedLength,
        T: Scalar,
    {
        Matrix<N, T, A>: Clone,
        Matrix<N, T, A>: Copy,

        for<'a> where T: 'a {
            Matrix<N, T, A>: 'a,
        }
        where T: Send {
            Matrix<N, T, A>: Send,
        }
        where T: Sync {
            Matrix<N, T, A>: Sync,
        }
        where T: Unpin {
            Matrix<N, T, A>: Unpin,
        }
        where T: UnwindSafe {
            Matrix<N, T, A>: UnwindSafe,
        }
        where T: RefUnwindSafe {
            Matrix<N, T, A>: RefUnwindSafe,
        }
    }
);
