//! GGMath proc macros.

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

mod utils;
use utils::*;

// External Macros

mod external;

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
/// // pub type FVector<const N: usize, A> = Vector<N, f32, A>;
/// // pub type FVec2 = Vec2<f32>;
/// // ...
/// ```
#[proc_macro]
#[inline(always)]
pub fn scalar_aliases(input: TokenStream1) -> TokenStream1 {
    external::scalar_aliases(input)
}

#[proc_macro]
#[inline(always)]
pub fn inner_vecs(input: TokenStream1) -> TokenStream1 {
    external::inner_vecs(input)
}

// Internal Macros

mod internal;

#[proc_macro]
#[inline(always)]
pub fn vec_interface(input: TokenStream1) -> TokenStream1 {
    internal::vec_interface(input)
}
#[proc_macro]
#[inline(always)]
pub fn interfaces_mod_traits(input: TokenStream1) -> TokenStream1 {
    internal::interfaces_mod_traits(input)
}

#[proc_macro]
#[inline(always)]
pub fn swizzles(input: TokenStream1) -> TokenStream1 {
    internal::swizzles(input)
}

#[proc_macro]
#[inline(always)]
pub fn self_ops(input: TokenStream1) -> TokenStream1 {
    internal::self_ops(input)
}
#[proc_macro]
#[inline(always)]
pub fn rhs_ops(input: TokenStream1) -> TokenStream1 {
    internal::rhs_ops(input)
}
#[proc_macro]
#[inline(always)]
pub fn assign_ops(input: TokenStream1) -> TokenStream1 {
    internal::assign_ops(input)
}
