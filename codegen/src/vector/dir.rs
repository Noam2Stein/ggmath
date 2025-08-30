use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::module::*;

pub fn write_mod(mut module: Mod) {
    let mut traits = Vec::new();
    let mut impls = Vec::new();

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

        traits.push(formatdoc! {r#"
            /// A trait for a `{dir_a_camel}` constant where {dir_a_lower} is the positive direction.
            #[cfg(feature = "{dir_a_lower}")]
            pub trait Positive{dir_a_camel}: Construct {{
                /// A value that points {dir_a_lower} with magnitude `1` where {dir_a_lower} is the positive direction.
                const {dir_a_upper}: Self;
            }}

            /// A trait for a `{dir_b_camel}` constant where {dir_b_lower} is the positive direction.
            #[cfg(feature = "{dir_b_lower}")]
            pub trait Positive{dir_b_camel}: Construct {{
                /// A value that points {dir_b_lower} with magnitude `1` where {dir_b_lower} is the positive direction.
                const {dir_b_upper}: Self;
            }}

            /// A trait for a `{dir_a_camel}` constant where {dir_a_lower} is the negative direction.
            #[cfg(feature = "{dir_b_lower}")]
            pub trait Negative{dir_a_camel}: Construct {{
                /// A value that points {dir_a_lower} with magnitude `1` where {dir_a_lower} is the negative direction.
                const {dir_a_upper}: Self;
            }}
            
            /// A trait for a `{dir_b_camel}` constant where {dir_b_lower} is the negative direction.
            #[cfg(feature = "{dir_a_lower}")]
            pub trait Negative{dir_b_camel}: Construct {{
                /// A value that points {dir_b_lower} with magnitude `1` where {dir_b_lower} is the negative direction.
                const {dir_b_upper}: Self;
            }}
        "#});

        impls.push(formatdoc! {
            r#"
            #[cfg(feature = "{dir_a_lower}")]
            impl<T: ScalarOne> Positive{dir_a_camel} for T {{
                const {dir_a_upper}: Self = Self::ONE;
            }}

            #[cfg(feature = "{dir_b_lower}")]
            impl<T: ScalarOne> Positive{dir_b_camel} for T {{
                const {dir_b_upper}: Self = Self::ONE;
            }}

            #[cfg(feature = "{dir_b_lower}")]
            impl<T: ScalarNegOne> Negative{dir_a_camel} for T {{
                const {dir_a_upper}: Self = Self::NEG_ONE;
            }}

            #[cfg(feature = "{dir_a_lower}")]
            impl<T: ScalarNegOne> Negative{dir_b_camel} for T {{
                const {dir_b_upper}: Self = Self::NEG_ONE;
            }}
        "#});

        for n in 2..=4 {
            let positive_values = (0..n)
                .map(|i| if i == axis_idx { "T::ONE" } else { "T::ZERO" })
                .collect::<Vec<_>>()
                .join(", ");

            let negative_values = (0..n)
                .map(|i| {
                    if i == axis_idx {
                        "T::NEG_ONE"
                    } else {
                        "T::ZERO"
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

            impls.push(formatdoc! {
                r#"
                #[cfg(feature = "{dir_a_lower}")]
                impl<T: ScalarZero + ScalarOne, A: VecAlignment> Positive{dir_a_camel} for Vector<{n}, T, A> {{
                    const {dir_a_upper}: Self = vec{n}g!({positive_values});
                }}

                #[cfg(feature = "{dir_b_lower}")]
                impl<T: ScalarZero + ScalarOne, A: VecAlignment> Positive{dir_b_camel} for Vector<{n}, T, A> {{
                    const {dir_b_upper}: Self = vec{n}g!({positive_values});
                }}

                #[cfg(feature = "{dir_b_lower}")]
                impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Negative{dir_a_camel} for Vector<{n}, T, A> {{
                    const {dir_a_upper}: Self = vec{n}g!({negative_values});
                }}

                #[cfg(feature = "{dir_a_lower}")]
                impl<T: ScalarZero + ScalarNegOne, A: VecAlignment> Negative{dir_b_camel} for Vector<{n}, T, A> {{
                    const {dir_b_upper}: Self = vec{n}g!({negative_values});
                }}
            "#});
        }
    }

    let traits = traits.join("\n");
    let impls = impls.join("\n");

    writedoc!(
        module,
        r#"
        use crate::{{
            Construct,
            vec2g,
            vec3g,
            vec4g,
            vector::{{ScalarZero, ScalarOne, ScalarNegOne, VecAlignment, Vector}},
        }};

        {traits}

        {impls}
        "#
    )
    .unwrap();
}
