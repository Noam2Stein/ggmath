use super::*;

pub fn alignment(input: &VecInterface) -> TokenStream {
    let scalar_trait = &input.scalar_trait.ident;
    let alignment_trait = alignment_trait_ident(input);

    let generic_params = input.generics.params.iter();

    let fn_sigs = input
        .fns
        .iter()
        .map(|r#fn| {
            let mut sig = r#fn.sig.clone();
            sig.generics.params.insert(
                0,
                parse_quote_spanned! { sig.ident.span() => T: #scalar_trait },
            );

            sig
        })
        .collect::<Box<[Signature]>>();

    let fn_declarations = fn_sigs.iter().map(|sig| {
        search_replace_fn(
            quote_spanned! { sig.fn_token.span => #[allow(missing_docs)] },
            sig.clone(),
            None,
            |span| quote_spanned! { span => N },
            |span| quote_spanned! { span => T },
            |span| quote_spanned! { span => Self },
        )
    });

    let impls = LEN_STRS
        .map(|n_str| {
            let n = LitInt::new(n_str, alignment_trait.span());
            
            ALIGN_STRS.map(|a_str| {
                let a = Ident::new(a_str, alignment_trait.span());
                
                let generic_params = input.generics.params.iter();

                let fn_impls = fn_sigs.iter().map(|sig| {
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
                    input.ident.span() =>
                    impl #alignment_trait<#n #(, #generic_params)*> for #a {
                        #(#fn_impls)*
                    }
                }
            })
        })
        .into_iter()
        .flatten();

    quote_spanned! {
        alignment_trait.span() =>

        pub(super) trait #alignment_trait<const N: usize #(, #generic_params)*>: alignment_seal::VecAlignment where ScalarCount<N>: VecLen<N> {
            #(#fn_declarations)*
        }

        #(#impls)*
    }
}
