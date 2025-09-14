use indoc::formatdoc;

use crate::{
    constants::{PRIMITIVE_PREFIXES, PRIMITIVES},
    r#gen::ModDir,
};

pub fn module() -> ModDir {
    let mods = PRIMITIVES
        .iter()
        .zip(PRIMITIVE_PREFIXES)
        .map(|(primitive, prefix)| {
            formatdoc! {r#"
                /// A module with `{primitive}` type aliases.
                #[cfg(feature = "primitive_aliases")]
                pub mod {primitive}_aliases {{
                    #[cfg(feature = "vector")]
                    use crate::vector_aliases;
                    #[cfg(feature = "vector")]
                    vector_aliases!(pub {prefix} => {primitive});
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    ModDir::new(
        "primitive_aliases",
        formatdoc! {r#"
            {mods}
        "#},
        vec![],
        vec![],
        vec![],
    )
}
