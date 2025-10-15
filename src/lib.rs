#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

pub mod vector;
pub use vector::*;

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

#[cfg(feature = "primitive_aliases")]
mod primitive_aliases;
#[cfg(feature = "primitive_aliases")]
pub use primitive_aliases::*;

/// The base trait for all `ggmath` types. Is automatically implemented for all
/// types that implement [`Send`], [`Sync`], [`Copy`], and [`'static`].
pub trait Construct: Send + Sync + Copy + 'static {}

impl<T: Send + Sync + Copy + 'static> Construct for T {}

mod sealed {
    pub trait Sealed {}
}

#[doc(hidden)]
pub mod hidden {
    pub use paste::paste;
}
