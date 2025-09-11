use indoc::formatdoc;

use crate::{COMPONENTS, module::*};

pub fn write_mod(module: ModFile) {
    let mut mods = Vec::new();

    for (axis_idx, &axis) in ["x", "y", "z"].iter().enumerate() {
        let (dir_a_lower, dir_a_upper, dir_a_camel) = match axis {
            "x" => ("right", "RIGHT", "Right"),
            "y" => ("up", "UP", "Up"),
            "z" => ("forwards", "FORWARD", "Forward"),
            _ => unreachable!(),
        };
        let (dir_b_lower, dir_b_upper, dir_b_camel) = match axis {
            "x" => ("left", "LEFT", "Left"),
            "y" => ("down", "DOWN", "Down"),
            "z" => ("backwards", "BACKWARD", "Backward"),
            _ => unreachable!(),
        };

        let mut vector_impls_a = Vec::new();
        let mut vector_impls_b = Vec::new();

        for n in 2.max(axis_idx + 1)..=4 {
            let axis_uppercase = COMPONENTS[axis_idx].to_uppercase();

            vector_impls_a.push(formatdoc! {r#"
                impl<T: ScalarZero + ScalarOne, A: VecAlignment> Positive{dir_a_camel} for Vector<{n}, T, A> {{
                    const {dir_a_upper}: Self = Self::{axis_uppercase};
                }}

                impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Negative{dir_b_camel} for Vector<{n}, T, A> {{
                    const {dir_b_upper}: Self = Self::NEG_{axis_uppercase};
                }}
            "#});

            vector_impls_b.push(formatdoc! {r#"
                impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Negative{dir_a_camel} for Vector<{n}, T, A> {{
                    const {dir_a_upper}: Self = Self::NEG_{axis_uppercase};
                }}

                impl<T: ScalarZero + ScalarOne, A: VecAlignment> Positive{dir_b_camel} for Vector<{n}, T, A> {{
                    const {dir_b_upper}: Self = Self::{axis_uppercase};
                }}
            "#});
        }

        let vector_impls_a = vector_impls_a.join("\n").replace("\n", "\n\t");
        let vector_impls_b = vector_impls_b.join("\n").replace("\n", "\n\t");

        mods.push(formatdoc! {
            r#"
            /// A module for traits with `{dir_a_upper}` and `{dir_b_upper}` constants,
            /// where {dir_a_lower} is the positive direction.
            #[cfg(feature = "{dir_a_lower}")]
            pub mod {dir_a_lower} {{
                use crate::{{
                    Construct,
                    ScalarZero,
                    ScalarOne,
                    ScalarNegOne,
                    VecAlignment,
                    Vector,
                }};

                /// A trait for a `{dir_a_upper}` constant where {dir_a_lower} is the positive direction.
                pub trait Positive{dir_a_camel}: Construct {{
                    /// A value that points {dir_a_lower} with magnitude `1` where {dir_a_lower} is the positive direction.
                    const {dir_a_upper}: Self;
                }}

                /// A trait for a `{dir_b_upper}` constant where {dir_a_lower} is the positive direction.
                pub trait Negative{dir_b_camel}: Construct {{
                    /// A value that points {dir_b_lower} with magnitude `1` where {dir_a_lower} is the positive direction.
                    const {dir_b_upper}: Self;
                }}

                impl<T: ScalarOne> Positive{dir_a_camel} for T {{
                    const {dir_a_upper}: Self = Self::ONE;
                }}
                
                impl<T: ScalarNegOne> Negative{dir_b_camel} for T {{
                    const {dir_b_upper}: Self = Self::NEG_ONE;
                }}

                {vector_impls_a}
            }}

            /// A module for vectors with `{dir_a_upper}` and `{dir_b_upper}` constants,
            /// where {dir_b_lower} is the positive direction.
            #[cfg(feature = "{dir_b_lower}")]
            pub mod {dir_b_lower} {{
                use crate::{{
                    Construct,
                    ScalarZero,
                    ScalarOne,
                    ScalarNegOne,
                    VecAlignment,
                    Vector,
                }};

                /// A trait for a `{dir_a_upper}` constant where {dir_b_lower} is the positive direction.
                pub trait Negative{dir_a_camel}: Construct {{
                    /// A value that points {dir_a_lower} with magnitude `1` where {dir_b_lower} is the positive direction.
                    const {dir_a_upper}: Self;
                }}

                /// A trait for a `{dir_b_upper}` constant where {dir_b_lower} is the positive direction.
                pub trait Positive{dir_b_camel}: Construct {{
                    /// A value that points {dir_b_lower} with magnitude `1` where {dir_b_lower} is the positive direction.
                    const {dir_b_upper}: Self;
                }}

                impl<T: ScalarOne> Positive{dir_b_camel} for T {{
                    const {dir_b_upper}: Self = Self::ONE;
                }}

                impl<T: ScalarNegOne> Negative{dir_a_camel} for T {{
                    const {dir_a_upper}: Self = Self::NEG_ONE;
                }}

                {vector_impls_b}
            }}
        "#});
    }

    let mods = mods.join("\n");

    module.finish(formatdoc! {r#"
        {mods}
    "#});
}
