use derive_syn_parse::Parse;
use proc_macro2::Literal;
use quote::quote;
use syn::{parse_macro_input, token::Paren, Attribute, Ident, Token, Type};

pub fn scalar_default_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        _lt,
        ty,
        _gt,
        _paren,
        size,
        _colon,
        attrs,
        vec2,
        _comma,
        vec4,
    } = parse_macro_input!(input as Input);

    quote! {
        impl gomath::scalar::default_impl::ScalarDefaultImpl for #ty {}

        gomath::vec::aligned_vecs!(
            <#ty>(#size):
            #(#attrs)*
            #vec2, #vec4
        );
    }
    .into()
}

#[derive(Parse)]
struct Input {
    _lt: Token![<],
    ty: Type,
    _gt: Token![>],
    #[paren]
    _paren: Paren,
    #[inside(_paren)]
    size: Literal,
    _colon: Token![:],
    #[call(Attribute::parse_outer)]
    attrs: Vec<Attribute>,
    vec2: Ident,
    _comma: Token![,],
    vec4: Ident,
}
