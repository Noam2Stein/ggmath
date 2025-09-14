use genco::quote;

use crate::{
    constants::{
        FLOAT_PRIMITIVES, INT_PRIMITIVES, NUM_PRIMITIVES, PRIMITIVES, SINT_PRIMITIVES,
        UINT_PRIMITIVES,
    },
    module::*,
};

mod bool_;
mod float;
mod int;
mod num;
mod primitive;
mod sint;
mod uint;

pub fn mod_() -> ModDir {
    let primitive_mods = PRIMITIVES
        .iter()
        .map(|&primitive| primitive_mod(primitive))
        .collect::<Vec<_>>();

    quote! {
        $(
            for &primitive in PRIMITIVES =>

            mod $primitive;
        )
    }
    .to_mod_dir("primitives")
    .with_submod_files(primitive_mods)
}

fn primitive_mod(primitive: &str) -> ModFile {
    let mut functions = Vec::new();
    let mut std_functions = Vec::new();
    let mut trait_impls = Vec::new();
    let mut use_crate_items = Vec::new();

    let primitive_is_num = NUM_PRIMITIVES.contains(&primitive);
    let primitive_is_int = INT_PRIMITIVES.contains(&primitive);
    let primitive_is_float = FLOAT_PRIMITIVES.contains(&primitive);
    let primitive_is_sint = SINT_PRIMITIVES.contains(&primitive);
    let primitive_is_uint = UINT_PRIMITIVES.contains(&primitive);

    primitive::push_fns(
        primitive,
        &mut functions,
        &mut std_functions,
        &mut trait_impls,
        &mut use_crate_items,
    );

    if primitive_is_num {
        num::push_fns(
            primitive,
            &mut functions,
            &mut std_functions,
            &mut trait_impls,
            &mut use_crate_items,
        );
    }

    if primitive_is_int {
        int::push_fns(
            primitive,
            &mut functions,
            &mut std_functions,
            &mut trait_impls,
            &mut use_crate_items,
        );
    }

    if primitive_is_float {
        float::push_fns(
            primitive,
            &mut functions,
            &mut std_functions,
            &mut trait_impls,
            &mut use_crate_items,
        );
    }

    if primitive_is_sint {
        sint::push_fns(
            primitive,
            &mut functions,
            &mut std_functions,
            &mut trait_impls,
            &mut use_crate_items,
        );
    }

    if primitive_is_uint {
        uint::push_fns(
            primitive,
            &mut functions,
            &mut std_functions,
            &mut trait_impls,
            &mut use_crate_items,
        );
    }

    if primitive == "bool" {
        bool_::push_fns(
            primitive,
            &mut functions,
            &mut std_functions,
            &mut trait_impls,
            &mut use_crate_items,
        );
    }

    let functions = functions.join("\n").replace("\n", "\n\t");
    let std_functions = std_functions.join("\n").replace("\n", "\n\t");
    let trait_impls = trait_impls.join("\n");
    let use_crate_items = use_crate_items.join(", ");

    let function_impls = [
        (!functions.is_empty()).then(|| {
            formatdoc! {r#"
            impl<const N: usize, A: VecAlignment> Vector<N, {primitive}, A>
            where
                Usize<N>: VecLen,
            {{
                {functions}
            }}
        "#}
        }),
        (!std_functions.is_empty()).then(|| {
            formatdoc! {r#"
            impl<const N: usize, A: VecAlignment> Vector<N, {primitive}, A>
            where
                Usize<N>: VecLen,
            {{
                {std_functions}
            }}
        "#}
        }),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join("\n");

    ModFile::new(
        primitive,
        formatdoc! {r#"
            use crate::{{{use_crate_items}}};

            {function_impls}

            {trait_impls}
        "#},
    )
}
