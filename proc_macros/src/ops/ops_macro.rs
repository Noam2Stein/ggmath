use derive_syn_parse::Parse;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, Ident};

use super::op_fragments;

pub fn ops_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        macro_ident: Ident,
        macro_output: TokenTree,
    }

    let Input {
        macro_ident,
        macro_output,
    } = parse_macro_input!(tokens as Input);

    let op_fragments = op_fragments(
        quote! { $element_trait:ident },
        quote! { $std_trait:ident },
        quote! { $std_fn:ident },
        quote! { $vec_trait:ident },
        quote! { $vec_fn:ident },
        quote! { $vecnum_trait:ident },
    );

    quote! {
        macro_rules! #macro_ident {
            ($(#op_fragments), *) => #macro_output
        }
    }
    .into()
}
