//#![warn(missing_docs)]

//! Generic-Graphics-Math with internal optimized SIMD.
//!
//! - Fully generic (Vector<Len, Scalar, Alignment>...).
//! - Optimized with SIMD internally.
//! - Simple API (FVec2...).
//! - Both column-major and row-major matricies.
//! - Num traits (FloatScalar...).
//! - Optimal for GPU structs.
//! - Optional additional types (Rect, Ray...).

mod abs_diff;
pub use abs_diff::*;
pub mod builder;

mod construct;
pub use construct::*;

pub mod scalar;
pub mod vector;

#[cfg(feature = "primitive_aliases")]
pub mod primitive_aliases;

#[cfg(feature = "matrix")]
pub mod matrix;

#[cfg(feature = "quaternion")]
pub mod quaternion;

#[cfg(feature = "rect")]
pub mod rectangle;

#[cfg(feature = "line")]
pub mod line;

#[cfg(feature = "ray")]
pub mod ray;

use crate as ggmath;
