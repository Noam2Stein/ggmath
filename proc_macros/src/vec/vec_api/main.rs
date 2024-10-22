use super::{perspective::*, r#fn::*, traits::*, *};

use syn::{Attribute, ItemFn, TraitItemFn, Visibility};

pub fn vec_api(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as Input);

    let input = match ProcessedInput::try_from(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };

    let errors = input.errors.iter().map(|err| err.to_compile_error());
    let impl_block = vec(&input);
    let scalar = scalar(&input);
    let len = len(&input);
    let storage = alignment(&input);

    quote_spanned! {
        input.api_ident.span() =>
        use crate::vec::api::prelude::*;

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

fn vec(input: &ProcessedInput) -> TokenStream {
    let impl_abstract_fns =
        input.abstract_fns.iter().cloned().map(|abstract_fn| {
            abstract_fn.impl_from_perspective(&input.api_ident, Perspective::Vec)
        });

    let impl_free_fns = input.free_fns.iter();

    quote_spanned! {
        input.api_ident.span() =>
        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A> where ScalarCount<N>: VecLen<N> {
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
    let attrs = &input.api_attrs;
    let trait_ident = scalar_trait_ident(&input.api_ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.from_perspective(Perspective::Scalar));

    quote_spanned! {
        trait_ident.span() =>
        #(#attrs)*
        pub trait #trait_ident<const N: usize, A: VecAlignment>: inner::ScalarInnerVecs where ScalarCount<N>: VecLen<N> {
            #(
                #fns
            )*
        }
    }
}
fn len(input: &ProcessedInput) -> TokenStream {
    let trait_ident = len_trait_ident(&input.api_ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.from_perspective(Perspective::Len(None)));

    let impls = (2..5).map(|n| {
        let n = KnownN(n);

        let fns = input.abstract_fns.iter().cloned().map(|abstract_fn| {
            abstract_fn.impl_from_perspective(&input.api_ident, Perspective::Len(Some(n)))
        });

        quote_spanned! {
            input.api_ident.span() =>
            impl #trait_ident<#n> for ScalarCount<#n> {
                #(
                    #fns
                )*
            }
        }
    });

    quote_spanned! {
        trait_ident.span() =>
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
    let trait_ident = alignment_trait_ident(&input.api_ident);

    let fns = input
        .abstract_fns
        .iter()
        .cloned()
        .map(|abstract_fn| abstract_fn.from_perspective(Perspective::Alignment(None)));

    let impls = (2..5)
        .map(|n| {
            let n = KnownN(n);

            let fns = input
                .abstract_fns
                .iter()
                .cloned()
                .map(|abstract_fn| {
                    abstract_fn
                        .impl_from_perspective(&input.api_ident, Perspective::Alignment(Some(n)))
                })
                .collect::<Box<[ItemFn]>>();

            ["VecPacked", "VecAligned"].map(|str| {
                let s = Ident::new(str, input.api_ident.span());

                quote_spanned! {
                    input.api_ident.span() =>
                    impl #trait_ident<#n> for #s {
                        #(
                            #fns
                        )*
                    }
                }
            })
        })
        .flatten();

    quote_spanned! {
        trait_ident.span() =>
        pub(super) trait #trait_ident<const N: usize>: inner::VecAlignmentInnerVecs where ScalarCount<N>: VecLen<N> {
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
    attrs: Vec<Attribute>,
    ident: Ident,
    fns: Vec<TraitItemFn>,
}
impl Parse for Input {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let attrs = Attribute::parse_outer(input)?;
        let ident = input.parse()?;
        input.parse::<Token![:]>()?;

        let mut fns = Vec::new();

        while !input.is_empty() {
            fns.push(input.parse()?);
        }

        Ok(Self { attrs, ident, fns })
    }
}

#[derive(Clone)]
struct ProcessedInput {
    api_attrs: Vec<Attribute>,
    api_ident: Ident,
    abstract_fns: Vec<AbstractApiFn>,
    free_fns: Vec<ItemFn>,
    errors: Vec<Error>,
}
impl TryFrom<Input> for ProcessedInput {
    type Error = Error;
    fn try_from(Input { attrs, ident, fns }: Input) -> Result<Self, Self::Error> {
        let mut abstract_fns = Vec::new();
        let mut free_fns = Vec::new();
        let mut errors = Vec::new();

        for fn_ in fns {
            if let Some(block) = fn_.default {
                free_fns.push(ItemFn {
                    attrs: fn_.attrs,
                    vis: Visibility::Public(
                        parse_quote_spanned! { fn_.sig.fn_token.span() => pub },
                    ),
                    sig: fn_.sig,
                    block: Box::new(block),
                });
            } else {
                match fn_.try_into() {
                    Ok(ok) => abstract_fns.push(ok),
                    Err(err) => errors.push(err),
                }
            }
        }

        Ok(Self {
            api_attrs: attrs,
            api_ident: ident,
            abstract_fns,
            free_fns,
            errors,
        })
    }
}
