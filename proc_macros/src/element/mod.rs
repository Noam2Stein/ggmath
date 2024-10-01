use derive_syn_parse::Parse;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, spanned::Spanned, Token, Type};

pub fn impl_element_inner_vecs(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        t: Type,
        _0: Token![:],
        vec2: Type,
        _1: Token![,],
        vec3: Type,
        _2: Token![,],
        vec4: Type,
        _3: Option<Token![,]>,
    }

    let Input {
        t,
        _0,
        vec2,
        _1,
        vec3,
        _2,
        vec4,
        _3,
    } = parse_macro_input!(tokens as Input);

    let impl_sig = quote_spanned! {
        t.span() =>
        unsafe impl ggmath::element::ElementInnerVecs for #t
    };
    let impl_vec2 = quote_spanned! {
        vec2.span() =>
        type InnerVec2 = #vec2;
    };
    let impl_vec3 = quote_spanned! {
        vec3.span() =>
        type InnerVec3 = #vec3;
    };
    let impl_vec4 = quote_spanned! {
        vec4.span() =>
        type InnerVec4 = #vec4;
    };
    let validate_vec2 = quote_spanned! {
        vec2.span() =>
        unsafe fn validate_vec2(value: <#t as ggmath::element::ElementInnerVecs>::InnerVec2) -> [$self; 2] { std::mem::transmute(value) }
    };
    let validate_vec3 = quote_spanned! {
        vec3.span() =>
        unsafe fn validate_vec3(value: <#t as ggmath::element::ElementInnerVecs>::InnerVec3) -> [$self; 4] { std::mem::transmute(value) }
    };
    let validate_vec4 = quote_spanned! {
        vec4.span() =>
        unsafe fn validate_vec4(value: <#t as ggmath::element::ElementInnerVecs>::InnerVec4) -> [$self; 4] { std::mem::transmute(value) }
    };

    quote! {
        #impl_sig {
            #impl_vec2
            #impl_vec3
            #impl_vec4
        }
        const _: () = {
            #validate_vec2
            #validate_vec3
            #validate_vec4
        };
    }
    .into()
}
