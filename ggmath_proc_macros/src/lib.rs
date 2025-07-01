//! GGMath proc-macros.

//#![warn(missing_docs)]

use derive_syn_parse::Parse;
use proc_macro::TokenStream as TokenStream1;
use proc_macro2::*;
use quote::ToTokens;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Token,
};

// External Macros

mod external;

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

#[proc_macro]
#[inline(always)]
pub fn test_assert(input: TokenStream1) -> TokenStream1 {
    external::test_assert(input)
}

#[proc_macro]
#[inline(always)]
pub fn vec_test_assert(input: TokenStream1) -> TokenStream1 {
    external::vec_test_assert(input)
}

#[proc_macro]
#[inline(always)]
pub fn mat_test_assert(input: TokenStream1) -> TokenStream1 {
    external::mat_test_assert(input)
}

#[proc_macro]
#[inline(always)]
pub fn rect_test_assert(input: TokenStream1) -> TokenStream1 {
    external::rect_test_assert(input)
}

// Internal Macros

mod internal;

#[proc_macro]
#[inline(always)]
pub fn for_swizzles(input: TokenStream1) -> TokenStream1 {
    internal::for_swizzles(input)
}
