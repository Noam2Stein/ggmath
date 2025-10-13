#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod vector;
pub use vector::*;

#[cfg(feature = "primitive_aliases")]
mod primitive_aliases;
#[cfg(feature = "primitive_aliases")]
pub use primitive_aliases::*;

/// The base trait for all `ggmath` types.
/// This trait is automatically implemented for all types that are [`Copy`], [`'static`], [`Send`], and [`Sync`].
pub trait Construct: Copy + 'static + Send + Sync {}

impl<T: Copy + 'static + Send + Sync> Construct for T {}

#[doc(hidden)]
pub mod hidden {
    #[cfg(feature = "primitive_aliases")]
    pub use paste::paste;
}

mod sealed {
    pub trait Sealed {}
}
