#![deny(missing_docs)]
#![doc = include_str!("../README.md")]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod constants;

mod aliases;
mod alignment;
mod length;
mod scalar;
mod vector;
pub use aliases::*;
pub use alignment::*;
pub use length::*;
pub use scalar::*;
pub use vector::*;

mod backend_impls;
mod utils;
