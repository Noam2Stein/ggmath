//! Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
//! For example: [```f32```], [```u8```] and [```bool```] are scalars.

use std::ops::{Add, Mul, Sub};

use splat_attribs::splat_attribs;

use super::*;

#[cfg(feature = "newnum")]
use ::newnum::*;

mod positive_dir;
pub use positive_dir::*;

/// Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
/// For example: [```f32```], [```u8```] and [```bool```] are scalars.
///
/// - References are NOT scalars.
///
/// ### Implementing this trait
///
/// To implement this trait for a type that wraps another ```Scalar``` type
/// (for example ```struct Meters(f32);```),
/// use the inexistant wrapper system.
///
/// To implement this trait for a unique type use this example:
///
/// ```
/// struct u256(u128, u128);
///
/// scalar_inner_vectors!(u256(32)); // 32 - size in bytes
///
/// impl Scalar for u256 {}
/// ```
pub trait Scalar: Construct {
    type Vec2Alignment: Construct;
    type Vec3Alignment: Construct;
    type Vec4Alignment: Construct;

    // **************************************************
    // ********************* Vector *********************
    // **************************************************

    // Ext

    scalar_defaults_vector_ext_cmp! {}
    scalar_defaults_vector_ext_ops! {}

    // Newnum
    splat_attribs! {
        #[cfg(feature = "newnum")]:

        scalar_defaults_vector_abs_diff! {}
        scalar_defaults_vector_round! {}
        scalar_defaults_vector_sign! {}
        scalar_defaults_vector_trig! {}
        scalar_defaults_vector_whole_equivalent! {}
    }

    // ********************************************************************************
    // ********************************************************************************
    // ************************************* API **************************************
    // ********************************************************************************
    // ********************************************************************************
}

#[macro_export(local_inner_macros)]
macro_rules! scalar_defaults_macro {
    ($macro_ident:ident: $($tt:tt)*) => {
        #[macro_export(local_inner_macros)]
        macro_rules! $macro_ident {
            () => {
                $($tt)*
            }
        }
    };
}
