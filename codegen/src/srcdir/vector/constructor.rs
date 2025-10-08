use std::collections::HashSet;

use genco::quote;

use crate::util::TokensExt;

pub fn generate() {
    quote! {
        $(let common_lengths = [2, 3, 4])

        pub use crate::{
            $(for n in common_lengths join(, ) => vec$n),
            $(for n in common_lengths join(, ) => vec$(n)s),
            $(for n in common_lengths join(, ) => vec$(n)g),
            Simdness, Vector, ElementOfVector,
        };

        $(
            for n in common_lengths join($['\n']) =>

            $(format!("/// Creates a [`Vector<{n}, T, Simd>`] from the given elements and vectors."))
            $(match n {
                2 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2, Vec2};")
                    $("///")
                    $("/// assert_eq!(vec2!(1, 2), Vec2::from_array([1, 2]));")
                    $("/// ```")
                }
                3 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2, vec3, Vec3};")
                    $("///")
                    $("/// assert_eq!(vec3!(1, 2, 3), Vec3::from_array([1, 2, 3]));")
                    $("/// assert_eq!(vec3!(1, vec2!(2, 3)), Vec3::from_array([1, 2, 3]));")
                    $("/// ```")
                }
                4 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2, vec4, Vec4};")
                    $("///")
                    $("/// assert_eq!(vec4!(1, 2, 3, 4), Vec4::from_array([1, 2, 3, 4]));")
                    $("/// assert_eq!(vec4!(1, vec2!(2, 3), 4), Vec4::from_array([1, 2, 3, 4]));")
                    $("/// ```")
                }
                _ => {}
            })
            #[macro_export]
            macro_rules! vec$n {
                ($$($$x:expr),* $$(,)?) => {
                    $$crate::Vector::<$n, _, $$crate::Simd>::from(($$($$x,)*))
                }
            }
        )

        $(
            for n in common_lengths join($['\n']) =>

            $(format!("/// Creates a [`Vector<{n}, T, NonSimd>`] from the given elements and vectors."))
            $(match n {
                2 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2s, Vec2S};")
                    $("///")
                    $("/// assert_eq!(vec2s!(1, 2), Vec2S::from_array([1, 2]));")
                    $("/// ```")
                }
                3 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2s, vec3s, Vec3S};")
                    $("///")
                    $("/// assert_eq!(vec3s!(1, 2, 3), Vec3S::from_array([1, 2, 3]));")
                    $("/// assert_eq!(vec3s!(1, vec2s!(2, 3)), Vec3S::from_array([1, 2, 3]));")
                    $("/// ```")
                }
                4 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2s, vec4s, Vec4S};")
                    $("///")
                    $("/// assert_eq!(vec4s!(1, 2, 3, 4), Vec4S::from_array([1, 2, 3, 4]));")
                    $("/// assert_eq!(vec4s!(1, vec2s!(2, 3), 4), Vec4S::from_array([1, 2, 3, 4]));")
                    $("/// ```")
                }
                _ => {}
            })
            #[macro_export]
            macro_rules! vec$(n)s {
                ($$($$x:expr),* $$(,)?) => {
                    $$crate::Vector::<$n, _, $$crate::NonSimd>::from(($$($$x,)*))
                }
            }
        )

        $(
            for n in common_lengths join($['\n']) =>

            $(format!("/// Creates a [`Vector<{n}, T, S>`] from the given elements and vectors,"))
            $("/// and uses type-inference to determine if the vector is [`Simd`] or [`NonSimd`].")
            $(match n {
                2 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2g, Vec2};")
                    $("///")
                    $("/// assert_eq!(vec2!(1, 2), Vec2::from_array([1, 2]));")
                    $("/// ```")
                }
                3 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2g, vec3g, Vec3, Vec3S};")
                    $("///")
                    $("/// assert_eq!(vec3g!(1, 2, 3), Vec3::from_array([1, 2, 3]));")
                    $("/// assert_eq!(vec3g!(1, vec2g!(2, 3)), Vec3S::from_array([1, 2, 3]));")
                    $("/// ```")
                }
                4 => {
                    $("///")
                    $("/// Example")
                    $("///")
                    $("/// ```")
                    $("/// use ggmath::{vec2g, vec4g, Vec4, Vec4S};")
                    $("///")
                    $("/// assert_eq!(vec4g!(1, 2, 3, 4), Vec4::from_array([1, 2, 3, 4]));")
                    $("/// assert_eq!(vec4g!(1, vec2g!(2, 3), 4), Vec4S::from_array([1, 2, 3, 4]));")
                    $("/// ```")
                }
                _ => {}
            })
            #[macro_export]
            macro_rules! vec$(n)g {
                ($$($$x:expr),* $$(,)?) => {
                    $$crate::Vector::<$n, _, _>::from(($$($$x,)*))
                }
            }
        )

        $(
            for n in common_lengths join($['\n']) => $(
                for split in Part::splits(n) join($['\n']) =>

                $(let t_bounds = {
                    let mut lengths =
                        split
                        .iter()
                        .filter(|part| matches!(part, Part::Vector { n: _ }))
                        .map(|part| part.len())
                        .collect::<HashSet<usize>>();

                    lengths.insert(n);

                    let mut lengths = lengths.into_iter().collect::<Vec<usize>>();
                    lengths.sort();

                    &lengths.iter().map(|n| format!("ElementOfVector<{n}, S>")).collect::<Vec<String>>().join(" + ")
                })

                $(let tuple_type = {
                    let fields =
                        split
                        .iter()
                        .map(|part| match part {
                            Part::Element => "T".to_string(),
                            Part::Vector { n } => format!("Vector<{n}, T, S>"),
                        })
                        .collect::<Vec<String>>()
                        .join(", ");

                    &format!("({fields})")
                })

                $(let array_items = {
                    let mut items = Vec::new();

                    for (part_idx, part) in split.iter().enumerate() {
                        match *part {
                            Part::Element => items.push(format!("value.{part_idx}")),
                            Part::Vector { n } => for i in 0..n {
                                items.push(format!("value.{part_idx}.get_const::<{i}>()"))
                            },
                        }
                    }

                    &items.join(", ")
                })

                impl<T: $t_bounds, S: Simdness> From<$tuple_type> for Vector<$n, T, S> {
                    #[inline(always)]
                    fn from(value: $tuple_type) -> Self {
                        Vector::from_array([$array_items])
                    }
                }
            )
        )

        impl<const N: usize, T: ElementOfVector<N, S>, S: Simdness> From<(Vector<N, T, S>,)> for Vector<N, T, S> {
            #[inline(always)]
            fn from(value: (Vector<N, T, S>,)) -> Self {
                value.0
            }
        }
    }
    .write_in_src("vector/constructor.rs");
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Part {
    Element,
    Vector { n: usize },
}

impl Part {
    fn splits(n: usize) -> Vec<Vec<Self>> {
        fn go_over(parts: &[Part], n: usize, result: &mut Vec<Vec<Part>>) {
            let length_so_far = parts.iter().map(|part| part.len()).sum::<usize>();

            if length_so_far == n {
                if parts.len() >= 2 {
                    result.push(parts.to_vec());
                }
            } else {
                let mut parts = parts.to_vec();

                parts.push(Part::Element);
                go_over(&parts, n, result);
                parts.pop();

                for n2 in 1..=n - length_so_far {
                    parts.push(Part::Vector { n: n2 });
                    go_over(&parts, n, result);
                    parts.pop();
                }
            }
        }

        let mut result = Vec::new();

        go_over(&[], n, &mut result);

        result
    }

    fn len(&self) -> usize {
        match self {
            Part::Element => 1,
            Part::Vector { n } => *n,
        }
    }
}
