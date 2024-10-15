use proc_macro2::{Literal, Span, TokenStream};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Error, Ident, ItemFn, Token, TraitItemFn, Visibility,
};

use crate::idents::*;

mod r#fn;
mod generics;
mod perspective;
mod traits;
mod ty;

use perspective::*;
use r#fn::*;
use traits::*;

pub fn vec_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let input = match ProcessedInput::try_from(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let impl_block = vec(&input);
    let scalar = scalar(&input);
    let len = len(&input);
    let storage = alignment(&input);

    quote! {
        #impl_block
        #scalar
        #len
        #storage
    }
    .into()
}

fn vec(input: &ProcessedInput) -> TokenStream {
    let impl_abstract_fns = input.abstract_fns.iter().cloned().map(|abstract_fn| {
        abstract_fn.impl_from_perspective(
            &input.ident,
            &Perspective::Vec,
            |input| input.pass_inner(),
            |expr, output| output.wrap_inner(expr),
        )
    });

    let impl_free_fns = input.free_fns.iter();

    quote! {
        impl<const #N: usize, #T: #Scalar, #A: #VecAlignment> #Vector<#N, #T, #A> where #ScalarCount<#N>: #VecLen<#N> {
            #(
                #impl_abstract_fns
            )*
            #(
                #impl_free_fns
            )*
        }
    }
}
fn scalar(input: &ProcessedInput) -> TokenStream {
    let trait_ident = scalar_trait_ident(&input.ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.from_perspective(&Perspective::Scalar));

    quote! {
        pub trait #trait_ident<const #N: usize, #A: #VecAlignment>: inner::#ScalarInnerVecs where #ScalarCount<#N>: #VecLen<#N> {
            #(
                #fns
            )*
        }
    }
}
fn len(input: &ProcessedInput) -> TokenStream {
    let trait_ident = len_trait_ident(&input.ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.from_perspective(&Perspective::Len(None)));

    let impls = (2..5).map(|n| {
        let n = Literal::usize_unsuffixed(n);

        let fns = input.abstract_fns.iter().cloned().map(|abstract_fn| {
            abstract_fn.impl_from_perspective(
                &input.ident,
                &Perspective::Len(Some(n.clone())),
                |input| input.pass(),
                |expr, _| expr,
            )
        });

        quote! {
            impl #trait_ident<#n> for ScalarCount<#n> {
                #(
                    #fns
                )*
            }
        }
    });

    quote! {
        pub(super) trait #trait_ident<const N: usize>: inner::VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
        #(
            #impls
        )*
    }
}
fn alignment(input: &ProcessedInput) -> TokenStream {
    let trait_ident = alignment_trait_ident(&input.ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.from_perspective(&Perspective::Alignment(None)));

    let impls = (2..5)
        .map(|n| {
            let n = Literal::usize_unsuffixed(n);

            let fns = input
                .abstract_fns
                .iter()
                .cloned()
                .map(|abstract_fn| {
                    abstract_fn.impl_from_perspective(
                        &input.ident,
                        &Perspective::Alignment(Some(n.clone())),
                        |input| input.pass(),
                        |expr, _| expr,
                    )
                })
                .collect::<Box<[ItemFn]>>();

            ["VecPacked", "VecAligned"].map(|s| {
                let s = Ident::new(s, Span::call_site());

                quote! {
                    impl #trait_ident<#n> for #s {
                        #(
                            #fns
                        )*
                    }
                }
            })
        })
        .flatten();

    quote! {
        pub(super) trait #trait_ident<const N: usize>: inner::VecStorageInnerVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
        #(
            #impls
        )*
    }
}

struct Input {
    ident: Ident,
    fns: Vec<TraitItemFn>,
}
impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let mut fns = Vec::new();

        while !input.is_empty() {
            fns.push(input.parse()?);
        }

        Ok(Self { ident, fns })
    }
}

#[derive(Clone)]
struct ProcessedInput {
    ident: Ident,
    abstract_fns: Vec<AbstractApiFn>,
    free_fns: Vec<ItemFn>,
}
impl TryFrom<Input> for ProcessedInput {
    type Error = Error;
    fn try_from(Input { ident, fns }: Input) -> Result<Self, Self::Error> {
        let mut abstract_fns = Vec::new();
        let mut free_fns = Vec::new();

        for fn_ in fns {
            if let Some(block) = fn_.default {
                free_fns.push(ItemFn {
                    attrs: fn_.attrs,
                    vis: Visibility::Public(Default::default()),
                    sig: fn_.sig,
                    block: Box::new(block),
                });
            } else {
                abstract_fns.push(fn_.try_into()?);
            }
        }

        Ok(Self {
            ident,
            abstract_fns,
            free_fns,
        })
    }
}
