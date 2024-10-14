use derive_syn_parse::Parse;
use quote::quote;
use syn::{
    parse_macro_input, parse_quote, AngleBracketedGenericArguments, Generics, Ident, Token,
    Visibility,
};

pub fn scalar_aliases(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let Input {
        vis,
        modness,
        ty,
        _colon,
        ident,
    } = parse_macro_input!(input as Input);

    let aliases_vis = match modness {
        None => vis.clone(),
        Some(_) => Visibility::Public(Default::default()),
    };
    let aliases = [
        VecAlias {
            ident: parse_quote!(Vector),
            params: parse_quote!(<const N: usize, S>),
            args: parse_quote!(<N, S>),
        },
        VecAlias {
            ident: parse_quote!(Vector2),
            params: parse_quote!(<S>),
            args: parse_quote!(<S>),
        },
        VecAlias {
            ident: parse_quote!(Vector3),
            params: parse_quote!(<S>),
            args: parse_quote!(<S>),
        },
        VecAlias {
            ident: parse_quote!(Vector4),
            params: parse_quote!(<S>),
            args: parse_quote!(<S>),
        },
        VecAlias {
            ident: parse_quote!(VecN),
            params: parse_quote!(<const N: usize>),
            args: parse_quote!(<N>),
        },
        VecAlias {
            ident: parse_quote!(Vec2),
            params: parse_quote!(<>),
            args: parse_quote!(<>),
        },
        VecAlias {
            ident: parse_quote!(Vec3),
            params: parse_quote!(<>),
            args: parse_quote!(<>),
        },
        VecAlias {
            ident: parse_quote!(Vec4),
            params: parse_quote!(<>),
            args: parse_quote!(<>),
        },
        VecAlias {
            ident: parse_quote!(VecNP),
            params: parse_quote!(<const N: usize>),
            args: parse_quote!(<N>),
        },
        VecAlias {
            ident: parse_quote!(Vec2P),
            params: parse_quote!(<>),
            args: parse_quote!(<>),
        },
        VecAlias {
            ident: parse_quote!(Vec3P),
            params: parse_quote!(<>),
            args: parse_quote!(<>),
        },
        VecAlias {
            ident: parse_quote!(Vec4P),
            params: parse_quote!(<>),
            args: parse_quote!(<>),
        },
    ]
    .map(|vec_alias| {
        let ident = Ident::new(&format!("{ident}{}", vec_alias.ident), ident.span());
        let params = vec_alias.params;

        let vec_ident = vec_alias.ident;
        let mut args = vec_alias.args;
        args.args.push(parse_quote!(#ty));

        quote! { #aliases_vis type #ident #params = gomath::vec::#vec_ident #args; }
    })
    .into_iter()
    .collect();

    if let Some(mod_token) = modness {
        quote! {
            #vis #mod_token #ty {
                use super::*;

                #aliases
            }
        }
    } else {
        aliases
    }
    .into()
}

#[derive(Parse)]
struct Input {
    vis: Visibility,
    modness: Option<Token![mod]>,
    ty: Ident,
    _colon: Token![:],
    ident: Ident,
}

struct VecAlias {
    ident: Ident,
    params: Generics,
    args: AngleBracketedGenericArguments,
}
