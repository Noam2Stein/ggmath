use std::{collections::HashMap};

use genco::{lang::{rust::Tokens}, quote};
use strum::IntoEnumIterator;

use crate::{backend::{SrcDir, TestFile, TokensExt}, iter::{Length, Primitive, PrimitiveInt, Simdness}};

mod bool_;
mod float;
mod int;
mod option;
mod primitive;
mod sint;
mod uint;

pub fn srcmod() -> SrcDir {
    quote! {
        $(for primitive in Primitive::iter() join($['\r']) => mod $primitive;)
        mod option;
    }
    .to_srcdir("primitives")
    .with_submod_dirs(Primitive::iter().map(primitive_srcmod))
    .with_submod_file(option::srcmod())
}

pub fn testmods() -> impl Iterator<Item = TestFile> {
    Primitive::iter()
        .map(primitive_testmod)
}

#[derive(Default)]
struct PrimitiveSrcMod {
    impl_items: Vec<Tokens>,
    std_impl_items: Vec<Tokens>,
    len_impl_items: HashMap<Length, Vec<Tokens>>,
    std_len_impl_items: HashMap<Length, Vec<Tokens>>,
    trait_impls: Vec<Tokens>,
}

#[derive(Default)]
struct PrimitiveTestMod {
    util: Vec<Tokens>,
    tests: Vec<Tokens>,
}

fn primitive_srcmod(primitive: Primitive) -> SrcDir {
    let mut output = PrimitiveSrcMod::default();

    primitive::push_src(primitive, &mut output);

    match primitive {
        Primitive::Float(primitive) => {
            float::push_src(primitive, &mut output);
        }
        Primitive::Int(primitive) => {
            int::push_src(primitive, &mut output);
            
            match primitive {
                PrimitiveInt::Sint(primitive) => {
                    sint::push_src(primitive, &mut output);
                }
                PrimitiveInt::Uint(primitive) => {
                    uint::push_src(primitive, &mut output);
                }
            }
        }
        Primitive::Bool => {
            bool_::push_src(&mut output);
        }
    }

    quote! {
        use crate::{Vector, Simdness, Simd, NonSimd, Usize, VecLen, Scalar, $(for n in Length::iter() join(, ) => Vec$(n))};

        $(
            if !output.impl_items.is_empty() =>

            impl<const N: usize, S: Simdness> Vector<N, $primitive, S>
            where
                Usize<N>: VecLen,
            {
                $(for impl_item in output.impl_items join($['\n']) => $impl_item)
            }
        )

        $(
            for (n, impl_items) in Length::iter().filter_map(|n| output.len_impl_items.get(&n).map(|impl_items| (n, impl_items))) join($['\n']) =>

            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for impl_item in impl_items join($['\n']) => $impl_item)
            }
        )

        $(
            if !output.std_impl_items.is_empty() =>

            #[cfg(feature = "std")]
            impl<const N: usize, S: Simdness> Vector<N, $primitive, S>
            where
                Usize<N>: VecLen,
            {
                $(for impl_item in output.std_impl_items join($['\n']) => $impl_item)
            }
        )

        $(
            for (n, impl_items) in Length::iter().filter_map(|n| output.std_len_impl_items.get(&n).map(|impl_items| (n, impl_items))) join($['\n']) =>

            #[cfg(feature = "std")]
            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for impl_item in impl_items join($['\n']) => $impl_item)
            }
        )

        $(for trait_impl in output.trait_impls join($['\n']) => $trait_impl)
    }
    .to_srcdir(primitive.to_string())
}

fn primitive_testmod(primitive: Primitive) -> TestFile {
    let mut output = PrimitiveTestMod::default();

    primitive::push_tests(primitive, &mut output);
    match primitive {
        Primitive::Float(primitive) => {
            float::push_tests(primitive, &mut output);
        }
        Primitive::Int(primitive) => {
            int::push_tests(primitive, &mut output);
            
            match primitive {
                PrimitiveInt::Sint(primitive) => {
                    sint::push_tests(primitive, &mut output);
                }
                PrimitiveInt::Uint(primitive) => {
                    uint::push_tests(primitive, &mut output);
                }
            }
        }
        Primitive::Bool => {
            bool_::push_tests(&mut output);
        }
    }

    quote! {
        $(
            if primitive.is_float() =>

            use core::mem::size_of;
        )

        use ggmath::*;

        $(
            if primitive.is_float() =>

            fn approx_eq(a: $primitive, b: $primitive) -> bool {
                if a.is_nan() && b.is_nan() {
                    true
                } else if a.is_infinite() && b.is_infinite() {
                    a.is_sign_positive() == b.is_sign_positive()
                } else {
                    (a - b).abs() < 0.1
                }
            }

            fn approx_vec_eq<const N: usize, S: Simdness>(a: Vector<N, $primitive, S>, b: Vector<N, $primitive, S>) -> bool
            where
                Usize<N>: VecLen,
            {
                (0..N).all(|i| approx_eq(a.index(i), b.index(i)))
            }

            macro_rules! assert_approx_eq {
                ($$a:expr, $$b:expr $$(,)?) => {
                    let a = $$a;
                    let b = $$b;
                    
                    if !approx_eq(a, b) {
                        panic!("approx_eq failed: {a:?} != {b:?}");
                    }
                };
            }

            macro_rules! assert_approx_vec_eq {
                ($$a:expr, $$b:expr $$(,)?) => {
                    let a = $$a;
                    let b = $$b;
                    
                    if !approx_vec_eq(a, b) {
                        panic!("approx_vec_eq failed: {a:?} != {b:?}");
                    }
                };
            }
        )

        $(for item in output.util join($['\n']) => $item)
                    
        $(for item in output.tests join($['\n']) => $item)
    }
    .to_testfile(primitive.as_str())
}


impl PrimitiveTestMod {
    fn push_tests_for_vector(&mut self, primitive: impl Into<Primitive>, mut f: impl FnMut(Length, Simdness) -> Tokens) {
        let primitive = primitive.into();

        for length in Length::iter() {
            for simdness in Simdness::iter() {
                if !primitive.is_primary() && !(length == Length::Four && simdness == Simdness::Simd) {
                    continue;
                }

                self.tests.push(f(length, simdness))
            }
        }
    }
}