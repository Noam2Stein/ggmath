use quote::quote;
use syn::{parse_quote, punctuated::Punctuated, Expr, Path};

use super::*;

macro_rules! builder_macros {
    ($($macro_ident:ident($ty:path)), * $(,)?) => {$(
        pub fn $macro_ident(input: TokenStream1) -> TokenStream1 {
            builder(parse_quote!($ty), input)
        }
    )*};
}

builder_macros!(
    vec2(ggmath::vector::Vec2),
    vec3(ggmath::vector::Vec3),
    vec4(ggmath::vector::Vec4),
    vec2p(ggmath::vector::Vec2P),
    vec3p(ggmath::vector::Vec3P),
    vec4p(ggmath::vector::Vec4P),
    mat2(ggmath::matrix::Mat2R),
    mat2x3(ggmath::matrix::Mat2x3R),
    mat2x4(ggmath::matrix::Mat2x4R),
    mat3x2(ggmath::matrix::Mat3x2R),
    mat3(ggmath::matrix::Mat3R),
    mat3x4(ggmath::matrix::Mat3x4R),
    mat4x2(ggmath::matrix::Mat4x2R),
    mat4x3(ggmath::matrix::Mat4x3R),
    mat4(ggmath::matrix::Mat4R),
    mat2c(ggmath::matrix::Mat2C),
    mat2x3c(ggmath::matrix::Mat2x3C),
    mat2x4c(ggmath::matrix::Mat2x4C),
    mat3x2c(ggmath::matrix::Mat3x2C),
    mat3c(ggmath::matrix::Mat3C),
    mat3x4c(ggmath::matrix::Mat3x4C),
    mat4x2c(ggmath::matrix::Mat4x2C),
    mat4x3c(ggmath::matrix::Mat4x3C),
    mat4c(ggmath::matrix::Mat4C),
    mat2p(ggmath::matrix::Mat2RP),
    mat2x3p(ggmath::matrix::Mat2x3RP),
    mat2x4p(ggmath::matrix::Mat2x4RP),
    mat3x2p(ggmath::matrix::Mat3x2RP),
    mat3p(ggmath::matrix::Mat3RP),
    mat3x4p(ggmath::matrix::Mat3x4RP),
    mat4x2p(ggmath::matrix::Mat4x2RP),
    mat4x3p(ggmath::matrix::Mat4x3RP),
    mat4p(ggmath::matrix::Mat4RP),
    mat2cp(ggmath::matrix::Mat2CP),
    mat2x3cp(ggmath::matrix::Mat2x3CP),
    mat2x4cp(ggmath::matrix::Mat2x4CP),
    mat3x2cp(ggmath::matrix::Mat3x2CP),
    mat3cp(ggmath::matrix::Mat3CP),
    mat3x4cp(ggmath::matrix::Mat3x4CP),
    mat4x2cp(ggmath::matrix::Mat4x2CP),
    mat4x3cp(ggmath::matrix::Mat4x3CP),
    mat4cp(ggmath::matrix::Mat4CP),
);

fn builder(ty: Path, input: TokenStream1) -> TokenStream1 {
    let Input { segments } = parse_macro_input!(input);
    let exprs = segments.iter().map(|segment| &segment.exprs);

    if segments.len() == 1 {
        quote! {
            {
                use ggmath::builder::FromBuilder;
                #ty::from_builder((#(#exprs)*))
            }
        }
        .into()
    } else {
        quote! {
            {
                use ggmath::builder::FromBuilder;
                #ty::from_builder((#(
                    FromBuilder::from_builder((#exprs))
                ), *))
            }
        }
        .into()
    }
}

#[derive(Parse)]
struct Input {
    #[call(Punctuated::parse_terminated)]
    segments: Punctuated<InputSegment, Token![;]>,
}
#[derive(Parse)]
struct InputSegment {
    #[call(Punctuated::parse_separated_nonempty)]
    exprs: Punctuated<Expr, Token![,]>,
}
