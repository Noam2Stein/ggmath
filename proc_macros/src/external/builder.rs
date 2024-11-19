use quote::quote;
use syn::{punctuated::Punctuated, Expr, Path};

use super::*;

pub fn vec2(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Vec2").unwrap(), input)
}
pub fn vec3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Vec3").unwrap(), input)
}
pub fn vec4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Vec4").unwrap(), input)
}

pub fn vec2p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Vec2P").unwrap(), input)
}
pub fn vec3p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Vec3P").unwrap(), input)
}
pub fn vec4p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Vec4P").unwrap(), input)
}

pub fn mat2(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2R").unwrap(), input)
}
pub fn mat2x3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x3R").unwrap(), input)
}
pub fn mat2x4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x4R").unwrap(), input)
}
pub fn mat3x2(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x2R").unwrap(), input)
}
pub fn mat3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3R").unwrap(), input)
}
pub fn mat3x4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x4R").unwrap(), input)
}
pub fn mat4x2(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x2R").unwrap(), input)
}
pub fn mat4x3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x3R").unwrap(), input)
}
pub fn mat4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4R").unwrap(), input)
}

pub fn mat2p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2RP").unwrap(), input)
}
pub fn mat2x3p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x3RP").unwrap(), input)
}
pub fn mat2x4p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x4RP").unwrap(), input)
}
pub fn mat3x2p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x2RP").unwrap(), input)
}
pub fn mat3p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3RP").unwrap(), input)
}
pub fn mat3x4p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x4RP").unwrap(), input)
}
pub fn mat4x2p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x2RP").unwrap(), input)
}
pub fn mat4x3p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x3RP").unwrap(), input)
}
pub fn mat4p(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4RP").unwrap(), input)
}

pub fn mat2c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2C").unwrap(), input)
}
pub fn mat2x3c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x3C").unwrap(), input)
}
pub fn mat2x4c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x4C").unwrap(), input)
}
pub fn mat3x2c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x2C").unwrap(), input)
}
pub fn mat3c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3C").unwrap(), input)
}
pub fn mat3x4c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x4C").unwrap(), input)
}
pub fn mat4x2c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x2C").unwrap(), input)
}
pub fn mat4x3c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x3C").unwrap(), input)
}
pub fn mat4c(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4C").unwrap(), input)
}

pub fn mat2cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2CP").unwrap(), input)
}
pub fn mat2x3cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x3CP").unwrap(), input)
}
pub fn mat2x4cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x4CP").unwrap(), input)
}
pub fn mat3x2cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x2CP").unwrap(), input)
}
pub fn mat3cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3CP").unwrap(), input)
}
pub fn mat3x4cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x4CP").unwrap(), input)
}
pub fn mat4x2cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x2CP").unwrap(), input)
}
pub fn mat4x3cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x3CP").unwrap(), input)
}
pub fn mat4cp(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4CP").unwrap(), input)
}

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
