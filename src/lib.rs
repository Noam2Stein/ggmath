#![deny(missing_docs)]
#![deny(unconditional_recursion)]
#![doc = include_str!("../README.md")]

use std::panic::{RefUnwindSafe, UnwindSafe};

#[cfg(feature = "aliases")]
pub mod aliases;

#[path = "generated/primitive_aliases/mod.rs"]
#[cfg(feature = "primitive_aliases")]
mod primitive_aliases;
#[cfg(feature = "primitive_aliases")]
#[allow(unused_imports)]
pub use primitive_aliases::*;

#[cfg(feature = "vector")]
pub mod vector;
#[cfg(feature = "vector")]
pub use vector::*;

/// An auto trait for types that can be sent anywhere anytime.
///
/// This trait is required for all `ggmath` types,
/// like scalars, vectors, matrices, etc.
pub trait Construct: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe {}

/// A simple marker type that is generic over a `usize` constant.
///
/// This is used to implement traits for specific `usize` values.
/// As is used in vectors with the [`VecLen`][vector::VecLen] trait.
pub struct Usize<const N: usize>;

#[doc(hidden)]
pub mod _hidden_ {
    #[cfg(feature = "aliases")]
    pub use paste;
}

impl<T: Sized + Send + Sync + 'static + Copy + UnwindSafe + RefUnwindSafe> Construct for T {}
