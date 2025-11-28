#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod vector;
pub use vector::*;

#[cfg(feature = "matrix")]
pub mod matrix;
#[cfg(feature = "matrix")]
pub use matrix::*;

#[cfg(feature = "quaternion")]
pub mod quaternion;
#[cfg(feature = "quaternion")]
pub use quaternion::*;

#[cfg(feature = "affine")]
pub mod affine;
#[cfg(feature = "affine")]
pub use affine::*;

mod dir;
mod primitive_aliases;
pub use dir::*;
pub use primitive_aliases::*;

macro_rules! assertion {
    ($e:expr, $m:literal $(, $arg:expr)*$(,)?) => {
        #[cfg(any(feature = "assertions", all(feature = "debug_assertions", debug_assertions)))]
        assert!($e, $m $(, $arg)*);
    };
}

use assertion;
