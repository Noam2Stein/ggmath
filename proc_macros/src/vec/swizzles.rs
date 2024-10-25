use super::*;

use quote::quote;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Paren},
};

pub fn swizzles_macro(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    gen_macro(tokens, quote! { $($ident:ident[$($component:literal)*])* })
}
pub fn swizzles(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    filter_swizzles(tokens, |_| true)
}
pub fn non_repeat_swizzles(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    filter_swizzles(tokens, |swizzle| {
        (0..swizzle.len())
            .all(|i0| (0..swizzle.len()).all(|i1| swizzle[i0] != swizzle[i1] || i0 == i1))
    })
}

#[derive(Parse)]
struct MacroInput {
    macro_ident: Ident,
    #[brace]
    _brace: Brace,
    #[inside(_brace)]
    macro_output: TokenStream,
}

#[derive(Parse)]
struct CallInput {
    macro_ident: Ident,
    #[paren]
    _paren: Paren,
    #[inside(_paren)]
    #[call(Self::parse_components)]
    components: Vec<char>,
}
impl CallInput {
    fn parse_components(input: ParseStream) -> syn::Result<Vec<char>> {
        Ok(Punctuated::<Ident, Token![,]>::parse_terminated(input)?
            .into_iter()
            .map(|component| {
                let str = component.to_string();
                let mut chars = str.chars();

                let c = chars.next().unwrap();
                assert!(chars.next().is_none());
                c
            })
            .collect())
    }
}

pub fn gen_macro(tokens: proc_macro::TokenStream, input: TokenStream) -> proc_macro::TokenStream {
    let MacroInput {
        macro_ident,
        _brace,
        macro_output,
    } = parse_macro_input!(tokens as MacroInput);

    quote! {
        macro_rules! #macro_ident {
            (#input) => { #macro_output }
        }
    }
    .into()
}
pub fn filter_swizzles(
    tokens: proc_macro::TokenStream,
    filter: impl FnMut(&[u8]) -> bool + Copy,
) -> proc_macro::TokenStream {
    let CallInput {
        macro_ident,
        _paren,
        components,
    } = parse_macro_input!(tokens as CallInput);

    let swizzles = collect_swizzles_1_to_n(components.len() as u8, filter)
        .into_iter()
        .map(|swizzle| {
            let name = swizzle_ident(&swizzle, &components);

            quote! { #name[#(#swizzle)*] }
        });

    quote! {
        #macro_ident!(#(#swizzles)*);
    }
    .into()
}

fn collect_swizzles_1_to_n(
    component_count: u8,
    filter: impl FnMut(&[u8]) -> bool + Copy,
) -> Vec<Vec<u8>> {
    (1..component_count + 1)
        .map(|swizzle_component_count| {
            collect_swizzles(component_count, swizzle_component_count, filter)
        })
        .flatten()
        .collect()
}
fn collect_swizzles(
    component_count: u8,
    swizzle_component_count: u8,
    mut filter: impl FnMut(&[u8]) -> bool,
) -> Vec<Vec<u8>> {
    let mut output =
        Vec::with_capacity((component_count as usize).pow(swizzle_component_count as u32));

    let mut stack = Vec::with_capacity(swizzle_component_count as usize);

    collect_swizzles_helper(
        component_count,
        swizzle_component_count,
        &mut filter,
        &mut output,
        &mut stack,
    );

    output
}
fn collect_swizzles_helper(
    component_count: u8,
    swizzle_component_count: u8,
    filter: &mut impl FnMut(&[u8]) -> bool,
    output: &mut Vec<Vec<u8>>,
    stack: &mut Vec<u8>,
) {
    if stack.len() == swizzle_component_count as usize {
        output.push(stack.clone());
    } else {
        for component in 0..component_count {
            stack.push(component);
            if filter(&stack) {
                collect_swizzles_helper(
                    component_count,
                    swizzle_component_count,
                    filter,
                    output,
                    stack,
                );
            }
            stack.pop();
        }
    }
}

fn swizzle_ident(swizzle: &[u8], components: &[char]) -> Ident {
    Ident::new(
        &swizzle
            .iter()
            .map(|swizzle_component| components[*swizzle_component as usize])
            .collect::<String>(),
        Span::call_site(),
    )
}
