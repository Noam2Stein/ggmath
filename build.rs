use std::{env, fs::{create_dir_all, remove_file, write}, ops::Range, path::PathBuf};

use lazy_static::lazy_static;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse2, parse_file, parse_str, token::{RArrow, Star}, Ident};

const VECS: Range<usize> = 2..5;
const COMPONENTS: [char; VECS.end - 1] = ['x', 'y', 'z', 'w'];
const OPS: [&str; 2] = ["Neg", "Not"];
const RHS_OPS: [&str; 10] = ["Add", "Sub", "Mul", "Div", "Rem", "BitAnd", "BitOr", "BitXor", "Shl", "Shr"];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct VecType {
    len: usize,
    is_aligned: bool,
}
impl VecType {
    fn name(self) -> String {
        if self.is_aligned {
            format!("Vec{}A", self.len)
        }
        else {
            format!("Vec{}", self.len)
        }
    }
    fn component_indicies(self) -> impl Iterator<Item = usize> {
        0..self.len
    }
    fn components(self) -> impl Iterator<Item = String> {
        self.component_indicies().map(|i| COMPONENTS[i].to_string())
    }
    fn a_len(self) -> usize {
        if self.is_aligned {
            self.len.next_power_of_two() - self.len
        }
        else {
            0
        }
    }
}

#[derive(Clone)]
struct CopyInstruction {
    src: Ident,
    dst: Ident,
    len: usize,
}
impl ToTokens for CopyInstruction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.src.to_tokens(tokens);
        RArrow::default().to_tokens(tokens);
        self.dst.to_tokens(tokens);
        if self.len > 1 {
            Star::default().to_tokens(tokens);
            Literal::usize_unsuffixed(self.len).to_tokens(tokens);
        };
    }
}

lazy_static!(
    static ref VEC_TYPES: Box<[VecType]> = {
        let mut result = Vec::with_capacity(VECS.len() * 2);
        for len in VECS
        {
            result.push(
                VecType {
                    len,
                    is_aligned: false,
                }   
            );
            if !len.is_power_of_two() {
                result.push(
                    VecType {
                        len,
                        is_aligned: true,
                    }   
                );
            };
        };
        result.into_boxed_slice()
    };
);

fn main() {
    let src_dir = PathBuf::from_iter([
        PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()),
        PathBuf::from("src/gen"),
    ].iter());

    if src_dir.exists() {
        for entry in std::fs::read_dir(&src_dir).unwrap() {
            remove_file(entry.unwrap().path()).unwrap();
        };
    }
    else {
        create_dir_all(src_dir.clone()).unwrap();
    }

    let write = |path_in_src: &str, str: &str| {
        let path = PathBuf::from_iter([&src_dir, &PathBuf::from(path_in_src)].iter());
        let dir = path.parent().unwrap();
        if !dir.exists() {
            create_dir_all(dir).unwrap();
        }
        write(path, str.as_bytes()).unwrap();
    };

    for vec_type in VEC_TYPES.iter() {
        write(&format!("{}.rs", vec_type.name().to_lowercase()), &vec_rs(*vec_type));
    };
    
    write("mod.rs", &mod_rs());
}

fn mod_rs() -> String {
    let vec_names = VEC_TYPES.iter().map(|vec_type|
        format_ident!("{}", vec_type.name().to_lowercase())
    ).collect::<Box<[Ident]>>();

    prettyplease::unparse(
        &parse_file(
            &quote! {
                #(
                    mod #vec_names;
                )*
                #(
                    pub use #vec_names::*;
                )*
            }
            .to_string()
        ).unwrap()
    )
}
fn vec_rs(vec_type: VecType) -> String {
    let _len = vec_type.len;
    let _is_aligned = vec_type.is_aligned;
    let _ident = format_ident!("{}", vec_type.name());
    let _fn_ident = format_ident!("{}", vec_type.name().to_lowercase());
    let _component_indicies = vec_type.component_indicies().collect::<Box<[usize]>>();
    let _components = vec_type.components().map(|c| format_ident!("{c}")).collect::<Box<[Ident]>>();
    let _a_len = vec_type.a_len();

    let a_field = if _is_aligned {
        quote! {
            pub(crate) _alignment: [T; #_a_len],
        }
    }
    else {
        quote! {

        }
    };

    let with_fn_idents = vec_type.components().map(|c| format_ident!("with_{c}"));
    
    let fmt_literal = format!("({})", _components.iter().map(|_| "{}").collect::<Box<[&str]>>().join(", "));

    let op_quotes = OPS.map(|op_str| {
        let op = format_ident!("{op_str}");
        let op_fn = format_ident!("{}", op_str.to_lowercase());
        quote! {
            impl<T> #op for #_ident<T>
            where
                T: Element + #op<Output: Element>,
            {
                type Output = #_ident<T::Output>;
                #[inline(always)]
                fn #op_fn(self) -> <Self as #op>::Output {
                    Self::Output::new(#(self.#_components.#op_fn()), *)
                }
            }
        }
    });
    let rhs_op_quotes = RHS_OPS.map(|op_str| {
        let op = format_ident!("{op_str}");
        let op_fn = format_ident!("{}", op_str.to_lowercase());
        quote! {
            impl<RhsElement, T> #op<#_ident<RhsElement>> for #_ident<T>
            where
                RhsElement: Element,
                T: Element + #op<RhsElement, Output: Element>,
            {
                type Output = #_ident<T::Output>;
                #[inline(always)]
                fn #op_fn(self, rhs: #_ident<RhsElement>) -> <Self as #op<#_ident<RhsElement>>>::Output {
                    Self::Output::new(#(self.#_components.#op_fn(rhs.#_components)), *)
                }
            }
        }
    });
    let assign_op_quotes = RHS_OPS.map(|op_str| {
        let op = format_ident!("{op_str}Assign");
        let op_fn = format_ident!("{}_assign", op_str.to_lowercase());
        quote! {
            impl<RhsElement, T> #op<#_ident<RhsElement>> for #_ident<T>
            where
                RhsElement: Element,
                T: Element + #op<RhsElement>,
            {
                #[inline(always)]
                fn #op_fn(&mut self, rhs: #_ident<RhsElement>) {
                    #(
                        self.#_components.#op_fn(rhs.#_components);
                    )*
                }
            }
        }
    });

    let swizzle = {
        let mut combination_str = String::new();
        let mut copy_instructions = Vec::<CopyInstruction>::new();

        let swizzle = VEC_TYPES.iter().map(|output_type| {
            let output_type_ident = format_ident!("{}", output_type.name());

            let fns = (0.._len.pow(output_type.len as u32)).map(|combination| {
                combination_str.clear();
                copy_instructions.clear();

                let mut previous_self_component = 0;
                for output_component in 0..output_type.len {
                    let self_component = combination / _len.pow(output_component as u32) % _len;
    
                    combination_str.push(COMPONENTS[self_component]);
                    if output_component > 0 && self_component == previous_self_component + 1 {
                        copy_instructions.last_mut().unwrap().len += 1;
                    }
                    else {
                        copy_instructions.push(
                            CopyInstruction {
                                src: parse_str(&COMPONENTS[self_component].to_string()).unwrap(),
                                dst: parse_str(&COMPONENTS[output_component].to_string()).unwrap(),
                                len: 1,
                            }
                        );
                    };

                    previous_self_component = self_component;
                }

                let fn_ident = if output_type.is_aligned {
                    format_ident!("{combination_str}_a")
                }
                else {
                    format_ident!("{combination_str}")
                };
    
                quote! {
                    (#fn_ident, #(#copy_instructions), *)
                }
            }).collect::<Box<[TokenStream]>>();

            quote! {
                swizzle! {
                    #output_type_ident<T>,
                    [
                        #(
                            #fns,
                        )*
                    ]
                }
            }
        }).collect::<Box<[TokenStream]>>();

        let set_swizzle = VEC_TYPES.iter().filter_map(|value_type| {
            if value_type.len > _len {
                return None;
            }

            let value_type_ident = format_ident!("{}", value_type.name());

            let fns = (0.._len.pow(value_type.len as u32)).filter_map(|combination| {
                combination_str.clear();
                copy_instructions.clear();

                let mut previous_self_component = 0;
                for value_component in 0..value_type.len {
                    let self_component = combination / _len.pow(value_component as u32) % _len;
                    if combination_str.contains(COMPONENTS[self_component]) {
                        return None;
                    };
    
                    combination_str.push(COMPONENTS[self_component]);
                    if value_component > 0 && self_component == previous_self_component + 1 {
                        copy_instructions.last_mut().unwrap().len += 1;
                    }
                    else {
                        copy_instructions.push(
                            CopyInstruction {
                                src: parse_str(&COMPONENTS[value_component].to_string()).unwrap(),
                                dst: parse_str(&COMPONENTS[self_component].to_string()).unwrap(),
                                len: 1,
                            }
                        );
                    };

                    previous_self_component = self_component;
                }

                let fn_ident = if value_type.is_aligned {
                    format_ident!("set_{combination_str}_a")
                }
                else {
                    format_ident!("set_{combination_str}")
                };
    
                Some(
                    quote! {
                        (#fn_ident, #(#copy_instructions), *)
                    }
                )
            });

            Some(
                quote! {
                    set_swizzle! {
                        #value_type_ident<T>,
                        [
                            #(
                                #fns,
                            )*
                        ]
                    }
                }
            )
        }).collect::<Box<[TokenStream]>>();

        let with_swizzle = VEC_TYPES.iter().filter_map(|value_type| {
            if value_type.len > _len {
                return None;
            }

            let value_type_ident = format_ident!("{}", value_type.name());

            let fns = (0.._len.pow(value_type.len as u32)).filter_map(|combination| {
                combination_str.clear();
                copy_instructions.clear();

                let mut previous_self_component = 0;
                for value_component in 0..value_type.len {
                    let self_component = combination / _len.pow(value_component as u32) % _len;
                    if combination_str.contains(COMPONENTS[self_component]) {
                        return None;
                    };
    
                    combination_str.push(COMPONENTS[self_component]);
                    if value_component > 0 && self_component == previous_self_component + 1 {
                        copy_instructions.last_mut().unwrap().len += 1;
                    }
                    else {
                        copy_instructions.push(
                            CopyInstruction {
                                src: parse_str(&COMPONENTS[value_component].to_string()).unwrap(),
                                dst: parse_str(&COMPONENTS[self_component].to_string()).unwrap(),
                                len: 1,
                            }
                        );
                    };

                    previous_self_component = self_component;
                }

                let fn_ident = if value_type.is_aligned {
                    format_ident!("with_{combination_str}_a")
                }
                else {
                    format_ident!("with_{combination_str}")
                };
    
                Some(
                    quote! {
                        (#fn_ident, #(#copy_instructions), *)
                    }
                )
            });

            Some(
                quote! {
                    with_swizzle! {
                        #value_type_ident<T>,
                        [
                            #(
                                #fns,
                            )*
                        ]
                    }
                }
            )
        }).collect::<Box<[TokenStream]>>();

        quote! {
            #(
                #swizzle
            )*
            #(
                #set_swizzle
            )*
            #(
                #with_swizzle
            )*
        }
    };

    let quote = quote! {
        use std::{fmt, ops::*};
        use crate::*;

        #[derive(Debug, Clone, Copy)]
        pub struct #_ident<T: Element> {
            #(
                pub #_components: T,
            )*
            #a_field
        }

        #[inline(always)]
        pub fn #_fn_ident<T: Element>(#(#_components: T), *) -> #_ident<T> {
            #_ident::new(#(#_components), *)
        }
        impl<T: Element> #_ident<T> {
            #[inline(always)]
            pub fn new(#(#_components: T), *) -> Self {
                let mut output = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
                #(
                    output.#_components = #_components;
                )*
                output
            }
            #[inline(always)]
            pub fn splat(value: T) -> Self {
                let mut output = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
                #(
                    output.#_components = value;
                )*
                output
            }

            #(
                #[inline(always)]
                pub fn #with_fn_idents(mut self, #_components: T) -> Self {
                    self.#_components = #_components;
                    self
                }
            )*
        }

        impl<T: Element> fmt::Display for #_ident<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, #fmt_literal, #(self.#_components), *)
            }
        }

        #(
            #op_quotes
        )*
        #(
            #rhs_op_quotes
        )*
        #(
            #assign_op_quotes
        )*

        impl<T: Element> #_ident<T> {
            #swizzle
        }
    };

    match &parse2(quote.clone()) {
        Ok(file) => {
            prettyplease::unparse(file)
        },
        Err(err) => {
            eprintln!("{quote}");
            eprintln!();
            panic!("{err}");
        }
    }
}