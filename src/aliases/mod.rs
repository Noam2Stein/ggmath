//! A module that contains:
//! - type aliases for math types.
//! - macros that generate scalar specific type aliases.
//!
//! # Vector naming scheme
//!
//! `Vec{N}` like `Vec2` means an aligned vector,
//! while `Vec{N}P` like `Vec2P` means a packed vector.
//! `VecAligned` is always considered the default.
//!
//! # Matrix naming scheme
//!
//! `Mat{C}x{R}C` like `Mat3x2C` means a column-major matrix with `C` columns and `R` rows,
//! while `Mat{C}x{R}R` like `Mat3x2R` means a row-major matrix with `C` columns and `R` rows.
//!
//! Square matrices use `Mat{N}{major-axis}` like `Mat3C`, not `Mat3x3C`.
//! Packed matrices use `Mat{C}x{R}{major-axis}P` like `Mat3x2CP`.

#[cfg(feature = "vector")]
mod vector;
#[cfg(feature = "vector")]
pub use vector::*;
