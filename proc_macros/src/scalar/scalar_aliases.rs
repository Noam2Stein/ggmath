use super::*;

use quote::quote;
use syn::{AngleBracketedGenericArguments, Generics, Type, Visibility};

pub fn scalar_aliases(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    #[derive(Parse)]
    struct Input {
        vis: Visibility,
        mod_token: Option<Token![mod]>,
        t: Type,
        _colon_token: Token![:],
        prefix: Ident,
    }
    let Input {
        vis,
        mod_token,
        t,
        _colon_token,
        prefix,
    } = parse_macro_input!(input as Input);

    let alias_vis = match mod_token {
        None => vis.clone(),
        Some(_) => Visibility::Public(Default::default()),
    };

    struct Alias {
        ident: Ident,
        path: Vec<Ident>,
        params: Generics,
        args: AngleBracketedGenericArguments,
    }
    macro_rules! aliases {
        [$($ident:ident $(($($param:tt)*))? => ($($arg:tt)*) $(in $($path_segment:ident)::*)?), * $(,)?] => {
            [$(Alias {
                ident: Ident::new(stringify!($ident), Span::call_site()),
                path: vec![$($(Ident::new(stringify!($path_segment), Span::call_site())), *)?],
                params: parse2(quote! { <$($($param)*)?> })
                    .unwrap_or_else(|err| panic!("failed to parse '{}' generic params: {err}", stringify!($ident))),
                args: parse2(quote! { <$($arg)*> })
                    .unwrap_or_else(|err| panic!("failed to parse '{}' generic arguments: {err}", stringify!($ident))),
            },)*]
        };
    }
    let aliases = aliases![
        Vector(const N: usize, A) => (N, #t, A),
        Vector2(A) => (#t, A),
        Vector3(A) => (#t, A),
        Vector4(A) => (#t, A),
        VecN(const N: usize) => (N, #t),
        Vec2 => (#t),
        Vec3 => (#t),
        Vec4 => (#t),
        VecNP(const N: usize) => (N, #t),
        Vec2P => (#t),
        Vec3P => (#t),
        Vec4P => (#t),
        VectorOrScalar(const N: usize, A) => (N, #t, A) in or_scalar,
        VecNOrScalar(const N: usize) => (N, #t) in or_scalar,
        VecNPOrScalar(const N: usize) => (N, #t) in or_scalar,
    ]
    .map(
        |Alias {
             ident,
             path,
             params,
             args,
         }| {
            let prefixed_ident = Ident::new(&format!("{prefix}{ident}"), ident.span());

            let docs: TokenStream = parse_str(&format!(
                "/// type-aliase for an [```{}```] [```{ident}```](ggmath::vec::{}{ident})",
                t.to_token_stream(),
                path.iter()
                    .map(|path| format!("{path}::"))
                    .collect::<String>()
            ))
            .unwrap_or_else(|err| panic!("failed to parse '{ident}' docs: {err}"));

            quote! {
                #docs
                #alias_vis type #prefixed_ident #params = ggmath::vec::#(#path::)* #ident #args;
            }
        },
    );

    if let Some(mod_token) = mod_token {
        let docs: TokenStream = parse_str(&format!(
            "/// mathamatical type-aliases for [```{}```]",
            t.to_token_stream()
        ))
        .unwrap_or_else(|err| panic!("failed to parse mod docs: {err}"));

        quote! {
            #docs
            #vis #mod_token #t {
                use super::*;

                #(#aliases)*
            }
        }
    } else {
        quote! {
            #(#aliases)*
        }
    }
    .into()
}
