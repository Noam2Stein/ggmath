use std::ops::Range;

use super::*;

pub fn write_mod(mut module: Mod) {
    writedoc!(
        module,
        r#"
        use super::*;

        "#
    )
    .unwrap();

    write_get_swizzle(&mut module);
    write_get_ref_swizzle(&mut module);
    write_get_mut_swizzle(&mut module);
    write_set_swizzle(&mut module);
    write_with_swizzle(&mut module);

    writedoc!(
        module,
        r#"
        #[cfg(test)]
        mod tests {{
            use super::*;

            #[test]
            fn test_swizzle() {{
                assert_eq!(vec2!(1, 2).xxy(), vec3!(1, 1, 2));
                assert_eq!(vec3!(1, 2, 3).xyzx(), vec4!(1, 2, 3, 1));
                assert_eq!(vec4!(1, 2, 3, 4).xyzw(), vec4!(1, 2, 3, 4));
                assert_eq!(vec2!(1, 2).y(), 2);

                assert_eq!(vec2p!(false, true).xyx(), vec3p!(false, true, false));
                assert_eq!(vec2p!(false, true).y(), true);
            }}

            #[test]
            fn test_swizzle_ref() {{
                assert_eq!(vec2!(1, 2).x_ref(), &1);
                assert_eq!(vec3p!(1, 2, 3).yz_ref(), &vec2p!(2, 3));
            }}

            #[test]
            fn test_swizzle_mut() {{
                assert_eq!(vec2!(1, 2).x_mut(), &mut 1);
                assert_eq!(vec3p!(1, 2, 3).yz_mut(), &mut vec2p!(2, 3));
                assert_eq!(vec4p!(1, 2, 3, 4).xy_z_mut(), (&mut vec2p!(1, 2), &mut 3));
            }}

            #[test]
            fn test_swizzle_set() {{
                let mut vec = vec2!(1, 2);
                vec.set_x(3);
                assert_eq!(vec, vec2!(3, 2));

                let mut vec = vec3p!(1, 2, 3);
                vec.set_zx(vec2!(4, 5));
                assert_eq!(vec, vec3p!(5, 2, 4));

                let mut vec = vec4p!(1, 2, 3, 4);
                vec.set_xzyw(vec4p!(5, 6, 7, 8));
                assert_eq!(vec, vec4p!(5, 7, 6, 8));
            }}

            #[test]
            fn test_swizzle_with() {{
                assert_eq!(vec2!(1, 2).with_x(3), vec2!(3, 2));
                assert_eq!(vec3p!(1, 2, 3).with_y(4), vec3p!(1, 4, 3));
                assert_eq!(vec4!(1, 2, 3, 4).with_zx(vec2!(5, 6)), vec4p!(6, 2, 5, 4));
            }}
        }}
        "#,
    )
    .unwrap();
}

fn write_get_swizzle(module: &mut Mod) {
    for len in LENGTHS {
        let fns = (0..len)
            .map(|i| {
                let comp = COMPONENTS[i];
                let comp_ordinal = ORDINALS[i];

                formatdoc! {r#"
                /// Returns the `{comp}` ({comp_ordinal}) component of the vector.
                #[inline(always)]
                pub const fn {comp}(self) -> T {{
                    self.index({i})
                }}
            "#}
            })
            .collect::<Vec<_>>()
            .join("\n");

        writedoc!(
            module,
            r#"
            impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                {fns}
            }}
            "#,
            fns = fns.replace("\n", "\n\t")
        )
        .unwrap();

        for len2 in MIN_LENGTH..=MAX_LENGTH {
            let fns = combinations(len, len2)
                .iter()
                .map(|indicies| {
                    let fn_name = indicies.iter().map(|i| COMPONENTS[*i]).collect::<Vec<_>>().join("");

                    let comp_list = join_and(indicies.iter().map(|i| format!("`{}`", COMPONENTS[*i])));
                    let comp_ordinal_list = join_and(indicies.iter().map(|i| ORDINALS[*i].to_string()));

                    let values = indicies.iter().map(|i| format!("self.index({i})")).collect::<Vec<_>>().join(", ");

                    formatdoc! {r#"
                        /// Returns a vector with the {comp_list} ({comp_ordinal_list}) components of the input vector.
                        #[inline(always)]
                        pub const fn {fn_name}(self) -> Vector<{len2}, T, A> {{
                            Vector::from_array([{values}])
                        }}
                    "#}
                })
                .collect::<Vec<_>>()
                .join("\n");

            writedoc!(
                module,
                r#"
                impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                    {fns}
                }}
                "#,
                fns = fns.replace("\n", "\n\t")
            )
            .unwrap();
        }
    }
}

fn write_get_ref_swizzle(module: &mut Mod) {
    for len in LENGTHS {
        let fns = (0..len)
            .map(|i| {
                let comp = COMPONENTS[i];
                let comp_ordinal = ORDINALS[i];

                formatdoc! {r#"
                /// Returns a reference to the `{comp}` ({comp_ordinal}) component of the vector.
                #[inline(always)]
                pub const fn {comp}_ref(&self) -> &T {{
                    &self.as_array_ref()[{i}]
                }}
            "#}
            })
            .collect::<Vec<_>>()
            .join("\n");

        writedoc!(
            module,
            r#"
            impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                {fns}
            }}
            "#,
            fns = fns.replace("\n", "\n\t")
        )
        .unwrap();

        for len2 in 2..=len {
            let fns = (0..len).filter(|start| start + len2 <= len).map(|start| {
                let fn_name = (start..start + len2)
                    .map(|i| COMPONENTS[i])
                    .collect::<Vec<_>>()
                    .join("");

                let comp_list = join_and((start..start + len2).map(|i| format!("`{}`", COMPONENTS[i])));
                let comp_ordinal_list = join_and((start..start + len2).map(|i| ORDINALS[i].to_string()));

                formatdoc! {r#"
                    /// Returns a reference to the {comp_list} ({comp_ordinal_list}) components part of the vector.
                    #[inline(always)]
                    pub const fn {fn_name}_ref(&self) -> &Vector<{len2}, T, VecPacked> {{
                        self.get_{len2}_ref({start}).unwrap()
                    }}
                "#}
            }).collect::<Vec<_>>().join("\n");

            if fns.is_empty() {
                continue;
            }

            writedoc!(
                module,
                r#"
                impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                    {fns}
                }}
                "#,
                fns = fns.replace("\n", "\n\t")
            )
            .unwrap();
        }
    }
}

fn write_get_mut_swizzle(module: &mut Mod) {
    for len in LENGTHS {
        let fns = disjoint_splits(0..len).iter().map(|split| {
            let comps = split
                .iter()
                .map(|range| range.clone().map(|i| COMPONENTS[i]).collect::<String>())
                .collect::<Vec<_>>()
                .join("_");

            if split.len() == 1 {
                let start = split[0].start;
                let len2 = split[0].len();

                let comp_list = join_and(split.iter().map(|range| range.clone().map(|i| format!("`{}`", COMPONENTS[i])).collect::<String>()));
                let comp_ordinal_list = join_and(split.iter().map(|range| range.clone().map(|i| ORDINALS[i]).collect::<String>()));

                if len2 == 1 {
                    formatdoc! {r#"
                        /// Returns a reference to the {comp_list} ({comp_ordinal_list}) component of the vector.
                        #[inline(always)]
                        pub const fn {comps}_mut(&mut self) -> &mut T {{
                            &mut self.as_array_mut()[{start}]
                        }}
                    "#}
                } else {
                    formatdoc! {r#"
                        /// Returns a reference to the {comp_list} ({comp_ordinal_list}) components part of the vector.
                        #[inline(always)]
                        pub const fn {comps}_mut(&mut self) -> &mut Vector<{len2}, T, VecPacked> {{
                            self.get_{len2}_mut({start}).unwrap()
                        }}
                    "#}
                }
            } else {
                let comp_list_lines = split.iter().map(|range| {
                    let comp_list = join_and(range.clone().map(|i| format!("`{}`", COMPONENTS[i])));
                    let comp_ordinal_list = join_and(range.clone().map(|i| ORDINALS[i].to_string()));

                    if range.len() == 1 {
                        format!("/// - the {comp_list} ({comp_ordinal_list}) component")
                    } else {
                        format!("/// - the {comp_list} ({comp_ordinal_list}) components")
                    }
                }).collect::<Vec<_>>().join(",\n") + ".";

                
                let tuple_fields = split.iter().map(|range| {
                    if range.len() == 1 {
                        format!("&mut T")
                    } else {
                        format!("&mut Vector<{}, T, VecPacked>", range.len())
                    }
                }).collect::<Vec<_>>().join(", ");
                
                let output_type = if split.len() == 1 {
                    format!("{tuple_fields}")
                } else {
                    format!("({tuple_fields})")
                };

                let output_fields = split.iter().map(|range| {
                    if range.len() == 1 {
                        format!("&mut *self.as_mut_ptr().add({})", range.start)
                    } else {
                        format!("&mut *(self.as_mut_ptr().add({}) as *mut Vector<_, _, _>)", range.start)
                    }
                }).collect::<Vec<_>>().join(", ");

                let output = if split.len() == 1 {
                    output_fields
                } else {
                    format!("({output_fields})")
                };

                formatdoc! {r#"
                    /// Splits the vector into these mutable references:
                    {comp_list_lines}
                    #[inline(always)]
                    pub const fn {comps}_mut(&mut self) -> {output_type} {{
                        unsafe {{
                            {output}
                        }}
                    }}
                "#}
            }
        }).collect::<Vec<_>>().join("\n");

        if fns.is_empty() {
            continue;
        }

        writedoc!(
            module,
            r#"
            impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                {fns}
            }}
            "#,
            fns = fns.replace("\n", "\n\t")
        )
        .unwrap();
    }
}

fn write_set_swizzle(module: &mut Mod) {
    for len in LENGTHS {
        for len2 in 1..=len {
            let fns = combinations_no_duplicates(len, len2).iter().map(|indicies| {
                let fn_name = "set_".to_string() + &indicies.iter().map(|i| COMPONENTS[*i]).collect::<String>();
    
                let comp_list = join_and(indicies.iter().map(|i| format!("`{}`", COMPONENTS[*i])));
                let comp_ordinal_list = join_and(indicies.iter().map(|i| ORDINALS[*i].to_string()));
    
                let len2 = indicies.len();
    
                if indicies.len() == 1 {
                    let idx = indicies[0];
                    formatdoc! {r#"
                        /// Sets the {comp_list} ({comp_ordinal_list}) component of the vector.
                        #[inline(always)]
                        pub const fn {fn_name}(&mut self, value: T) {{
                            self.as_array_mut()[{idx}] = value;
                        }}
                    "#}
                } else {
                    let stmts = indicies.iter().enumerate().map(|(src_idx, &dst_idx)| {
                        format!("self.as_array_mut()[{dst_idx}] = value.to_array()[{src_idx}];")
                    }).collect::<Vec<_>>().join("\n\t");
    
                    formatdoc! {r#"
                        /// Sets the {comp_list} ({comp_ordinal_list}) components of the vector.
                        #[inline(always)]
                        pub const fn {fn_name}(&mut self, value: Vector<{len2}, T, impl VecAlignment>) {{
                            {stmts}
                        }}
                    "#}
                }
            }).collect::<Vec<_>>().join("\n");

            if fns.is_empty() {
                continue;
            }

            writedoc!(
                module,
                r#"
                impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                    {fns}
                }}
                "#,
                fns = fns.replace("\n", "\n\t")
            )
            .unwrap();
        }
    }
}

fn write_with_swizzle(module: &mut Mod) {
    for len in LENGTHS {
        for len2 in 1..=len {
            let fns = combinations_no_duplicates(len, len2).iter().map(|indicies| {
                let comps = indicies.iter().map(|i| COMPONENTS[*i]).collect::<String>();
    
                let comp_list = join_and(indicies.iter().map(|i| format!("`{}`", COMPONENTS[*i])));
                let comp_ordinal_list = join_and(indicies.iter().map(|i| ORDINALS[*i].to_string()));
    
                let len2 = indicies.len();
    
                if indicies.len() == 1 {
                    formatdoc! {r#"
                        /// Returns the input vector with the {comp_list} ({comp_ordinal_list}) component modified.
                        #[inline(always)]
                        pub const fn with_{comps}(self, value: T) -> Self {{
                            let mut result = self;
                            result.set_{comps}(value);
                            result
                        }}
                    "#}
                } else {
                    formatdoc! {r#"
                        /// Returns the input vector with the {comp_list} ({comp_ordinal_list}) components modified.
                        #[inline(always)]
                        pub const fn with_{comps}(self, value: Vector<{len2}, T, impl VecAlignment>) -> Self {{
                            let mut result = self;
                            result.set_{comps}(value);
                            result
                        }}
                    "#}
                }
            }).collect::<Vec<_>>().join("\n");

            if fns.is_empty() {
                continue;
            }

            writedoc!(
                module,
                r#"
                impl<T: Scalar, A: VecAlignment> Vector<{len}, T, A> {{
                    {fns}
                }}
                "#,
                fns = fns.replace("\n", "\n\t")
            )
            .unwrap();
        }
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
