use super::*;

use quote::quote;
use syn::{
    punctuated::Punctuated, token::Paren, AngleBracketedGenericArguments, FnArg, Generics, Pat,
    Signature, Type, Visibility,
};

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

    let prefix_lower = Ident::new(&prefix.to_string().to_lowercase(), prefix.span());

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
        VectorOrScalar(const N: usize, A) => (N, #t, A),
        VecNOrScalar(const N: usize) => (N, #t),
        VecNPOrScalar(const N: usize) => (N, #t),
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

    #[derive(Parse)]
    struct FnAlias {
        sig: Signature,
        #[prefix(Token![=>])]
        call_generics: AngleBracketedGenericArguments,
    }
    struct FnAliases {
        vec: Vec<FnAlias>,
    }
    impl Parse for FnAliases {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            Ok(Self {
                vec: Punctuated::<FnAlias, Token![,]>::parse_terminated(input)?
                    .into_iter()
                    .collect(),
            })
        }
    }
    macro_rules! fn_aliases {
        [$($tt:tt)*] => {
            match parse2::<FnAliases>(quote! { $($tt)* }) {
                Ok(ok) => ok,
                Err(err) => return err.to_compile_error().into(),
            }.vec
        };
    }
    let fn_aliases = fn_aliases![
        fn vector<const N: usize, A: VecAlignment>(value: impl IntoVector<N, #t>) -> Vector<N, #t, A> where ScalarCount<N>: VecLen<N> => <N, #t, A>,
        fn vec2(value: impl IntoVector<2, #t>) -> Vec2<#t> => <#t>,
        fn vec3(value: impl IntoVector<3, #t>) -> Vec3<#t> => <#t>,
        fn vec4(value: impl IntoVector<4, #t>) -> Vec4<#t> => <#t>,
        fn vec2p(value: impl IntoVector<2, #t>) -> Vec2P<#t> => <#t>,
        fn vec3p(value: impl IntoVector<3, #t>) -> Vec3P<#t> => <#t>,
        fn vec4p(value: impl IntoVector<4, #t>) -> Vec4P<#t> => <#t>,
    ]
    .into_iter()
    .map(|FnAlias { sig, call_generics }| {
        let Signature {
            constness,
            asyncness,
            unsafety,
            abi,
            fn_token,
            ident,
            generics,
            paren_token: _,
            inputs,
            variadic,
            output,
        } = sig;

        let prefixed_ident = Ident::new(
            &format!("{prefix_lower}{ident}"),
            ident.span(),
        );

        let where_clause = &generics.where_clause;

        let input_idents = inputs.iter().map(|input| match input {
            FnArg::Receiver(receiver) => Ident::new("self", receiver.span()),
            FnArg::Typed(typed) => match &*typed.pat {
                Pat::Type(pat) => match &*pat.pat {
                    Pat::Ident(pat) => pat.ident.clone(),
                    _ => panic!("unsupported argument pat pat"),
                }
                Pat::Ident(pat) => pat.ident.clone(),
                _ => panic!("unsupported argument pat"),
            }
        });

        let docs: TokenStream = parse_str(&format!(
            "/// type-alias for [```{ident}```] with [```{}```]",
            t.to_token_stream(),
        ))
        .unwrap_or_else(|err| panic!("failed to parse '{ident}' docs: {err}"));

        quote! {
            #docs
            pub #constness #asyncness #unsafety #abi #fn_token #prefixed_ident #generics (#inputs) #variadic #output #where_clause {
                #ident::#call_generics(#(#input_idents), *)
            }
        }
    });

    let mod_docs: TokenStream = parse_str(&format!(
        "/// mathamatical type-aliases for [```{}```]",
        t.to_token_stream()
    ))
    .unwrap_or_else(|err| panic!("failed to parse mod docs: {err}"));

    quote! {
        #mod_docs
        #vis mod #mod_ident {
            use super::ggmath::vector::{alignment::*, length::*, or_scalar::*, *};

            #(#type_aliases)*
            #(#fn_aliases)*
        }
    }
    .into()
}
