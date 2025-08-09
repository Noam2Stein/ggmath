#![deny(missing_docs)]
#![doc = include_str!("../README.md")]

use repetitive::repetitive;

mod alias_macros;
mod align;
mod construct;
mod dir;
mod primitives;
mod usize_;
pub use align::*;
pub use construct::*;
pub use dir::*;
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
pub mod _private_ {
    pub use repetitive::repetitive;
}

// Crate integration

#[cfg(feature = "serde")]
mod serde_;

#[cfg(feature = "crevice")]
pub mod crevice_;
