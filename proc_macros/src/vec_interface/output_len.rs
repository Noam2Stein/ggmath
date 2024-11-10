use super::*;

pub fn len(interface: &VecInterface) -> TokenStream {
    let scalar_trait_ident = &interface.scalar_trait.ident;
    let len_trait_ident = len_trait_ident(interface);
    let alignment_trait_ident = alignment_trait_ident(interface);

    let interface_generics = &interface.generics;

    let modified_fn_sigs = interface
        .fns
        .iter()
        .map(|r#fn| {
            let mut fn_sig = r#fn.sig.clone();
            fn_sig.generics.params.insert(
                0,
                parse_quote_spanned! { fn_sig.ident.span() => T: #scalar_trait_ident },
            );
            fn_sig.generics.params.insert(
                1,
                parse_quote_spanned! { fn_sig.ident.span() => A: VecAlignment },
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

    let output_trait_fns = modified_fn_sigs.iter().map(|sig| {
        search_replace_fn(
            quote_spanned! { sig.fn_token.span => #[allow(missing_docs)] },
            sig.clone(),
            None,
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => A },
        )
    });

    let output_trait_impls = LEN_STRS.map(|n_str| {
        let n = Lit::Int(LitInt::new(n_str, len_trait_ident.span()));

        let fn_impls = modified_fn_sigs.iter().map(|fn_sig| {
            let n = Lit::Int(LitInt::new(n_str, fn_sig.ident.span()));

            let fn_ident = &fn_sig.ident;
            let fn_generics = &generic_args(&fn_sig.generics)[2..];
            let fn_input_idents = fn_sig.inputs.iter().map(|input| arg_ident(input));

            search_replace_fn(
                quote_spanned! { fn_sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                fn_sig.clone(),
                Some(quote_spanned! { fn_sig.ident.span() => {
                    <A as #alignment_trait_ident<N>>
                        ::#fn_ident::<#(#fn_generics), *>(#(#fn_input_idents), *)
                } }),
                |span| quote_spanned! { span => #n },
                |span| quote_spanned! { span => T },
                |span| quote_spanned! { span => A },
            )
        });

        quote_spanned! {
            interface.scalar_trait.span() =>
            impl #interface_generics #len_trait_ident<#n> for ScalarCount<#n> {
                #(#fn_impls)*
            }
        }
    });

    quote_spanned! {
        len_trait_ident.span() =>

        pub(super) trait #len_trait_ident<const N: usize>: VecLenInnerVec where ScalarCount<N>: VecLen<N> {
            #(#output_trait_fns)*
        }
        #(#output_trait_impls)*
    }
}
