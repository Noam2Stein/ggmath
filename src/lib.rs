#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod vector;
pub use vector::*;

mod primitive_aliases;
pub use primitive_aliases::*;

#[cfg(any(
    feature = "right",
    feature = "left",
    feature = "up",
    feature = "down",
    feature = "forwards",
    feature = "backwards"
))]
mod dir;
#[cfg(any(
    feature = "right",
    feature = "left",
    feature = "up",
    feature = "down",
    feature = "forwards",
    feature = "backwards"
))]
pub use dir::*;

macro_rules! ggmath_assert {
    ($e:expr, $msg:literal $(, $arg:expr)* $(,)?) => {
        #[allow(unused_braces)]
        {
            #[cfg(any(feature = "assert", all(feature = "debug_assert", debug_assertions)))]
            assert!($e, $msg, $($arg),*);
        }
    };
}

use ggmath_assert;
