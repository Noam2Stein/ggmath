//! proc macros for GGMath.
//!
//! - Will be replaced by declarative macros once rust supports

//#![warn(missing_docs)]

macro_rules! export {
    ($(#[$meta:meta])* $ident:ident) => {
        $(#[$meta])*
        #[proc_macro]
        pub fn $ident(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
            $ident::$ident(tokens)
        }
    };
}

mod utils;
use utils::*;

use derive_syn_parse::Parse;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::*;
use quote::{quote_spanned, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream},
    parse2, parse_macro_input, parse_quote_spanned, parse_str,
    spanned::Spanned,
    Error, Token,
};

mod assign_ops;
mod inner_vecs;

mod interfaces_mod_traits;

mod rhs_ops;

mod scalar_aliases;
mod self_ops;
mod swizzles;
mod vec_interface;

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
    scalar_aliases
);

export!(inner_vecs);

export!(vec_interface);

export!(swizzles);

export!(interfaces_mod_traits);

export!(self_ops);
export!(rhs_ops);
export!(assign_ops);
