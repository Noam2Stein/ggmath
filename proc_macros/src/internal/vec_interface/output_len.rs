use super::*;

pub fn len(interface: &VecInterface) -> TokenStream {
    let modified_fn_sigs = interface.impls.iter().map(|r#impl| r#impl
        .fns
        .iter()
        .map(|r#fn| {
            let scalar_trait_ident = with_span(&interface.ident, r#fn.sig.ident.span());
            let interface_generic_args = generic_args(&interface.generics).into_iter().map(|arg| with_span(&arg, r#fn.sig.ident.span()));

            let mut fn_sig = r#fn.sig.clone();
            fn_sig.generics.params.insert(
                0,
                parse_quote_spanned! { fn_sig.ident.span() => T: #scalar_trait_ident<#(#interface_generic_args), *>},
            );
            fn_sig.generics.params.insert(
                1,
                parse_quote_spanned! { fn_sig.ident.span() => A: VecAlignment },
            );
            fn_sig
                .generics
                .params
                .extend(interface.generics.params.iter().map(|param| with_span(param, r#fn.sig.generics.params.span())));
            fn_sig
                .generics
                .params
                .extend(r#impl.generics.params.iter().map(|param| with_span(param, r#fn.sig.generics.params.span())));

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
            if let Some(impl_where_clause) = &r#impl.generics.where_clause {
                if let Some(fn_where_clause) = &mut fn_sig.generics.where_clause {
                    let span = fn_where_clause.span();
                    fn_where_clause
                        .predicates
                        .extend(impl_where_clause.predicates.iter().map(|predicate| with_span(predicate, span)));
                } else {
                    fn_sig.generics.where_clause = Some(with_span(impl_where_clause, fn_sig.ident.span()));
                }
            }

            fn_sig
        })).flatten()
        .collect::<Box<[Signature]>>();

    let output_trait_fns = modified_fn_sigs.iter().map(|modified_fn_sig| {
        search_replace_fn(
            quote_spanned! { modified_fn_sig.fn_token.span => #[allow(missing_docs)] },
            modified_fn_sig.clone(),
            None,
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => A },
        )
    });

    let output_trait_impls = LEN_STRS.map(|n_str| {
        let fn_impls = modified_fn_sigs.iter().map(|modified_fn_sig| {
            let alignment_trait_ident = with_span(&alignment_trait_ident(interface), modified_fn_sig.ident.span());

            let n = Lit::Int(LitInt::new(n_str, modified_fn_sig.ident.span()));
            
            let fn_ident = &modified_fn_sig.ident;
            let fn_generics = &generic_args(&modified_fn_sig.generics)[2..];
            let fn_input_idents = modified_fn_sig.inputs.iter().map(|input| arg_ident(input));
            
            search_replace_fn(
                quote_spanned! { modified_fn_sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                modified_fn_sig.clone(),
                Some(quote_spanned! { modified_fn_sig.ident.span() => {
                    <A as #alignment_trait_ident<N>>
                    ::#fn_ident::<T #(, #fn_generics)*>(#(#fn_input_idents), *)
                } }),
                |span| quote_spanned! { span => #n },
                |span| quote_spanned! { span => T },
                |span| quote_spanned! { span => A },
            )
        });

        let len_trait_ident = len_trait_ident(interface);

        let n = Lit::Int(LitInt::new(n_str, len_trait_ident.span()));

        quote_spanned! {
            interface.ident.span() =>
            impl #len_trait_ident<#n> for ScalarCount<#n> {
                #(#fn_impls)*
            }
        }
    });

    let len_trait_ident = len_trait_ident(interface);

    quote_spanned! {
        len_trait_ident.span() =>

        pub(in crate::vector::interfaces) trait #len_trait_ident<const N: usize>: VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(#output_trait_fns)*
        }
        #(#output_trait_impls)*
    }
}
