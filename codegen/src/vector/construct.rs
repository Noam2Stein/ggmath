use std::iter::once;

use super::*;

pub fn write_mod(mut module: Mod) {
    let macros = LENGTHS
        .map(|len| {
            formatdoc! {r#"
            /// Constructs a new aligned vector from flexible arguments like shaders.
            ///
            /// ```
            /// use ggmath::*;
            ///
            /// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
            /// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
            /// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
            /// ```
            #[macro_export]
            macro_rules! vec{len} {{
                ($($item:expr), * $(,)?) => {{{{
                    let output: $crate::Vector<{len}, _, $crate::VecAligned>
                        = $crate::construct_vector::<{len}, _, $crate::VecAligned, _>(($($item,)*));

                    output
                }}}};
            }}

            /// Constructs a new packed vector from flexible arguments like shaders.
            ///
            /// ```
            /// use ggmath::*;
            ///
            /// const EXAMPLE_2: Vec2<f32> = vec2!(1.0, 2.0);
            /// const EXAMPLE_3: Vec3<f32> = vec3!(1.0, 2.0, 3.0);
            /// const EXAMPLE_4: Vec4<f32> = vec4!(1.0, EXAMPLE_2, 4.0);
            /// ```
            #[macro_export]
            macro_rules! vec{len}p {{
                ($($item:expr), * $(,)?) => {{{{
                    let output: $crate::Vector<{len}, _, $crate::VecPacked>
                        = $crate::construct_vector::<{len}, _, $crate::VecPacked, _>(($($item,)*));

                    output
                }}}};
            }}

            /// Constructs a new vector from flexible arguments like shaders, generic over alignment.
            ///
            /// Unlike `vec{{n}}!` and `vec{{n}}p!` macros,
            /// this macro does not decide the alignment of the vector.
            ///
            /// Instead, it is not specified and requires an inference hint.
            #[macro_export]
            macro_rules! vec{len}g {{
                ($($item:expr), * $(,)?) => {{{{
                    let output: $crate::Vector<{len}, _, _>
                        = $crate::construct_vector::<{len}, _, _, _>(($($item,)*));

                    output
                }}}};
            }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    let compositions = LENGTHS
        .map(|len| {
            compositions(len).into_iter().map(move |composition| {
                let generic_params = once("T: Scalar".to_string())
                    .chain(
                        composition
                            .iter()
                            .enumerate()
                            .map(|(i, &n)| match n {
                                0 => unreachable!(),
                                1 => None,
                                2.. => Some(format!("A{i}: VecAlignment")),
                            })
                            .flatten(),
                    )
                    .collect::<Vec<_>>()
                    .join(", ");

                let tuple_fields = composition
                    .iter()
                    .enumerate()
                    .map(|(i, &n)| match n {
                        0 => unreachable!(),
                        1 => format!("T"),
                        n => format!("Vector<{n}, T, A{i}>"),
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let sources = composition
                    .iter()
                    .enumerate()
                    .map(|(i, &n)| {
                        (0..n).map(move |j| format!("offset_of!(Self, {i}) + size_of::<T>() * {j}"))
                    })
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(", ");

                formatdoc! {r#"
                    unsafe impl<{generic_params}> IntoVector<{len}, T> for ({tuple_fields},) {{
                        const SOURCES: [usize; {len}] = [{sources}];
                    }}
                "#}
            })
        })
        .flatten()
        .collect::<Vec<_>>()
        .join("\n");

    writedoc!(
        module,
        "
        use std::{{
            mem::{{MaybeUninit, offset_of}},
            ptr::copy_nonoverlapping,
        }};

        use super::*;

        {macros}

        #[doc(hidden)]
        #[inline(always)]
        pub const fn construct_vector<const N: usize, T: Scalar, A: VecAlignment, I: IntoVector<N, T>>(
            value: I,
        ) -> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            let mut output = unsafe {{ MaybeUninit::uninit().assume_init() }};

            let value_ptr = (&value) as *const _ as *const T;
            let output_ptr = (&mut output) as *mut _ as *mut T;

            let mut i = 0;
            while i < N {{
                let src_offset = I::SOURCES[i];
                let dst_offset = size_of::<T>() * i;

                let src_ptr = unsafe {{ value_ptr.byte_add(src_offset) }};
                let dst_ptr = unsafe {{ output_ptr.byte_add(dst_offset) }};

                unsafe {{
                    copy_nonoverlapping(src_ptr, dst_ptr, 1);
                }}

                i += 1;
            }}

            output
        }}

        #[doc(hidden)]
        pub unsafe trait IntoVector<const N: usize, T: Scalar>: Construct {{
            const SOURCES: [usize; N];
        }}

        {compositions}
        "
    )
    .unwrap();
}

fn compositions(n: usize) -> Vec<Vec<usize>> {
    let mut result = Vec::new();
    let mut stack = vec![(vec![], n)];

    while let Some((prefix, remaining)) = stack.pop() {
        if remaining == 0 {
            result.push(prefix);
            continue;
        }

        for i in 1..=remaining {
            let mut new_prefix = prefix.clone();
            new_prefix.push(i);
            stack.push((new_prefix, remaining - i));
        }
    }

    result
}
