use super::*;

pub fn write_mod(mut module: Mod, scalar_trait: &mut ScalarTrait) {
    let mask_fns = ["eq", "ne", "lt", "le", "gt", "ge"]
        .iter()
        .zip([
            "PartialEq",
            "PartialEq",
            "PartialOrd",
            "PartialOrd",
            "PartialOrd",
            "PartialOrd",
        ])
        .map(|(op, op_trait)| {
            scalar_trait.push_overridable_fn(
                op,
                formatdoc! {r#"
                #[inline(always)]
                fn vec_{op}_mask<const N: usize, A: VecAlignment, T2: Scalar>(
                    vec: &Vector<N, Self, A>,
                    rhs: &Vector<N, T2, impl VecAlignment>,
                ) -> Vector<N, bool, A>
                where
                    Usize<N>: VecLen,
                    Self: {op_trait}<T2>,
                {{
                    Vector::from_fn(|i| vec[i].{op}(rhs[i]))
                }}
            "#},
            );

            formatdoc! {r#"
                #[inline(always)]
                fn {op}_mask<T2: Scalar>(
                    &self,
                    rhs: &Vector<N, T2, impl VecAlignment>,
                ) -> Vector<N, bool, A>
                where
                    T: {op_trait}<T2>,
                {{
                    T::vec_{op}_mask(self, rhs)
                }}
            "#}
        })
        .collect::<Vec<_>>()
        .join("\n");

    writedoc! {
        module,
        r#"
        use super::*;
        
        impl<const N: usize, T: Scalar, A: VecAlignment, T2: Scalar, A2: VecAlignment>
            PartialEq<Vector<N, T2, A2>> for Vector<N, T, A>
        where
            Usize<N>: VecLen,
            T: PartialEq<T2>,
        {{
            #[inline(always)]
            fn eq(&self, other: &Vector<N, T2, A2>) -> bool {{
                T::vec_eq(self, other)
            }}

            #[inline(always)]
            fn ne(&self, other: &Vector<N, T2, A2>) -> bool {{
                T::vec_ne(self, other)
            }}
        }}

        impl<const N: usize, T: Scalar + Eq, A: VecAlignment> Eq for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
        }}

        impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            {mask_fns}
        }}
        "#
    }
    .unwrap();

    scalar_trait.push_overridable_fn(
        "eq",
        formatdoc! {r#"
        #[inline(always)]
        fn vec_eq<const N: usize, A: VecAlignment, T2: Scalar>(
            vec: &Vector<N, Self, A>,
            rhs: &Vector<N, T2, impl VecAlignment>,
        ) -> bool
        where
            Usize<N>: VecLen,
            Self: PartialEq<T2>,
        {{
            Self::vec_eq_mask(vec, rhs).into_iter().all(|x| x)
        }}
        "#},
    );

    scalar_trait.push_overridable_fn(
        "ne",
        formatdoc! {r#"
        #[inline(always)]
        fn vec_ne<const N: usize, A: VecAlignment, T2: Scalar>(
            vec: &Vector<N, Self, A>,
            rhs: &Vector<N, T2, impl VecAlignment>,
        ) -> bool
        where
            Usize<N>: VecLen,
            Self: PartialEq<T2>,
        {{
            !Self::vec_eq(vec, rhs)
        }}
        "#},
    );
}
