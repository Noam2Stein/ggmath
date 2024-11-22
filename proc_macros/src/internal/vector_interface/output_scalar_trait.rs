use super::*;

pub fn scalar_trait(interface: &VecInterface) -> TokenStream {
    let output_fns = interface.impls.iter().map(|r#impl| r#impl.fns
        .iter()
        .map(|r#fn| {
            let mut modified_fn_sig = r#fn.sig.clone();

            modified_fn_sig.ident = scalar_trait_fn_ident(&modified_fn_sig.ident);

            modified_fn_sig.generics = {
                let impl_generic_params = r#impl.generics.params.iter();
                let fn_generic_params = r#fn.sig.generics.params.iter();

                parse_quote_spanned! {
                    r#fn.sig.generics.span() =>
    
                    <const N: usize,
                    A: ggmath::vector::VecAlignment
                    #(, #impl_generic_params)*
                    #(, #fn_generic_params)*>
                }
            };
            modified_fn_sig.generics.where_clause = {
                let impl_where_clause = r#impl.generics.where_clause.as_ref().map_or(Vec::new(),|where_clause| where_clause.predicates.iter().collect());
                let fn_where_clause = r#fn.sig.generics.where_clause.as_ref().map_or(Vec::new(),|where_clause| where_clause.predicates.iter().collect());

                Some(parse_quote_spanned! {
                    modified_fn_sig.generics.span() =>
                    
                    where
                        ggmath::vector::ScalarCount<N>: ggmath::vector::VecLen
                        #(, #impl_where_clause)*
                        #(, #fn_where_clause)*
                })
            };

            search_replace_fn(
                quote_spanned! { modified_fn_sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                modified_fn_sig.clone(),
                Some(r#fn.default.to_token_stream()),
                |span| quote_spanned! { span => N },
                |span| quote_spanned! { span => Self },
                |span| quote_spanned! { span => A },
            )
        }))
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

pub fn scalar_trait_fn_ident(fn_ident: &Ident) -> Ident {
    Ident::new(&format!("vector_{fn_ident}"), fn_ident.span())
}
