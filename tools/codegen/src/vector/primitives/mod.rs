use std::{collections::HashMap};

use genco::{lang::{rust::Tokens}, quote};

use crate::{
    constants::LENGTHS,
    module::{SrcDir, TestFile, TokensExt}, primitives::{Primitive, PrimitiveInt},
};

mod bool_;
mod float;
mod int;
mod option;
mod primitive;
mod sint;
mod uint;

pub fn src_mod() -> SrcDir {
    quote! {
        $(for primitive in Primitive::iter() join($['\r']) => mod $primitive;)
        mod option;
    }
    .to_src_dir("primitives")
    .with_submod_dirs(Primitive::iter().map(primitive_src_mod))
    .with_submod_file(option::src_mod())
}

pub fn test_mods() -> impl Iterator<Item = TestFile> {
    Primitive::iter()
        .map(primitive_test_mod)
}

#[expect(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum SimdBackend {
    X86,
    Arm,
    Wasm,
    Scalar,
}

#[derive(Default)]
struct PrimitiveSrcMod {
    impl_items: Vec<Tokens>,
    std_impl_items: Vec<Tokens>,
    len_impl_items: HashMap<usize, Vec<Tokens>>,
    std_len_impl_items: HashMap<usize, Vec<Tokens>>,
    trait_impls: Vec<Tokens>,
}

fn primitive_src_mod(primitive: Primitive) -> SrcDir {
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
        use crate::{Vector, Simdness, Simd, NonSimd, Usize, VecLen, Scalar, $(for &n in LENGTHS join(, ) => Vec$(n))};

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
            for (n, impl_items) in LENGTHS.iter().filter_map(|&n| output.len_impl_items.get(&n).map(|impl_items| (n, impl_items)))
                join($['\n']) =>

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
            for (n, impl_items) in LENGTHS.iter().filter_map(|&n| output.std_len_impl_items.get(&n).map(|impl_items| (n, impl_items)))
                join($['\n']) =>

            #[cfg(feature = "std")]
            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for impl_item in impl_items join($['\n']) => $impl_item)
            }
        )

        $(for trait_impl in output.trait_impls join($['\n']) => $trait_impl)
    }
    .to_src_dir(primitive.to_string())
}

fn primitive_test_mod(primitive: Primitive) -> TestFile {
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

        $(
            for &n in LENGTHS join($['\n']) =>

            $(
                for is_simd in [true, false] join($['\r']) =>

                $(
                    if is_simd || primitive.is_primary() =>

                    $(let tests = {
                        let mut tests = Vec::new();

                        primitive::push_tests(n, primitive, is_simd, &mut tests);
                        if primitive_is_num {
                            num::push_tests(n, primitive, is_simd, &mut tests);
                        }
                        if primitive_is_int {
                            int::push_tests(n, primitive, is_simd, &mut tests);
                        }
                        if primitive_is_float {
                            float::push_tests(n, primitive, is_simd, &mut tests);
                        }
                        if primitive_is_sint {
                            sint::push_tests(n, primitive, is_simd, &mut tests);
                        }
                        if primitive_is_uint {
                            uint::push_tests(n, primitive, is_simd, &mut tests);
                        }
                        if primitive_is_bool {
                            bool_::push_tests(n, primitive, is_simd, &mut tests);
                        }

                        tests
                    })

                    $(for test in tests join($['\n']) => $test)
                )
            )
        )
    }
    .to_test_file(primitive)
}

