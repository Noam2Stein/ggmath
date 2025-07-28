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

macro_rules! feature_mod {
    ($feature:ident) => {
        macro_loop! {
            #[cfg(feature = @[$feature => str])]
            pub mod $feature;

            #[cfg(feature = @[$feature => str])]
            pub use $feature::*;
        }
    };
}
feature_mod! { matrix }
feature_mod! { quaternion }
feature_mod! { rectangle }

#[doc(hidden)]
pub use macro_loop::macro_loop;
