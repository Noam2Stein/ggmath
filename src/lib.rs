#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(any(not(feature = "std"), feature = "libm"), no_std)]

pub mod constants;

mod affine;
mod aliases;
mod alignment;
mod float_ext;
mod length;
mod mask;
mod matrix;
mod quaternion;
mod scalar;
mod vector;
pub use affine::*;
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
#[cfg(feature = "libm")]
mod libm;
mod repr;
mod safe_arch;
mod specialize;
mod third_party;
mod transmute;
