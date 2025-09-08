use std::ops::Range;

use indoc::formatdoc;

use crate::{join_and, module::*};

const LENGTHS: &[usize] = &[2, 3, 4];
const COMPONENTS: &[&str] = &["x", "y", "z", "w"];
const COMPONENT_ORDINALS: &[&str] = &["1st", "2nd", "3rd", "4th"];

pub fn write_mod(module: Mod) {
    let mut vector_impls = Vec::new();

    for &n in LENGTHS {
        let mut functions = Vec::new();

        for split in disjoint_splits(0..n) {
            let components = split
                .iter()
                .map(|range| {
                    COMPONENTS[range.clone()]
                        .iter()
                        .copied()
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("_");

            if split.len() == 1 {
                let range = split[0].clone();
                let start = range.start;
                let len = range.len();

                let components_list = join_and(
                    range
                        .clone()
                        .into_iter()
                        .map(|i| format!("`{}`", COMPONENTS[i])),
                );

                let component_ordinals = join_and(
                    range
                        .clone()
                        .into_iter()
                        .map(|i| COMPONENT_ORDINALS[i].to_string()),
                );

                if range.len() == 1 {
                    functions.push(formatdoc! {r#"
                        /// Returns a mutable reference to the {components_list} ({component_ordinals}) component of `self`.
                        #[inline(always)]
                        pub const fn {components}_mut(&mut self) -> &mut T {{
                            &mut self.as_array_mut()[{start}]
                        }}
                    "#});
                } else {
                    functions.push(formatdoc! {r#"
                        /// Returns a mutable vector reference to the {components_list} ({component_ordinals}) components of `self`.
                        #[inline(always)]
                        pub const fn {components}_mut(&mut self) -> &mut Vector<{len}, T, VecPacked> {{
                            unsafe {{
                                &mut *(self.as_mut_ptr().add({start}) as *mut Vector<{len}, T, VecPacked>)
                            }}
                        }}
                    "#});
                }
            } else {
                let documentation_lines = split
                    .iter()
                    .map(|range| {
                        let component_list = join_and(
                            range
                                .clone()
                                .into_iter()
                                .map(|i| format!("`{}`", COMPONENTS[i])),
                        );

                        let component_ordinals = join_and(
                            range
                                .clone()
                                .into_iter()
                                .map(|i| COMPONENT_ORDINALS[i].to_string()),
                        );

                        let component_punct = if range.len() == 1 {
                            "component"
                        } else {
                            "components"
                        };

                        format!(
                            "- The {component_list} ({component_ordinals}) {component_punct} of `self`"
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n/// ");

                let tuple_field_types = split
                    .iter()
                    .map(|range| {
                        if range.len() == 1 {
                            "&mut T".to_string()
                        } else {
                            format!("&mut Vector<{}, T, VecPacked>", range.len())
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let tuple_fields = split
                .iter()
                .map(|range| {
                    if range.len() == 1 {
                        format!("&mut *self.as_mut_ptr().add({})", range.start)
                    } else {
                        format!(
                            "&mut *(self.as_mut_ptr().add({}) as *mut Vector<{}, T, VecPacked>)",
                            range.start,
                            range.len()
                        )
                    }
                })
                .collect::<Vec<_>>()
                .join(", ");

                functions.push(formatdoc! {r#"
                    /// Returns a tuple with mutable references to:
                    /// {documentation_lines}
                    #[inline(always)]
                    pub const fn {components}_mut(&mut self) -> ({tuple_field_types},) {{
                        unsafe {{
                            ({tuple_fields},)
                        }}
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

    module.finish(formatdoc! {r#"
        use crate::vector::{{Scalar, VecAlignment, VecPacked, Vector}};

        {vector_impls}
    "#});
}

fn disjoint_splits(range: Range<usize>) -> Vec<Vec<Range<usize>>> {
    fn helper(start: usize, end: usize) -> Vec<Vec<Range<usize>>> {
        let mut result = Vec::new();

        for split_start in start..end {
            for split_end in split_start + 1..=end {
                let first = split_start..split_end;

                // just this segment
                result.push(vec![first.clone()]);

                // extend with further disjoint pieces
                for mut rest in helper(split_end, end) {
                    let mut combo = vec![first.clone()];
                    combo.append(&mut rest);
                    result.push(combo);
                }
            }
        }

        result
    }

    helper(range.start, range.end)
}
