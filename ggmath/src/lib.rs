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

mod alias_macros;
mod align;
mod construct;
mod primitives;
mod usize_;
pub use align::*;
pub use construct::*;
pub use primitives::*;
pub use usize_::*;

pub mod scalar;
pub mod vector;
pub use scalar::*;
pub use vector::*;

pub mod intergration;

macro_loop! {
    @for feature_mod in [matrix, quaternion, aabb] {
        #[cfg(feature = @[@feature_mod => str])]
        pub mod @feature_mod;

        #[cfg(feature = @[@feature_mod => str])]
        pub use @feature_mod::*;
    }
}

#[doc(hidden)]
pub use macro_loop::macro_loop;
