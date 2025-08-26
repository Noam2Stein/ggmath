use super::*;

pub fn write_mod(mut module: Mod, scalar_trait: &mut ScalarTrait) {
    writedoc! {
        module,
        r#"
        use std::hash::{{Hash, Hasher}};

        use super::*;

        impl<const N: usize, T: Scalar + Hash, A: VecAlignment> Hash for Vector<N, T, A>
        where
            Usize<N>: VecLen,
        {{
            #[inline(always)]
            fn hash<H: Hasher>(&self, state: &mut H) {{
                T::vec_hash(self, state);
            }}
        }}
        "#,
    }
    .unwrap();

    scalar_trait.push_overridable_fn(
        "hash",
        formatdoc! {r#"
        #[inline(always)]
        fn vec_hash<const N: usize, A: VecAlignment, H: std::hash::Hasher>(
            vec: &Vector<N, Self, A>,
            state: &mut H,
        )
        where
            Usize<N>: VecLen,
            Self: std::hash::Hash,
        {{
            vec.as_array().hash(state);
        }}
        "#},
    );
}
