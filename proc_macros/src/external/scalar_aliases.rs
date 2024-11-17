use super::*;

use quote::quote;
use syn::{token::Paren, AngleBracketedGenericArguments, Generics, Type, Visibility};

pub fn scalar_aliases(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        vis: Visibility,
        #[prefix(Token![mod])]
        mod_ident: Ident,
        #[prefix(Token![for])]
        t: Type,
        #[paren]
        _t_paren: Paren,
        #[inside(_t_paren)]
        prefix: Ident,
    }
    let Input {
        vis,
        mod_ident,
        t,
        _t_paren,
        prefix,
    } = parse_macro_input!(input as Input);

    struct TypeAlias {
        ident: Ident,
        params: Generics,
        args: AngleBracketedGenericArguments,
    }
    macro_rules! type_aliases {
        [$($ident:ident $(($($param:tt)*))? => ($($arg:tt)*)), * $(,)?] => {
            [$(TypeAlias {
                ident: Ident::new(stringify!($ident), Span::call_site()),
                params: parse2(quote! { <$($($param)*)?> })
                    .unwrap_or_else(|err| panic!("failed to parse '{}' generic params: {err}", stringify!($ident))),
                args: parse2(quote! { <$($arg)*> })
                    .unwrap_or_else(|err| panic!("failed to parse '{}' generic arguments: {err}", stringify!($ident))),
            },)*]
        };
    }
    let type_aliases = type_aliases![
        Vector(const N: usize, A) => (N, #t, A),
        Vec2 => (#t),
        Vec3 => (#t),
        Vec4 => (#t),
        Vec2P => (#t),
        Vec3P => (#t),
        Vec4P => (#t),
    ]
    .map(
        |TypeAlias {
             ident,
             params,
             args,
         }| {
            let prefixed_ident = Ident::new(&format!("{prefix}{ident}"), ident.span());

            let docs: TokenStream = parse_str(&format!(
                "/// type-alias for an [```{}```] [```{ident}```]",
                t.to_token_stream(),
            ))
            .unwrap_or_else(|err| panic!("failed to parse '{ident}' docs: {err}"));

            quote! {
                #docs
                pub type #prefixed_ident #params = #ident #args;
            }
        },
    );

    let mod_docs: TokenStream = parse_str(&format!(
        "/// mathamatical type-aliases for [```{}```]",
        t.to_token_stream()
    ))
    .unwrap_or_else(|err| panic!("failed to parse mod docs: {err}"));

    quote! {
        #mod_docs
        #vis mod #mod_ident {
            use super::*;
            use ggmath::vector::{alignment::*, into_vec::*, length::*, *};

            #(#type_aliases)*
        }
    }
    .into()
}
