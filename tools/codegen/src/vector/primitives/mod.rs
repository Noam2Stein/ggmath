use std::collections::HashMap;

use genco::quote;

use crate::{
    constants::{
        FLOAT_PRIMITIVES, INT_PRIMITIVES, LENGTHS, NUM_PRIMITIVES, PRIMARY_PRIMITIVES, PRIMITIVES,
        SINT_PRIMITIVES, UINT_PRIMITIVES,
    },
    module::*,
};

mod bool_;
mod float;
mod int;
mod num;
mod option;
mod primitive;
mod sint;
mod uint;

pub fn src_mod() -> SrcDir {
    let primitive_mods = PRIMITIVES
        .iter()
        .map(|&primitive| primitive_src_mod(primitive))
        .collect::<Vec<_>>();

    quote! {
        $(for &primitive in PRIMITIVES join($['\r']) => mod $primitive;)
        mod option;
    }
    .to_src_dir("primitives")
    .with_submod_files(primitive_mods)
    .with_submod_file(option::src_mod())
}

pub fn test_mods() -> impl Iterator<Item = TestFile> {
    PRIMITIVES
        .iter()
        .map(|&primitive| primitive_test_mod(primitive))
}

fn primitive_src_mod(primitive: &str) -> SrcFile {
    let mut use_crate_items = Vec::new();
    let mut functions = Vec::new();
    let mut len_functions = HashMap::new();
    let mut std_functions = Vec::new();
    let mut std_len_functions = HashMap::new();
    let mut trait_impls = Vec::new();

    primitive::push_src(
        primitive,
        &mut use_crate_items,
        &mut functions,
        &mut len_functions,
        &mut std_functions,
        &mut std_len_functions,
        &mut trait_impls,
    );

    if NUM_PRIMITIVES.contains(&primitive) {
        num::push_src(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if INT_PRIMITIVES.contains(&primitive) {
        int::push_src(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if FLOAT_PRIMITIVES.contains(&primitive) {
        float::push_src(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if SINT_PRIMITIVES.contains(&primitive) {
        sint::push_src(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if UINT_PRIMITIVES.contains(&primitive) {
        uint::push_src(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if primitive == "bool" {
        bool_::push_src(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    quote! {
        use crate::{$(for item in use_crate_items join(, ) => $item)};

        $(
            if !functions.is_empty() =>

            impl<const N: usize, S: Simdness> Vector<N, $primitive, S>
            where
                Usize<N>: VecLen,
            {
                $(for function in functions join($['\n']) => $function)
            }
        )

        $(
            for (n, n_functions) in LENGTHS.iter().filter_map(|&n| len_functions.get(&n).map(|functions| (n, functions)))
                join($['\n']) =>

            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for function in n_functions join($['\n']) => $function)
            }
        )

        $(
            if !std_functions.is_empty() =>

            #[cfg(feature = "std")]
            impl<const N: usize, S: Simdness> Vector<N, $primitive, S>
            where
                Usize<N>: VecLen,
            {
                $(for function in std_functions join($['\n']) => $function)
            }
        )

        $(
            for (n, n_functions) in LENGTHS.iter().filter_map(|&n| std_len_functions.get(&n).map(|functions| (n, functions)))
                join($['\n']) =>

            #[cfg(feature = "std")]
            impl<S: Simdness> Vector<$n, $primitive, S> {
                $(for function in n_functions join($['\n']) => $function)
            }
        )

        $(for trait_impl in trait_impls join($['\n']) => $trait_impl)
    }
    .to_src_file(primitive)
}

fn primitive_test_mod(primitive: &str) -> TestFile {
    let primitive_is_num = NUM_PRIMITIVES.contains(&primitive);
    let primitive_is_int = INT_PRIMITIVES.contains(&primitive);
    let primitive_is_float = FLOAT_PRIMITIVES.contains(&primitive);
    let primitive_is_sint = SINT_PRIMITIVES.contains(&primitive);
    let primitive_is_uint = UINT_PRIMITIVES.contains(&primitive);
    let primitive_is_bool = primitive == "bool";

    quote! {
        $(
            if PRIMARY_PRIMITIVES.contains(&primitive) =>

            use core::mem::size_of;
        )

        use ggmath::*;

        $(
            if FLOAT_PRIMITIVES.contains(&primitive) =>

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
                    if is_simd || PRIMARY_PRIMITIVES.contains(&primitive) =>

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
