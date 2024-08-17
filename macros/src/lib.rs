mod vec;
mod swizzle;

use proc_macro2::TokenStream;

use quote::quote;
use syn::{parenthesized, parse::{Parse, ParseStream}, parse_macro_input, Error, Ident, Token, Type};
use swizzle::*;

struct DeclareMacroInput {
    ident: Ident,
    output: TokenStream,
}
impl Parse for DeclareMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;
        
        let output;
        let _ = syn::braced!(output in input);
        let output = output.parse::<TokenStream>()?;

        if !input.is_empty() {
            return Err(input.error("unexpected token."));
        }

        Ok(
            Self {
                ident: ident,
                output,
            }
        )
    }
}
struct MacroCalls {
    idents: Vec<Ident>,
}
impl Parse for MacroCalls {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut idents = Vec::new();
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            let _ = input.parse::<Token![!]>()?;
            
            let params;
            parenthesized!(params in input);
            if !params.is_empty() {
                return Err(Error::new(params.span(), "swizzle macros can't have parameters."));
            }

            let _ = input.parse::<Token![;]>()?;

            idents.push(ident);
        }

        Ok(
            Self {
                idents,
            }
        )
    }
}
struct ApplyMacroInput {
    element_ty: Type,
    calls: MacroCalls,
}
impl Parse for ApplyMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let element_ty = input.parse()?;
        
        let calls;
        let _ = syn::braced!(calls in input);
        let calls = calls.parse::<MacroCalls>()?;

        if !input.is_empty() {
            return Err(input.error("unexpected token."));
        }

        Ok(
            Self {
                element_ty,
                calls,
            }
        )
    }
}

#[proc_macro]
pub fn swizzle_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeclareMacroInput);
    
    let ident = input.ident;
    let output = input.output;

    quote! {
        macro_rules! #ident {
            (
                $ident:ident, $inner_ident:ident,
                $self_ty:ty, $inner_self_ty:ty, $self_ident:ident,
                $value_ty:ty, $inner_value_ty:ty, $value_ident:ident,
                $($self_component:literal, $value_component:literal, $len:literal), +) => {
                #output
            }
        }
    }.into()
}
#[proc_macro]
pub fn mut_swizzle_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeclareMacroInput);
    
    let ident = input.ident;
    let output = input.output;

    quote! {
        macro_rules! #ident {
            (
                $ident:ident, $inner_ident:ident,
                $self_ty:ty, $inner_self_ty:ty, $self_ident:ident,
                $($self_component:literal, $value_ty:ty, $inner_value_ty:ty, $value_ident:ident), +) => {
                #output
            }
        }
    }.into()
}

macro_rules! apply_swizzle_macro {
    ($ident:ident, $iter:expr) => {
        #[proc_macro]
        pub fn $ident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            process_swizzle(input.into(), $iter).into()
        }
    };
}
macro_rules! apply_mut_swizzle_macro {
    ($ident:ident, $iter:expr) => {
        #[proc_macro]
        pub fn $ident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            process_mut_swizzle(input.into(), $iter).into()
        }
    };
}
apply_swizzle_macro!(vec2_get_swizzle, SWIZZLES.vec2.get.iter());
apply_swizzle_macro!(vec3_get_swizzle, SWIZZLES.vec3.get.iter());
apply_swizzle_macro!(vec3a_get_swizzle, SWIZZLES.vec3a.get.iter());
apply_swizzle_macro!(vec4_get_swizzle, SWIZZLES.vec4.get.iter());
apply_mut_swizzle_macro!(vec2_mut_swizzle, SWIZZLES.vec2.mut_.iter());
apply_mut_swizzle_macro!(vec3_mut_swizzle, SWIZZLES.vec3.mut_.iter());
apply_mut_swizzle_macro!(vec3a_mut_swizzle, SWIZZLES.vec3a.mut_.iter());
apply_mut_swizzle_macro!(vec4_mut_swizzle, SWIZZLES.vec4.mut_.iter());
apply_swizzle_macro!(vec2_set_swizzle, SWIZZLES.vec2.set.iter());
apply_swizzle_macro!(vec3_set_swizzle, SWIZZLES.vec3.set.iter());
apply_swizzle_macro!(vec3a_set_swizzle, SWIZZLES.vec3a.set.iter());
apply_swizzle_macro!(vec4_set_swizzle, SWIZZLES.vec4.set.iter());
apply_swizzle_macro!(vec2_with_swizzle, SWIZZLES.vec2.with.iter());
apply_swizzle_macro!(vec3_with_swizzle, SWIZZLES.vec3.with.iter());
apply_swizzle_macro!(vec3a_with_swizzle, SWIZZLES.vec3a.with.iter());
apply_swizzle_macro!(vec4_with_swizzle, SWIZZLES.vec4.with.iter());
apply_swizzle_macro!(vec2_swizzle, SWIZZLES.vec2.iter());
apply_swizzle_macro!(vec3_swizzle, SWIZZLES.vec3.iter());
apply_swizzle_macro!(vec3a_swizzle, SWIZZLES.vec3a.iter());
apply_swizzle_macro!(vec4_swizzle, SWIZZLES.vec4.iter());
apply_swizzle_macro!(get_swizzle, SWIZZLES.get_iter());
apply_mut_swizzle_macro!(mut_swizzle, SWIZZLES.mut_iter());
apply_swizzle_macro!(set_swizzle, SWIZZLES.set_iter());
apply_swizzle_macro!(with_swizzle, SWIZZLES.with_iter());
apply_swizzle_macro!(swizzle, SWIZZLES.iter());

fn process_swizzle<'a>(input: proc_macro::TokenStream, swizzles: impl Iterator<Item = &'a Swizzle> + Clone) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplyMacroInput);

    input.calls.idents.iter().map(
        |macro_ident| {
            swizzles.clone().map(
                |swizzle| {
                    let ident = swizzle.ident();
                    let inner_ident = swizzle.inner_ident();
                    let self_ty = swizzle.self_ty.ty(&input.element_ty);
                    let inner_self_ty = swizzle.self_ty.inner_ty(&input.element_ty);
                    let self_ident = swizzle.self_ty.ident();
                    let value_ty = swizzle.value_ty.ty(&input.element_ty);
                    let inner_value_ty = swizzle.value_ty.inner_ty(&input.element_ty);
                    let value_ident = swizzle.value_ty.ident();
        
                    let reflections = swizzle.reflections.iter().map(
                        |SwizzleReflection { self_component, value_component, len }| {
                            quote! {
                                #self_component, #value_component, #len
                            }
                        }
                    );
                
                    quote! {
                        #macro_ident!(
                            #ident, #inner_ident,
                            #self_ty, #inner_self_ty, #self_ident,
                            #value_ty, #inner_value_ty, #value_ident,
                            #(#reflections), *
                        );
                    }        
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}
fn process_mut_swizzle<'a>(input: proc_macro::TokenStream, swizzles: impl Iterator<Item = &'a MutSwizzle> + Clone) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplyMacroInput);

    input.calls.idents.iter().map(
        |macro_ident| {
            swizzles.clone().map(
                |swizzle| {
                    let ident = swizzle.ident();
                    let inner_ident = swizzle.inner_ident();
                    let self_ty = swizzle.self_ty.ty(&input.element_ty);
                    let inner_self_ty = swizzle.self_ty.inner_ty(&input.element_ty);
                    let self_ident = swizzle.self_ty.ident();
        
                    let reflections = swizzle.reflections.iter().map(
                        |MutSwizzleReflection { self_component, value_ty }| {
                            let inner_value_ty = value_ty.inner_ty(&input.element_ty);
                            let value_ident = value_ty.ident();
                            let value_ty = value_ty.ty(&input.element_ty);

                            quote! {
                                #self_component, #value_ty, #inner_value_ty, #value_ident
                            }
                        }
                    );
                
                    quote! {
                        #macro_ident!(
                            #ident, #inner_ident,
                            #self_ty, #inner_self_ty, #self_ident,
                            #(#reflections), *
                        );
                    }        
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}