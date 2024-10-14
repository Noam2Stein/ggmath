use std::hash::{DefaultHasher, Hash, Hasher};

use derive_syn_parse::Parse;
use proc_macro2::{Literal, Span};
use quote::quote;
use syn::{parse_macro_input, token::Paren, Ident, Type};

pub fn scalar_default_impl(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input { ty, _paren, size } = parse_macro_input!(input as Input);

    let aligned_vecs_mod = Ident::new(
        &format!("aligned_vecs_mod_{}", {
            let mut state = DefaultHasher::default();
            ty.hash(&mut state);
            state.finish()
        }),
        Span::call_site(),
    );

    quote! {
        impl gomath::scalar::default_impl::ScalarDefaultImpl for #ty {}

        mod #aligned_vecs_mod {
            use super::*;

            gomath::vec::inner::inner_vecs!(
                <#ty>(#size):
                DefaultImplAlignedVec2, DefaultImplAlignedVec4
            );
        }
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
