//! proc macros for GGMath.

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
            $($path::)*$ident::$ident(tokens)
        }
    };
}

mod scalar;
mod vec;

export!(
/// Implements ```Scalar``` for the given type using the default implementation.
///
/// ```
/// scalar_default_impl!($type($size_in_bytes));
/// ```
///
/// - ```$type``` has to be: Construct + PartialOrd.
/// - if ```$size_in_bytes``` is incorrect the code will not compile.
///
/// # Examples
/// ```
/// #[derive(Debug, Clone, Copy, PartialEq, Default)]
/// struct Fraction {
///     numerator: f32,
///     denominator: f32,
/// }
/// scalar_default_impl!(Fraction(8));
/// ```
    scalar_default_impl in scalar
);

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
    scalar_aliases in scalar
);

export!(inner_vecs in vec);

export!(vec_api in vec);
