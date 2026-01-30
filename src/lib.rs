#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(any(not(feature = "std"), feature = "libm"), no_std)]

pub mod constants;

mod aliases;
mod alignment;
mod float_ext;
mod length;
mod mask;
mod scalar;
mod vector;
pub use aliases::*;
pub use alignment::*;
pub use float_ext::*;
pub use length::*;
pub use mask::*;
pub use scalar::*;
pub use vector::*;

mod backend_impls;
mod integrations;
mod utils;

#[cfg(feature = "libm")]
mod libm;
