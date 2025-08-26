use super::*;

pub fn write_mod(mut module: Mod) {
    let free_splat_fns = LENGTHS
        .map(|len| {
            formatdoc! {r#"
            /// Creates a `Vec{len}` where all components are given the same value.
            #[inline(always)]
            pub const fn splat{len}<T: Scalar>(value: T) -> Vec{len}<T> {{
                Vector::splat(value)
            }}

            /// Creates a `Vec{len}P` where all components are given the same value.
            #[inline(always)]
            pub const fn splat{len}p<T: Scalar>(value: T) -> Vec{len}P<T> {{
                Vector::splat(value)
            }}

            /// Creates a `Vector<{len}, _, _>` where all components are given the same value.
            #[inline(always)]
            pub const fn splat{len}g<T: Scalar, A: VecAlignment>(value: T) -> Vector<{len}, T, A> {{
                Vector::splat(value)
            }}
        "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    writedoc!(
        module,
        r#"
            use super::*;
            
            {free_splat_fns}

            impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                /// Creates a vector where all components are given the same value.
                #[inline(always)]
                pub const fn splat(value: T) -> Self {{
                    Self::from_array([value; N])
                }}
            }}
        "#,
        free_splat_fns = free_splat_fns.trim(),
    )
    .unwrap();
}
