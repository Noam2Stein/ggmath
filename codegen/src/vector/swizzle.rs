use std::ops::Range;

use genco::quote;

use crate::{
    constants::{COMPONENT_ORDINALS, COMPONENTS, LENGTHS},
    join_and,
    module::{ModFile, TokensExt},
};

pub fn mod_() -> ModFile {
    quote! {
        use crate::{Scalar, VecAlignment, Vector, return_for_types, $(for &n in LENGTHS => Vec$(n)P<T>,)};

        $(
            for &n in LENGTHS =>

            impl<T: Scalar, A: VecAlignment> Vector<$n, T, A> {
                $(
                    for i in 0..n =>

                    $(format!("/// Returns the `{}` ({}) component of `self`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub fn $(COMPONENTS[i])(self) -> T {
                        self.swizzle1::<$i>()
                    }
                )

                $(
                    for &n2 in LENGTHS => $(
                        for combination in combinations(n, n2) =>

                        $(format!("/// Returns a new vector with the {} ({}) components of `self`.", join_and(combination.iter().map(|&i| format!("`{}`", COMPONENTS[i]))), join_and(combination.iter().map(|&i| COMPONENT_ORDINALS[i].to_string()))))
                        #[inline(always)]
                        pub fn $(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(self) -> Vector<$n2, T, A> {
                            self.swizzle$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i])_SRC)>>()
                        }
                    )
                )

                $(
                    for i in 0..n =>

                    $(format!("/// Returns `self` but with the `{}` ({}) component set to `value`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub fn with_$(COMPONENTS[i])(self, value: T) -> Self {
                        self.with_swizzle1::<$i>(value)
                    }
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) => $(
                        for combination in combinations_no_duplicates(n, n2) =>

                        /// Returns `self` but with:
                        $(
                            for (src, &dst) in combination.iter().enumerate() =>

                            $(format!("/// - The `{}` ({}) component set to the `{}` ({}) component of `other`", COMPONENTS[dst], COMPONENT_ORDINALS[dst], COMPONENTS[src], COMPONENT_ORDINALS[src]))
                        )
                        #[inline(always)]
                        pub fn with_$(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(self, value: Vector<$n2, T, impl VecAlignment>) -> Self {
                            self.with_swizzle$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i])_DST)>>(value)
                        }
                    )
                )
                
                #[inline(always)]
                fn swizzle1<const SRC: usize>(self) -> T {
                    return_for_types! {
                        (self: Vector<$n, T, A> => Vector<$n, T, VecAligned>) -> T => T {
                            |(vec,)| T::vec$(n)_swizzle1::<SRC>(vec)
                        }
                    }

                    self.index(SRC)
                }

                $(
                    for &n2 in LENGTHS =>

                    #[inline(always)]
                    fn swizzle$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i])_SRC: usize)>>(self) -> Vector<$n2, T, A> {
                        return_for_types! {
                            (self: Vector<$n, T, A> => Vector<$n, T, VecAligned>) -> Vector<$n2, T, VecAligned> => Vector<$n2, T, A> {
                                |(vec,)| T::vec$(n)_swizzle$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i])_SRC)>>(vec)
                            }
                        }

                        Vector::<$n2, T, A>::from_array([$(for i in 0..n2 join(, ) => self.index($(COMPONENTS[i])_SRC))])
                    }
                )

                $(
                    for i in 0..n =>

                    $(format!("/// Mutates `self` by setting the `{}` ({}) component to `value`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    fn set_$(COMPONENTS[i])(&mut self, value: T) {
                        *self = self.with_$(COMPONENTS[i])(value);
                    }
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) => $(
                        for combination in combinations_no_duplicates(n, n2) =>

                        /// Mutates `self` by setting:
                        $(
                            for (src, &dst) in combination.iter().enumerate() =>
                            $(format!("/// - The `{}` ({}) component set to the `{}` ({}) component of `other`", COMPONENTS[dst], COMPONENT_ORDINALS[dst], COMPONENTS[src], COMPONENT_ORDINALS[src]))
                        )
                        #[inline(always)]
                        fn set_$(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(&mut self, value: Vector<$n2, T, impl VecAlignment>) {
                            *self = self.with_$(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(value);
                        }
                    )
                )

                #[inline(always)]
                fn with_swizzle1<const DST: usize>(self, value: T) -> Self {
                    return_for_types! {
                        (self: Vector<$n, T, A> => Vector<$n, T, VecAligned>) -> Vector<$n, T, VecAligned> => Vector<$n, T, A> {
                            |(vec,)| T::vec$(n)_with_swizzle1::<DST>(vec, value)
                        }
                    }

                    let mut output = self;
                    output.set(DST, value);

                    output
                }

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) =>

                    #[inline(always)]
                    fn with_swizzle$(n2)<$(for i in 0..n2 join(, ) => const $(COMPONENTS[i])_DST: usize)>>(self, value: Vector<$n2, T, impl VecAlignment>) -> Self {
                        return_for_types! {
                            (
                                self: Vector<$n, T, A> => Vector<$n, T, VecAligned>,
                                value: Vector<$n2, T, _> => Vector<$n2, T, VecAligned>,
                            ) -> Vector<$n, T, VecAligned> => Vector<$n, T, A> {
                                |(vec,)| T::vec$(n)_with_swizzle$(n2)::<$(for i in 0..n2 join(, ) => $(COMPONENTS[i])_DST)>>(vec, value)
                            }
                        }

                        let mut output = self;
                        $(
                            for i in 0..n2 =>

                            output.set($(COMPONENTS[i])_DST, value.index(i));
                        )

                        output
                    }
                )
            }
        )

        $(
            for &n in LENGTHS =>

            impl<T: Scalar> Vec$(n)P<T> {
                $(
                    for i in 0..n =>

                    $(format!("/// Returns a reference to the `{}` ({}) component of `self`.", COMPONENTS[i], COMPONENT_ORDINALS[i]))
                    #[inline(always)]
                    pub const fn $(COMPONENTS[i])_ref(&self) -> &T {
                        &self.0[i]
                    }
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) => $(
                        for start in 0..n - n2 =>

                        $(format!("/// Returns a reference to the {} ({}) components of `self`.", join_and((start..start + n2).map(|i| format!("`{}`", COMPONENTS[i]))), join_and((start..start + n2).map(|i| COMPONENT_ORDINALS[i].to_string()))))
                        #[inline(always)]
                        pub const fn $((start..start + n2).map(|i| COMPONENTS[i]).collect::<String>())_ref(&self) -> &Vec$(n2)P<T> {
                            Vector::from_array_ref(unsafe {{ &*(self.as_ptr().add({start}) as *const [T; {n2}]) }})
                        }
                    )
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) => $(
                        for start in 0..n - n2 =>

                        $(format!("/// Returns a mutable reference to the {} ({}) components of `self`.", join_and((start..start + n2).map(|i| format!("`{}`", COMPONENTS[i]))), join_and((start..start + n2).map(|i| COMPONENT_ORDINALS[i].to_string()))))
                        #[inline(always)]
                        pub const fn $((start..start + n2).map(|i| COMPONENTS[i]).collect::<String>())_mut(&mut self) -> &mut Vec$(n2)P<T> {
                            Vector::from_mut_array(unsafe {{ &*(self.as_mut_ptr().add({start}) as *mut [T; {n2}]) }})
                        }
                    )
                )

                $(
                    for split in disjoint_splits(0..n).iter().filter(|split| split.len() > 1) =>

                    /// Returns a tuple with mutable references to:
                    $(
                        for range in split =>
                        $(format!("/// - The {} ({}) components of `self`", join_and(range.clone().into_iter().map(|i| format!("`{}`", COMPONENTS[i]))), join_and(range.clone().into_iter().map(|i| COMPONENT_ORDINALS[i].to_string()))))
                    )
                    #[inline(always)]
                    pub const fn $(split.iter().map(|range| range.clone().into_iter().map(|i| COMPONENTS[i]).collect::<String>()).collect::<String>())_mut(&mut self) -> ($(for range in split => &mut ,)) {
                        unsafe {{
                            ($(for _ in split => &mut *(self.as_mut_ptr().add({range.start}) as *mut T),),)
                        }}
                    }
                )
            }
        )
    }
    .to_mod_file("swizzle")
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
