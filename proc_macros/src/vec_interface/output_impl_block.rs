use super::*;

pub fn impl_block(interface: &VecInterface, r#impl: &VecInterfaceImpl) -> TokenStream {
    let output_impl_generics = {
        let scalar_trait_ident = &interface.type_param.ident;
        let interface_generic_args = generic_args(&r#impl.generics);
        let interface_generic_params = interface.type_param..params.iter();

        quote_spanned! {
            interface.generics.span() =>
            <
                const N: usize,
                T: #scalar_trait_ident<#(#interface_generic_args), *>,
                A: VecAlignment
                #(, #interface_generic_params)*
            >
        }
    };

    let output_impl_types = if let Some(impl_trait) = &r#impl.r#trait {
        quote_spanned! {
            interface.type_param.span() =>

            #impl_trait for Vector<N, T, A>
        }
    } else {
        quote_spanned! {
            interface.type_param.span() =>

            Vector<N, T, A>
        }
    };

    let output_where_clause = if let Some(interface_where_clause) = &r#impl.generics.where_clause {
        quote_spanned! {
            interface_where_clause.where_token.span() =>

            #interface_where_clause, ScalarCount<N>: VecLen<N>
        }
    } else {
        quote_spanned! {
            interface.type_param.span() =>
            where ScalarCount<N>: VecLen<N>
        }
    };

    let output_fns = interface.fns.iter().map(|r#fn| {
        let len_trait_ident = with_span(&len_trait_ident(interface), r#fn.sig.ident.span());
        let interface_generic_args = generic_args(&interface.generics);

        let item_vis = &interface.vis.map(|_| quote_spanned! { r#fn.sig.fn_token.span() => pub });
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

    let output_assoc_types = &interface.assoc_types;

    quote_spanned! {
        interface.ident.span() =>

        impl #output_impl_generics #output_impl_types #output_where_clause {
            #(#output_fns)*
            #(#output_assoc_types)*
        }
    }
}
