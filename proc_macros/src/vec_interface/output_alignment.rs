use super::*;

pub fn alignment(interface: &VecInterface) -> TokenStream {
    let scalar_trait_ident = &interface.scalar_trait.ident;
    let alignment_trait_ident = alignment_trait_ident(interface);

    let modified_fn_sigs = interface
        .fns
        .iter()
        .map(|r#fn| {
            let mut fn_sig = r#fn.sig.clone();
            fn_sig.generics.params.insert(
                0,
                parse_quote_spanned! { fn_sig.ident.span() => T: #scalar_trait_ident },
            );
            fn_sig
                .generics
                .params
                .extend(interface.generics.params.iter().cloned());

            if let Some(interface_where_clause) = &interface.generics.where_clause {
                if let Some(fn_where_clause) = &mut fn_sig.generics.where_clause {
                    fn_where_clause
                        .predicates
                        .extend(interface_where_clause.predicates.iter().cloned());
                } else {
                    fn_sig.generics.where_clause = Some(interface_where_clause.clone());
                }
            }

            fn_sig
        })
        .collect::<Box<[Signature]>>();

        

    let output_trait_fn_declarations = modified_fn_sigs.iter().map(|sig| {
        search_replace_fn(
            quote_spanned! { sig.fn_token.span => #[allow(missing_docs)] },
            sig.clone(),
            None,
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => Self },
        )
    });

    let output_trait_impls = LEN_STRS
        .map(|n_str| {
            let n = LitInt::new(n_str, alignment_trait_ident.span());
            
            ALIGN_STRS.map(|a_str| {
                let a = Ident::new(a_str, alignment_trait_ident.span());

                let fn_impls = modified_fn_sigs.iter().map(|sig| {
                    let scalar_fn = scalar_fn_ident(&sig.ident, n_str, a_str);

                    let n = LitInt::new(n_str, sig.ident.span());

                    let generics = &generic_args(&sig.generics)[1..];
                    let input_idents = sig.inputs.iter().map(|input| arg_ident(input));

                    search_replace_fn(
                        quote_spanned! { sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                        sig.clone(),
                        Some(quote_spanned! { sig.ident.span() => {
                            T::#scalar_fn::<#(#generics), *>(#(#input_idents), *)
                        } }),
                        |span| quote_spanned! { span => #n },
                        |span| quote_spanned! { span => T },
                        |span| quote_spanned! { span => Self },
                    )
                });

                quote_spanned! {
                    interface.scalar_trait.span() =>
                    impl #alignment_trait_ident<#n> for #a {
                        #(#fn_impls)*
                    }
                }
            })
        })
        .into_iter()
        .flatten();

    quote_spanned! {
        alignment_trait_ident.span() =>

        pub(super) trait #alignment_trait_ident<const N: usize>: alignment_seal::VecAlignment where ScalarCount<N>: VecLen<N> {
            #(#output_trait_fn_declarations)*
        }

        #(#output_trait_impls)*
    }
}
