use super::*;

use syn::{
    punctuated::Punctuated, Block, ConstParam, FnArg, GenericParam, Generics, Pat, Receiver,
    Signature, Type, Visibility,
};

mod input;
mod output_impl_block;
mod output_scalar_trait;
mod search_replace;
use input::*;
use output_impl_block::*;
use output_scalar_trait::*;
use search_replace::*;

pub fn vector_interface(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as VecInterface);

    let impl_blocks = input.impls.iter().map(|r#impl| impl_block(&input, r#impl));
    let scalar = scalar_trait(&input);

    quote_spanned! {
        input.ident.span() =>

        #[allow(unused_imports)]
        use crate::vector::{alignment::*, length::*, *};

        #(#impl_blocks)*

        #scalar
    }
    .into()
}

fn generic_args(generics: &Generics) -> Vec<TokenStream> {
    generics
        .params
        .iter()
        .filter_map(|param| match param {
            GenericParam::Const(ConstParam {
                attrs: _,
                const_token,
                ident,
                colon_token,
                ty,
                eq_token: _,
                default: _,
            }) => Some(quote_spanned! { ident.span() => #const_token #ident #colon_token #ty }),
            GenericParam::Lifetime(_) => None,
            GenericParam::Type(param) => Some(param.ident.to_token_stream()),
        })
        .collect()
}

fn arg_ident(arg: &FnArg) -> Ident {
    match arg {
        FnArg::Receiver(_) => Ident::new("self", arg.span()),
        FnArg::Typed(arg) => match &*arg.pat {
            Pat::Ident(pat) => pat.ident.clone(),
            _ => panic!("non-ident arguments are not supported"),
        },
    }
}
