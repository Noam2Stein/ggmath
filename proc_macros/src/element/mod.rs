use quote::quote_spanned;
use syn::{parse_macro_input, spanned::Spanned, ItemImpl};

pub mod default_impl;

pub fn impl_element(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let tokens = parse_macro_input!(tokens as ItemImpl);
    let self_ty = &tokens.self_ty;

    quote_spanned! {
        self_ty.span() =>
        unsafe #tokens

        const _: () = {
            unsafe fn validate_vec2(value: ggmath::inner::Vec2Inner<#self_ty>) -> [$self; 2] { std::mem::transmute(value) }
            unsafe fn validate_vec3(value: ggmath::inner::Vec3Inner<#self_ty>) -> [$self; 4] { std::mem::transmute(value) }
            unsafe fn validate_vec4(value: ggmath::inner::Vec4Inner<#self_ty>) -> [$self; 4] { std::mem::transmute(value) }
        };
    }.into()
}