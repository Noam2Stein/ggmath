use super::*;

use is_sorted::IsSorted;
use quote::quote;
use syn::{
    punctuated::Punctuated,
    token::{Brace, Paren},
    Lit, LitInt,
};

pub fn swizzles(tokens: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(tokens as Input);
    let Input {
        flags: _,
        block_lengths: _,
        ident_prefix: _,
        _paren,
        block_seperator: _,
        components: _,
        ident_postfix: _,
        _brace: _,
        macro_output,
    } = &input;

    let swizzles = collect_swizzles(&input).into_iter().map(|swizzle| {
        let ident = swizzle_ident(&input, &swizzle);
        let poses = swizzle.blocks.iter().map(|block| block.pos);
        let lengths = swizzle.blocks.iter().map(|block| block.len);

        quote! { #ident[#(#poses #lengths)*] }
    });

    quote! {
        macro_rules! some_swizzles_macro {
            ($($ident:ident[$($pos:literal $len:literal)*])*) => { #macro_output }
        }
        some_swizzles_macro!(#(#swizzles)*);
    }
    .into()
}

#[derive(Parse)]
struct Input {
    flags: InputFlags,
    #[call(Self::parse_blocks)]
    block_lengths: Vec<u8>,
    ident_prefix: Option<Ident>,
    #[paren]
    _paren: Paren,
    #[inside(_paren)]
    block_seperator: Option<Ident>,
    #[inside(_paren)]
    #[call(Self::parse_components)]
    components: Vec<char>,
    ident_postfix: Option<Ident>,
    #[prefix(Token![=>])]
    #[brace]
    _brace: Brace,
    #[inside(_brace)]
    macro_output: TokenStream,
}
#[derive(Default)]
struct InputFlags {
    only_sorted: bool,
    only_unique: bool,
}
impl Input {
    fn parse_blocks(input: ParseStream) -> syn::Result<Vec<u8>> {
        let mut output = Vec::new();
        while input.peek(Lit) {
            output.push(input.parse::<LitInt>()?.base10_parse()?);
        }

        Ok(output)
    }
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
impl Parse for InputFlags {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut output = Self::default();

        while let Some(ident) = input.parse::<Option<Ident>>()? {
            match ident.to_string().as_str() {
                "sorted" => output.only_sorted = true,
                "unique" => output.only_unique = true,
                _ => {
                    return Err(Error::new(
                        ident.span(),
                        "expected either 'sorted' or 'unique'",
                    ))
                }
            }
        }

        Ok(output)
    }
}

#[derive(Debug, Clone)]
struct Swizzle {
    blocks: Box<[SwizzleBlock]>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct SwizzleBlock {
    pos: u8,
    len: u8,
}

fn collect_swizzles(input: &Input) -> Vec<Swizzle> {
    let mut output =
        Vec::with_capacity(input.block_lengths.len().pow(input.components.len() as u32));

    let mut stack = Vec::with_capacity(input.block_lengths.len());

    collect_swizzles_helper(input, &mut output, &mut stack);

    output
}
fn collect_swizzles_helper(
    input: &Input,
    output: &mut Vec<Swizzle>,
    stack: &mut Vec<SwizzleBlock>,
) {
    if let Some(block_len) = input.block_lengths.get(stack.len()) {
        for block_pos in 0..input.components.len() {
            stack.push(SwizzleBlock {
                pos: block_pos as u8,
                len: *block_len,
            });
            if filter_swizzle_stack(input, &stack) {
                collect_swizzles_helper(input, output, stack);
            }
            stack.pop();
        }
    } else {
        output.push(Swizzle {
            blocks: stack.clone().into_boxed_slice(),
        });
    }
}
fn filter_swizzle_stack(input: &Input, stack: &[SwizzleBlock]) -> bool {
    if input.flags.only_sorted && stack.iter().is_sorted() {
        false
    } else {
        true
    }
}

fn swizzle_ident(input: &Input, swizzle: &Swizzle) -> Ident {
    Ident::new(
        &format!(
            "{}{}{}",
            input
                .ident_prefix
                .as_ref()
                .map_or("".to_string(), |prefix| prefix.to_string()),
            &swizzle
                .blocks
                .iter()
                .map(|block| (block.pos..block.pos + block.len)
                    .map(|component| input.components[component as usize])
                    .collect::<String>())
                .collect::<Box<[String]>>()
                .join(
                    &input
                        .block_seperator
                        .as_ref()
                        .map_or_else(|| String::new(), |ident| ident.to_string())
                ),
            input
                .ident_postfix
                .as_ref()
                .map_or("".to_string(), |postfix| postfix.to_string()),
        ),
        Span::call_site(),
    )
}
