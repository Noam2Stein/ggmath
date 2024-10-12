use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, GenericParam, ItemImpl, PathArguments, Type};

pub fn impl_vecn(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);

    input.generics.params.insert(
        0,
        GenericParam::Const(
            parse2(quote! { const N: usize })
                .unwrap_or_else(|err| panic!("failed to parse N: {err}")),
        ),
    );
    input.generics.params.insert(
        1,
        GenericParam::Const(
            parse2(quote! { const A: bool })
                .unwrap_or_else(|err| panic!("failed to parse A: {err}")),
        ),
    );
    input.generics.params.insert(
        2,
        GenericParam::Type(
            parse2(quote! { T: Scalar }).unwrap_or_else(|err| panic!("failed to parse T: {err}")),
        ),
    );

    let where_clause = input.generics.where_clause.get_or_insert_with(|| {
        parse2(quote! { where }).unwrap_or_else(|err| panic!("failed to parse where: {err}"))
    });
    where_clause.predicates.insert(
        0,
        parse2(quote! { ScalarCount<N>: VecLen<N> })
            .unwrap_or_else(|err| panic!("failed to parse where N: {err}")),
    );
    where_clause.predicates.insert(
        1,
        parse2(quote! { VecAligned<A>: VecAlignment<A> })
            .unwrap_or_else(|err| panic!("failed to parse where A: {err}")),
    );

    match &mut *input.self_ty {
        Type::Path(ty) => {
            let args = &mut ty
                .path
                .segments
                .last_mut()
                .unwrap_or_else(|| panic!("failed to get self path"))
                .arguments;

            if let PathArguments::None = args {
                *args = PathArguments::AngleBracketed(
                    parse2(quote! { <> })
                        .unwrap_or_else(|err| panic!("failed to parse new args: {err}")),
                )
            }

            match args {
                PathArguments::AngleBracketed(args) => {
                    args.args.insert(
                        0,
                        parse2(quote! { N })
                            .unwrap_or_else(|err| panic!("failed to parse self N: {err}")),
                    );
                    args.args.insert(
                        1,
                        parse2(quote! { A })
                            .unwrap_or_else(|err| panic!("failed to parse self A: {err}")),
                    );
                    args.args.insert(
                        2,
                        parse2(quote! { T })
                            .unwrap_or_else(|err| panic!("failed to parse self T: {err}")),
                    );
                }
                _ => {}
            }
        }
        _ => {}
    }

    input.into_token_stream().into()
}
