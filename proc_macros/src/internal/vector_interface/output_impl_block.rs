use super::*;

pub fn impl_block(interface: &VecInterface, r#impl: &VecInterfaceImpl) -> TokenStream {
    let output_impl_generics = if r#impl.r#trait.is_some() {
        let interface_ident = &interface.ident;
        let interface_generic_params = interface.generics.params.iter();
        let interface_generic_args = generic_args(&interface.generics);
        let impl_generic_params = r#impl.generics.params.iter();

        quote_spanned! {
            interface.generics.span() =>
            <
                const N: usize,
                T: #interface_ident<#(#interface_generic_args), *>,
                A: ggmath::vector::VecAlignment
                #(, #interface_generic_params)*
                #(, #impl_generic_params)*
            >
        }
    } else {
        quote_spanned! {
            interface.generics.span() =>
            <
                const N: usize,
                T: ggmath::scalar::Scalar,
                A: ggmath::vector::VecAlignment
            >
        }
    };

    let output_impl_types = if let Some(impl_trait) = &r#impl.r#trait {
        quote_spanned! {
            interface.ident.span() =>

            #impl_trait for ggmath::vector::Vector<N, T, A>
        }
    } else {
        quote_spanned! {
            interface.ident.span() =>

            ggmath::vector::Vector<N, T, A>
        }
    };

    let output_where_clause = if r#impl.r#trait.is_some() {
        if let Some(interface_where_clause) = &interface.generics.where_clause {
            if let Some(impl_where_clause) = &r#impl.generics.where_clause {
                let impl_where_clause_predicates = impl_where_clause.predicates.iter();

                quote_spanned! {
                    interface_where_clause.where_token.span() =>

                    #interface_where_clause, #(#impl_where_clause_predicates), *, ggmath::vector::ScalarCount<N>: ggmath::vector::VecLen
                }
            } else {
                quote_spanned! {
                    interface_where_clause.where_token.span() =>

                    #interface_where_clause, ggmath::vector::ScalarCount<N>: ggmath::vector::VecLen
                }
            }
        } else {
            quote_spanned! {
                interface.ident.span() =>

                where ggmath::vector::ScalarCount<N>: ggmath::vector::VecLen
            }
        }
    } else {
        quote_spanned! {
            interface.ident.span() =>

            where ggmath::vector::ScalarCount<N>: ggmath::vector::VecLen
        }
    };

    let impl_generic_args = generic_args(&r#impl.generics);

    let output_fns = r#impl.fns.iter().map(|r#fn| {
        let item_vis = &r#impl
            .vis
            .map(|_| quote_spanned! { r#fn.sig.fn_token.span() => pub });

        let mut fn_sig = r#fn.sig.clone();
        if r#impl.r#trait.is_none() {
            let interface_generic_params = interface.generics.params.iter();
            let impl_generic_params = r#impl.generics.params.iter();
            let fn_generic_params = fn_sig.generics.params.iter();

            fn_sig.generics = parse_quote_spanned! {
                fn_sig.generics.span() =>
                <
                    #(#fn_generic_params, )*
                    #(#interface_generic_params, )*
                    #(#impl_generic_params, )*
                >
            };

            let interface_ident = &interface.ident;
            let interface_generic_args = generic_args(&interface.generics);
            let interface_predicates = interface
                .generics
                .where_clause
                .as_ref()
                .map(|where_clause| where_clause.predicates.iter())
                .into_iter()
                .flatten();
            let impl_predicates = r#impl
                .generics
                .where_clause
                .as_ref()
                .map(|where_clause| where_clause.predicates.iter())
                .into_iter()
                .flatten();
            let fn_predicates = r#fn
                .sig
                .generics
                .where_clause
                .as_ref()
                .map(|where_clause| where_clause.predicates.iter())
                .into_iter()
                .flatten();

            fn_sig.generics.where_clause = Some(parse_quote_spanned! {
                fn_sig.generics.span() =>

                where
                    T: #interface_ident<#(#interface_generic_args), *>
                    #(, #interface_predicates)*
                    #(, #impl_predicates)*
                    #(, #fn_predicates)*
            });
        }

        let fn_call_ident = scalar_trait_fn_ident(&r#fn.sig.ident);

        let fn_generic_args = generic_args(&r#fn.sig.generics);

        let fn_call_inputs = r#fn
            .sig
            .inputs
            .iter()
            .map(arg_ident)
            .collect::<Box<[Ident]>>();

        quote_spanned! {
            r#fn.sig.ident.span() =>

            #[inline(always)]
            #[allow(unused_mut)]
            #item_vis #fn_sig {
                T::#fn_call_ident::<
                    N, A
                    #(, #impl_generic_args)*
                    #(, #fn_generic_args)*
                >(#(#fn_call_inputs), *)
            }
        }
    });

    let output_assoc_types = &r#impl.assoc_types;

    let output_errors = r#impl.errors.iter().map(|err| err.to_compile_error());

    quote_spanned! {
        interface.ident.span() =>

        impl #output_impl_generics #output_impl_types #output_where_clause {
            #(#output_fns)*
            #(#output_assoc_types)*
        }

        #(#output_errors;)*
    }
}
