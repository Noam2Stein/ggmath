mod vec;
mod swizzle;

use std::iter::once;

use proc_macro2::{Group, Literal, Span, TokenStream, TokenTree};

use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::{self, Parse}, parse_macro_input, token::Brace, Type};
use swizzle::*;

#[proc_macro]
pub fn vec2_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec2.get.iter()).into()
}
#[proc_macro]
pub fn vec3_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3.get.iter()).into()
}
#[proc_macro]
pub fn vec3a_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3a.get.iter()).into()
}
#[proc_macro]
pub fn vec4_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec4.get.iter()).into()
}

#[proc_macro]
pub fn vec2_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec2.mut_.iter()).into()
}
#[proc_macro]
pub fn vec3_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3.mut_.iter()).into()
}
#[proc_macro]
pub fn vec3a_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3a.mut_.iter()).into()
}
#[proc_macro]
pub fn vec4_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec4.mut_.iter()).into()
}

#[proc_macro]
pub fn vec2_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec2.set.iter()).into()
}
#[proc_macro]
pub fn vec3_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3.set.iter()).into()
}
#[proc_macro]
pub fn vec3a_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3a.set.iter()).into()
}
#[proc_macro]
pub fn vec4_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec4.set.iter()).into()
}

#[proc_macro]
pub fn vec2_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec2.with.iter()).into()
}
#[proc_macro]
pub fn vec3_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3.with.iter()).into()
}
#[proc_macro]
pub fn vec3a_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3a.with.iter()).into()
}
#[proc_macro]
pub fn vec4_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec4.with.iter()).into()
}

#[proc_macro]
pub fn vec2_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec2.iter()).into()
}
#[proc_macro]
pub fn vec3_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3.iter()).into()
}
#[proc_macro]
pub fn vec3a_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec3a.iter()).into()
}
#[proc_macro]
pub fn vec4_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.vec4.iter()).into()
}

#[proc_macro]
pub fn any_vec_get(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(
        input.into(),
        SWIZZLES.vec2.get.iter().chain(
            SWIZZLES.vec3.get.iter().chain(
                SWIZZLES.vec3a.get.iter().chain(
                    SWIZZLES.vec4.get.iter()
                )
            )
        )
    ).into()
}
#[proc_macro]
pub fn any_vec_mut(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(
        input.into(),
        SWIZZLES.vec2.mut_.iter().chain(
            SWIZZLES.vec3.mut_.iter().chain(
                SWIZZLES.vec3a.mut_.iter().chain(
                    SWIZZLES.vec4.mut_.iter()
                )
            )
        )
    ).into()
}
#[proc_macro]
pub fn any_vec_set(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(
        input.into(),
        SWIZZLES.vec2.set.iter().chain(
            SWIZZLES.vec3.set.iter().chain(
                SWIZZLES.vec3a.set.iter().chain(
                    SWIZZLES.vec4.set.iter()
                )
            )
        )
    ).into()
}
#[proc_macro]
pub fn any_vec_with(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(
        input.into(),
        SWIZZLES.vec2.with.iter().chain(
            SWIZZLES.vec3.with.iter().chain(
                SWIZZLES.vec3a.with.iter().chain(
                    SWIZZLES.vec4.with.iter()
                )
            )
        )
    ).into()
}

#[proc_macro]
pub fn any_vec_any(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    swizzle(input.into(), SWIZZLES.iter()).into()
}

struct MacroInput {
    element_ty: Type,
    brace: Brace,
    content: TokenStream,
}
impl Parse for MacroInput {
    fn parse(input: parse::ParseStream) -> syn::Result<Self> {
        let element_ty = input.parse()?;
        
        let content;
        let brace = syn::braced!(content in input);
        let content = content.parse::<proc_macro2::TokenStream>()?.into();

        Ok(
            Self {
                element_ty,
                brace,
                content,
            }
        )
    }
}

fn swizzle<'a>(input: proc_macro::TokenStream, swizzles: impl Iterator<Item = &'a Swizzle>) -> proc_macro::TokenStream {
    fn process_stream<'a>(element_ty: &Type, mut input: impl Iterator<Item = TokenTree>, repeatition: &mut [(&'a Swizzle, TokenStream)], close_span: Span) {
        fn process_stream_with_reflection<'a>(element_ty: &Type, mut input: impl Iterator<Item = TokenTree>, repeatition: &mut [(&'a Swizzle, &'a SwizzleReflection, TokenStream)], close_span: Span) {
            const EXPECTATION: &str = "either: '#', 'ident', 'self_ty', 'value_ty', 'inner_ident', 'inner_self', 'inner_value', 'self_component', 'value_component', or 'len'";

            while let Some(token) = input.next() {
                match &token {
                    TokenTree::Punct(punct) => {
                        if punct.as_char() != '#' {
                            for (_, _, output) in repeatition.iter_mut() {
                                output.extend(once(token.clone()));
                            }
                        }
                        else if let Some(token) = input.next() {
                            match &token {
                                TokenTree::Ident(ident) => {
                                    for (swizzle, reflection, output) in repeatition.iter_mut() {
                                        match ident.to_string().as_str() {
                                            "ident" => output.extend(once(TokenTree::Ident(swizzle.ident()))),
                                            "self_ty" => output.extend(swizzle.self_ty.ty(element_ty).into_token_stream()),
                                            "value_ty" => output.extend(swizzle.value_ty.ty(element_ty).into_token_stream()),
                                            "inner_ident" => output.extend(once(TokenTree::Ident(swizzle.inner_ident()))),
                                            "inner_self" => output.extend(swizzle.self_ty.inner_ty(element_ty).into_token_stream()),
                                            "inner_value" => output.extend(swizzle.value_ty.inner_ty(element_ty).into_token_stream()),
                                            "self_component" => output.extend(once(TokenTree::Literal(Literal::usize_suffixed(reflection.self_component)))),
                                            "value_component" => output.extend(once(TokenTree::Literal(Literal::usize_suffixed(reflection.self_component)))),
                                            "len" => output.extend(once(TokenTree::Literal(Literal::usize_suffixed(reflection.len)))),
                                            _ => {
                                                let err = format!("expected {EXPECTATION}. found '{ident}'.");
                                                output.extend(
                                                    quote_spanned! {
                                                        ident.span() => compile_error!(#err);
                                                    }.into_token_stream()
                                                )
                                            }
                                        }
                                    }
                                }
                                TokenTree::Group(group) => {
                                    let err = format!("expected {EXPECTATION}. found '{{'.");
                                    for (_, _, output) in repeatition.iter_mut() {
                                        output.extend(
                                            quote_spanned! {
                                                group.span_open() => compile_error!(#err);
                                            }.into_token_stream()
                                        )
                                    }
                                }
                                TokenTree::Punct(punct) => {
                                    for (_, _, output) in repeatition.iter_mut() {
                                        if punct.as_char() == '#' {
                                            output.extend(once(token.clone()));
                                        }
                                        else {
                                            let err = format!("expected {EXPECTATION}. found '{{'.");
                                            output.extend(
                                                quote_spanned! {
                                                    punct.span() => compile_error!(#err);
                                                }.into_token_stream()
                                            )
                                        }
                                    }
                                }
                                TokenTree::Literal(literal) => {
                                    let err = format!("expected {EXPECTATION}. found '{literal}'.");
                                    for (_, _, output) in repeatition.iter_mut() {
                                        output.extend(
                                            quote_spanned! {
                                                literal.span() => compile_error!(#err);
                                            }.into_token_stream()
                                        )
                                    }
                                }
                            }
                        }
                        else {
                            let err = format!("unexpected end of repeatition. expected {EXPECTATION}.");
                            for (_, _, output) in repeatition.iter_mut() {
                                output.extend(
                                    quote_spanned! {
                                        close_span => compile_error!(#err);
                                    }.into_token_stream()
                                )
                            }
                        }
                    }
                    TokenTree::Group(group) => {
                        let mut group_repeatition = repeatition.iter().map(|(swizzle, reflection, _)| (*swizzle, *reflection, TokenStream::new())).collect::<Box<[(&Swizzle, &SwizzleReflection, TokenStream)]>>();

                        process_stream_with_reflection(element_ty, group.stream().into_iter(), &mut group_repeatition, close_span);
                        
                        for ((_, _, output), (_, _, stream)) in repeatition.iter_mut().zip(group_repeatition.into_vec().into_iter()) {
                            output.extend(
                                once(
                                    TokenTree::Group(
                                        Group::new(group.delimiter(), stream)
                                    )
                                )
                            );
                        }
                    }
                    TokenTree::Ident(_) => {
                        for (_, _, output) in repeatition.iter_mut() {
                            output.extend(once(token.clone()));
                        }
                    }
                    TokenTree::Literal(_) => {
                        for (_, _, output) in repeatition.iter_mut() {
                            output.extend(once(token.clone()));
                        }
                    }
                }
            }
        }

        const EXPECTATION: &str = "either: '#', 'ident', 'self_ty', 'value_ty', 'inner_ident', 'inner_self', 'inner_value', or '{{'";

        while let Some(token) = input.next() {
            match &token {
                TokenTree::Punct(punct) => {
                    if punct.as_char() != '#' {
                        for (_, output) in repeatition.iter_mut() {
                            output.extend(once(token.clone()));
                        }
                    }
                    else if let Some(token) = input.next() {
                        match &token {
                            TokenTree::Ident(ident) => {
                                for (swizzle, output) in repeatition.iter_mut() {
                                    match ident.to_string().as_str() {
                                        "ident" => output.extend(once(TokenTree::Ident(swizzle.ident()))),
                                        "self_ty" => output.extend(swizzle.self_ty.ty(element_ty).into_token_stream()),
                                        "value_ty" => output.extend(swizzle.value_ty.ty(element_ty).into_token_stream()),
                                        "inner_ident" => output.extend(once(TokenTree::Ident(swizzle.inner_ident()))),
                                        "inner_self" => output.extend(swizzle.self_ty.inner_ty(element_ty).into_token_stream()),
                                        "inner_value" => output.extend(swizzle.value_ty.inner_ty(element_ty).into_token_stream()),
                                        _ => {
                                            let err = format!("expected {EXPECTATION}. found '{ident}'.");
                                            output.extend(
                                                quote_spanned! {
                                                    ident.span() => compile_error!(#err);
                                                }.into_token_stream()
                                            )
                                        }
                                    }
                                }
                            }
                            TokenTree::Group(group) => {
                                let mut reflection_repeatition = repeatition.iter().map(|(swizzle, _)| swizzle.reflections.iter().map(|reflection| (*swizzle, reflection, TokenStream::new()))).flatten().collect::<Box<[(&Swizzle, &SwizzleReflection, TokenStream)]>>();
                                
                                process_stream_with_reflection(element_ty, group.stream().into_token_stream().into_iter(), &mut reflection_repeatition, close_span);

                                let mut reflection_repeatition = reflection_repeatition.into_vec().into_iter().peekable();
                                for (swizzle, output) in repeatition.iter_mut() {
                                    while let Some((peek_swizzle, _, _)) = reflection_repeatition.peek() {
                                        if *peek_swizzle == *swizzle {
                                            let (_, _, stream) = reflection_repeatition.next().unwrap();

                                            output.extend(stream.into_iter());
                                        }
                                        else {
                                            break;
                                        }
                                    }
                                }
                            }
                            TokenTree::Punct(punct) => {
                                for (_, output) in repeatition.iter_mut() {
                                    if punct.as_char() == '#' {
                                        output.extend(once(token.clone()));
                                    }
                                    else {
                                        let err = format!("expected {EXPECTATION}. found '{{'.");
                                        output.extend(
                                            quote_spanned! {
                                                punct.span() => compile_error!(#err);
                                            }.into_token_stream()
                                        )
                                    }
                                }
                            }
                            TokenTree::Literal(literal) => {
                                let err = format!("expected {EXPECTATION}. found '{literal}'.");
                                for (_, output) in repeatition.iter_mut() {
                                    output.extend(
                                        quote_spanned! {
                                            literal.span() => compile_error!(#err);
                                        }.into_token_stream()
                                    )
                                }
                            }
                        }
                    }
                    else {
                        let err = format!("unexpected end of repeatition. expected {EXPECTATION}.");
                        for (_, output) in repeatition.iter_mut() {
                            output.extend(
                                quote_spanned! {
                                    close_span => compile_error!(#err);
                                }.into_token_stream()
                            )
                        }
                    }
                }
                TokenTree::Group(group) => {
                    let mut group_repeatition = repeatition.iter().map(|(swizzle, _)| (*swizzle, TokenStream::new())).collect::<Box<[(&Swizzle, TokenStream)]>>();

                    process_stream(element_ty, group.stream().into_iter(), &mut group_repeatition, close_span);
                    
                    for ((_, output), (_, stream)) in repeatition.iter_mut().zip(group_repeatition.into_vec().into_iter()) {
                        output.extend(
                            once(
                                TokenTree::Group(
                                    Group::new(group.delimiter(), stream)
                                )
                            )
                        );
                    }
                }
                TokenTree::Ident(_) => {
                    for (_, output) in repeatition.iter_mut() {
                        output.extend(once(token.clone()));
                    }
                }
                TokenTree::Literal(_) => {
                    for (_, output) in repeatition.iter_mut() {
                        output.extend(once(token.clone()));
                    }
                }
            }
        }
    }

    
    let input = parse_macro_input!(input as MacroInput);
    
    let mut repeatition = swizzles.map(|swizzle| (swizzle, TokenStream::new())).collect::<Box<[(&Swizzle, TokenStream)]>>();

    process_stream(&input.element_ty, input.content.into_iter(), &mut repeatition, input.brace.span.close());

    println!("{}", repeatition.iter().fold(TokenStream::new(), |acc, (_, stream)| {
        let new_stream = quote! {
            #acc
            #stream
        };
        new_stream
    }));

    repeatition.iter().fold(TokenStream::new(), |acc, (_, stream)| {
        let new_stream = quote! {
            #acc
            #stream
        };
        new_stream
    }).into()
}