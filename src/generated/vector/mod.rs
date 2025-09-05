mod swizzle;
mod primitives;
mod transmute;
mod typematch;
mod ops;

#[cfg(any(
    feature = "right",
    feature = "backwards",
    feature = "down",
    feature = "forwards",
    feature = "left",
    feature = "up"
))]
mod dir;
#[cfg(any(
    feature = "right",
    feature = "backwards",
    feature = "down",
    feature = "forwards",
    feature = "left",
    feature = "up"
))]
pub use dir::*;
