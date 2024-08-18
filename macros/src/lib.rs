mod vec;
mod swizzle;
mod op;

use proc_macro2::TokenStream;

use quote::{format_ident, quote};
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
    $trait:ident, $fn:ident,
    $vec2_fn:ident, $vec3_fn:ident, $vec3a_fn:ident, $vec4_fn:ident,
);
declare_macro_macro!(
    rhs_op_macro,
    $trait:ident, $fn:ident,
    $vec2_fn:ident, $vec3_fn:ident, $vec3a_fn:ident, $vec4_fn:ident,
    $vec2_splat_fn:ident, $vec3_splat_fn:ident, $vec3a_splat_fn:ident, $vec4_splat_fn:ident,
    // scalar 'op' vec currently impossible because of rustc false recursion-detection
    /*
    $splat_vec2_fn:ident, $splat_vec3_fn:ident, $splat_vec3a_fn:ident, $splat_vec4_fn:ident,
    */
    $assign_trait:ident, $assign_fn:ident,
    $vec2_assign_fn:ident, $vec3_assign_fn:ident, $vec3a_assign_fn:ident, $vec4_assign_fn:ident,
    $vec2_splat_assign_fn:ident, $vec3_splat_assign_fn:ident, $vec3a_splat_assign_fn:ident, $vec4_splat_assign_fn:ident,
);
declare_macro_macro!(
    vec_macro,
    $self:ident, $self_lower:ident, $inner:ident,
    $inner_new:ident, $inner_splat:ident,
    $swizzle:ident, $get_swizzle:ident, $mut_swizzle:ident, $set_swizzle:ident, $with_swizzle:ident,
    $display_literal:literal,
    ($($component:ident, $component_index:literal), +),
    ($($op_trait:ident, $op_fn:ident, $inner_op_fn:ident), +),
    ($(
        // scalar 'op' vec currently impossible because of rustc false recursion-detection
        $rhs_op_trait:ident, $rhs_op_fn:ident, $inner_rhs_op_fn:ident, $inner_splat_rhs_op_fn:ident, /*$inner_splat_self_rhs_op_fn:ident,*/
        $assign_op_trait:ident, $assign_op_fn:ident, $inner_assign_op_fn:ident, $inner_splat_assign_op_fn:ident
    ), +)
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
#[proc_macro]
pub fn vecs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    process_vecs(input.into()).into()
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
                    let trait_ = op.trait_();
                    let fn_ = op.fn_();
                    let vec2_fn = op.vec_fn(VecType::Vec2);
                    let vec3_fn = op.vec_fn(VecType::Vec3);
                    let vec3a_fn = op.vec_fn(VecType::Vec3A);
                    let vec4_fn = op.vec_fn(VecType::Vec4);
                
                    process_call(call,
                        quote! {
                            #trait_, #fn_,
                            #vec2_fn, #vec3_fn, #vec3a_fn, #vec4_fn,
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
                    let trait_ = op.trait_();
                    let fn_ = op.fn_();
                    let vec2_fn = op.vec_fn(VecType::Vec2);
                    let vec3_fn = op.vec_fn(VecType::Vec3);
                    let vec3a_fn = op.vec_fn(VecType::Vec3A);
                    let vec4_fn = op.vec_fn(VecType::Vec4);
                    let vec2_splat_fn = op.vec_splat_fn(VecType::Vec2);
                    let vec3_splat_fn = op.vec_splat_fn(VecType::Vec3);
                    let vec3a_splat_fn = op.vec_splat_fn(VecType::Vec3A);
                    let vec4_splat_fn = op.vec_splat_fn(VecType::Vec4);
                    // scalar 'op' vec currently impossible because of rustc false recursion-detection
                    /*
                    let splat_vec2_fn = op.splat_vec_fn(VecType::Vec2);
                    let splat_vec3_fn = op.splat_vec_fn(VecType::Vec3);
                    let splat_vec3a_fn = op.splat_vec_fn(VecType::Vec3A);
                    let splat_vec4_fn = op.splat_vec_fn(VecType::Vec4);
                    */

                    let assign_trait = op.assign_trait();
                    let assign_fn = op.assign_fn();
                    let vec2_assign_fn = op.vec_assign_fn(VecType::Vec2);
                    let vec3_assign_fn = op.vec_assign_fn(VecType::Vec3);
                    let vec3a_assign_fn = op.vec_assign_fn(VecType::Vec3A);
                    let vec4_assign_fn = op.vec_assign_fn(VecType::Vec4);
                    let vec2_splat_assign_fn = op.vec_splat_assign_fn(VecType::Vec2);
                    let vec3_splat_assign_fn = op.vec_splat_assign_fn(VecType::Vec3);
                    let vec3a_splat_assign_fn = op.vec_splat_assign_fn(VecType::Vec3A);
                    let vec4_splat_assign_fn = op.vec_splat_assign_fn(VecType::Vec4);
                    
                    process_call(call,
                        quote! {
                            #trait_, #fn_,
                            #vec2_fn, #vec3_fn, #vec3a_fn, #vec4_fn,
                            #vec2_splat_fn, #vec3_splat_fn, #vec3a_splat_fn, #vec4_splat_fn,
                            // scalar 'op' vec currently impossible because of rustc false recursion-detection
                            /*
                            #splat_vec2_fn, #splat_vec3_fn, #splat_vec3a_fn, #splat_vec4_fn,
                            */
                            #assign_trait, #assign_fn,
                            #vec2_assign_fn, #vec3_assign_fn, #vec3a_assign_fn, #vec4_assign_fn,
                            #vec2_splat_assign_fn, #vec3_splat_assign_fn, #vec3a_splat_assign_fn, #vec4_splat_assign_fn,
                        }
                    )
                }
            ).collect::<TokenStream>()
        }
    ).collect::<TokenStream>().into()
}
fn process_vecs(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ApplyMacroInput);

    input.calls.iter().map(
        |call| {
            VecType::iter().filter_map(
                |vec| {
                    if vec.len() <= 1 {
                        return None;
                    };

                    let self_ = vec.ident();
                    let self_lower = vec.ident_lower();
                    let inner = vec.inner_ident();
                    let inner_new = format_ident!("new_{self_lower}");
                    let inner_splat = format_ident!("splat_{self_lower}");
                    let swizzle = vec.swizzle();
                    let get_swizzle = vec.get_swizzle();
                    let mut_swizzle = vec.mut_swizzle();
                    let set_swizzle = vec.set_swizzle();
                    let with_swizzle = vec.with_swizzle();
                    let display_literal = vec.display_literal();
                    let components = vec.components();
                    let component_indicies = vec.component_indicies();

                    let ops = Op::iter().map(
                        |op| {
                            let trait_ = op.trait_();
                            let fn_ = op.fn_();
                            let vec_fn = op.vec_fn(vec);
                        
                            quote! {
                                #trait_, #fn_, #vec_fn
                            }
                        }
                    );

                    let rhs_ops = RhsOp::iter().map(
                        |op| {
                            let trait_ = op.trait_();
                            let fn_ = op.fn_();
                            let vec_fn = op.vec_fn(vec);
                            let vec_splat_fn = op.vec_splat_fn(vec);
                            
                            // scalar 'op' vec currently impossible because of rustc false recursion-detection
                            /*
                            let splat_vec_fn = op.splat_vec_fn(vec);
                            */
        
                            let assign_trait = op.assign_trait();
                            let assign_fn = op.assign_fn();
                            let vec_assign_fn = op.vec_assign_fn(vec);
                            let vec_splat_assign_fn = op.vec_splat_assign_fn(vec);
                            
                            quote! {
                                #trait_, #fn_, #vec_fn, #vec_splat_fn, /*#splat_vec_fn,*/
                                #assign_trait, #assign_fn, #vec_assign_fn, #vec_splat_assign_fn
                            }
                        }
                    );

                    Some(
                        process_call(call,
                            quote! {
                                #self_, #self_lower, #inner,
                                #inner_new, #inner_splat,
                                #swizzle, #get_swizzle, #mut_swizzle, #set_swizzle, #with_swizzle,
                                #display_literal,
                                (#(#components, #component_indicies), *),
                                (#(#ops), *),
                                (#(#rhs_ops), *)
                            }
                        )
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