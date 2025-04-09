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

mod construct;
pub use construct::*;

pub mod scalar;
pub mod vector;

#[cfg(feature = "matrix")]
pub mod matrix;

#[cfg(feature = "quaternion")]
pub mod quaternion;

#[cfg(feature = "rectangle")]
pub mod rectangle;

#[cfg(feature = "testing")]
pub mod testing;

use crate as ggmath;
