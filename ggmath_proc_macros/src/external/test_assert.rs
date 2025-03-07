use super::*;

use quote::quote;
use syn::{punctuated::Punctuated, Expr};

#[inline(always)]
pub fn test_assert(input: TokenStream1) -> TokenStream1 {
    let Input {
        fn_desc,
        value,
        expected,
        inputs,
    } = parse_macro_input!(input);

    test_assert_helper(quote! { #fn_desc }, value, expected, inputs)
}

#[inline(always)]
pub fn vec_test_assert(input: TokenStream1) -> TokenStream1 {
    let TypeInput {
        fn_desc,
        value,
        expected,
        inputs,
    } = parse_macro_input!(input);

    test_assert_helper(
        quote! { ggmath::testing::TestFnDesc::vector::<N, T, A>(stringify!(#fn_desc)) },
        value,
        expected,
        inputs,
    )
}

#[inline(always)]
pub fn mat_test_assert(input: TokenStream1) -> TokenStream1 {
    let TypeInput {
        fn_desc,
        value,
        expected,
        inputs,
    } = parse_macro_input!(input);

    test_assert_helper(
        quote! { ggmath::testing::TestFnDesc::matrix::<C, R, T, A, M>(stringify!(#fn_desc)) },
        value,
        expected,
        inputs,
    )
}

#[inline(always)]
pub fn rect_test_assert(input: TokenStream1) -> TokenStream1 {
    let TypeInput {
        fn_desc,
        value,
        expected,
        inputs,
    } = parse_macro_input!(input);

    test_assert_helper(
        quote! { ggmath::testing::TestFnDesc::rectangle::<N, T, A, R>(stringify!(#fn_desc)) },
        value,
        expected,
        inputs,
    )
}

#[derive(Parse)]
struct Input {
    fn_desc: Expr,
    #[prefix(Token![,])]
    value: Expr,
    #[prefix(Token![,])]
    expected: Expr,
    #[call(|input: ParseStream| Ok(
        if input.parse::<Option<Token![;]>>()?.is_some() {
            Punctuated::parse_terminated(input)?
        } else {
            Punctuated::new()
        }
    ))]
    inputs: Punctuated<Ident, Token![,]>,
}

#[derive(Parse)]
struct TypeInput {
    fn_desc: Ident,
    #[prefix(Token![:])]
    value: Expr,
    #[prefix(Token![,])]
    expected: Expr,
    #[call(|input: ParseStream| Ok(
        if input.parse::<Option<Token![;]>>()?.is_some() {
            Punctuated::parse_terminated(input)?
        } else {
            Punctuated::new()
        }
    ))]
    inputs: Punctuated<Ident, Token![,]>,
}

fn test_assert_helper(
    fn_desc: TokenStream,
    value: Expr,
    expected: Expr,
    inputs: Punctuated<Ident, Token![,]>,
) -> TokenStream1 {
    let inputs_lit = inputs
        .iter()
        .map(|input| format!(" * {} = {{:?}}\n", input))
        .collect::<String>();

    let inputs = inputs.into_iter();

    quote! {
        ggmath::testing::_test_assert_helper(
            || #fn_desc,
            || #value,
            || #expected,
            || format!(#inputs_lit, #(#inputs), *),
        )?;
    }
    .into()
}
