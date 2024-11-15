use super::*;

pub fn impl_block(interface: &VecInterface, r#impl: &VecInterfaceImpl) -> TokenStream {
    let output_impl_generics = {
        let interface_ident = &interface.ident;
        let interface_generic_params = interface.generics.params.iter();
        let interface_generic_args = generic_args(&interface.generics);
        let impl_generic_params = r#impl.generics.params.iter();

        quote_spanned! {
            interface.generics.span() =>
            <
                const N: usize,
                T: #interface_ident<#(#interface_generic_args), *>,
                A: VecAlignment
                #(, #interface_generic_params)*
                #(, #impl_generic_params)*
            >
        }
    };

    let output_impl_types = if let Some(impl_trait) = &r#impl.r#trait {
        quote_spanned! {
            interface.ident.span() =>

            #impl_trait for Vector<N, T, A>
        }
    } else {
        quote_spanned! {
            interface.ident.span() =>

            Vector<N, T, A>
        }
    };

    let output_where_clause = if let Some(interface_where_clause) = &interface.generics.where_clause
    {
        if let Some(impl_where_clause) = &r#impl.generics.where_clause {
            let impl_where_clause_predicates = impl_where_clause.predicates.iter();

            quote_spanned! {
                interface_where_clause.where_token.span() =>

                #interface_where_clause, #(#impl_where_clause_predicates), *, ScalarCount<N>: VecLen<N>
            }
        } else {
            quote_spanned! {
                interface_where_clause.where_token.span() =>

                #interface_where_clause, ScalarCount<N>: VecLen<N>
            }
        }
    } else {
        quote_spanned! {
            interface.ident.span() =>

            where ScalarCount<N>: VecLen<N>
        }
    };

    let output_fns = r#impl.fns.iter().map(|r#fn| {
        let len_trait_ident = with_span(&len_trait_ident(interface), r#fn.sig.ident.span());
        let interface_generic_args = generic_args(&interface.generics);

        let item_vis = &r#impl.vis.map(|_| quote_spanned! { r#fn.sig.fn_token.span() => pub });
        let fn_sig = &r#fn.sig;

        let fn_call_ident = &r#fn.sig.ident;
        let fn_call_generic_args = generic_args(&r#fn.sig.generics);
        let fn_call_inputs = r#fn.sig.inputs.iter().map(arg_ident);

        quote_spanned! {
            fn_call_ident.span() =>

            #[inline(always)]
            #[allow(unused_mut)]
            #item_vis #fn_sig {
                <ScalarCount<N> as #len_trait_ident<N>>
                    ::#fn_call_ident::<T, A #(, #interface_generic_args)* #(, #fn_call_generic_args)*>
                        (#(#fn_call_inputs), *)
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
