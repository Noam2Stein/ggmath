use proc_macro2::{Literal, TokenStream};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse2, parse_macro_input, punctuated::Punctuated, spanned::Spanned, FnArg, GenericParam, Generics, Ident, ItemTrait, Pat, PathArguments, TraitItem, TypeParamBound
};

pub mod const_swizzle;
pub mod inner;

pub fn vecnum_trait(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(tokens as ItemTrait);
    let mut errs = Vec::new();

    if input.generics.params.len() != 0 {
        errs.push(quote_spanned! { input.generics.span() => "vecnum traits are automatically generic over N and can't specify additional generics" });
        input.generics = Generics::default();
    };
    input.generics = parse2(quote! { <const N: usize> })
        .unwrap_or_else(|err| panic!("vecnum_trait generics: {err}"));
    input.generics.where_clause = Some(
        parse2(quote! { where MaybeVecNum<N>: VecNum<N> })
            .unwrap_or_else(|err| panic!("vecnum_trait generic where: {err}")),
    );

    input.supertraits = Punctuated::from_iter(input.supertraits.into_pairs().filter_map(|mut pair| {
        match &mut pair.value_mut() {
            TypeParamBound::Trait(supertrait) => {
                let args = &mut supertrait.path.segments.last_mut().unwrap().arguments;
                if !args.is_empty() {
                    errs.push(quote_spanned! {
                        args.span() => "VecNum supertraits have automatic arguments/generics and additional ones can't be added" }
                    );
                };

                *args = PathArguments::AngleBracketed(
                    parse2(quote! { <N> })
                        .unwrap_or_else(|err| panic!("vecnum_trait supertrait generics: {err}")),
                );

                Some(pair)
            }
            supertrait => {
                errs.push(quote_spanned! { supertrait.span() => "VecNum traits can only have *trait* bounds" });

                None
            }
        }
    }));

    input.items.retain_mut(|item| {
        match item {
            TraitItem::Fn(item) => {
                if !item
                    .sig
                    .generics
                    .type_params()
                    .any(|param| param.ident.to_string() == "T")
                {
                    errs.push(quote_spanned! { item.sig.generics.span() => "expected 1 generic parameter to be T" });
                    item.sig.generics.params.push_value(
                        parse2(quote! { T })
                            .unwrap_or_else(|err| panic!("vecnum_trait fn push missing T: {err}")),
                    );
                }
                
                if let Some(default_span) = item.default.as_ref().map(|default| default.span()) {
                    errs.push(quote_spanned! { default_span => "VecNum trait fns are impl automatically and can't be impl manually" });
                    item.default = None;
                }
            }
            TraitItem::Type(item) => {
                if !item
                    .generics
                    .type_params()
                    .any(|param| param.ident.to_string() == "T")
                {
                    errs.push(quote_spanned! { item.generics.span() => "expected 1 generic parameter to be T" });
                    item.generics.params.push_value(
                        parse2(quote! { T })
                            .unwrap_or_else(|err| panic!("vecnum_trait fn push missing T: {err}")),
                    );
                }
            }
            _ => {
                errs.push(quote_spanned! { item.span() => "VecNum traits can only declare fns and types" });
                return false;
            }
        }
        true
    });

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
            TraitItem::Fn(item) => {
                let mut sig = item.sig.clone();
                for input in &mut sig {
                    
                } 

                let call_fn_ident = Ident::new(&format!("vec_{}", item.sig.ident), item.sig.ident.span());
                let call_generics_params =
                    sig.generics.params.iter().filter_map(|param| match param {
                        GenericParam::Type(param) => if param.ident.to_string() != "T" {
                            Some(param.ident.to_token_stream())
                        } else {
                            None
                        },
                        GenericParam::Lifetime(param) => Some(param.to_token_stream()),
                        GenericParam::Const(param) => Some(param.ident.to_token_stream()),
                    });
                let call_args = sig.inputs.iter().filter_map(|arg| if let FnArg::Typed(arg) = arg {
                    if let Pat::Ident(arg) = &*arg.pat {
                        Some(arg.ident.to_token_stream())
                    }
                    else {
                        errs.push(quote_spanned! { arg.span() => "only ident args are valid in VecNum trait fns" });
                        None
                    }
                }
                else {
                    errs.push(quote_spanned! { arg.span() => "only ident args are valid in VecNum trait fns" });
                    None
                });
                quote! {
                    #sig {
                        T::#call_fn_ident<#(#call_generics_params), *>(#(#call_args), *)
                    }
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
    
        const _: () = {
            #(compile_error!(#errs);)*
        };
    }
    .into()
}
