use derive_syn_parse::Parse;
use proc_macro2::{Literal, TokenStream};
use quote::quote;
use syn::{parse_macro_input, token::Paren, Type};

use crate::idents::*;

pub fn scalar_default_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, _paren, size } = parse_macro_input!(input as Input);

    let inner_vecs = TokenStream::from(crate::inner_vecs(
        quote! {
            #ty(#size)
        }
        .into(),
    ));

    quote! {
        impl #gomath::scalar::default_impl::#ScalarDefaultImpl for #ty {}

        #inner_vecs
    }
    .into()
}

#[derive(Parse)]
struct Input {
    ty: Type,
    #[paren]
    _paren: Paren,
    #[inside(_paren)]
    size: Literal,
}
