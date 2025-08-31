use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::module::*;

mod bool_;
mod float;
mod sint;
mod uint;

pub fn write_mod(mut module: ModDir) {
    let mut primitive_mods = Vec::new();

    for primitive in [
        "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
        "u128", "usize", "bool",
    ] {
        write_primitive_mod(module.submod(primitive), primitive);

        primitive_mods.push(formatdoc! {r#"
            mod {primitive};
            #[allow(unused_imports)]
            pub use {primitive}::*;
        "#});
    }

    let primitive_mods = primitive_mods.join("\n");

    writedoc!(
        module,
        r#"
        {primitive_mods}
    "#
    )
    .unwrap();
}

fn write_primitive_mod(mut module: Mod, primitive: &str) {
    let prefix = match primitive {
        "f32" => "F",
        "f64" => "D",
        "i8" => "I8",
        "i16" => "I16",
        "i32" => "I",
        "i64" => "I64",
        "i128" => "I128",
        "isize" => "Isize",
        "u8" => "U8",
        "u16" => "U16",
        "u32" => "U",
        "u64" => "U64",
        "u128" => "U128",
        "usize" => "Usize",
        "bool" => "B",
        _ => panic!("unhandled primitive in prefix: {primitive}"),
    };

    let mut functions = Vec::new();
    let mut trait_impls = Vec::new();

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

    let primitive_is_bool = primitive == "bool";

    if primitive_is_float {
        float::push_fns(primitive, &mut functions);
    } else if primitive_is_sint {
        sint::push_fns(primitive, &mut functions);
    } else if primitive_is_uint {
        uint::push_fns(primitive, &mut functions);
    } else if primitive_is_bool {
        bool_::push_fns(primitive, &mut functions);
    } else {
        panic!("unhandled primitive in push_fns: {primitive}");
    }

    if primitive_is_float || primitive_is_sint || primitive_is_uint {
        trait_impls.push(formatdoc! {r#"
            impl crate::vector::ScalarZero for {primitive} {{
                const ZERO: {primitive} = 0 as Self;
            }}

            impl crate::vector::ScalarOne for {primitive} {{
                const ONE: {primitive} = 1 as Self;
            }}
        "#});
    }

    if primitive_is_float || primitive_is_sint {
        trait_impls.push(formatdoc! {r#"
            impl crate::vector::ScalarNegOne for {primitive} {{
                const NEG_ONE: {primitive} = -1 as Self;
            }}
        "#});
    }

    let functions = functions.join("\n").replace("\n", "\n\t");
    let trait_impls = trait_impls.join("\n");

    writedoc!(
        module,
        r#"
        use crate::{{vector::{{VecAlignment, Vector, VecLen}}, Usize}};

        /// A module with `{primitive}` type aliases.
        #[cfg(feature = "primitive_aliases")]
        pub mod {primitive}_aliases {{
            use crate::vector_aliases;

            vector_aliases!(pub {prefix} => {primitive});
        }}

        impl<const N: usize, A: VecAlignment> Vector<N, {primitive}, A>
        where
            Usize<N>: VecLen,
        {{
            {functions}
        }}
        
        {trait_impls}
    "#
    )
    .unwrap();
}
