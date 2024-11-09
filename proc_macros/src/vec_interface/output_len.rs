use super::*;

pub fn len(input: &VecInterface) -> TokenStream {
    let scalar_trait_ident = &input.scalar_trait.ident;
    let len_trait_ident = len_trait_ident(input);
    let alignment_trait_ident = alignment_trait_ident(input);

    let interface_generics = &input.generics;
    let interface_generic_args = generic_args(&input.generics);

    let output_trait_where_clause =
        if let Some(interface_where_clause) = &input.generics.where_clause {
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

    let modified_fn_sigs = input
        .fns
        .iter()
        .map(|r#fn| {
            let mut sig = r#fn.sig.clone();
            sig.generics.params.insert(
                0,
                parse_quote_spanned! { sig.ident.span() => T: #scalar_trait_ident },
            );
            sig.generics.params.insert(
                1,
                parse_quote_spanned! { sig.ident.span() => A: VecAlignment },
            );

            sig
        })
        .collect::<Box<[Signature]>>();

    let output_trait_fn_declarations = modified_fn_sigs.iter().map(|sig| {
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

        let fn_impls = modified_fn_sigs.iter().map(|sig| {
            let n = Lit::Int(LitInt::new(n_str, sig.ident.span()));

            let fn_ident = &sig.ident;
            let fn_generics = &generic_args(&sig.generics)[2..];
            let fn_input_idents = sig.inputs.iter().map(|input| arg_ident(input));

            search_replace_fn(
                quote_spanned! { sig.fn_token.span => #[allow(unused_mut)] #[allow(missing_docs)] },
                sig.clone(),
                Some(quote_spanned! { sig.ident.span() => {
                    <A as #alignment_trait_ident<N>>
                        ::#fn_ident::<T #(, #fn_generics)*>(#(#fn_input_idents), *)
                } }),
                |span| quote_spanned! { span => #n },
                |span| quote_spanned! { span => T },
                |span| quote_spanned! { span => A },
            )
        });

        quote_spanned! {
            input.ident.span() =>
            impl #interface_generics #len_trait_ident<#n #(, #interface_generic_args)*> for ScalarCount<#n> {
                #(#fn_impls)*
            }
        }
    });

    let interface_generic_params = input.generics.params.iter();
    quote_spanned! {
        len_trait_ident.span() =>

        pub(super) trait #len_trait_ident<const N: usize #(, #interface_generic_params)*>: VecLenInnerVec #output_trait_where_clause {
            #(#output_trait_fn_declarations)*
        }
        #(#output_trait_impls)*
    }
}
