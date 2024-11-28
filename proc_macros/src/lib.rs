//! GGMath proc-macros.

#![warn(missing_docs)]

use derive_syn_parse::Parse;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::*;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_str, Error, Token,
};

// External Macros

mod external;

#[proc_macro]
#[inline(always)]
pub fn vector_aliases(input: TokenStream1) -> TokenStream1 {
    external::vector_aliases(input)
}
#[proc_macro]
#[inline(always)]
pub fn matrix_aliases(input: TokenStream1) -> TokenStream1 {
    external::matrix_aliases(input)
}
#[proc_macro]
#[inline(always)]
pub fn rectangle_aliases(input: TokenStream1) -> TokenStream1 {
    external::rectangle_aliases(input)
}

#[proc_macro]
#[inline(always)]
pub fn inner_vectors(input: TokenStream1) -> TokenStream1 {
    external::inner_vectors(input)
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
