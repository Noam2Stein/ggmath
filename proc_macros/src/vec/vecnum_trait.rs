use proc_macro2::{Group, Literal, TokenStream, TokenTree};
use quote::{quote, quote_spanned, ToTokens};
use syn::{
    parse2, parse_macro_input, punctuated::Punctuated, spanned::Spanned, token::Plus, FnArg,
    GenericParam, Generics, Ident, ItemTrait, Pat, PathArguments, ReturnType, Signature, TraitItem,
    TraitItemFn, TraitItemType, Type, TypeParamBound,
};

pub fn vecnum_trait(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut errs = Vec::new();

    let mut trait_ = parse_macro_input!(tokens as ItemTrait);
    modify_trait_generics(&mut trait_.generics, &mut errs);
    modify_trait_supertraits(&mut trait_.supertraits, &mut errs);
    modify_trait_items(&mut trait_.items, &mut errs);

    let trait_ident = &trait_.ident;

    let impls = (2..5)
        .map(|n| {
            let item_impls = trait_.items.iter_mut().map(|item| {
                let validate_item = n == 2;
                match item {
                    TraitItem::Fn(item) => impl_fn(item, n, &mut errs, validate_item),
                    TraitItem::Type(item) => impl_type(item, n, &mut errs, validate_item),
                    _ => unreachable!("impls non fn or type item"),
                }
            });

            let n_unsuffixed = Literal::usize_unsuffixed(n);
            quote! {
                impl #trait_ident<#n_unsuffixed> for MaybeVecNum<#n_unsuffixed> {
                    #(
                        #item_impls
                    )*
                }
            }
        })
        .collect::<Box<[TokenStream]>>();

    quote! {
        #trait_
        #(
            #impls
        )*

        const _: () = {
            #(compile_error!(#errs);)*
        };
    }
    .into()
}

#[inline(always)]
fn modify_trait_generics(generics: &mut Generics, errs: &mut Vec<TokenStream>) {
    if generics.params.len() != 0 {
        errs.push(quote_spanned! { generics.span() => "vecnum traits are automatically generic over N and can't specify additional generics" });
        *generics = Generics::default();
    };

    *generics = parse2(quote! { <const N: usize> })
        .unwrap_or_else(|err| panic!("vecnum_trait generics: {err}"));

    generics.where_clause = Some(
        parse2(quote! { where MaybeVecNum<N>: VecNum<N> })
            .unwrap_or_else(|err| panic!("vecnum_trait generic where: {err}")),
    );
}
#[inline(always)]
fn modify_trait_supertraits(
    supertraits: &mut Punctuated<TypeParamBound, Plus>,
    errs: &mut Vec<TokenStream>,
) {
    *supertraits = Punctuated::from_iter(supertraits.clone().into_pairs().filter_map(|mut pair| {
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
}
#[inline(always)]
fn modify_trait_items(items: &mut Vec<TraitItem>, errs: &mut Vec<TokenStream>) {
    items.retain_mut(|item| match item {
        TraitItem::Fn(_) => true,
        TraitItem::Type(_) => true,
        item => {
            errs.push(
                quote_spanned! { item.span() => "VecNum traits can only contain fns and types" },
            );

            false
        }
    });
}

#[inline(always)]
fn impl_fn(
    item: &mut TraitItemFn,
    n: usize,
    errs: &mut Vec<TokenStream>,
    validate_item: bool,
) -> TokenStream {
    #[inline(always)]
    fn modify_sig(
        sig: &mut Signature,
        sig_is_valid: &mut bool,
        n: usize,
        errs: &mut Vec<TokenStream>,
        validate_item: bool,
    ) {
        for input in &mut sig.inputs {
            match input {
                FnArg::Receiver(_) => {
                    if validate_item {
                        errs.push(
                            quote_spanned! { input.span() => "VecNum trait fns disallow self args" },
                        );

                        *sig_is_valid = false;
                    };
                }
                FnArg::Typed(input) => ty_apply_n(n, &mut input.ty),
            }
        }
        match &mut sig.output {
            ReturnType::Type(_, ty) => ty_apply_n(n, ty),
            ReturnType::Default => {}
        }
    }

    if validate_item {
        if let Some(default_span) = item.default.as_ref().map(|default| default.span()) {
            errs.push(quote_spanned! { default_span => "VecNum trait fns are impl automatically and can't be impl manually" });
            item.default = None;
        }
    }

    let mut sig = item.sig.clone();
    let mut sig_is_valid = true;
    modify_sig(&mut sig, &mut sig_is_valid, n, errs, validate_item);
    if !sig_is_valid {
        return quote! {
            #sig {
                todo!()
            }
        };
    };

    let call_fn_ident = Ident::new(&format!("vec_{}", sig.ident), sig.ident.span());
    let call_generics_params = sig.generics.params.iter().filter_map(|param| match param {
        GenericParam::Type(param) => {
            if param.ident.to_string() == "T" {
                None
            } else {
                Some(param.ident.to_token_stream())
            }
        }
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
            T::#call_fn_ident::<#(#call_generics_params), *>(#(#call_args), *)
        }
    }
}
#[inline(always)]
fn impl_type(
    item: &mut TraitItemType,
    n: usize,
    errs: &mut Vec<TokenStream>,
    validate_item: bool,
) -> TokenStream {
    if validate_item {
        if !item
            .generics
            .type_params()
            .any(|param| param.ident.to_string() == "T")
        {
            errs.push(
                quote_spanned! { item.generics.span() => "expected 1 generic parameter to be T" },
            );
            item.generics.params.push_value(
                parse2(quote! { T })
                    .unwrap_or_else(|err| panic!("vecnum_trait type push missing T: {err}")),
            );
        }
    }

    let item_ident = &item.ident;
    let item_generics = &item.generics;

    let value_ident = Ident::new(&format!("{}{}", item.ident, n), item.span());
    let value_generics_params = item_generics.params.iter().filter_map(|param| match param {
        GenericParam::Type(param) => {
            if param.ident.to_string() != "T" {
                Some(param.ident.to_token_stream())
            } else {
                None
            }
        }
        GenericParam::Lifetime(param) => Some(param.to_token_stream()),
        GenericParam::Const(param) => Some(param.ident.to_token_stream()),
    });

    quote! {
        type #item_ident #item_generics = T::#value_ident<#(#value_generics_params), *>;
    }
}

fn apply_n(n: usize, tokens: TokenStream) -> TokenStream {
    tokens
        .into_iter()
        .map(|token| match token {
            TokenTree::Ident(token) => {
                if token.to_string() == "N" {
                    TokenTree::Literal(Literal::usize_unsuffixed(n))
                } else {
                    TokenTree::Ident(token)
                }
            }
            TokenTree::Group(token) => {
                TokenTree::Group(Group::new(token.delimiter(), apply_n(n, token.stream())))
            }
            TokenTree::Literal(token) => TokenTree::Literal(token),
            TokenTree::Punct(token) => TokenTree::Punct(token),
        })
        .collect()
}
fn ty_apply_n(n: usize, ty: &mut Type) {
    *ty = parse2(apply_n(n, ty.to_token_stream())).unwrap();
}
