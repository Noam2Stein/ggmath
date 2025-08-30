use std::fmt::Write;

use indoc::{formatdoc, writedoc};

use crate::{join_and, module::*};

const LENGTHS: &[usize] = &[2, 3, 4];
const COMPONENTS: &[&str] = &["x", "y", "z", "w"];
const COMPONENT_ORDINALS: &[&str] = &["1st", "2nd", "3rd", "4th"];

pub fn write_mod(mut module: Mod) {
    let mut vector_impls = Vec::new();

    for &n in LENGTHS {
        let mut functions = Vec::new();

        for &n2 in LENGTHS {
            for combination in combinations(n, n2) {
                let fn_name = combination
                    .iter()
                    .map(|i| COMPONENTS[*i])
                    .collect::<Vec<_>>()
                    .join("");

                let combination_args = combination
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                let components_list =
                    join_and(combination.iter().map(|i| format!("`{}`", COMPONENTS[*i])));

                let component_oridinal_list = join_and(
                    combination
                        .iter()
                        .map(|i| format!("{}", COMPONENT_ORDINALS[*i])),
                );

                functions.push(formatdoc! {r#"
                    /// Returns a new vector with the {components_list} ({component_oridinal_list}) components of the input vector.
                    #[inline(always)]
                    pub fn {fn_name}(self) -> Vector<{n2}, T, A> {{
                        T::vec_swizzle{n2}::<{n}, A, {combination_args}>(self)
                    }}
                "#});
            }
        }

        let functions = functions.join("\n").replace("\n", "\n\t");

        vector_impls.push(formatdoc! {r#"
            impl<T: Scalar, A: VecAlignment> Vector<{n}, T, A> {{
                {functions}
            }}
        "#});
    }

    let vector_impls = vector_impls.join("\n");

    writedoc!(
        module,
        r#"
        use crate::vector::{{Scalar, VecAlignment, Vector}};

        {vector_impls}
        "#
    )
    .unwrap();
}

fn combinations(max: usize, len: usize) -> Vec<Vec<usize>> {
    if len == 0 {
        return vec![vec![]];
    }

    let mut result = vec![];
    for i in 0..max {
        for mut tail in combinations(max, len - 1) {
            tail.insert(0, i);
            result.push(tail);
        }
    }
    result
}
