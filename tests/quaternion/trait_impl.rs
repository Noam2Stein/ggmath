use std::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use assert_impl_trait::assert_impl;
use ggmath::{
    Alignment, Quaternion, Scalar,
    constants::{One, Zero},
};

assert_impl!(
    for<T, A: Alignment>
    where
        T: Scalar,
    {
        Quaternion<T, A>: Clone,
        Quaternion<T, A>: Copy,

        for<'a> where T: 'a {
            Quaternion<T, A>: 'a,
        }
        where T: Send {
            Quaternion<T, A>: Send,
        }
        where T: Sync {
            Quaternion<T, A>: Sync,
        }
        where T: Unpin {
            Quaternion<T, A>: Unpin,
        }
        where T: UnwindSafe {
            Quaternion<T, A>: UnwindSafe,
        }
        where T: RefUnwindSafe {
            Quaternion<T, A>: RefUnwindSafe,
        }
        where T: Debug {
            Quaternion<T, A>: Debug,
        }
        where T: Display {
            Quaternion<T, A>: Display,
        }
        where T: PartialEq {
            Quaternion<T, A>: PartialEq,
        }
        where T: Eq {
            Quaternion<T, A>: Eq,
        }
        where T: Hash {
            Quaternion<T, A>: Hash,
        }
        where T: Zero + One {
            Quaternion<T, A>: Default,
        }

        where T: Neg<Output = T> {
            Quaternion<T, A>: Neg,
        }
        where T: Add<Output = T> {
            Quaternion<T, A>: Add,
            Quaternion<T, A>: AddAssign,
        }
        where T: Sub<Output = T> {
            Quaternion<T, A>: Sub,
            Quaternion<T, A>: SubAssign,
        }
        where T: Mul<Output = T> {
            Quaternion<T, A>: Mul<T>,
            Quaternion<T, A>: MulAssign<T>,
        }
    }
);
