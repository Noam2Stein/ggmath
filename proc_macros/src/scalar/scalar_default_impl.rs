use super::*;

use quote::quote;
use syn::{token::Paren, Type};

pub fn scalar_default_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        ty: Type,
        #[paren]
        _paren_token: Paren,
        #[inside(_paren_token)]
        size: Literal,
    }
    let Input {
        ty,
        _paren_token,
        size,
    } = parse_macro_input!(input as Input);

    quote! {
        impl ggmath::scalar::default_impl::ScalarDefaultImpl for #ty {}

        ggmath::vec::inner::inner_vecs!(#ty(#size));
    }
    .into()
}
