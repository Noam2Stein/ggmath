use std::ops::{Range, RangeInclusive};

use genco::quote;
use strum::IntoEnumIterator;

use crate::{
    backend::{SrcFile, TokensExt},
    code::join_and,
    iter::{Axis, Length},
};

pub fn srcmod() -> SrcFile {
    quote! {
        use crate::{Scalar, Simdness, Vector, $(for n in Length::iter() join(, ) => Vec$(n)S)};

        $(
            for n in Length::iter() join($['\n']) =>

            impl<T: Scalar, S: Simdness> Vector<$n, T, S> {
                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for combination in swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = combination.iter().map(|axis| axis.lowercase_str()).collect::<String>())

                        $(format!(
                            "/// Returns a new vector with the {components} ({components_ordinals}) components of `self`.",
                            components = join_and(combination.iter().map(|axis| format!("`{}`", axis.lowercase_str()))),
                            components_ordinals = join_and(combination.iter().map(|axis| axis.ordinal_str())),
                        ))
                        #[inline(always)]
                        pub fn $combination_str(self) -> Vector<$n2, T, S> {
                            self.shuffle_$(n2)::<$(for axis in combination join(, ) => $(axis.as_usize()))>()
                        }
                    )
                )

                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for combination in with_swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = combination.iter().map(|(_, dst)| dst.lowercase_str()).collect::<String>())

                        $("/// Returns `self` but with:")
                        $(for (src, dst) in &combination join($['\r']) => $(format!(
                            "/// - The `{}` ({}) component set to the `{}` ({}) component of `value`",
                            dst.lowercase_str(),
                            dst.ordinal_str(),
                            src.lowercase_str(),
                            src.ordinal_str(),
                        )))
                        #[inline(always)]
                        pub fn with_$combination_str(self, value: Vector<$n2, T, S>) -> Self {
                            self.with_shuffle_$(n2)::<$(for (_, dst_axis) in &combination join(, ) => dst_axis.as_usize())>(value)
                        }
                    )
                )

                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for combination in with_swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = combination.iter().map(|(_, dst)| dst.lowercase_str()).collect::<String>())

                        $(for (src, dst) in combination join($['\r']) => $(format!(
                            "/// - Sets the `{}` ({}) component of `self` to the `{}` ({}) component of `value`",
                            dst.lowercase_str(),
                            dst.ordinal_str(),
                            src.lowercase_str(),
                            src.ordinal_str(),
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
                    for n2 in Length::iter() join($['\n']) => $(
                        for range in ref_swizzle_ranges(n, n2) join($['\n']) =>

                        $(let range_vec = (range.start().as_usize()..=range.end().as_usize()).map(|i| Axis::from_usize(i).unwrap()).collect::<Vec<Axis>>())

                        $(let range_str = range_vec.iter().map(|axis| axis.lowercase_str()).collect::<String>())

                        $(format!(
                            "/// Returns a reference to the {components} ({components_ordinals}) components of `self`.",
                            components = join_and(range_vec.iter().map(|axis| format!("`{}`", axis.lowercase_str()))),
                            components_ordinals = join_and(range_vec.iter().map(|axis| axis.ordinal_str())),
                        ))
                        #[inline(always)]
                        pub const fn $(range_str)_ref(&self) -> &Vec$(n2)S<T> {
                            Vector::from_array_ref(unsafe { &*(self.as_ptr().add($(range.start().as_usize())) as *const [T; $n2]) })
                        }
                    )
                )

                $(
                    for n2 in Length::iter() join($['\n']) => $(
                        for range in ref_swizzle_ranges(n, n2) join($['\n']) =>

                        $(let range_vec = (range.start().as_usize()..=range.end().as_usize()).map(|i| Axis::from_usize(i).unwrap()).collect::<Vec<Axis>>())

                        $(let range_str = range_vec.iter().map(|axis| axis.lowercase_str()).collect::<String>())

                        $(format!(
                            "/// Returns a mutable reference to the {components} ({components_ordinals}) components of `self`.",
                            components = join_and(range_vec.iter().map(|axis| format!("`{}`", axis.lowercase_str()))),
                            components_ordinals = join_and(range_vec.iter().map(|axis| axis.ordinal_str())),
                        ))
                        #[inline(always)]
                        pub const fn $(range_str)_mut(&mut self) -> &mut Vec$(n2)S<T> {
                            Vector::from_mut_array(unsafe { &mut *(self.as_mut_ptr().add($(range.start().as_usize())) as *mut [T; $n2]) })
                        }
                    )
                )

                $(
                    for split in mut_swizzle_splits(n) join($['\n']) =>

                    $(let split_str = split
                        .iter()
                        .map(|range| {
                            let range_vec = (range.start().as_usize()..=range.end().as_usize()).map(|i| Axis::from_usize(i).unwrap()).collect::<Vec<Axis>>();
                            range_vec.iter().map(|axis| axis.lowercase_str()).collect::<String>()
                        })
                        .collect::<Vec<String>>()
                        .join("_")
                    )

                    $(let split_tuple_type = format!(
                        "({})",
                        split
                            .iter()
                            .map(|range| {
                                if range.end() == range.start() {
                                    "&mut T".to_string()
                                } else {
                                    format!("&mut Vec{}S<T>", range.start().as_usize() - range.end().as_usize() + 1)
                                }
                            })
                            .collect::<Vec<String>>()
                            .join(", ")
                    ))

                    $("/// Returns a tuple with mutable references to:")
                    $(for range in &split join($['\r']) => $(
                        if range.end() == range.start() {
                            $(format!(
                                "/// - The `{component}` ({component_ordinal}) component of `self`",
                                component = range.start().lowercase_str(),
                                component_ordinal = range.start().ordinal_str(),
                            ))
                        } else {
                            $(let range_vec = (range.start().as_usize()..=range.end().as_usize()).map(|i| Axis::from_usize(i).unwrap()).collect::<Vec<Axis>>())
                            $(format!(
                                "/// - The {components} ({components_ordinals}) components of `self`",
                                components = join_and(range_vec.iter().map(|axis| format!("`{}`", axis.lowercase_str()))),
                                components_ordinals = join_and(range_vec.iter().map(|axis| axis.ordinal_str().to_string())),
                            ))
                        }
                    ))
                    #[inline(always)]
                    pub const fn $(split_str)_mut(&mut self) -> $split_tuple_type {
                        unsafe {
                            ($(
                                for range in &split => $(
                                    if range.end() == range.start() {
                                        &mut *self.as_mut_ptr().add($(range.start().as_usize()))
                                    } else {
                                        Vector::from_mut_array(&mut *(self.as_mut_ptr().add($(range.start().as_usize())) as *mut [T; $(range.start().as_usize() - range.end().as_usize() + 1)]))
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

fn swizzle_combinations(vec_len: Length, output_len: Length) -> Vec<Vec<Axis>> {
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
        .into_iter()
        .map(|combination| {
            combination
                .into_iter()
                .map(|i| Axis::from_usize(i).unwrap())
                .collect()
        })
        .collect()
}

fn with_swizzle_combinations(vec_len: Length, value_len: Length) -> Vec<Vec<(Axis, Axis)>> {
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
        .into_iter()
        .map(|combination| {
            combination
                .into_iter()
                .enumerate()
                .map(|(src, dst)| {
                    (
                        Axis::from_usize(src).unwrap(),
                        Axis::from_usize(dst).unwrap(),
                    )
                })
                .collect()
        })
        .collect()
}

fn ref_swizzle_ranges(vec_len: Length, ref_len: Length) -> Vec<RangeInclusive<Axis>> {
    (0..=vec_len.as_usize() - ref_len.as_usize())
        .map(|start| Axis::from_usize(start).unwrap())
        .map(|start| start..=start + ref_len - 1)
        .collect()
}

fn mut_swizzle_splits(vec_len: Length) -> Vec<Vec<RangeInclusive<Axis>>> {
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
        .map(|combination| {
            combination
                .into_iter()
                .map(|range| {
                    Axis::from_usize(range.start).unwrap()
                        ..=Axis::from_usize(range.end - 1).unwrap()
                })
                .collect::<Vec<RangeInclusive<Axis>>>()
        })
        .collect()
}
