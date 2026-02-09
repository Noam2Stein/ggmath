#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(any(not(feature = "std"), feature = "libm"), no_std)]

/*
When enough float functions move to `core`, the crate should be made always
`no_std` and the cargo features `std` and `libm` should be removed.
*/

pub mod constants;

mod aliases;
mod alignment;
mod float_ext;
mod length;
mod mask;
mod matrix;
mod quaternion;
mod scalar;
mod vector;
pub use aliases::*;
pub use alignment::*;
pub use float_ext::*;
pub use length::*;
pub use mask::*;
pub use matrix::*;
pub use quaternion::*;
pub use scalar::*;
pub use vector::*;

mod backend_impls;
mod integrations;
mod utils;

#[cfg(feature = "libm")]
mod libm;
