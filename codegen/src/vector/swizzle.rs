use std::ops::Range;

use indoc::formatdoc;

use crate::{
    constants::{COMPONENT_ORDINALS, COMPONENTS, LENGTHS},
    r#gen::ModFile,
    join_and,
};

pub fn module(scalar_fns: &mut Vec<String>) -> ModFile {
    let mut vector_impls = Vec::new();

    write_swizzle(&mut vector_impls, scalar_fns);
    write_swizzle_with(&mut vector_impls, scalar_fns);
    write_swizzle_set(&mut vector_impls, scalar_fns);
    write_swizzle_ref(&mut vector_impls, scalar_fns);
    write_swizzle_mut(&mut vector_impls, scalar_fns);

    let vector_impls = vector_impls.join("\n");

    ModFile::new(
        "swizzle",
        formatdoc! {r#"
            use crate::vector::{{Scalar, VecAlignment, VecPacked, Vector}};

            {vector_impls}
        "#},
    )
}

fn write_swizzle(vector_impls: &mut Vec<String>, scalar_fns: &mut Vec<String>) {
    for &n in LENGTHS {
        let mut functions = Vec::new();

        scalar_fns.push(formatdoc! {r#"
            /// Overridable implementation of aligned vec{n} getters like `Vec{n}::x`.
            #[inline(always)]
            pub fn vec{n}_swizzle1<const SRC: usize>(vec: Vec{n}<Self>) -> Self {{
                vec.index(SRC)
            }}
        "#});

        for i in 0..n {
            let component = COMPONENTS[i];
            let component_ordinal = COMPONENT_ORDINALS[i];

            functions.push(formatdoc! {r#"
                /// Returns the `{component}` ({component_ordinal}) component of `self`.
                #[inline(always)]
                pub fn {component}(self) -> T {{
                    return_for_types! {{
                        (self: Vector<{n}, T, A> => Vector<{n}, T, VecAligned>) -> T => T {{
                            |vec| T::vec{n}_swizzle1::<{i}>(vec)
                        }}
                    }}

                    self.index(SRC)
                }}
            "#});
        }

        for &n2 in LENGTHS {
            let combinations = combinations_no_duplicates(n, n2);

            let example_combination = combinations[combinations.len() / 5]
                .iter()
                .map(|i| COMPONENTS[*i])
                .collect::<Vec<_>>()
                .join("");

            let src_generic_params = (0..n2)
                .map(|i| format!("const {}_SRC: usize", COMPONENTS[i]))
                .collect::<Vec<_>>()
                .join(", ");

            let dst_items = (0..n2)
                .map(|i| format!("vec.index({}_SRC)", COMPONENTS[i]))
                .collect::<Vec<_>>()
                .join(", ");

            scalar_fns.push(formatdoc! {r#"
                /// Overridable implementation of aligned vec{n} swizzles that return vec{n2}s, like `Vec{n}::{example_combination}`.
                #[inline(always)]
                pub fn vec{n}_swizzle{n2}<{src_generic_params}>(vec: Vec{n}<Self>) -> Vec{n2}<Self> {{
                    Vec{n2}::from_array([{dst_items}])
                }}
            "#});

            for indicies in combinations {
                let combination = indicies
                    .iter()
                    .map(|i| COMPONENTS[*i])
                    .collect::<Vec<_>>()
                    .join("");

                let components_list =
                    join_and(indicies.iter().map(|i| format!("`{}`", COMPONENTS[*i])));

                let component_oridinal_list = join_and(
                    indicies
                        .iter()
                        .map(|i| format!("{}", COMPONENT_ORDINALS[*i])),
                );

                let combination_generic_args = indicies
                    .iter()
                    .map(|i| i.to_string())
                    .collect::<Vec<_>>()
                    .join(", ");

                let dst_items = indicies
                    .iter()
                    .map(|i| format!("self.index({})", COMPONENTS[*i]))
                    .collect::<Vec<_>>()
                    .join(", ");

                functions.push(formatdoc! {r#"
                    /// Returns a new vector with the {components_list} ({component_oridinal_list}) components of `self`.
                    #[inline(always)]
                    pub fn {combination}(self) -> Vector<{n2}, T, A> {{
                        return_for_types! {{
                            (self: Vector<{n}, T, A> => Vector<{n}, T, VecAligned>) -> Vector<{n2}, T, VecAligned> => Vector<{n2}, T, A> {{
                                |vec| T::vec{n}_swizzle{n2}::<{combination_generic_args}>(vec)
                            }}
                        }}

                        Vector::<{n2}, T, A>::from_array([{dst_items}])
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
}

fn write_swizzle_with(vector_impls: &mut Vec<String>, scalar_fns: &mut Vec<String>) {
    for &n in LENGTHS {
        let mut functions = Vec::new();
        let mut const_functions = Vec::new();

        for n2 in 1..=n {
            let combinations = combinations_no_duplicates(n, n2);

            let example_combination = combinations[combinations.len() / 3]
                .iter()
                .map(|&i| COMPONENTS[i])
                .collect::<String>();

            if n2 == 1 {
                scalar_fns.push(formatdoc! {r#"
                    /// Overridable implementation of aligned vec{n} "with swizzles" that replaces scalars, like `Vec{n}::with_{example_combination}`.
                    #[inline(always)]
                    pub fn vec{n}_with_swizzle{n2}<const DST: usize>(vec: Vec{n}<Self>, value: Self) -> Vec{n}<Self> {{
                        let mut output = vec;
                        output.set(DST, value);

                        output
                    }}
                "#});
            } else {
                let dst_generic_params = (0..n2)
                    .map(|i| format!("const {}_DST: usize", COMPONENTS[i]))
                    .collect::<Vec<_>>()
                    .join(", ");

                let set_stmts = (0..n2)
                    .map(|i| format!("output.set({}_DST, value.index({i}));", COMPONENTS[i]))
                    .collect::<Vec<_>>()
                    .join("\n\t");

                scalar_fns.push(formatdoc! {r#"
                    /// Overridable implementation of aligned vec{n} "with swizzles" that replaces vec{n2}s, like `Vec{n}::with_{example_combination}`.
                    #[inline(always)]
                    pub fn vec{n}_with_swizzle{n2}<{dst_generic_params}>(vec: Vec{n}<Self>, value: Self) -> Vec{n2}<Self> {{
                        let mut output = vec;
                        {set_stmts}

                        output
                    }}
                "#});
            }

            for combination in combinations {
                let combination_len = combination.len();

                let components = combination
                    .iter()
                    .map(|&i| COMPONENTS[i])
                    .collect::<String>();

                let components_list =
                    join_and(combination.iter().map(|&i| format!("`{}`", COMPONENTS[i])));

                let component_ordinals = join_and(
                    combination
                        .iter()
                        .map(|&i| COMPONENT_ORDINALS[i].to_string()),
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
                    .map(|(src, &dst)| {
                        format!("self.as_array_mut()[{dst}] = value.as_array()[{src}];")
                    })
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
}

fn write_swizzle_set(vector_impls: &mut Vec<String>, _scalar_fns: &mut Vec<String>) {
    for &n in LENGTHS {
        let mut functions = Vec::new();

        for n2 in 1..=n {
            for combination in combinations_no_duplicates(n, n2) {
                let combination_len = combination.len();

                let components = combination
                    .iter()
                    .map(|&i| COMPONENTS[i])
                    .collect::<String>();

                let components_list =
                    join_and(combination.iter().map(|&i| format!("`{}`", COMPONENTS[i])));

                let component_ordinals = join_and(
                    combination
                        .iter()
                        .map(|&i| COMPONENT_ORDINALS[i].to_string()),
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

                if combination.len() == 1 {
                    functions.push(formatdoc! {r#"
                        /// Mutates `self` by setting the {components_list} ({component_ordinals}) component to `value`.
                        #[inline(always)]
                        pub fn set_{components}(&mut self, value: T) {{
                            *self = self.with_{components}(value);
                        }}
                    "#});
                } else {
                    functions.push(formatdoc! {r#"
                        /// Mutates `self` by setting:
                        /// {documentation_lines}
                        #[inline(always)]
                        pub fn set_{components}(&mut self, other: Vector<{combination_len}, T, impl VecAlignment>) {{
                            *self = self.with_{components}(other);
                        }}
                    "#});
                }
            }
        }

        let functions = functions.join("\n").replace("\n", "\n\t");

        vector_impls.push(formatdoc! {r#"
            impl<T: Scalar, A: VecAlignment> Vector<{n}, T, A> {{
                {functions}
            }}
        "#});
    }
}

fn write_swizzle_ref(vector_impls: &mut Vec<String>, _scalar_fns: &mut Vec<String>) {
    for &n in LENGTHS {
        let mut functions = Vec::new();

        for start in 0..n {
            for n2 in 1..=n - start {
                let components = (start..start + n2)
                    .map(|i| COMPONENTS[i])
                    .collect::<String>();

                let component_ordinals =
                    join_and((start..start + n2).map(|i| COMPONENT_ORDINALS[i].to_string()));

                let component_list =
                    join_and((start..start + n2).map(|i| format!("`{}`", COMPONENTS[i])));

                if n2 == 1 {
                    functions.push(formatdoc! {r#"
                        /// Returns a reference to the {component_list} ({component_ordinals}) component of `self`.
                        #[inline(always)]
                        pub const fn {components}_ref(&self) -> &T {{
                            &self.0[{start}]
                        }}
                    "#});
                } else {
                    functions.push(formatdoc! {r#"
                        /// Returns a vector reference to the {component_list} ({component_ordinals}) components of `self`.
                        #[inline(always)]
                        pub const fn {components}_ref(&self) -> &Vec{n2}P<T> {{
                            Vector::from_array_ref(unsafe {{ &*(self.as_ptr().add({start}) as *const [T; {n2}]) }})
                        }}
                    "#});
                }
            }
        }

        let functions = functions.join("\n").replace("\n", "\n\t");

        vector_impls.push(formatdoc! {r#"
            impl<T: Scalar> Vec{n}P<T> {{
                {functions}
            }}
        "#});
    }
}

fn write_swizzle_mut(vector_impls: &mut Vec<String>, _scalar_fns: &mut Vec<String>) {
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
