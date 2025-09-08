use indoc::formatdoc;

use crate::{join_and, module::*};

const LENGTHS: &[usize] = &[2, 3, 4];
const COMPONENTS: &[&str] = &["x", "y", "z", "w"];
const COMPONENT_ORDINALS: &[&str] = &["1st", "2nd", "3rd", "4th"];

pub fn write_mod(module: Mod) {
    let mut vector_impls = Vec::new();

    for &n in LENGTHS {
        let mut functions = Vec::new();
        let mut const_functions = Vec::new();

        for combination in (1..=n)
            .map(|len| combinations_no_duplicates(n, len))
            .flatten()
        {
            let combination_len = combination.len();

            let components = combination
                .iter()
                .map(|i| COMPONENTS[*i])
                .collect::<String>();

            let components_list =
                join_and(combination.iter().map(|i| format!("`{}`", COMPONENTS[*i])));

            let component_ordinals = join_and(
                combination
                    .iter()
                    .map(|i| COMPONENT_ORDINALS[*i].to_string()),
            );

            let documentation_lines = combination
                .iter()
                .enumerate()
                .map(|(src, &dst)| {
                    format!(
                        "- The `{}` ({}) component of `self` set to the `{}` ({}) component of `other`",
                        COMPONENTS[dst],
                        COMPONENT_ORDINALS[dst],
                        COMPONENTS[src],
                        COMPONENT_ORDINALS[src],
                    )
                })
                .collect::<Vec<_>>()
                .join("\n/// ");

            let combination_generic_args = combination
                .iter()
                .map(|i| i.to_string())
                .collect::<Vec<_>>()
                .join(", ");

            let const_set_stmts = combination
                .iter()
                .enumerate()
                .map(|(src, &dst)| format!("self.as_array_mut()[{dst}] = value.as_array()[{src}];"))
                .collect::<Vec<_>>()
                .join("\n\t");

            if combination.len() == 1 {
                functions.push(formatdoc! {r#"
                    /// Returns `self` but with the {components_list} ({component_ordinals}) component set to `value`.
                    #[inline(always)]
                    pub fn with_{components}(self, value: T) -> Self {{
                        T::vec_with_swizzle1::<_, _, {combination_generic_args}>(self, value)
                    }}
                "#});

                const_functions.push(formatdoc! {r#"
                    /// Version of `Vector::with_{components}` that supports const contexts.
                    /// This version may be less performant than the non-const version.
                    /// 
                    /// When rust const-capabilities improve, this function will be deprecated.
                    #[inline(always)]
                    pub const fn const_with_{components}(mut self, value: T) -> Self {{
                        self.as_array_mut()[{combination_generic_args}] = value;

                        self
                    }}
                "#});
            } else {
                functions.push(formatdoc! {r#"
                    /// Returns `self` but with:
                    /// {documentation_lines}
                    #[inline(always)]
                    pub fn with_{components}(self, value: Vector<{combination_len}, T, impl VecAlignment>) -> Self {{
                        T::vec_with_swizzle{combination_len}::<_, _, {combination_generic_args}>(self, value)
                    }}
                "#});

                const_functions.push(formatdoc! {r#"
                    /// Version of `Vector::with_{components}` that supports const contexts.
                    /// This version may be less performant than the non-const version.
                    /// 
                    /// When rust const-capabilities improve, this function will be deprecated.
                    #[inline(always)]
                    pub const fn const_with_{components}(mut self, value: Vector<{combination_len}, T, impl VecAlignment>) -> Self {{
                        {const_set_stmts}

                        self
                    }}
                "#});
            }
        }

        let functions = functions.join("\n").replace("\n", "\n\t");
        let const_functions = const_functions.join("\n").replace("\n", "\n\t");

        vector_impls.push(formatdoc! {r#"
            impl<T: Scalar, A: VecAlignment> Vector<{n}, T, A> {{
                {functions}
            }}
            
            impl<T: Scalar, A: VecAlignment> Vector<{n}, T, A> {{
                {const_functions}
            }}
        "#});
    }

    let vector_impls = vector_impls.join("\n");

    module.finish(formatdoc! {r#"
        use crate::vector::{{Scalar, VecAlignment, Vector}};

        {vector_impls}
    "#});
}

fn combinations_no_duplicates(max: usize, len: usize) -> Vec<Vec<usize>> {
    if len == 0 {
        return vec![vec![]];
    }

    let mut result = Vec::new();
    for i in 0..max {
        for mut tail in combinations_no_duplicates(max, len - 1) {
            // Skip if `i` already exists in tail
            if tail.contains(&i) {
                continue;
            }
            tail.insert(0, i);
            result.push(tail);
        }
    }
    result
}
