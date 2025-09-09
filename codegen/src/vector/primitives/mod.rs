use indoc::formatdoc;

use crate::module::*;

mod bool_;
mod float;
mod int;
mod num;
mod primitive;
mod signed;
mod sint;
mod uint;

pub fn write_mod(module: ModDir) {
    let mut primitive_mods = Vec::new();

    for primitive in [
        "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
        "u128", "usize", "bool",
    ] {
        write_primitive_mod(module.submod(primitive), primitive);

        primitive_mods.push(formatdoc! {r#"
            mod {primitive};
        "#});
    }

    let primitive_mods = primitive_mods.join("");

    module.finish(formatdoc! {r#"
        {primitive_mods}
    "#});
}

fn write_primitive_mod(module: Mod, primitive: &str) {
    let mut functions = Vec::new();
    let mut const_functions = Vec::new();
    let mut std_functions = Vec::new();
    let mut std_const_functions = Vec::new();
    let mut trait_impls = Vec::new();
    let mut test_functions = Vec::new();

    let primitive_is_num = primitive == "f32"
        || primitive == "f64"
        || primitive == "i8"
        || primitive == "i16"
        || primitive == "i32"
        || primitive == "i64"
        || primitive == "i128"
        || primitive == "isize"
        || primitive == "u8"
        || primitive == "u16"
        || primitive == "u32"
        || primitive == "u64"
        || primitive == "u128"
        || primitive == "usize";

    let primitive_is_int = primitive == "i8"
        || primitive == "i16"
        || primitive == "i32"
        || primitive == "i64"
        || primitive == "i128"
        || primitive == "isize"
        || primitive == "u8"
        || primitive == "u16"
        || primitive == "u32"
        || primitive == "u64"
        || primitive == "u128"
        || primitive == "usize";

    let primitive_is_float = primitive == "f32" || primitive == "f64";

    let primitive_is_sint = primitive == "i8"
        || primitive == "i16"
        || primitive == "i32"
        || primitive == "i64"
        || primitive == "i128"
        || primitive == "isize";

    let primitive_is_uint = primitive == "u8"
        || primitive == "u16"
        || primitive == "u32"
        || primitive == "u64"
        || primitive == "u128"
        || primitive == "usize";

    let primitive_is_signed = primitive_is_float || primitive_is_sint;

    primitive::push_fns(
        primitive,
        &mut functions,
        &mut const_functions,
        &mut std_functions,
        &mut std_const_functions,
        &mut test_functions,
    );

    if primitive_is_num {
        num::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    }

    if primitive_is_int {
        int::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    }

    if primitive_is_signed {
        signed::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    }

    if primitive_is_float {
        float::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    } else if primitive_is_sint {
        sint::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    } else if primitive_is_uint {
        uint::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    } else if primitive == "bool" {
        bool_::push_fns(
            primitive,
            &mut functions,
            &mut const_functions,
            &mut std_functions,
            &mut std_const_functions,
            &mut test_functions,
        );
    }

    if primitive_is_float || primitive_is_sint || primitive_is_uint {
        trait_impls.push(formatdoc! {r#"
            impl ScalarZero for {primitive} {{
                const ZERO: {primitive} = 0 as Self;
            }}

            impl ScalarOne for {primitive} {{
                const ONE: {primitive} = 1 as Self;
            }}
        "#});
    }

    if primitive_is_float || primitive_is_sint {
        trait_impls.push(formatdoc! {r#"
            impl ScalarNegOne for {primitive} {{
                const NEG_ONE: {primitive} = -1 as Self;
            }}
        "#});
    }

    let functions = functions.join("\n").replace("\n", "\n\t");
    let const_functions = const_functions.join("\n").replace("\n", "\n\t");
    let std_functions = std_functions.join("\n").replace("\n", "\n\t");
    let std_const_functions = std_const_functions.join("\n").replace("\n", "\n\t");
    let trait_impls = trait_impls.join("\n");
    let test_functions = test_functions.join("\n").replace("\n", "\n\t");

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
        (!const_functions.is_empty()).then(|| {
            formatdoc! {r#"
            impl<const N: usize, A: VecAlignment> Vector<N, {primitive}, A>
            where
                Usize<N>: VecLen,
            {{
                {const_functions}
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
        (!std_const_functions.is_empty()).then(|| {
            formatdoc! {r#"
            impl<const N: usize, A: VecAlignment> Vector<N, {primitive}, A>
            where
                Usize<N>: VecLen,
            {{
                {std_const_functions}
            }}
        "#}
        }),
    ]
    .into_iter()
    .flatten()
    .collect::<Vec<_>>()
    .join("\n");

    module.finish(formatdoc! {r#"
        use crate::{{vector::*, Usize}};

        {function_impls}

        {trait_impls}

        #[cfg(test)]
        mod tests {{
            use crate::*;

            {test_functions}
        }}
    "#});
}
