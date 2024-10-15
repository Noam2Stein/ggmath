use crate::idents::*;
use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, parse_str, spanned::Spanned, AngleBracketedGenericArguments,
    Generics, Ident, Token, Type, Visibility,
};

pub fn scalar_aliases(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        vis,
        mod_,
        ty,
        _colon,
        prefix,
    } = parse_macro_input!(input as Input);

    let ty = ty.to_token_stream();

    let aliases_vis = match mod_ {
        None => vis.clone(),
        Some(_) => Visibility::Public(Default::default()),
    };
    let vec_aliases = [
        VecAlias {
            ident: Vector,
            params: parse_quote!(<const N: usize, S>),
            args: parse_quote!(<N, #ty, S>),
        },
        VecAlias {
            ident: Vector2,
            params: parse_quote!(<S>),
            args: parse_quote!(<#ty, S>),
        },
        VecAlias {
            ident: Vector3,
            params: parse_quote!(<S>),
            args: parse_quote!(<#ty, S>),
        },
        VecAlias {
            ident: Vector4,
            params: parse_quote!(<S>),
            args: parse_quote!(<#ty, S>),
        },
        VecAlias {
            ident: VecN,
            params: parse_quote!(<const N: usize>),
            args: parse_quote!(<N, #ty>),
        },
        VecAlias {
            ident: Vec2,
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: Vec3,
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: Vec4,
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: VecNP,
            params: parse_quote!(<const N: usize>),
            args: parse_quote!(<N, #ty>),
        },
        VecAlias {
            ident: Vec2P,
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: Vec3P,
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: Vec4P,
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
    ]
    .map(
        |VecAlias {
             ident,
             params,
             args,
         }| {
            let prefixed_ident = Ident::new(&format!("{prefix}{ident}"), ident.span());

            let docs: TokenStream = parse_str(&format!(
                "/// type-aliase for an [```{ty}```] [```{ident}```](gomath::vec::{ident})"
            ))
            .unwrap();

            quote! {
                #docs
                #aliases_vis type #prefixed_ident #params = #gomath::vec::#ident #args;
            }
        },
    )
    .into_iter()
    .collect();

    if let Some(mod_token) = mod_ {
        let docs: TokenStream =
            parse_str(&format!("/// mathamatical type-aliases for [```{ty}```]")).unwrap();
        quote! {
            #docs
            #vis #mod_token #ty {
                use super::*;

                #vec_aliases
            }
        }
    } else {
        vec_aliases
    }
    .into()
}

#[derive(Parse)]
struct Input {
    vis: Visibility,
    mod_: Option<Token![mod]>,
    ty: Type,
    _colon: Token![:],
    prefix: Ident,
}

struct VecAlias {
    ident: ConstIdent,
    params: Generics,
    args: AngleBracketedGenericArguments,
}
