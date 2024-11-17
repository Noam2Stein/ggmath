use quote::quote;
use syn::{punctuated::Punctuated, Expr};

use super::*;

pub fn vec2(input: TokenStream1) -> TokenStream1 {
    let Input { exprs } = parse_macro_input!(input);

    quote! {
        ggmath::vector::Vec2::from_array(ggmath::vector::into_vec::IntoVector::into_vec_array((#exprs)))
    }
    .into()
}
pub fn vec3(input: TokenStream1) -> TokenStream1 {
    let Input { exprs } = parse_macro_input!(input);

    quote! {
        ggmath::vector::Vec3::from_array(ggmath::vector::into_vec::IntoVector::into_vec_array((#exprs)))
    }
    .into()
}
pub fn vec4(input: TokenStream1) -> TokenStream1 {
    let Input { exprs } = parse_macro_input!(input);

    quote! {
        ggmath::vector::Vec4::from_array(ggmath::vector::into_vec::IntoVector::into_vec_array((#exprs)))
    }
    .into()
}
pub fn vec2p(input: TokenStream1) -> TokenStream1 {
    let Input { exprs } = parse_macro_input!(input);

    quote! {
        ggmath::vector::Vec2P::from_array(ggmath::vector::into_vec::IntoVector::into_vec_array((#exprs)))
    }
    .into()
}
pub fn vec3p(input: TokenStream1) -> TokenStream1 {
    let Input { exprs } = parse_macro_input!(input);

    quote! {
        ggmath::vector::Vec3P::from_array(ggmath::vector::into_vec::IntoVector::into_vec_array((#exprs)))
    }
    .into()
}
pub fn vec4p(input: TokenStream1) -> TokenStream1 {
    let Input { exprs } = parse_macro_input!(input);

    quote! {
        ggmath::vector::Vec4P::from_array(ggmath::vector::into_vec::IntoVector::into_vec_array((#exprs)))
    }
    .into()
}

#[derive(Parse)]
struct Input {
    #[call(Punctuated::parse_terminated)]
    exprs: Punctuated<Expr, Token![,]>,
}
