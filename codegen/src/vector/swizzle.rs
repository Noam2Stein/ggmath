use std::ops::Range;

use genco::quote;

use crate::{
    constants::{COMPONENT_ORDINALS, COMPONENTS, LENGTHS},
    join_and,
    module::{ModFile, TokensExt},
};

pub fn mod_() -> ModFile {
    quote! {
        use crate::{Scalar, VecAlignment, Vector, $(for &n in LENGTHS join(, ) => Vec$(n)P)};

        $(
            for &n in LENGTHS join($['\n']) =>

            impl<T: Scalar, A: VecAlignment> Vector<$n, T, A> {
                $(
                    for &n2 in LENGTHS join($['\n']) => $(
                        for combination in combinations(n, n2) join($['\n']) =>

                        $(format!("/// Returns a new vector with the {} ({}) components of `self`.", join_and(combination.iter().map(|&i| format!("`{}`", COMPONENTS[i]))), join_and(combination.iter().map(|&i| COMPONENT_ORDINALS[i].to_string()))))
                        #[inline(always)]
                        pub fn $(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(self) -> Vector<$n2, T, A> {
                            self.shuffle_$(n2)::<$(for i in combination join(, ) => $i)>()
                        }
                    )
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) join($['\n']) => $(
                        for combination in combinations_no_duplicates(n, n2) join($['\n']) =>

                        $("/// Returns `self` but with:")
                        $(
                            for (src, &dst) in combination.iter().enumerate() join($['\r']) =>

                            $(format!("/// - The `{}` ({}) component set to the `{}` ({}) component of `value`", COMPONENTS[dst], COMPONENT_ORDINALS[dst], COMPONENTS[src], COMPONENT_ORDINALS[src]))
                        )
                        #[inline(always)]
                        pub fn with_$(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(self, value: Vector<$n2, T, impl VecAlignment>) -> Self {
                            self.with_shuffle_$(n2)::<$(for &i in &combination join(, ) => $i)>(value)
                        }
                    )
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) join($['\n']) => $(
                        for combination in combinations_no_duplicates(n, n2) join($['\n']) =>

                        $(
                            for (src, &dst) in combination.iter().enumerate() join($['\r']) =>
                            $(format!("/// - Sets the `{}` ({}) component of `self` to the `{}` ({}) component of `value`", COMPONENTS[dst], COMPONENT_ORDINALS[dst], COMPONENTS[src], COMPONENT_ORDINALS[src]))
                        )
                        #[inline(always)]
                        pub fn set_$(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(&mut self, value: Vector<$n2, T, impl VecAlignment>) {
                            *self = self.with_$(combination.iter().map(|&i| COMPONENTS[i]).collect::<String>())(value);
                        }
                    )
                )
            }
        )

        $(
            for &n in LENGTHS join($['\n']) =>

            impl<T: Scalar> Vec$(n)P<T> {
                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) join($['\n']) => $(
                        for start in 0..=n - n2 =>

                        $(format!("/// Returns a reference to the {} ({}) components of `self`.", join_and((start..start + n2).map(|i| format!("`{}`", COMPONENTS[i]))), join_and((start..start + n2).map(|i| COMPONENT_ORDINALS[i].to_string()))))
                        #[inline(always)]
                        pub const fn $((start..start + n2).map(|i| COMPONENTS[i]).collect::<String>())_ref(&self) -> &Vec$(n2)P<T> {
                            Vector::from_array_ref(unsafe { &*(self.as_ptr().add($start) as *const [T; $n2]) })
                        }
                    )
                )

                $(
                    for &n2 in LENGTHS.iter().filter(|&&n2| n2 <= n) join($['\n']) => $(
                        for start in 0..=n - n2 join($['\n']) =>

                        $(format!("/// Returns a mutable reference to the {} ({}) components of `self`.", join_and((start..start + n2).map(|i| format!("`{}`", COMPONENTS[i]))), join_and((start..start + n2).map(|i| COMPONENT_ORDINALS[i].to_string()))))
                        #[inline(always)]
                        pub const fn $((start..start + n2).map(|i| COMPONENTS[i]).collect::<String>())_mut(&mut self) -> &mut Vec$(n2)P<T> {
                            Vector::from_mut_array(unsafe { &mut *(self.as_mut_ptr().add($start) as *mut [T; $n2]) })
                        }
                    )
                )

                $(
                    for split in disjoint_splits(0..n).into_iter().filter(|split| split.len() > 1) join($['\n']) =>

                    $("/// Returns a tuple with mutable references to:")
                    $(
                        for range in &split join($['\r']) => $(
                            if range.len() == 1 {
                                $(format!("/// - The `{}` ({}) component of `self`", COMPONENTS[range.start], COMPONENT_ORDINALS[range.start]))
                            } else {
                                $(format!("/// - The {} ({}) components of `self`", join_and(range.clone().into_iter().map(|i| format!("`{}`", COMPONENTS[i]))), join_and(range.clone().into_iter().map(|i| COMPONENT_ORDINALS[i].to_string()))))
                            }
                        )
                    )
                    #[inline(always)]
                    pub const fn $(for range in &split join(_) => $(for i in range.clone() => $(COMPONENTS[i])))_mut(&mut self) -> ($(for range in &split join(, ) => &mut $(if range.len() == 1 { T } else { Vec$(range.len())P<T> } ))$(if split.len() == 1 { , })) {
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
