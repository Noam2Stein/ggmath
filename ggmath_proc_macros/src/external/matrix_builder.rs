use quote::{quote, quote_spanned};
use syn::{parse_quote, punctuated::Punctuated, spanned::Spanned, Expr, LitInt};

use super::*;

macro_rules! matrix_builder_macro {
    ($ident:ident($c:literal, $r:literal, $a:ident, $m:ident)) => {
        pub fn $ident(input: TokenStream1) -> TokenStream1 {
            matrix_builder(
                parse_quote!($c),
                parse_quote!($r),
                parse_quote!($a),
                parse_quote!($m),
                input,
            )
        }
    };
}
macro_rules! matrix_builder_macro_set {
    (($ident:ident, $ident_p:ident, $ident_c:ident, $ident_cp:ident)($c:literal, $r:literal)) => {
        matrix_builder_macro!($ident($c, $r, VecAligned, RowMajor));
        matrix_builder_macro!($ident_p($c, $r, VecPacked, RowMajor));
        matrix_builder_macro!($ident_c($c, $r, VecAligned, ColumnMajor));
        matrix_builder_macro!($ident_cp($c, $r, VecPacked, ColumnMajor));
    };
}

matrix_builder_macro_set!((mat2, mat2p, mat2c, mat2cp)(2, 2));
matrix_builder_macro_set!((mat2x3, mat2x3p, mat2x3c, mat2x3cp)(2, 3));
matrix_builder_macro_set!((mat2x4, mat2x4p, mat2x4c, mat2x4cp)(2, 4));
matrix_builder_macro_set!((mat3x2, mat3x2p, mat3x2c, mat3x2cp)(3, 2));
matrix_builder_macro_set!((mat3, mat3p, mat3c, mat3cp)(3, 3));
matrix_builder_macro_set!((mat3x4, mat3x4p, mat3x4c, mat3x4cp)(3, 4));
matrix_builder_macro_set!((mat4x2, mat4x2p, mat4x2c, mat4x2cp)(4, 2));
matrix_builder_macro_set!((mat4x3, mat4x3p, mat4x3c, mat4x3cp)(4, 3));
matrix_builder_macro_set!((mat4, mat4p, mat4c, mat4cp)(4, 4));

fn matrix_builder(c: LitInt, r: LitInt, a: Ident, m: Ident, input: TokenStream1) -> TokenStream1 {
    let Input { fields } = parse_macro_input!(input);

    let fields = fields.into_iter().map(|field| {
        if field.len() == 1 {
            field.to_token_stream()
        } else {
            quote_spanned! {
                field.span() =>

                ggmath::vector::VectorBuilder::<#c>::build::<ggmath::vector::#a>((#field))
            }
        }
    });

    quote! {
        ggmath::matrix::MatrixBuilder::<#c, #r>::build::<ggmath::vector::#a, ggmath::matrix::#m>((#(#fields), *))
    }
    .into()
}

#[derive(Parse)]
struct Input {
    #[call(|input| Punctuated::parse_terminated_with(input, Punctuated::parse_separated_nonempty))]
    fields: Punctuated<Punctuated<Expr, Token![,]>, Token![;]>,
}
