//! proc macros for GGMath.
//!
//! - Will be replaced by declarative macros once rust supports

#![warn(missing_docs)]

use derive_syn_parse::Parse;
use proc_macro2::*;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input, parse_quote_spanned, parse_str,
    spanned::Spanned,
    Error, Token,
};

macro_rules! export {
    ($(#[$meta:meta])* $ident:ident in $($path:ident)::*) => {
        $(#[$meta])*
        #[proc_macro]
        pub fn $ident(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $($path::)*$ident(tokens)
        }
    };
}

mod array_ext;
mod scalar;
mod vec;

export!(
/// expands into type aliases for vector, matricies... for a specific scalar type.
///
/// ```
/// scalar_aliases!($vis $(mod)? $scalar: $prefix);
/// ```
/// - if ```mod``` is written, the aliases will be placed inside a sub-module as public.
///
/// # Examples
/// ```
/// scalar_aliases!(pub f32: F);
/// // expands into:
/// // pub type FVector<const N: usize, S> = Vector<N, f32, S>;
/// // pub type FVec2 = Vec2<f32>;
/// // ...
/// ```
    scalar_aliases in scalar::scalar_aliases
);

export!(inner_vecs in vec::inner_vecs);

export!(vec_interface in vec::vec_interface);

export!(swizzles_macro in vec::swizzles);
export!(swizzles in vec::swizzles);
export!(non_repeat_swizzles in vec::swizzles);
