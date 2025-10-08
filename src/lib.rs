//! TODO

#![deny(missing_docs)]

pub mod vector;
pub use vector::*;

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
