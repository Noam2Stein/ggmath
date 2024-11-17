use super::*;

pub fn scalar_trait(interface: &VecInterface) -> TokenStream {
    let output_fns = interface.impls.iter().map(|r#impl| r#impl.fns
        .iter()
        .map(|r#fn| {
            ALIGN_STRS
                .zip(r#fn.defaults.clone())
                .map(|(a_str, defaults)| {
                    let a = Ident::new(a_str, r#fn.sig.span());

                    LEN_STRS.zip(defaults).map(|(n_str, default)| {
                        let n = LitInt::new(n_str, r#fn.sig.span());

                        let mut fn_sig = r#fn.sig.clone();
                        fn_sig.ident = scalar_fn_ident(&fn_sig.ident, n_str, a_str);

                        search_replace_fn(
                            quote_spanned! { fn_sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                            fn_sig.clone(),
                            Some(default.to_token_stream()),
                            |span| quote_spanned! { span => #n },
                            |span| quote_spanned! { span => Self },
                            |span| quote_spanned! { span => #a },
                        )
                    })
                })
        }))
        .flatten()
        .flatten()
        .flatten();

    let VecInterface {
        ident: interface_ident,
        generics: interface_generics,
        supertraits: interface_supertraits,
        scalar_items: interface_scalar_items,
        impls: _,
    } = interface;
    let interface_where_clause = &interface.generics.where_clause;

    let interface_supertraits_with_sep = if interface_supertraits.len() > 0 {
        quote_spanned! {
            interface.ident.span() =>

            : #(#interface_supertraits + )*
        }
    } else {
        TokenStream::new()
    };

    let trait_declaration = search_replace_generics(
        quote_spanned! {
            interface.ident.span() =>

            pub trait #interface_ident #interface_generics #interface_supertraits_with_sep #interface_where_clause
        },
        |span| quote_spanned! { span => N },
        |span| quote_spanned! { span => Self },
        |span| quote_spanned! { span => A },
    );

    quote_spanned! {
        interface.ident.span() =>

        #trait_declaration {
            #(#output_fns)*
            #(#interface_scalar_items)*
        }
    }
}
