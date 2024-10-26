use super::*;

use quote::quote;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Paren},
    LitInt,
};

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
struct Input {
    swizzle_component_count: LitInt,
    ident_prefix: Option<Ident>,
    #[paren]
    _paren: Paren,
    #[inside(_paren)]
    #[call(Self::parse_components)]
    components: Vec<char>,
    #[prefix(Token![=>])]
    #[brace]
    _brace: Brace,
    #[inside(_brace)]
    macro_output: TokenStream,
}
impl Input {
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

pub fn filter_swizzles(
    tokens: proc_macro::TokenStream,
    filter: impl FnMut(&[u8]) -> bool + Copy,
) -> proc_macro::TokenStream {
    let Input {
        swizzle_component_count,
        ident_prefix,
        _paren,
        components,
        _brace,
        macro_output,
    } = parse_macro_input!(tokens as Input);

    let swizzle_component_count = match swizzle_component_count.base10_parse() {
        Ok(ok) => ok,
        Err(err) => return err.into_compile_error().into(),
    };

    let swizzles = collect_swizzles(components.len() as u8, swizzle_component_count, filter)
        .into_iter()
        .map(|swizzle| {
            let ident = swizzle_ident(ident_prefix.as_ref(), &swizzle, &components);
            let swizzle = swizzle
                .into_iter()
                .map(|component| LitInt::new(component.to_string().as_str(), Span::call_site()));

            quote! { #ident[#(#swizzle)*] }
        });

    quote! {
        macro_rules! some_swizzles_macro {
            ($($ident:ident[$($component:literal)*])*) => { #macro_output }
        }
        some_swizzles_macro!(#(#swizzles)*);
    }
    .into()
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

fn swizzle_ident(prefix: Option<&Ident>, swizzle: &[u8], components: &[char]) -> Ident {
    Ident::new(
        &format!(
            "{}{}",
            prefix.map_or("".to_string(), |prefix| prefix.to_string()),
            &swizzle
                .iter()
                .map(|swizzle_component| components[*swizzle_component as usize])
                .collect::<String>(),
        ),
        Span::call_site(),
    )
}
