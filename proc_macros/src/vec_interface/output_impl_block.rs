use super::*;

pub fn impl_block(input: &VecInterface) -> TokenStream {
    let scalar_trait_ident = &input.scalar_trait.ident;
    let len_trait_ident = len_trait_ident(input);

    let interface_generic_args = generic_args(&input.generics);

    let interface_generic_params = input.generics.params.iter();
    let output_impl_generics = quote_spanned! {
        input.ident.span() =>
        <
            const N: usize,
            T: #scalar_trait_ident<#(#interface_generic_args), *>,
            A: VecAlignment
            #(, #interface_generic_params)*
        >
    };

    let output_impl_types = if input.impl_trait.is_some() {
        let trait_ident = &input.ident;
        quote_spanned! {
            input.ident.span() =>
            #trait_ident<#(#interface_generic_args), *> for Vector<N, T, A>
        }
    } else {
        quote_spanned! {
            input.ident.span() =>
            Vector<N, T, A>
        }
    };

    let output_where_clause = if let Some(interface_where_clause) = &input.generics.where_clause {
        quote_spanned! {
            input.ident.span() =>
            #interface_where_clause, ScalarCount<N>: VecLen<N>
        }
    } else {
        quote_spanned! {
            input.ident.span() =>
            where ScalarCount<N>: VecLen<N>
        }
    };

    let output_fn_impls = input.fns.iter().map(|r#fn| {
        let item_vis = &input.vis;
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

    let output_assoc_types = input.assoc_types.iter().map(|assoc_type| {
        let item_vis = &input.vis;
        let assoc_type_ident = &assoc_type.ident;
        let assoc_type_generics = &assoc_type.generics;

        let value_generic_args = generic_args(&assoc_type.generics);

        quote_spanned! {
            assoc_type.ident.span() =>

            #item_vis type #assoc_type_ident #assoc_type_generics =
                <ScalarCount<N> as #len_trait_ident<N>>
                    ::#assoc_type_ident::<T, A  #(, #interface_generic_args)* #(, #value_generic_args)*>;
        }
    });

    quote_spanned! {
        input.ident.span() =>
        impl #output_impl_generics #output_impl_types #output_where_clause {
            #(#output_fn_impls)*
            #(#output_assoc_types)*
        }
    }
}
