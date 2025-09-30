use std::ops::Range;

use genco::quote;
use strum::IntoEnumIterator;

use crate::{
    backend::{SrcFile, TokensExt},
    code::join_and,
    iter::{Axis, Length},
};

pub fn srcmod() -> SrcFile {
    quote! {
        use crate::{Scalar, Simdness, Vector, NonSimd, $(for n in Length::iter() join(, ) => Vec$(n)S)};

        $(
            for n in Length::iter() join($['\n']) =>

            impl<T: Scalar, S: Simdness> Vector<$n, T, S> {
                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for combination in swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = combination.iter().copied().map(|i| Axis(i).lowercase_str()).collect::<String>())

                        $(format!(
                            "/// Returns a new vector with the {components} ({components_ordinals}) components of `self`.",
                            components = join_and(combination.iter().copied().map(|i| format!("`{}`", Axis(i).lowercase_str()))),
                            components_ordinals = join_and(combination.iter().copied().map(|i| Axis(i).ordinal_str())),
                        ))
                        #[inline(always)]
                        pub fn $combination_str(self) -> Vector<$n2, T, S> {
                            self.shuffle_$(n2)::<$(for i in 0..n2.as_usize() join(, ) => $(combination[i]))>()
                        }
                    )
                )

                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for combination in with_swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = combination.iter().copied().map(|dst| Axis(dst).lowercase_str()).collect::<String>())

                        $("/// Returns `self` but with:")
                        $(for (src, dst) in combination.iter().copied().enumerate() join($['\r']) => $(format!(
                            "/// - The `{}` ({}) component set to the `{}` ({}) component of `value`",
                            Axis(dst).lowercase_str(),
                            Axis(dst).ordinal_str(),
                            Axis(src).lowercase_str(),
                            Axis(src).ordinal_str(),
                        )))
                        #[inline(always)]
                        pub fn with_$combination_str(self, value: Vector<$n2, T, S>) -> Self {
                            self.with_shuffle_$(n2)::<$(for dst in combination.iter().copied() join(, ) => $dst)>(value)
                        }
                    )
                )

                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for combination in with_swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = &combination.iter().copied().map(|dst| Axis(dst).lowercase_str()).collect::<String>())

                        $(for (src, dst) in combination.iter().copied().enumerate() join($['\r']) => $(format!(
                            "/// - Sets the `{}` ({}) component of `self` to the `{}` ({}) component of `value`",
                            Axis(dst).lowercase_str(),
                            Axis(dst).ordinal_str(),
                            Axis(src).lowercase_str(),
                            Axis(src).ordinal_str(),
                        )))
                        #[inline(always)]
                        pub fn set_$combination_str(&mut self, value: Vector<$n2, T, S>) {
                            *self = self.with_$combination_str(value);
                        }
                    )
                )
            }
        )

        $(
            for n in Length::iter() join($['\n']) =>

            impl<T: Scalar> Vector<$n, T, NonSimd> {
                $(
                    for n2 in Length::iter().filter(|&n2| n2 <= n) join($['\n']) => $(
                        for range in ref_swizzle_ranges(n, n2) join($['\n']) =>

                        $(let range_str = range.clone().map(|i| Axis(i).lowercase_str()).collect::<String>())

                        $(format!(
                            "/// Returns a reference to the {components} ({components_ordinals}) components of `self`.",
                            components = join_and(range.clone().map(|i| format!("`{}`", Axis(i).lowercase_str()))),
                            components_ordinals = join_and(range.clone().map(|i| Axis(i).ordinal_str())),
                        ))
                        #[inline(always)]
                        pub const fn $(range_str)_ref(&self) -> &Vec$(n2)S<T> {
                            Vector::from_array_ref(unsafe { &*(self.as_ptr().add($(range.start)) as *const [T; $n2]) })
                        }
                    )
                )

                $(
                    for n2 in Length::iter().filter(|&n2| n2 <= n) join($['\n']) => $(
                        for range in ref_swizzle_ranges(n, n2) join($['\n']) =>

                        $(let range_str = range.clone().map(|i| Axis(i).lowercase_str()).collect::<String>())

                        $(format!(
                            "/// Returns a mutable reference to the {components} ({components_ordinals}) components of `self`.",
                            components = join_and(range.clone().map(|i| format!("`{}`", Axis(i).lowercase_str()))),
                            components_ordinals = join_and(range.clone().map(|i| Axis(i).ordinal_str())),
                        ))
                        #[inline(always)]
                        pub const fn $(range_str)_mut(&mut self) -> &mut Vec$(n2)S<T> {
                            Vector::from_mut_array(unsafe { &mut *(self.as_mut_ptr().add($(range.start)) as *mut [T; $n2]) })
                        }
                    )
                )

                $(
                    for split in mut_swizzle_splits(n) join($['\n']) =>

                    $(let split_str = split
                        .iter()
                        .map(|range| range.clone().map(|i| Axis(i).lowercase_str()).collect::<String>())
                        .collect::<Vec<String>>()
                        .join("_")
                    )

                    $(let split_tuple_type = format!(
                        "({})",
                        split
                            .iter()
                            .map(|range| {
                                if range.len() == 1 {
                                    "&mut T".to_string()
                                } else {
                                    format!("&mut Vec{}S<T>", range.len())
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))

                    $("/// Returns a tuple with mutable references to:")
                    $(for range in &split join($['\r']) => $(
                        if range.len() == 1 {
                            $(format!(
                                "/// - The `{component}` ({component_ordinal}) component of `self`",
                                component = Axis(range.start).lowercase_str(),
                                component_ordinal = Axis(range.start).ordinal_str(),
                            ))
                        } else {
                            $(format!(
                                "/// - The {components} ({components_ordinals}) components of `self`",
                                components = join_and(range.clone().map(|i| format!("`{}`", Axis(i).lowercase_str()))),
                                components_ordinals = join_and(range.clone().map(|i| Axis(i).ordinal_str().to_string())),
                            ))
                        }
                    ))
                    #[inline(always)]
                    pub const fn $(split_str)_mut(&mut self) -> $split_tuple_type {
                        unsafe {
                            ($(
                                for range in &split => $(
                                    if range.len() == 1 {
                                        &mut *self.as_mut_ptr().add($(range.start))
                                    } else {
                                        Vector::from_mut_array(&mut *(self.as_mut_ptr().add($(range.start)) as *mut [T; $(range.len())]))
                                    }
                                ),
                            ))
                        }
                    }
                )
            }
        )
    }
    .to_srcfile("swizzle")
}

fn swizzle_combinations(vec_len: Length, output_len: Length) -> Vec<Vec<usize>> {
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

    combinations(vec_len.as_usize(), output_len.as_usize())
}

fn with_swizzle_combinations(vec_len: Length, value_len: Length) -> Vec<Vec<usize>> {
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

    combinations_no_duplicates(vec_len.as_usize(), value_len.as_usize())
}

fn ref_swizzle_ranges(vec_len: Length, ref_len: Length) -> Vec<Range<usize>> {
    (0..=vec_len.as_usize() - ref_len.as_usize())
        .map(|start| start)
        .map(|start| start..start + ref_len.as_usize())
        .collect()
}

fn mut_swizzle_splits(vec_len: Length) -> Vec<Vec<Range<usize>>> {
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

    disjoint_splits(0..vec_len.as_usize())
        .into_iter()
        .filter(|split| split.len() > 1)
        .collect()
}
