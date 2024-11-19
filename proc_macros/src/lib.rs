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

macro_rules! vector_macro {
    ($($fn_ident:ident), *) => {$(
        #[proc_macro]
        #[inline(always)]
        pub fn $fn_ident(input: TokenStream1) -> TokenStream1 {
            external::$fn_ident(input)
        }
    )*};
}
vector_macro!(vec2, vec3, vec4);

macro_rules! packed_vector_macro {
    ($($fn_ident:ident), *) => {$(
        #[proc_macro]
        #[inline(always)]
        pub fn $fn_ident(input: TokenStream1) -> TokenStream1 {
            external::$fn_ident(input)
        }
    )*};
}
packed_vector_macro!(vec2p, vec3p, vec4p);

macro_rules! matrix_macro {
    ($($fn_ident:ident), *) => {$(
        #[proc_macro]
        #[inline(always)]
        pub fn $fn_ident(input: TokenStream1) -> TokenStream1 {
            external::$fn_ident(input)
        }
    )*};
}
matrix_macro!(mat2, mat2x3, mat2x4, mat3x2, mat3, mat3x4, mat4x2, mat4x3, mat4);

macro_rules! packed_matrix_macro {
    ($($fn_ident:ident), *) => {$(
        #[proc_macro]
        #[inline(always)]
        pub fn $fn_ident(input: TokenStream1) -> TokenStream1 {
            external::$fn_ident(input)
        }
    )*};
}
packed_matrix_macro!(mat2p, mat2x3p, mat2x4p, mat3x2p, mat3p, mat3x4p, mat4x2p, mat4x3p, mat4p);

macro_rules! columnmajor_matrix_macro {
    ($($fn_ident:ident), *) => {$(
        #[proc_macro]
        #[inline(always)]
        pub fn $fn_ident(input: TokenStream1) -> TokenStream1 {
            external::$fn_ident(input)
        }
    )*};
}
columnmajor_matrix_macro!(
    mat2c, mat2x3c, mat2x4c, mat3x2c, mat3c, mat3x4c, mat4x2c, mat4x3c, mat4c
);

macro_rules! packed_columnmajor_matrix_macro {
    ($($fn_ident:ident), *) => {$(
        #[proc_macro]
        #[inline(always)]
        pub fn $fn_ident(input: TokenStream1) -> TokenStream1 {
            external::$fn_ident(input)
        }
    )*};
}
packed_columnmajor_matrix_macro!(
    mat2cp, mat2x3cp, mat2x4cp, mat3x2cp, mat3cp, mat3x4cp, mat4x2cp, mat4x3cp, mat4cp
);

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
