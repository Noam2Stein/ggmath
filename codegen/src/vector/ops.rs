use super::*;

pub fn write_mod(mut module: Mod, scalar_trait: &mut ScalarTrait) {
    let mut impls = Vec::new();

    for op_trait in ["Neg", "Not"] {
        let op_fn = op_trait.to_lowercase();

        impls.push(formatdoc! {r#"
            impl<const N: usize, T: Scalar + {op_trait}<Output: Scalar>, A: VecAlignment> {op_trait} for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self) -> Vector<N, T::Output, A> {{
                    T::vec_{op_fn}(self)
                }}
            }}
        "#});

        scalar_trait.push_overridable_fn(
            &op_fn,
            formatdoc! {r#"
                #[inline(always)]
                fn vec_{op_fn}<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Vector<N, Self::Output, A>
                where
                    Usize<N>: VecLen,
                    Self: {op_trait}<Output: Scalar>,
                {{
                    Vector::from_array(vec.to_array().map(|x| x.{op_fn}()))
                }}
            "#},
        );
    }

    for op_trait in [
        "Add", "Sub", "Mul", "Div", "Rem", "Shl", "Shr", "BitAnd", "BitOr", "BitXor",
    ] {
        let op_fn = op_trait.to_lowercase();

        impls.push(formatdoc! {r#"
            impl<
                const N: usize,
                T: Scalar + {op_trait}<T2, Output: Scalar>,
                A: VecAlignment,
                T2: Scalar,
                A2: VecAlignment,
            > {op_trait}<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: Vector<N, T2, A2>) -> Self::Output {{
                    T::vec_{op_fn}(self, rhs)
                }}
            }}
        "#});

        impls.push(formatdoc! {r#"
            impl<
                const N: usize,
                T: Scalar + {op_trait}Assign<T2>,
                A: VecAlignment,
                T2: Scalar,
                A2: VecAlignment,
            >
                {op_trait}Assign<Vector<N, T2, A2>> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: Vector<N, T2, A2>) {{
                    T::vec_{op_fn}_assign(self, rhs)
                }}
            }}
        "#});

        scalar_trait.push_overridable_fn(
            &op_fn,
            formatdoc! {r#"
                #[inline(always)]
                fn vec_{op_fn}<const N: usize, A: VecAlignment, T2: Scalar>(
                    vec: Vector<N, Self, A>,
                    rhs: Vector<N, T2, impl VecAlignment>,
                ) -> Vector<N, Self::Output, A>
                where
                    Usize<N>: VecLen,
                    Self: {op_trait}<T2, Output: Scalar>,
                {{
                    Vector::from_fn(|i| vec[i].{op_fn}(rhs[i]))
                }}
            "#},
        );

        scalar_trait.push_overridable_fn(
            &format!("{op_fn}_assign"),
            formatdoc! {r#"
                #[inline(always)]
                fn vec_{op_fn}_assign<const N: usize, A: VecAlignment, T2: Scalar>(
                    vec: &mut Vector<N, Self, A>,
                    rhs: Vector<N, T2, impl VecAlignment>,
                )
                where
                    Usize<N>: VecLen,
                    Self: {op_trait}Assign<T2>,
                {{
                    for i in 0..N {{
                        vec[i].{op_fn}_assign(rhs[i]);
                    }}
                }}
            "#},
        );
    }

    for op_trait in ["Mul", "Div", "Rem"] {
        let op_fn = op_trait.to_lowercase();

        impls.push(formatdoc! {r#"
            impl<
                const N: usize,
                T: Scalar + {op_trait}<T2, Output: Scalar>,
                A: VecAlignment,
                T2: Scalar,
            > {op_trait}<T2> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                type Output = Vector<N, T::Output, A>;

                #[inline(always)]
                fn {op_fn}(self, rhs: T2) -> Self::Output {{
                    T::vec_scalar_{op_fn}(self, rhs)
                }}
            }}
        "#});

        impls.push(formatdoc! {r#"
            impl<
                const N: usize,
                T: Scalar + {op_trait}Assign<T2>,
                A: VecAlignment,
                T2: Scalar,
            > {op_trait}Assign<T2> for Vector<N, T, A>
            where
                Usize<N>: VecLen,
            {{
                #[inline(always)]
                fn {op_fn}_assign(&mut self, rhs: T2) {{
                    T::vec_scalar_{op_fn}_assign(self, rhs)
                }}
            }}
        "#});

        scalar_trait.push_overridable_fn(
            &op_fn,
            formatdoc! {r#"
                #[inline(always)]
                fn vec_scalar_{op_fn}<const N: usize, A: VecAlignment, T2: Scalar>(
                    vec: Vector<N, Self, A>,
                    rhs: T2,
                ) -> Vector<N, Self::Output, A>
                where
                    Usize<N>: VecLen,
                    Self: {op_trait}<T2, Output: Scalar>,
                {{
                    Vector::from_fn(|i| vec[i].{op_fn}(rhs))
                }}
            "#},
        );

        scalar_trait.push_overridable_fn(
            &format!("{op_fn}_assign"),
            formatdoc! {r#"
                #[inline(always)]
                fn vec_scalar_{op_fn}_assign<const N: usize, A: VecAlignment, T2: Scalar>(
                    vec: &mut Vector<N, Self, A>,
                    rhs: T2,
                )
                where
                    Usize<N>: VecLen,
                    Self: {op_trait}Assign<T2>,
                {{
                    for i in 0..N {{
                        vec[i].{op_fn}_assign(rhs);
                    }}
                }}
            "#},
        );
    }

    let impls = impls.join("\n");

    writedoc!(
        module,
        r#"
        use super::*;
        
        {impls}
        "#
    )
    .unwrap();
}
