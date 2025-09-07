#[cfg(feature = "vector")]
pub(crate) mod vector;

#[cfg(feature = "primitive_aliases")]
mod primitive_aliases;
#[cfg(feature = "primitive_aliases")]
pub use primitive_aliases::*;
