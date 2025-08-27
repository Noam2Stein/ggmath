use indoc::formatdoc;

use super::*;

const FLOATS: &[&str] = &["f32", "f64"];
const SINTS: &[&str] = &["i8", "i16", "i32", "i64", "i128", "isize"];
const UINTS: &[&str] = &["u8", "u16", "u32", "u64", "u128", "usize"];
const PRIMITIVES: &[&str] = &[
    "bool", "f32", "f64", "i8", "i16", "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64",
    "u128", "usize",
];

pub fn write_mod(mut module: ModDir) {
    let primitive_mods = PRIMITIVES
        .iter()
        .map(|primitive| {
            formatdoc! {r#"
                mod {primitive};
                pub use {primitive}::*;
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    writedoc!(
        module,
        r#"
        use super::*;

        {primitive_mods}
        "#
    )
    .unwrap();

    for primitive in PRIMITIVES {
        let mut module = module.submod(primitive);

        let prefix = match *primitive {
            "bool" => "B",
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
            _ => panic!("unhandled primitive prefix: {primitive}"),
        };

        let garbage = if FLOATS.contains(&primitive) {
            "0.0"
        } else if SINTS.contains(&primitive) {
            "0"
        } else if UINTS.contains(&primitive) {
            "0"
        } else if *primitive == "bool" {
            "false"
        } else {
            panic!("unhandled primitive garbage: {primitive}");
        };

        writedoc!(
            module,
            r#"
            use super::*;

            #[cfg(feature = "primitive_aliases")]
            pub mod {primitive}_aliases {{
                use super::*;

                vector_aliases!(pub {prefix} => {primitive});
            }}

            #[cfg(feature = "primitive_aliases")]
            pub use {primitive}_aliases::*;

            impl Scalar for {primitive} {{
                type InnerVec2A = [Self; 2];
                type InnerVec3A = [Self; 3];
                type InnerVec4A = [Self; 4];

                const INNER_VEC2A_GARBAGE: Self::InnerVec2A = [{garbage}; 2];
                const INNER_VEC3A_GARBAGE: Self::InnerVec3A = [{garbage}; 3];
                const INNER_VEC4A_GARBAGE: Self::InnerVec4A = [{garbage}; 4];
            }}
            "#
        )
        .unwrap();
    }
}
