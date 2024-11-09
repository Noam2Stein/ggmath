use super::*;

pub fn scalar_trait(input: &VecInterface) -> TokenStream {
    let trait_attrs = &input.scalar_trait.attrs;
    let trait_ident = &input.scalar_trait.ident;
    let trait_supertraits = &input.scalar_trait.bounds;

    let interface_generics = &input.generics;
    let interface_where_clause = &input.generics.where_clause;

    let output_fns = input
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

    let output_assoc_types = input
        .assoc_types
        .iter()
        .map(|assoc_type| {
            ALIGN_STRS
                .zip(assoc_type.value.clone())
                .map(|(a_str, type_values)| {
                    let a = Ident::new(a_str, assoc_type.ident.span());

                    LEN_STRS.zip(type_values).map(|(n_str, type_value)| {
                        let n = LitInt::new(n_str, assoc_type.ident.span());

                        let assoc_type_ident =
                            scalar_assoc_type_ident(&assoc_type.ident, n_str, a_str);
                        let assoc_type_generics = &assoc_type.generics;

                        search_replace_assoc_type(
                            quote_spanned! { assoc_type_ident.span() => #[allow(missing_docs)] },
                            assoc_type_ident,
                            assoc_type_generics,
                            Some(type_value.to_token_stream()),
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
            #(#output_assoc_types)*
        }
    }
}
