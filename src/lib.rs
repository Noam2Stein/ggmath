#![warn(missing_docs)]

//! Generic-Graphics-Math with internal optimized SIMD.
//!
//! - Fully generic (Vector<Len, Scalar, Alignment>...).
//! - Optimized with SIMD internally.
//! - Simple API (FVec2...).
//! - Both column-major and row-major matricies.
//! - Num traits (FloatScalar...).
//! - Optimal for GPU structs.
//! - Optional additional types (Rect, Ray...).

pub mod construct;
pub mod primitive_aliases;
pub mod scalar;
pub mod vec;
