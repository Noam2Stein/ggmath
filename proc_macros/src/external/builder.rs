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
    builder(parse_str("Mat2").unwrap(), input)
}
pub fn mat2x3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x3").unwrap(), input)
}
pub fn mat2x4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x4").unwrap(), input)
}
pub fn mat3x2(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x2").unwrap(), input)
}
pub fn mat3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3").unwrap(), input)
}
pub fn mat3x4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x4").unwrap(), input)
}
pub fn mat4x2(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x2").unwrap(), input)
}
pub fn mat4x3(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x3").unwrap(), input)
}
pub fn mat4(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4").unwrap(), input)
}

pub fn mat2a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2A").unwrap(), input)
}
pub fn mat2x3a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x3A").unwrap(), input)
}
pub fn mat2x4a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat2x4A").unwrap(), input)
}
pub fn mat3x2a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x2A").unwrap(), input)
}
pub fn mat3a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3A").unwrap(), input)
}
pub fn mat3x4a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat3x4A").unwrap(), input)
}
pub fn mat4x2a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x2A").unwrap(), input)
}
pub fn mat4x3a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4x3A").unwrap(), input)
}
pub fn mat4a(input: TokenStream1) -> TokenStream1 {
    builder(parse_str("Mat4A").unwrap(), input)
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
