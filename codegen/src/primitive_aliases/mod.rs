use indoc::formatdoc;

use crate::{PRIMITIVES, gen_mod::GenModDir};

pub fn write_mod(module: GenModDir) {
    let mut mods = Vec::new();

    for &primitive in PRIMITIVES {
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

        mods.push(formatdoc! {r#"
            /// A module with `{primitive}` type aliases.
            #[cfg(feature = "primitive_aliases")]
            pub mod {primitive}_aliases {{
                #[cfg(feature = "vector")]
                use crate::vector_aliases;
                #[cfg(feature = "vector")]
                vector_aliases!(pub {prefix} => {primitive});
            }}
        "#});
    }

    let mods = mods.join("\n");

    module.finish(formatdoc! {r#"
        {mods}
    "#});
}
