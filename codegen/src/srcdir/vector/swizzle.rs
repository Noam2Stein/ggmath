use std::iter::repeat;

use genco::quote;

use crate::util::{TokensExt, join_list};

pub fn generate() {
    quote! {
        $(let common_lengths = [2, 3, 4])
        $(let axes_lowercase = ['x', 'y', 'z', 'w'])
        $(let axes_ordinals = ["1st", "2nd", "3rd", "4th"])

        use crate::{Vector, Simdness, Scalar};

        $(
            for n in common_lengths join($['\n']) =>

            impl<T: Scalar<$n>, S: Simdness> Vector<$n, T, S> {
                $(
                    for n2 in common_lengths join($['\n']) => $(
                        for combination in get_swizzle_combinations(n, n2) join($['\n']) =>

                        $(let combination_str = combination.iter().copied().map(|i| axes_lowercase[i]).collect::<String>())

                        $(let generic_args = combination.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(", "))

                        $(format!(
                            "/// Returns a vector{n2} with the {combination_list} ({combination_ordinals}) components of `self`.",
                            combination_list = join_list(combination.iter().copied().map(|i| format!("`{}`", axes_lowercase[i]))),
                            combination_ordinals = join_list(combination.iter().copied().map(|i| axes_ordinals[i])),
                        ))
                        #[inline(always)]
                        pub fn $combination_str(self) -> Vector<$n2, T, S>
                        $(
                            if n2 != n =>

                            where
                                T: Scalar<$n2>,
                        )
                        {
                            self.get_const_vec$n2::<$generic_args>()
                        }
                    )
                )
            }
        )
    }
    .write_in_src("vector/swizzle.rs");
}

fn get_swizzle_combinations(n: usize, n2: usize) -> impl Iterator<Item = Vec<usize>> {
    struct Iter {
        n: usize,
        n2: usize,
        indices: Vec<usize>,
    }

    impl Iterator for Iter {
        type Item = Vec<usize>;

        fn next(&mut self) -> Option<Self::Item> {
            for i in (0..self.n2).rev() {
                self.indices[i] += 1;
                if self.indices[i] >= self.n {
                    self.indices[i] = 0;
                } else {
                    return Some(self.indices.clone());
                }
            }

            None
        }
    }

    Iter {
        n,
        n2,
        indices: repeat(0).take(n2).collect(),
    }
}
