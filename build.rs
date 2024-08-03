use std::{env, fs::{create_dir_all, remove_file, write}, ops::Range, path::PathBuf};

use lazy_static::lazy_static;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote, ToTokens};
use syn::{parse2, parse_file, parse_str, token::{FatArrow, Semi}, Expr, Ident, Type};

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
    src: TokenStream,
    dst: TokenStream,
    len: usize,
}
impl ToTokens for CopyInstruction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.src.to_tokens(tokens);
        FatArrow::default().to_tokens(tokens);
        self.dst.to_tokens(tokens);
        if self.len > 1 {
            Semi::default().to_tokens(tokens);
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
    let _component_indicies = vec_type.component_indicies().map(|i| Literal::usize_unsuffixed(i)).collect::<Box<[Literal]>>();
    let _components = vec_type.components().map(|c| format_ident!("{c}")).collect::<Box<[Ident]>>();
    let _a_len = vec_type.a_len();

    let _component_indicies_but_first = &_component_indicies[1.._len];
    let _components_but_first = &_components[1.._len];
    let _component_indicies_but_last = &_component_indicies[0.._len - 1];
    let _components_but_last = &_components[0.._len - 1];
    let _component_indicies_but_firstlast = &_component_indicies[1.._len - 1];
    let _components_but_firstlast = &_components[1.._len - 1];
    let _first_component = &_components[0];
    let _last_component = _components.last().unwrap();
    let _last_component_index = Literal::usize_unsuffixed(_len - 1);
    
    let a_field = if _is_aligned {
        quote! {
            pub(crate) _alignment: [T; #_a_len],
        }
    }
    else {
        quote! {
            
        }
    };
    
    let with_fn_idents = vec_type.components().map(|c| format_ident!("with_{c}")).collect::<Box<[Ident]>>();

    let tuple = parse_str::<Type>(&format!("({})", vec_type.components().map(|_| "T").collect::<Box<[&str]>>().join(", "))).unwrap();
    
    let fmt_literal = format!("({})", _components.iter().map(|_| "{}").collect::<Box<[&str]>>().join(", "));

    let tuple_casts = {
        let mut tuple_casts = Vec::new();

        let mut field_lens = Vec::new();
        push_fields(&_ident, _len, &mut tuple_casts, &mut field_lens);

        fn push_fields(ident: &Ident, len: usize, output: &mut Vec<TokenStream>, field_lens: &mut Vec<usize>) {
            for field in 1..VECS.end {
                field_lens.push(field);

                let sum = field_lens.iter().sum::<usize>();
                if sum == len && field_lens.len() > 1 {
                    let fields = field_lens.iter().map(|field_len|
                        match field_len {
                            1 => parse_str::<Type>("T").unwrap(),
                            _ => parse_str::<Type>(&format!("Vec{field_len}<T>")).unwrap(),
                        }
                    );

                    output.push(
                        quote! {
                            cast!(#ident<T>, (#(#fields), *), T: Element);
                        }
                    )
                }
                else if sum < len {
                    push_fields(ident, len, output, field_lens);   
                }

                field_lens.pop();
            }
        }

        tuple_casts
    };

    let (min_element_expr, max_element_expr) = {
        let mut str = String::new();
        for c in _components_but_last.iter() {
            str += &format!("self.{c}.fn(");
        };
        str += &format!("self.{}", _last_component);
        str += &")".repeat(_len - 1);

        (
            parse_str::<Expr>(&str.replace("fn", "min")).unwrap(),
            parse_str::<Expr>(&str.replace("fn", "max")).unwrap(),
        )
    };

    let component_const_idents = vec_type.components().map(|c| format_ident!("{}", c.to_uppercase()));

    let op_traits = OPS.map(|op| format_ident!("{op}"));
    let op_fns = OPS.map(|op| format_ident!("{}", op.to_lowercase()));
    let rhs_op_traits = RHS_OPS.map(|op| format_ident!("{op}"));
    let rhs_op_fns = RHS_OPS.map(|op| format_ident!("{}", op.to_lowercase()));
    let assign_op_traits = RHS_OPS.map(|op| format_ident!("{op}Assign"));
    let assign_op_fns = RHS_OPS.map(|op| format_ident!("{}_assign", op.to_lowercase()));

    let (swizzle, swizzle_mut, set_swizzle, with_swizzle) = {
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

        let swizzle_mut = VEC_TYPES.iter().filter_map(|output_type| {
            if output_type.is_aligned || output_type.len > _len {
                return None;
            }

            let output_type_ident = format_ident!("{}", output_type.name());

            let fns = (0.._len - output_type.len + 1).map(|component_index| {
                let fn_ident = format_ident!("{}_mut", COMPONENTS[component_index..component_index + output_type.len].iter().collect::<String>());
                let component = format_ident!("{}", COMPONENTS[component_index]);

                quote! {
                    (#fn_ident, #component)
                }
            }).collect::<Box<[TokenStream]>>();

            Some(
                quote! {
                    swizzle_mut! {
                        #output_type_ident<T>,
                        [
                            #(
                                #fns,
                            )*
                        ]
                    }
                }
            )
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

        (swizzle, swizzle_mut, set_swizzle, with_swizzle)
    };

    let _len = Literal::usize_unsuffixed(_len);

    let quote = quote! {
        use std::{cmp::Ordering, fmt, ops::*, slice::{Iter, IterMut, SliceIndex}};
        use crate::*;

        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct #_ident<T: Element> {
            #(
                pub #_components: T,
            )*
            #a_field
        }

        #[inline(always)]
        pub fn #_fn_ident<T: Element, V: Into<#_ident<T>>>(value: V) -> #_ident<T> {
            value.into()
        }

        impl<T: Element> #_ident<T> {
            #[inline(always)]
            pub const fn new(#(#_components: T), *) -> Self {
                let mut output = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
                #(
                    output.#_components = #_components;
                )*
                output
            }
            #[inline(always)]
            pub const fn splat(value: T) -> Self {
                let mut output = unsafe { std::mem::MaybeUninit::<Self>::uninit().assume_init() };
                #(
                    output.#_components = value;
                )*
                output
            }

            #(
                #[inline(always)]
                pub const fn #with_fn_idents(mut self, #_components: T) -> Self {
                    self.#_components = #_components;
                    self
                }
            )*

            #[inline(always)]
            pub fn map<B: Element, F: FnMut(T) -> B>(self, mut f: F) -> #_ident<B> {
                #_ident::new(#(f(self.#_components)), *)
            }
            #[inline(always)]
            pub fn count<F: FnMut(T) -> bool>(self, mut f: F) -> u8 {
                #(f(self.#_components) as u8) + *
            }
            #[inline(always)]
            pub fn any<F: FnMut(T) -> bool>(self, mut f: F) -> bool {
                #(f(self.#_components)) || *
            }
            #[inline(always)]
            pub fn all<F: FnMut(T) -> bool>(self, mut f: F) -> bool {
                #(f(self.#_components)) && *
            }
            #[inline(always)]
            pub fn none<F: FnMut(T) -> bool>(self, f: F) -> bool {
                !self.any(f)
            }
            #[inline(always)]
            pub fn find<F: FnMut(T) -> bool>(self, mut f: F) -> Option<T> {
                if f(self.#_first_component) {
                    Some(
                        self.#_first_component
                    )
                }
                #(
                    else if f(self.#_components_but_first) {
                        Some(
                            self.#_components_but_first
                        )
                    }
                )*
                else {
                    None
                }
            }            #[inline(always)]
            pub fn position<F: FnMut(T) -> bool>(self, mut f: F) -> Option<u8> {
                if f(self.#_first_component) {
                    Some(
                        0
                    )
                }
                #(
                    else if f(self.#_components_but_first) {
                        Some(
                            #_component_indicies_but_first
                        )
                    }
                )*
                else {
                    None
                }
            }

            #[inline(always)]
            pub fn from_array(value: [T; #_len]) -> Self {
                unsafe {
                    *(&value as *const [T; #_len] as *const Self)
                }
            }
            #[inline(always)]
            pub const fn from_slice(value: &[T; #_len]) -> &Self {
                unsafe {
                    &*(value as *const [T; #_len] as *const Self)
                }
            }
            #[inline(always)]
            pub const fn from_slice_mut(value: &mut [T; #_len]) -> &mut Self {
                unsafe {
                    &mut *(value as *mut [T; #_len] as *mut Self)
                }
            }
            #[inline(always)]
            pub fn into_array(self) -> [T; #_len] {
                unsafe {
                    *(&self as *const Self as *const [T; #_len])
                }
            }
            #[inline(always)]
            pub const fn as_slice(&self) -> &[T; #_len] {
                unsafe {
                    &*(self as *const Self as *const [T; #_len])
                }
            }
            #[inline(always)]
            pub const fn as_slice_mut(&mut self) -> &mut [T; #_len] {
                unsafe {
                    &mut *(self as *mut Self as *mut [T; #_len])
                }
            }
            #[inline(always)]
            pub fn get<I>(&self, index: I) -> Option<&<I as SliceIndex<[T]>>::Output>
            where
            I: SliceIndex<[T]>
            {
                self.as_slice().get(index)
            }
            #[inline(always)]
            pub fn get_mut<I>(&mut self, index: I) -> Option<&mut <I as SliceIndex<[T]>>::Output>
            where
            I: SliceIndex<[T]>
            {
                self.as_slice_mut().get_mut(index)
            }
            #[inline(always)]
            pub fn iter(&self) -> Iter<T> {
                self.as_slice().iter()
            }
            #[inline(always)]
            pub fn iter_mut(&mut self) -> IterMut<T> {
                self.as_slice_mut().iter_mut()
            }

            #[inline(always)]
            pub const fn from_tuple(value: &#tuple) -> &Self {
                unsafe {
                    &*(value as *const #tuple as *const Self)
                }
            }
            #[inline(always)]
            pub const fn from_tuple_mut(value: &mut #tuple) -> &mut Self {
                unsafe {
                    &mut *(value as *mut #tuple as *mut Self)
                }
            }
            #[inline(always)]
            pub const fn as_tuple(&self) -> &#tuple {
                unsafe {
                    &*(self as *const Self as *const #tuple)
                }
            }
            #[inline(always)]
            pub const fn as_tuple_mut(&mut self) -> &mut #tuple {
                unsafe {
                    &mut *(self as *mut Self as *mut #tuple)
                }
            }
        }
        impl<T: Element, I: SliceIndex<[T]>> Index<I> for #_ident<T> {
            type Output = <I as SliceIndex<[T]>>::Output;
            #[inline(always)]
            fn index(&self, index: I) -> &Self::Output {
                &self.as_slice()[index]
            }
        }
        impl<T: Element, I: SliceIndex<[T]>> IndexMut<I> for #_ident<T> {
            #[inline(always)]
            fn index_mut(&mut self, index: I) -> &mut Self::Output {
                &mut self.as_slice_mut()[index]
            }
        }
        impl<T: Element> fmt::Display for #_ident<T> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, #fmt_literal, #(self.#_components), *)
            }
        }
        impl<T: Element> IntoIterator for #_ident<T> {
            type Item = T;
            type IntoIter = std::array::IntoIter<Self::Item, #_len>;

            fn into_iter(self) -> Self::IntoIter {
                self.into_array().into_iter()
            }
        }
        impl<T: Element + Eq> Eq for #_ident<T> {

        }
        impl<T: Element + Ord> #_ident<T> {
            #[inline(always)]
            pub fn min_element(self) -> T {
                #min_element_expr
            }
            #[inline(always)]
            pub fn max_element(self) -> T {
                #max_element_expr
            }
            #[inline(always)]
            pub fn min(self, other: Self) -> Self {
                Self::new(#(self.#_components.min(other.#_components)), *)
            }
            #[inline(always)]
            pub fn max(self, other: Self) -> Self {
                Self::new(#(self.#_components.max(other.#_components)), *)
            }
        }
        impl<T: Element + Ord> Ord for #_ident<T> {
            fn cmp(&self, other: &Self) -> Ordering {
                self.partial_cmp(other).unwrap()
            }
        }
        impl<T: Element + BitAnd<T, Output = T>> #_ident<T> {
            #[inline(always)]
            pub fn bit_all(self) -> T {
                #(self.#_components) & *
            }
        }
        impl<T: Element + BitOr<T, Output = T>> #_ident<T> {
            #[inline(always)]
            pub fn bit_any(self) -> T {
                #(self.#_components) | *
            }
        }
        impl<T: Element + BitOr<T, Output = T> + Not<Output = T>> #_ident<T> {
            #[inline(always)]
            pub fn bit_none(self) -> T {
                !self.bit_any()
            }
        }
        impl<T: Num> #_ident<T> {
            pub const ZERO: Self = Self::splat(T::ZERO);
            pub const ONE: Self = Self::splat(T::ONE);
            #(
                pub const #component_const_idents: Self = Self::ZERO.#with_fn_idents(T::ONE);
            )*

            #[inline(always)]
            pub fn sum(self) -> T {
                #(self.#_components) + *
            }
            #[inline(always)]
            pub fn filter_sum<F: FnMut(T) -> bool>(self, mut f: F) -> T {
                let mut output = T::ZERO;

                #(
                    if f(self.#_components) {
                        output += self.#_components
                    }
                )*

                output
            }
        }
        impl #_ident<bool> {
            pub fn bit_count(self) -> u8 {
                #(self.#_components as u8) + *
            }
            pub fn bit_position(self) -> u8 {
                if self.#_first_component {
                    0
                }
                #(
                    else if self.#_components_but_firstlast {
                        #_component_indicies_but_firstlast
                    }
                )*
                else {
                    #_last_component_index
                }
            }
        }

        cast!(#_ident<T>, [T; #_len], T: Element);
        #(
            #tuple_casts
        )*
        ops! {
            #_ident { #(#_components), * }, [#((#op_traits, #op_fns)), *]
        }
        rhs_ops! {
            #_ident { #(#_components), * }, [#((#rhs_op_traits, #rhs_op_fns)), *]
        }
        assign_ops! {
            #_ident { #(#_components), * }, [#((#assign_op_traits, #assign_op_fns)), *]
        }
        impl<T: Element> #_ident<T> {
            #(
                #swizzle
            )*
            #(
                #swizzle_mut
            )*
            #(
                #set_swizzle
            )*
            #(
                #with_swizzle
            )*
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