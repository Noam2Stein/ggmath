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

#[proc_macro]
#[inline(always)]
pub fn vec2(input: TokenStream1) -> TokenStream1 {
    external::vec2(input)
}
#[proc_macro]
#[inline(always)]
pub fn vec3(input: TokenStream1) -> TokenStream1 {
    external::vec3(input)
}
#[proc_macro]
#[inline(always)]
pub fn vec4(input: TokenStream1) -> TokenStream1 {
    external::vec4(input)
}
#[proc_macro]
#[inline(always)]
pub fn vec2p(input: TokenStream1) -> TokenStream1 {
    external::vec2p(input)
}
#[proc_macro]
#[inline(always)]
pub fn vec3p(input: TokenStream1) -> TokenStream1 {
    external::vec3p(input)
}
#[proc_macro]
#[inline(always)]
pub fn vec4p(input: TokenStream1) -> TokenStream1 {
    external::vec4p(input)
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
pub fn collect_vec_interfaces(input: TokenStream1) -> TokenStream1 {
    internal::collect_vec_interfaces(input)
}

#[proc_macro]
#[inline(always)]
pub fn for_swizzles(input: TokenStream1) -> TokenStream1 {
    internal::for_swizzles(input)
}

#[proc_macro]
#[inline(always)]
pub fn for_self_ops(input: TokenStream1) -> TokenStream1 {
    internal::for_self_ops(input)
}
#[proc_macro]
#[inline(always)]
pub fn for_rhs_ops(input: TokenStream1) -> TokenStream1 {
    internal::for_rhs_ops(input)
}
#[proc_macro]
#[inline(always)]
pub fn for_assign_ops(input: TokenStream1) -> TokenStream1 {
    internal::for_assign_ops(input)
}
