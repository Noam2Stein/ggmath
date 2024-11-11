use super::*;

use syn::{
    punctuated::Punctuated, token::Brace, Block, ConstParam, FnArg, GenericParam, Generics, Lit,
    LitInt, Pat, Receiver, Signature, Type, TypeParam, Visibility,
};

mod input;
mod output_alignment;
mod output_impl_block;
mod output_len;
mod output_scalar_trait;
mod search_replace;
use input::*;
use output_alignment::*;
use output_impl_block::*;
use output_len::*;
use output_scalar_trait::*;
use search_replace::*;

const LEN_STRS: [&str; 3] = ["2", "3", "4"];
const ALIGN_STRS: [&str; 2] = ["VecAligned", "VecPacked"];

fn scalar_fn_ident(ident: &Ident, n: &str, a: &str) -> Ident {
    Ident::new(
        &format!(
            "{}_vec{n}_{ident}",
            match a {
                "VecAligned" => "aligned",
                "VecPacked" => "packed",
                _ => unreachable!(),
            }
        ),
        ident.span(),
    )
}
fn len_trait_ident(input: &VecInterface) -> Ident {
    Ident::new(
        &format!("VecLen{}", input.scalar_trait.ident),
        input.scalar_trait.ident.span(),
    )
}
fn alignment_trait_ident(input: &VecInterface) -> Ident {
    Ident::new(
        &format!("VecAlignment{}", input.scalar_trait.ident),
        input.scalar_trait.ident.span(),
    )
}

pub fn vec_interface(input: TokenStream1) -> TokenStream1 {
    let input = parse_macro_input!(input as VecInterface);

    let errors = input.errors.iter().map(|err| err.to_compile_error());
    let impl_block = impl_block(&input);
    let scalar = scalar_trait(&input);
    let len = len(&input);
    let storage = alignment(&input);

    quote_spanned! {
        input.scalar_trait.ident.span() =>
        #[allow(unused_imports)]
        use crate::vector::{alignment::*, inner::*, length::*, *};

        const _: () = {
            #(#errors)*
        };
        #impl_block
        #scalar
        #len
        #storage
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

fn with_span<T: ToTokens + Parse>(tokens: &T, span: Span) -> T {
    parse2(
        tokens
            .to_token_stream()
            .clone()
            .into_iter()
            .map(|token| match token {
                TokenTree::Group(token) => {
                    let mut token = Group::new(token.delimiter(), with_span(&token.stream(), span));
                    token.set_span(span);

                    TokenTree::Group(token)
                }
                TokenTree::Ident(mut token) => {
                    token.set_span(span);

                    TokenTree::Ident(token)
                }
                TokenTree::Literal(mut token) => {
                    token.set_span(span);

                    TokenTree::Literal(token)
                }
                TokenTree::Punct(mut token) => {
                    token.set_span(span);

                    TokenTree::Punct(token)
                }
            })
            .collect::<TokenStream>(),
    )
    .unwrap()
}
