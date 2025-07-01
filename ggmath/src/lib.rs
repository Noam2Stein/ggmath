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
mod construct;
mod primitives;
pub use construct::*;
pub use primitives::*;

pub mod scalar;
pub mod vector;
pub use scalar::*;
pub use vector::*;

macro_rules! feature_mod {
    ($feature:ident) => {
        paste::paste! {
            #[cfg(feature = "" $feature "")]
            pub mod $feature;

            #[cfg(feature = "" $feature "")]
            pub use $feature::*;
        }
    };
}
feature_mod! { matrix }
feature_mod! { quaternion }
feature_mod! { rectangle }
feature_mod! { testing }

#[doc(hidden)]
pub use paste::paste;
