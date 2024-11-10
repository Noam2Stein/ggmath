use super::*;

pub fn scalar_trait(interface: &VecInterface) -> TokenStream {
    let trait_attrs = &interface.scalar_trait.attrs;
    let trait_ident = &interface.scalar_trait.ident;
    let trait_supertraits = &interface.scalar_trait.bounds;

    let interface_generics = &interface.generics;
    let interface_where_clause = &interface.generics.where_clause;

    let output_fns = interface
        .fns
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
        })
        .flatten()
        .flatten();

    quote_spanned! {
        trait_ident.span() =>

        #(#trait_attrs)*
        pub trait #trait_ident #interface_generics: #trait_supertraits #interface_where_clause {
            #(#output_fns)*
        }
    }
}
