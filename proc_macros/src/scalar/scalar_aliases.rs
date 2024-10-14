use derive_syn_parse::Parse;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{
    parse_macro_input, parse_quote, parse_str, AngleBracketedGenericArguments, Generics, Ident,
    Token, Type, Visibility,
};

pub fn scalar_aliases(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        vis,
        modness,
        ty,
        _colon,
        ident,
    } = parse_macro_input!(input as Input);

    let ty = ty.to_token_stream();

    let aliases_vis = match modness {
        None => vis.clone(),
        Some(_) => Visibility::Public(Default::default()),
    };
    let vec_aliases = [
        VecAlias {
            ident: parse_quote!(Vector),
            params: parse_quote!(<const N: usize, S>),
            args: parse_quote!(<N, #ty, S>),
        },
        VecAlias {
            ident: parse_quote!(Vector2),
            params: parse_quote!(<S>),
            args: parse_quote!(<#ty, S>),
        },
        VecAlias {
            ident: parse_quote!(Vector3),
            params: parse_quote!(<S>),
            args: parse_quote!(<#ty, S>),
        },
        VecAlias {
            ident: parse_quote!(Vector4),
            params: parse_quote!(<S>),
            args: parse_quote!(<#ty, S>),
        },
        VecAlias {
            ident: parse_quote!(VecN),
            params: parse_quote!(<const N: usize>),
            args: parse_quote!(<N, #ty>),
        },
        VecAlias {
            ident: parse_quote!(Vec2),
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: parse_quote!(Vec3),
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: parse_quote!(Vec4),
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: parse_quote!(VecNP),
            params: parse_quote!(<const N: usize>),
            args: parse_quote!(<N, #ty>),
        },
        VecAlias {
            ident: parse_quote!(Vec2P),
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: parse_quote!(Vec3P),
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
        VecAlias {
            ident: parse_quote!(Vec4P),
            params: parse_quote!(<>),
            args: parse_quote!(<#ty>),
        },
    ]
    .map(|vec_alias| {
        let ident = Ident::new(&format!("{ident}{}", vec_alias.ident), ident.span());
        let params = vec_alias.params;

        let vec_ident = vec_alias.ident;
        let args = vec_alias.args;

        let docs: TokenStream = parse_str(&format!(
            "/// type-aliase for an [```{ty}```] [```{vec_ident}```](gomath::vec::{vec_ident})"
        ))
        .unwrap();

        quote! {
            #docs
            #aliases_vis type #ident #params = gomath::vec::#vec_ident #args;
        }
    })
    .into_iter()
    .collect();

    if let Some(mod_token) = modness {
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
    modness: Option<Token![mod]>,
    ty: Type,
    _colon: Token![:],
    ident: Ident,
}

struct VecAlias {
    ident: Ident,
    params: Generics,
    args: AngleBracketedGenericArguments,
}
