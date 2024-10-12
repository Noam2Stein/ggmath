use quote::{quote, ToTokens};
use syn::{parse2, parse_macro_input, ItemTrait};

pub fn inner_vecn_trait(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemTrait);

    input.generics.params.insert(
        0,
        parse2(quote! { const N: usize }).unwrap_or_else(|err| panic!("failed to parse N: {err}")),
    );
    input.generics.params.insert(
        0,
        parse2(quote! { const A: bool }).unwrap_or_else(|err| panic!("failed to parse A: {err}")),
    );
    input.generics.params.insert(
        0,
        parse2(quote! { T: ScalarVecInner })
            .unwrap_or_else(|err| panic!("failed to parse T: {err}")),
    );

    let where_caluse = input.generics.where_clause.get_or_insert_with(|| {
        parse2(quote! { where }).unwrap_or_else(|err| panic!("failed to parse new where: {err}"))
    });
    where_caluse.predicates.insert(
        0,
        parse2(quote! { ScalarCount<N>: VecLen<N> })
            .unwrap_or_else(|err| panic!("failed to parse where N: {err}")),
    );
    where_caluse.predicates.insert(
        1,
        parse2(quote! { VecAligned<A>: VecAlignment<A> })
            .unwrap_or_else(|err| panic!("failed to parse where A: {err}")),
    );

    input.to_token_stream().into()
}
