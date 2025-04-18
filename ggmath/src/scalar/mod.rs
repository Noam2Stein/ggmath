//! Trait for types that can be put inside math-types like ```Vector``` and ```Matrix```.
//! For example: [```f32```], [```u8```] and [```bool```] are scalars.

use std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use splat_attribs::splat_attribs;

use crate::{vector::*, *};

#[cfg(feature = "newnum")]
use ::newnum::*;

mod positive_dir;
mod wrapper;
pub use positive_dir::*;
pub use wrapper::*;

pub use ggmath_proc_macros::scalar_inner_vectors;

mod primitive_impls;

/// Specifies the inner aligned-vector types for a scalar type,
/// because Rust doesn't have a type which is generic over alignment.
///
/// Required by ```Scalar```.
/// Use the [```scalar_inner_vectors```] macro to implement this correctly.
pub unsafe trait ScalarInnerAlignedVecs {
    type InnerAlignedVec2: Construct;
    type InnerAlignedVec4: Construct;
}

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
pub trait Scalar: Construct + ScalarInnerAlignedVecs {
    // **************************************************
    // ********************* Vector *********************
    // **************************************************

    // STD
    scalar_defaults_vector_eq! {}
    scalar_defaults_vector_default! {}
    scalar_defaults_vector_ops! {}

    // Core
    scalar_defaults_vector_splat! {}
    scalar_defaults_vector_get! {}
    scalar_defaults_vector_with! {}

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

scalar_inner_vectors!(std::cmp::Ordering(1));

impl Scalar for std::cmp::Ordering {}
