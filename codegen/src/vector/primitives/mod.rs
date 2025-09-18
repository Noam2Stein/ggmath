use std::collections::HashMap;

use genco::quote;

use crate::{
    constants::{
        FLOAT_PRIMITIVES, INT_PRIMITIVES, LENGTHS, NUM_PRIMITIVES, PRIMITIVES, SINT_PRIMITIVES,
        UINT_PRIMITIVES,
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

pub fn mod_() -> ModDir {
    let primitive_mods = PRIMITIVES
        .iter()
        .map(|&primitive| primitive_mod(primitive))
        .collect::<Vec<_>>();

    quote! {
        $(for &primitive in PRIMITIVES join($['\r']) => mod $primitive;)
        mod option;
    }
    .to_mod_dir("primitives")
    .with_submod_files(primitive_mods)
    .with_submod_file(option::mod_())
}

fn primitive_mod(primitive: &str) -> ModFile {
    let mut use_crate_items = Vec::new();
    let mut functions = Vec::new();
    let mut len_functions = HashMap::new();
    let mut std_functions = Vec::new();
    let mut std_len_functions = HashMap::new();
    let mut trait_impls = Vec::new();

    let primitive_is_num = NUM_PRIMITIVES.contains(&primitive);
    let primitive_is_int = INT_PRIMITIVES.contains(&primitive);
    let primitive_is_float = FLOAT_PRIMITIVES.contains(&primitive);
    let primitive_is_sint = SINT_PRIMITIVES.contains(&primitive);
    let primitive_is_uint = UINT_PRIMITIVES.contains(&primitive);
    let primitive_is_bool = primitive == "bool";

    primitive::push_fns(
        primitive,
        &mut use_crate_items,
        &mut functions,
        &mut len_functions,
        &mut std_functions,
        &mut std_len_functions,
        &mut trait_impls,
    );

    if primitive_is_num {
        num::push_fns(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if primitive_is_int {
        int::push_fns(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if primitive_is_float {
        float::push_fns(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if primitive_is_sint {
        sint::push_fns(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if primitive_is_uint {
        uint::push_fns(
            primitive,
            &mut use_crate_items,
            &mut functions,
            &mut len_functions,
            &mut std_functions,
            &mut std_len_functions,
            &mut trait_impls,
        );
    }

    if primitive_is_bool {
        bool_::push_fns(
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

            impl<const N: usize, A: VecAlignment> Vector<N, $primitive, A>
            where
                Usize<N>: VecLen,
            {
                $(for function in functions join($['\n']) => $function)
            }
        )

        $(
            for (n, n_functions) in LENGTHS.iter().filter_map(|&n| len_functions.get(&n).map(|functions| (n, functions)))
                join($['\n']) =>

            impl<A: VecAlignment> Vector<$n, $primitive, A> {
                $(for function in n_functions join($['\n']) => $function)
            }
        )

        $(
            if !std_functions.is_empty() =>

            #[cfg(feature = "std")]
            impl<const N: usize, A: VecAlignment> Vector<N, $primitive, A>
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
            impl<A: VecAlignment> Vector<$n, $primitive, A> {
                $(for function in n_functions join($['\n']) => $function)
            }
        )

        $(for trait_impl in trait_impls join($['\n']) => $trait_impl)
    }
    .to_mod_file(primitive)
}
