#![deny(missing_docs)]

//! Generic-Graphics-Math with internal optimized SIMD.
//!
//! - Fully generic (Vector<Len, Scalar, Alignment>...).
//! - Optimized with SIMD internally.
//! - Simple API (FVec2...).
//! - Both column-major and row-major matricies.
//! - Num traits (FloatScalar...).
//! - Optimal for GPU structs.
//! - Optional additional types (Rect, Ray...).

mod alias_macros;
mod align;
mod construct;
mod primitives;
mod usize_;
pub use align::*;
pub use construct::*;
pub use primitives::*;
pub use usize_::*;

repetitive! {
    @for feature_mod in ['vector, 'matrix, 'quaternion, 'aabb] {
        #[cfg(feature = @str[feature_mod])]
        pub mod @feature_mod;

        #[cfg(feature = @str[feature_mod])]
        pub use @feature_mod::*;
    }
}

#[doc(hidden)]
pub use repetitive::repetitive;

// Crate integration

#[cfg(feature = "serde")]
mod serde_;

#[cfg(feature = "crevice")]
pub mod crevice_;
