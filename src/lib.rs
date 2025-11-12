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
