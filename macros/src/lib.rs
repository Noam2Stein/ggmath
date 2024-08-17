mod vec;
mod swizzle;
mod op;

use proc_macro2::TokenStream;

use quote::quote;
use strum::IntoEnumIterator;
use syn::{parenthesized, parse::{Parse, ParseStream}, parse_macro_input, token, Ident, Token, Type};

use vec::*;
use op::*;
use swizzle::*;

struct DeclareMacroInput {
    ident: Ident,
    params: TokenStream,
    output: TokenStream,
}
impl Parse for DeclareMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = input.parse()?;

        let params = if input.peek(token::Paren) {
            let params;
            let _ = syn::parenthesized!(params in input);
            params.parse::<TokenStream>().unwrap_or_default()
        }
        else {
            TokenStream::new()
        };
        
        let output;
        let _ = syn::braced!(output in input);
        let output = output.parse::<TokenStream>()?;

        if !input.is_empty() {
            return Err(input.error("unexpected token."));
        }

        Ok(
            Self {
                ident,
                params,
                output,
            }
        )
    }
}
struct MacroCall {
    ident: Ident,
    params: TokenStream,
}
impl Parse for MacroCall {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident: Ident = input.parse()?;
        let _ = input.parse::<Token![!]>()?;
        
        let params;
        parenthesized!(params in input);
        let params = params.parse::<TokenStream>()?;

        let _ = input.parse::<Token![;]>()?;

        Ok(
            MacroCall {
                ident,
                params,
            }
        )
    }
}

struct ApplyMacroInput {
    calls: Vec<MacroCall>,
}
impl Parse for ApplyMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut calls = Vec::new();
        while !input.is_empty() {
            calls.push(input.parse::<MacroCall>()?);
        }

        Ok(
            Self {
                calls,
            }
        )
    }
}

struct ApplySwizzleMacroInput {
    element_ty: Type,
    calls: Vec<MacroCall>,
}
impl Parse for ApplySwizzleMacroInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let element_ty = input.parse()?;
        
        let call_buffer;
        let _ = syn::braced!(call_buffer in input);
        let mut calls = Vec::new();
        while !call_buffer.is_empty() {
            calls.push(call_buffer.parse::<MacroCall>()?);
        }

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

macro_rules! declare_macro_macro {
    ($ident:ident, $($tt:tt)*) => {
        #[proc_macro]
        pub fn $ident(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
            let DeclareMacroInput { ident, params, output } = parse_macro_input!(input as DeclareMacroInput);
        
            quote! {
                macro_rules! #ident {
                    ((#params), $($tt)*) => {
                        #output
                    }
                }
            }.into()
        }
    };
}
declare_macro_macro!(
    swizzle_macro,
    $ident:ident, $inner_ident:ident,
    $self_ty:ty, $inner_self_ty:ty, $self_ident:ident,
    $value_ty:ty, $inner_value_ty:ty, $value_ident:ident,
    $($self_component:literal, $value_component:literal, $len:literal), +
);
declare_macro_macro!(
    mut_swizzle_macro,
    $ident:ident, $inner_ident:ident,
    $self_ty:ty, $inner_self_ty:ty, $self_ident:ident,
    $($self_component:literal, $value_ty:ty, $inner_value_ty:ty, $value_ident:ident), +
);
declare_macro_macro!(
    op_macro,
    $trait:ident, $fn:ident, $element_trait:ident,
    $vec2_fn:ident, $vec3_fn:ident, $vec3a_fn:ident, $vec4_fn:ident,
);
declare_macro_macro!(
    rhs_op_macro,
    $trait:ident, $fn:ident, $element_trait:ident,
    $vec2_fn:ident, $vec3_fn:ident, $vec3a_fn:ident, $vec4_fn:ident,
    $assign_trait:ident, $assign_fn:ident, $assign_element_trait:ident,
    $assign_vec2_fn:ident, $assign_vec3_fn:ident, $assign_vec3a_fn:ident, $assign_vec4_fn:ident,
);

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

#[proc_macro]
pub fn ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    process_ops(input.into()).into()
}
#[proc_macro]
pub fn rhs_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    process_rhs_ops(input.into()).into()
}

fn process_swizzle<'a>(input: proc_macro::TokenStream, swizzles: impl Iterator<Item = &'a Swizzle> + Clone) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplySwizzleMacroInput);

    input.calls.iter().map(
        |call| {
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
                
                    process_call(call,
                        quote! {
                            #ident, #inner_ident,
                            #self_ty, #inner_self_ty, #self_ident,
                            #value_ty, #inner_value_ty, #value_ident,
                            #(#reflections), *
                        }
                    )
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}
fn process_mut_swizzle<'a>(input: proc_macro::TokenStream, swizzles: impl Iterator<Item = &'a MutSwizzle> + Clone) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplySwizzleMacroInput);

    input.calls.iter().map(
        |call| {
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
                
                    process_call(call,
                        quote! {
                            #ident, #inner_ident,
                            #self_ty, #inner_self_ty, #self_ident,
                            #(#reflections), *
                        }
                    )
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}
fn process_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplyMacroInput);

    input.calls.iter().map(
        |call| {
            Op::iter().map(
                |op| {
                    let trait_ident = op.trait_ident();
                    let fn_ident = op.fn_ident();
                    let element_trait_ident = op.element_trait_ident();
                    let vec2_fn_ident = op.element_fn_ident(VecType::Vec2);
                    let vec3_fn_ident = op.element_fn_ident(VecType::Vec3);
                    let vec3a_fn_ident = op.element_fn_ident(VecType::Vec3A);
                    let vec4_fn_ident = op.element_fn_ident(VecType::Vec4);
                
                    process_call(call,
                        quote! {
                            #trait_ident, #fn_ident, #element_trait_ident,
                            #vec2_fn_ident, #vec3_fn_ident, #vec3a_fn_ident, #vec4_fn_ident,
                        }
                    )
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}
fn process_rhs_ops(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplyMacroInput);

    input.calls.iter().map(
        |call| {
            RhsOp::iter().map(
                |op| {
                    let trait_ident = op.trait_ident();
                    let fn_ident = op.fn_ident();
                    let element_trait_ident = op.element_trait_ident();
                    let vec2_fn_ident = op.element_fn_ident(VecType::Vec2);
                    let vec3_fn_ident = op.element_fn_ident(VecType::Vec3);
                    let vec3a_fn_ident = op.element_fn_ident(VecType::Vec3A);
                    let vec4_fn_ident = op.element_fn_ident(VecType::Vec4);
                    let assign_trait_ident = op.assign_trait_ident();
                    let assign_fn_ident = op.assign_fn_ident();
                    let assign_element_trait_ident = op.assign_element_trait_ident();
                    let assign_vec2_fn_ident = op.assign_element_fn_ident(VecType::Vec2);
                    let assign_vec3_fn_ident = op.assign_element_fn_ident(VecType::Vec3);
                    let assign_vec3a_fn_ident = op.assign_element_fn_ident(VecType::Vec3A);
                    let assign_vec4_fn_ident = op.assign_element_fn_ident(VecType::Vec4);
                    
                    process_call(call,
                        quote! {
                            #trait_ident, #fn_ident, #element_trait_ident,
                            #vec2_fn_ident, #vec3_fn_ident, #vec3a_fn_ident, #vec4_fn_ident,
                            #assign_trait_ident, #assign_fn_ident, #assign_element_trait_ident,
                            #assign_vec2_fn_ident, #assign_vec3_fn_ident, #assign_vec3a_fn_ident, #assign_vec4_fn_ident,
                        }
                    )
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}
fn process_call(call: &MacroCall, input: TokenStream) -> TokenStream {
    let MacroCall { ident, params } = call;
    quote! {
        #ident!(
            (#params),
            #input
        );
    }
}