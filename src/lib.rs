#![warn(missing_docs)]

//! graphics-math with generics and internal SIMD.
//!
//! - Fully generic (...Vector<Len, Scalar, Storage>)
//! - Optimized with SIMD internally
//! - Simple API (...FVec2)
//! - Optimal for GPU structs
//! - Optional additional types (...Rect, Ray)

pub mod construct;
pub mod scalar;
pub mod vec;
