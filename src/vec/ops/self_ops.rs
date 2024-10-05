use gomath_proc_macros::{ops_macro, self_ops};

use super::*;

ops_macro!(
    declare_rhs_ops {
        $(
            pub trait $vec_trait<const N: usize>: Element + $std_trait<Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn $vec_fn(vec: VecN<N, Self>) -> VecN<N, <Self as $std_trait>::Output>;
            }

            trait $vecnum_trait<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn $std_fn<T: $element_trait>(vec: VecN<N, T>) -> VecN<N, <T as $std_trait>::Output>;
            }
            impl $vecnum_trait<2> for MaybeVecNum<2> {
                fn $std_fn<T: $element_trait>(vec: VecN<2, T>) -> VecN<2, <T as $std_trait>::Output> {
                    T::$vec_fn(vec)
                }
            }
            impl $vecnum_trait<3> for MaybeVecNum<3> {
                fn $std_fn<T: $element_trait>(vec: VecN<3, T>) -> VecN<3, <T as $std_trait>::Output> {
                    T::$vec_fn(vec)
                }
            }
            impl $vecnum_trait<4> for MaybeVecNum<4> {
                fn $std_fn<T: $element_trait>(vec: VecN<4, T>) -> VecN<4, <T as $std_trait>::Output> {
                    T::$vec_fn(vec)
                }
            }
        )*

        pub(super) trait VecNumSelfOps<const N: usize>: $($vecnum_trait<N> + )*
        where
            MaybeVecNum<N>: VecNum<N>,
        {
        }
        impl VecNumSelfOps<2> for MaybeVecNum<2> {}
        impl VecNumSelfOps<3> for MaybeVecNum<3> {}
        impl VecNumSelfOps<4> for MaybeVecNum<4> {}
    }
);
self_ops!(declare_rhs_ops);
