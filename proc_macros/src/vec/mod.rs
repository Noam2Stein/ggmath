use proc_macro2::{Literal, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse2, parse_macro_input, spanned::Spanned, GenericParam, Ident, ItemTrait, PathArguments,
    TraitItem, TypeParamBound,
};

pub mod const_swizzle;
pub mod inner;

pub fn vecnum_trait(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(tokens as ItemTrait);

    if input.generics.params.len() != 0 {
        let err: proc_macro2::TokenStream = quote_spanned! {
            input.generics.span() =>
            compile_error!("VecNum traits have automatic generics and can't have additional ones")
        };
        return quote! {
            #input
            const _: () = {
                fn err() {
                    #err;
                }
            };
        }
        .into();
    };
    for item in &input.items {
        match item {
            TraitItem::Fn(item) => {
                if !item
                    .sig
                    .generics
                    .type_params()
                    .any(|param| param.ident.to_string() == "T")
                {
                    let err: proc_macro2::TokenStream = quote_spanned! {
                        item.sig.generics.span() =>
                        compile_error!("expected the Element parameter T")
                    };
                    return quote! {
                        #input
                        const _: () = {
                            fn err() {
                                #err;
                            }
                        };
                    }
                    .into();
                }
            }
            TraitItem::Type(item) => {
                if !item
                    .generics
                    .type_params()
                    .any(|param| param.ident.to_string() == "T")
                {
                    let err: proc_macro2::TokenStream = quote_spanned! {
                        item.generics.span() =>
                        compile_error!("expected the Element parameter T")
                    };
                    return quote! {
                        #input
                        const _: () = {
                            fn err() {
                                #err;
                            }
                        };
                    }
                    .into();
                }
            }
            _ => {
                let err: proc_macro2::TokenStream = quote_spanned! {
                    item.span() =>
                    compile_error!("VecNum traits can only declare fns and types")
                };
                return quote! {
                    #input
                    const _: () = {
                        fn err() {
                            #err;
                        }
                    };
                }
                .into();
            }
        }
    }

    input.generics = parse2(quote! { <const N: usize> })
        .unwrap_or_else(|err| panic!("vecnum_trait generics: {err}"));
    input.generics.where_clause = Some(
        parse2(quote! { where MaybeVecNum<N>: VecNum<N> })
            .unwrap_or_else(|err| panic!("vecnum_trait generic where: {err}")),
    );

    for supertrait in &mut input.supertraits {
        match supertrait {
            TypeParamBound::Trait(supertrait) => {
                let args = &mut supertrait.path.segments.last_mut().unwrap().arguments;
                if !args.is_empty() {
                    let err: proc_macro2::TokenStream = quote_spanned! {
                        supertrait.span() =>
                        compile_error!("VecNum supertraits have automatic arguments/generics and additional ones can't be added")
                    };
                    return quote! {
                        #input
                        const _: () = {
                            fn err() {
                                #err;
                            }
                        };
                    }
                    .into();
                };

                *args = PathArguments::AngleBracketed(
                    parse2(quote! { <N> })
                        .unwrap_or_else(|err| panic!("vecnum_trait supertrait generics: {err}")),
                );
            }
            _ => {
                let err: proc_macro2::TokenStream = quote_spanned! {
                    supertrait.span() =>
                    compile_error!("VecNum traits can only have *trait* bounds")
                };
                return quote! {
                    #input
                    const _: () = {
                        fn err() {
                            #err;
                        }
                    };
                }
                .into();
            }
        }
    }

    let trait_ident = &input.ident;

    let impls = (2..5).map(|n| {
        let n_literal = Literal::usize_unsuffixed(n);

        let item_impls = input.items.iter().map(|item| match item {
            TraitItem::Type(item) => {
                let item_ident = &item.ident;
                let item_generics = &item.generics;
                let item_value_ident = Ident::new(&format!("{}{}", item.ident, n), item.span());
                let item_value_generics_params =
                    item_generics.params.iter().filter_map(|param| match param {
                        GenericParam::Type(param) => if param.ident.to_string() != "T" {
                            Some(param.ident.to_token_stream())
                        } else {
                            None
                        },
                        GenericParam::Lifetime(param) => Some(param.to_token_stream()),
                        GenericParam::Const(param) => Some(param.ident.to_token_stream()),
                    });
                quote! {
                    type #item_ident #item_generics = T::#item_value_ident<#(#item_value_generics_params), *>;
                }
            }
            _ => TokenStream::new(),
        });

        quote! {
            impl #trait_ident<#n_literal> for MaybeVecNum<#n_literal> {
                #(
                    #item_impls
                )*
            }
        }
    });

    quote! {
        #input
        #(
            #impls
        )*
    }
    .into()
}
