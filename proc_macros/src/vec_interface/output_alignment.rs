use super::*;

pub fn alignment(interface: &VecInterface) -> TokenStream {
    let modified_fn_sigs = interface
        .fns
        .iter()
        .map(|r#fn| {
            let scalar_trait_ident = with_span(&interface.type_param.ident, r#fn.sig.ident.span());
            let interface_generic_args = generic_args(&interface.generics).into_iter().map(|arg| with_span(&arg, r#fn.sig.ident.span()));

            let mut fn_sig = r#fn.sig.clone();
            fn_sig.generics.params.insert(
                0,
                parse_quote_spanned! { fn_sig.ident.span() => T: #scalar_trait_ident<#(#interface_generic_args), *>},
            );
            fn_sig
                .generics
                .params
                .extend(interface.generics.params.iter().map(|param| with_span(param, r#fn.sig.generics.params.span())));

            if let Some(interface_where_clause) = &interface.generics.where_clause {
                if let Some(fn_where_clause) = &mut fn_sig.generics.where_clause {
                    let span = fn_where_clause.span();
                    fn_where_clause
                        .predicates
                        .extend(interface_where_clause.predicates.iter().map(|predicate| with_span(predicate, span)));
                } else {
                    fn_sig.generics.where_clause = Some(with_span(interface_where_clause, fn_sig.ident.span()));
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
            ALIGN_STRS.map(|a_str| {
                let fn_impls = modified_fn_sigs.iter().zip(interface.fns.iter()).map(|(modified_fn_sig, original_fn)| {
                    let scalar_fn = scalar_fn_ident(&modified_fn_sig.ident, n_str, a_str);

                    let n = LitInt::new(n_str, modified_fn_sig.ident.span());

                    let fn_generics = generic_args(&original_fn.sig.generics);
                    let fn_input_idents = modified_fn_sig.inputs.iter().map(|input| arg_ident(input));

                    search_replace_fn(
                        quote_spanned! { modified_fn_sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                        modified_fn_sig.clone(),
                        Some(quote_spanned! { modified_fn_sig.ident.span() => {
                            T::#scalar_fn::<#(#fn_generics), *>(#(#fn_input_idents), *)
                        } }),
                        |span| quote_spanned! { span => #n },
                        |span| quote_spanned! { span => T },
                        |span| quote_spanned! { span => Self },
                    )
                });

                let alignment_trait_ident = alignment_trait_ident(interface);

                let n = LitInt::new(n_str, alignment_trait_ident.span());
                let a = Ident::new(a_str, alignment_trait_ident.span());

                quote_spanned! {
                    interface.type_param.span() =>
                    impl #alignment_trait_ident<#n> for #a {
                        #(#fn_impls)*
                    }
                }
            })
        })
        .into_iter()
        .flatten();

    let alignment_trait_ident = alignment_trait_ident(interface);

    quote_spanned! {
        alignment_trait_ident.span() =>

        pub(in crate::vector::interfaces) trait #alignment_trait_ident<const N: usize>: alignment_seal::VecAlignment where ScalarCount<N>: VecLen<N> {
            #(#output_trait_fn_declarations)*
        }

        #(#output_trait_impls)*
    }
}
