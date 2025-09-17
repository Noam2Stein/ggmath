use std::ops::Range;

use genco::quote;

use crate::{
    constants::{COMPONENTS, LENGTHS},
    module::{ModFile, TokensExt},
};

pub fn mod_() -> ModFile {
    quote! {
        use crate::{Scalar, VecAlignment, Vector};

        $(for &n in LENGTHS join($['\r']) => pub use crate::vec$(n);)
        $(for &n in LENGTHS join($['\r']) => pub use crate::vec$(n)p;)
        $(for &n in LENGTHS join($['\r']) => pub use crate::vec$(n)g;)

        $(
            for &n in LENGTHS join($['\n']) =>

            #[macro_export]
            macro_rules! vec$(n) {
                ($$($$field:expr),* $$(,)?) => {
                    $crate::Vec$(n)::from(($$($$field,)*))
                }
            }
        )

        $(
            for &n in LENGTHS join($['\n']) =>

            #[macro_export]
            macro_rules! vec$(n)p {
                ($$($$field:expr),* $$(,)?) => {
                    $crate::Vec$(n)P::from(($$($$field,)*))
                }
            }
        )

        $(
            for &n in LENGTHS join($['\n']) =>

            #[macro_export]
            macro_rules! vec$(n)g {
                ($$($$field:expr),* $$(,)?) => {
                    $crate::Vector::<$n, _, _>::from(($$($$field,)*))
                }
            }
        )

        $(
            for &n in LENGTHS join($['\n']) => $(
                for split in splits(n) join($['\n']) =>

                $(
                    let alignment_generic_params = split
                        .iter()
                        .enumerate()
                        .filter(|(_, range)| range.len() > 1)
                        .map(|(range_idx, _)| format!("A{}: VecAlignment", range_idx))
                        .collect::<Vec<_>>()
                        .join(", ")
                )

                $(
                    let input_type = &format!(
                        "({})",
                        split
                            .iter()
                            .enumerate()
                            .map(|(range_idx, range)|
                                if range.len() > 1 {
                                    format!("Vector<{}, T, A{range_idx}>,", range.len())
                                } else {
                                    format!("T,")
                                }
                            )
                            .collect::<String>()
                    )
                )

                impl<T: Scalar, A: VecAlignment, $alignment_generic_params> From<$input_type> for Vector<$n, T, A> {
                    fn from(value: $input_type) -> Self {
                        Self::from_array([$(
                            for (range_idx, range) in split.iter().enumerate() join(, ) => $(
                                for i in 0..range.len() join(, ) => $(
                                    if range.len() > 1 {
                                        value.$range_idx.$(COMPONENTS[i])()
                                    } else {
                                        value.$range_idx
                                    }
                                )
                            )
                        )])
                    }
                }
            )
        )
    }.to_mod_file("constructor")
}

fn splits(n: usize) -> Vec<Vec<Range<usize>>> {
    if n == 0 {
        return vec![vec![]];
    }

    let max_mask = 1usize << (n - 1);
    let mut out = Vec::with_capacity(max_mask);

    for mask in 0..max_mask {
        let mut part = Vec::new();
        let mut start = 0usize;

        for i in 1..=n {
            let cut = i == n || ((mask >> (i - 1)) & 1) != 0;
            if cut {
                part.push(start..i);
                start = i;
            }
        }

        out.push(part);
    }

    out
}
