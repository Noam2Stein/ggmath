use std::ops::Range;

use genco::quote;
use strum::IntoEnumIterator;

use crate::{
    backend::{SrcFile, TokensExt},
    iter::{Axis, Length},
};

pub fn srcmod() -> SrcFile {
    quote! {
        use crate::{Scalar, Simdness, Vector};

        $(for n in Length::iter() join($['\r']) => pub use crate::vec$(n);)
        $(for n in Length::iter() join($['\r']) => pub use crate::vec$(n)s;)
        $(for n in Length::iter() join($['\r']) => pub use crate::vec$(n)g;)

        $(
            for n in Length::iter() join($['\n']) =>

            $(format!("/// Creates a `Vec{n}<_>` from the given components and vectors."))
            $("///")
            $("/// # Example")
            $("/// ```")
            $("/// use ggmath::*;")
            $("///")
            $(format!("/// fn example() -> Vec{n}<f32> {{"))
            $(
                match n {
                    Length::N2 => $("///     vec2!(1.0, 2.0)"),
                    Length::N3 => $("///     vec3!(1.0, vec2!(2.0, 3.0))"),
                    Length::N4 => $("///     vec4!(1.0, vec2!(2.0, 3.0), 4.0)"),
                }
            )
            $("/// }")
            $("/// ```")
            #[macro_export]
            macro_rules! vec$(n) {
                ($$($$field:expr),* $$(,)?) => {
                    $crate::Vec$(n)::from(($$($$field,)*))
                }
            }
        )

        $(
            for n in Length::iter() join($['\n']) =>

            $(format!("/// Creates a `Vec{n}S<_>` from the given components and vectors."))
            $("///")
            $("/// # Example")
            $("/// ```")
            $("/// use ggmath::*;")
            $("///")
            $(format!("/// fn example() -> Vec{n}S<f32> {{"))
            $(
                match n {
                    Length::N2 => $("///     vec2s!(1.0, 2.0)"),
                    Length::N3 => $("///     vec3s!(1.0, vec2s!(2.0, 3.0))"),
                    Length::N4 => $("///     vec4s!(1.0, vec2s!(2.0, 3.0), 4.0)"),
                }
            )
            $("/// }")
            $("/// ```")
            #[macro_export]
            macro_rules! vec$(n)s {
                ($$($$field:expr),* $$(,)?) => {
                    $crate::Vec$(n)S::from(($$($$field,)*))
                }
            }
        )

        $(
            for n in Length::iter() join($['\n']) =>

            $(format!("/// Creates a `Vector<{n}, _, _>` from the given components and vectors."))
            $("/// This macro needs type inference to decide if the vector is SIMD or not.")
            $("///")
            $("/// # Example")
            $("/// ```")
            $("/// use ggmath::*;")
            $("///")
            $(format!("/// fn example<S: Simdness>() -> Vector<{n}, f32, S> {{"))
            $(
                match n {
                    Length::N2 => $("///     vec2g!(1.0, 2.0)"),
                    Length::N3 => $("///     vec3g!(1.0, vec2g!(2.0, 3.0))"),
                    Length::N4 => $("///     vec4g!(1.0, vec2g!(2.0, 3.0), 4.0)"),
                }
            )
            $("/// }")
            $("/// ```")
            #[macro_export]
            macro_rules! vec$(n)g {
                ($$($$field:expr),* $$(,)?) => {
                    $crate::Vector::<$n, _, _>::from(($$($$field,)*))
                }
            }
        )

        $(
            for n in Length::iter() join($['\n']) => $(
                for split in splits(n.as_usize()) join($['\n']) =>

                $(
                    let input_type = &format!(
                        "({})",
                        split
                            .iter()
                            .map(|range|
                                if range.len() > 1 {
                                    format!("Vector<{}, T, S>,", range.len())
                                } else {
                                    format!("T,")
                                }
                            )
                            .collect::<String>()
                    )
                )

                impl<T: Scalar, S: Simdness> From<$input_type> for Vector<$n, T, S> {
                    fn from(value: $input_type) -> Self {
                        Self::from_array([$(
                            for (range_idx, range) in split.iter().enumerate() join(, ) => $(
                                if range.len() > 1 {
                                    $(
                                        for i in 0..range.len() join(, ) =>

                                        value.$range_idx.$(Axis(i).lowercase_str())()
                                    )
                                } else {
                                    value.$range_idx
                                }
                            )
                        )])
                    }
                }
            )
        )
    }
    .to_srcfile("constructor")
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
