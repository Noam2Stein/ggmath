mod vec;
mod swizzle;

use std::iter::once;

use const_format::formatcp;
use proc_macro2::{Delimiter, Group, Literal, Span, TokenStream, TokenTree};

use quote::{quote, quote_spanned, ToTokens};
use syn::{parse::{self, Parse}, parse_macro_input, token::Brace, Type};
use swizzle::*;

macro_rules! swizzle_macro {
    ($ident:ident, $iter:expr) => {
        #[proc_macro]
        pub fn $ident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            process_swizzle(input.into(), $iter).into()
        }
    };
}

swizzle_macro!(vec2_get_swizzle, SWIZZLES.vec2.get.iter());
swizzle_macro!(vec3_get_swizzle, SWIZZLES.vec3.get.iter());
swizzle_macro!(vec3a_get_swizzle, SWIZZLES.vec3a.get.iter());
swizzle_macro!(vec4_get_swizzle, SWIZZLES.vec4.get.iter());
swizzle_macro!(vec2_mut_swizzle, SWIZZLES.vec2.mut_.iter());
swizzle_macro!(vec3_mut_swizzle, SWIZZLES.vec3.mut_.iter());
swizzle_macro!(vec3a_mut_swizzle, SWIZZLES.vec3a.mut_.iter());
swizzle_macro!(vec4_mut_swizzle, SWIZZLES.vec4.mut_.iter());
swizzle_macro!(vec2_set_swizzle, SWIZZLES.vec2.set.iter());
swizzle_macro!(vec3_set_swizzle, SWIZZLES.vec3.set.iter());
swizzle_macro!(vec3a_set_swizzle, SWIZZLES.vec3a.set.iter());
swizzle_macro!(vec4_set_swizzle, SWIZZLES.vec4.set.iter());
swizzle_macro!(vec2_with_swizzle, SWIZZLES.vec2.with.iter());
swizzle_macro!(vec3_with_swizzle, SWIZZLES.vec3.with.iter());
swizzle_macro!(vec3a_with_swizzle, SWIZZLES.vec3a.with.iter());
swizzle_macro!(vec4_with_swizzle, SWIZZLES.vec4.with.iter());
swizzle_macro!(vec2_swizzle, SWIZZLES.vec2.iter());
swizzle_macro!(vec3_swizzle, SWIZZLES.vec3.iter());
swizzle_macro!(vec3a_swizzle, SWIZZLES.vec3a.iter());
swizzle_macro!(vec4_swizzle, SWIZZLES.vec4.iter());
swizzle_macro!(get_swizzle, SWIZZLES.get_iter());
swizzle_macro!(mut_swizzle, SWIZZLES.mut_iter());
swizzle_macro!(set_swizzle, SWIZZLES.set_iter());
swizzle_macro!(with_swizzle, SWIZZLES.with_iter());
swizzle_macro!(swizzle, SWIZZLES.iter());

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

macro_rules! group_open {
    ($group:expr) => {
        match $group.delimiter() {
            Delimiter::Brace => "{",
            Delimiter::Bracket => "[",
            Delimiter::Parenthesis => "(",
            Delimiter::None => ""
        }
    };
}

fn process_swizzle<'a>(input: proc_macro::TokenStream, swizzles: impl Iterator<Item = &'a Swizzle>) -> proc_macro::TokenStream {
    fn process_stream<'a>(element_ty: &Type, mut input: impl Iterator<Item = TokenTree>, repeatition: &mut [(&'a Swizzle, TokenStream)], close_span: Span) -> Result<(), TokenStream> {
        const IDENT: &str = "ident";
        const INNER_IDENT: &str = "inner_ident";
        const SELF_TY: &str = "self_ty";
        const INNER_SELF_TY: &str = "inner_self_ty";
        const VALUE_TY: &str = "value_ty";
        const INNER_VALUE_TY: &str = "inner_value_ty";

        macro_rules! err {
            ($span:expr, $($tt:tt)*) => {
                let err = format!($($tt)*);
                return Err(
                    quote_spanned! {
                        $span => compile_error!(#err);
                    }.into()
                );
            };
        }
        
        fn process_stream_with_reflection<'a>(element_ty: &Type, mut input: impl Iterator<Item = TokenTree>, repeatition: &mut [(&'a Swizzle, &'a SwizzleReflection, TokenStream)], close_span: Span) -> Result<(), TokenStream> {
            const SELF_COMPONENT: &str = "self_component";
            const VALUE_COMPONENT: &str = "value_component";
            const LEN: &str = "len";

            const EXPECTATION: &str = formatcp!("either: '#', '{IDENT}', '{INNER_IDENT}', '{SELF_TY}', '{INNER_SELF_TY}', '{VALUE_TY}', '{INNER_VALUE_TY}', '{SELF_COMPONENT}', '{VALUE_COMPONENT}', or '{LEN}'");

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
                                            IDENT => output.extend(once(TokenTree::Ident(swizzle.ident()))),
                                            INNER_IDENT => output.extend(once(TokenTree::Ident(swizzle.inner_ident()))),
                                            SELF_TY => output.extend(swizzle.self_ty.ty(element_ty).into_token_stream()),
                                            INNER_SELF_TY => output.extend(swizzle.self_ty.inner_ty(element_ty).into_token_stream()),
                                            VALUE_TY => output.extend(swizzle.value_ty.ty(element_ty).into_token_stream()),
                                            INNER_VALUE_TY => output.extend(swizzle.value_ty.inner_ty(element_ty).into_token_stream()),
                                            SELF_COMPONENT => output.extend(once(TokenTree::Literal(Literal::usize_suffixed(reflection.self_component)))),
                                            VALUE_COMPONENT => output.extend(once(TokenTree::Literal(Literal::usize_suffixed(reflection.self_component)))),
                                            LEN => output.extend(once(TokenTree::Literal(Literal::usize_suffixed(reflection.len)))),
                                            _ => {
                                                err!(ident.span(), "expected {EXPECTATION}. found '{ident}'.");
                                            }
                                        }
                                    }
                                }
                                TokenTree::Group(group) => {
                                    err!(group.span_open(), "expected {EXPECTATION}. found '{}'.", group_open!(group));
                                }
                                TokenTree::Punct(punct) => {
                                    for (_, _, output) in repeatition.iter_mut() {
                                        if punct.as_char() == '#' {
                                            output.extend(once(token.clone()));
                                        }
                                        else {
                                            err!(punct.span(), "expected {EXPECTATION}. found '{punct}'.");
                                        }
                                    }
                                }
                                TokenTree::Literal(literal) => {
                                    err!(literal.span(), "expected {EXPECTATION}. found '{literal}'.");
                                }
                            }
                        }
                        else {
                            err!(close_span, "unexpected end of repeatition. expected {EXPECTATION}.");
                        }
                    }
                    TokenTree::Group(group) => {
                        let mut group_repeatition = repeatition.iter().map(|(swizzle, reflection, _)| (*swizzle, *reflection, TokenStream::new())).collect::<Box<[(&Swizzle, &SwizzleReflection, TokenStream)]>>();

                        process_stream_with_reflection(element_ty, group.stream().into_iter(), &mut group_repeatition, close_span)?;
                        
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

            Ok(())
        }

        const EXPECTATION: &str = formatcp!("either: '#', '{IDENT}', '{INNER_IDENT}', '{SELF_TY}', '{INNER_SELF_TY}', '{VALUE_TY}', '{INNER_VALUE_TY}', or '('");

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
                                        IDENT => output.extend(once(TokenTree::Ident(swizzle.ident()))),
                                        INNER_IDENT => output.extend(once(TokenTree::Ident(swizzle.inner_ident()))),
                                        SELF_TY => output.extend(swizzle.self_ty.ty(element_ty).into_token_stream()),
                                        INNER_SELF_TY => output.extend(swizzle.self_ty.inner_ty(element_ty).into_token_stream()),
                                        VALUE_TY => output.extend(swizzle.value_ty.ty(element_ty).into_token_stream()),
                                        INNER_VALUE_TY => output.extend(swizzle.value_ty.inner_ty(element_ty).into_token_stream()),
                                        _ => {
                                            err!(ident.span(), "expected {EXPECTATION}. found '{ident}'.");
                                        }
                                    }
                                }
                            }
                            TokenTree::Group(group) => {
                                if group.delimiter() == Delimiter::Parenthesis {
                                    let mut reflection_repeatition = repeatition.iter().map(|(swizzle, _)| swizzle.reflections.iter().map(|reflection| (*swizzle, reflection, TokenStream::new()))).flatten().collect::<Box<[(&Swizzle, &SwizzleReflection, TokenStream)]>>();
                                
                                    process_stream_with_reflection(element_ty, group.stream().into_token_stream().into_iter(), &mut reflection_repeatition, close_span)?;
    
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
                                else {
                                    err!(group.span_open(), "expected {EXPECTATION}. found '{}'.", group_open!(group));
                                }
                            }
                            TokenTree::Punct(punct) => {
                                for (_, output) in repeatition.iter_mut() {
                                    if punct.as_char() == '#' {
                                        output.extend(once(token.clone()));
                                    }
                                    else {
                                        err!(punct.span(), "expected {EXPECTATION}. found '{punct}'.");
                                    }
                                }
                            }
                            TokenTree::Literal(literal) => {
                                err!(literal.span(), "expected {EXPECTATION}. found '{literal}'.");
                            }
                        }
                    }
                    else {
                        err!(close_span, "unexpected end of repeatition. expected {EXPECTATION}.");
                    }
                }
                TokenTree::Group(group) => {
                    let mut group_repeatition = repeatition.iter().map(|(swizzle, _)| (*swizzle, TokenStream::new())).collect::<Box<[(&Swizzle, TokenStream)]>>();

                    process_stream(element_ty, group.stream().into_iter(), &mut group_repeatition, close_span)?;
                    
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

        Ok(())
    }

    
    let input = parse_macro_input!(input as MacroInput);
    
    let mut repeatition = swizzles.map(|swizzle| (swizzle, TokenStream::new())).collect::<Box<[(&Swizzle, TokenStream)]>>();

    match process_stream(&input.element_ty, input.content.into_iter(), &mut repeatition, input.brace.span.close()) {
        Ok(()) => {
            repeatition.iter().fold(TokenStream::new(), |acc, (_, stream)| {
                let new_stream = quote! {
                    #acc
                    #stream
                };
                new_stream
            }).into()
        }
        Err(err) => {
            err.into()
        }
    }
}