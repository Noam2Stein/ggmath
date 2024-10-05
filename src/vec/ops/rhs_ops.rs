use gomath_proc_macros::{ops_macro, rhs_ops};

use super::*;

ops_macro!(
    declare_rhs_ops {
        $(
            pub trait $vec_trait<const N: usize, Rhs: Element>: Element + $std_trait<Rhs, Output: Element>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn $vec_fn(vec: VecN<N, Self>, rhs: VecN<N, Rhs>) -> VecN<N, <Self as $std_trait<Rhs>>::Output>;
            }

            impl<const N: usize, T: $element_trait<Rhs>, Rhs: Element> $std_trait<VecN<N, Rhs>> for VecN<N, T> where MaybeVecNum<N>: VecNum<N> {
                type Output = VecN<N, T::Output>;
                fn $std_fn(self, rhs: VecN<N, Rhs>) -> Self::Output {
                    MaybeVecNum::<N>::$vec_fn(self, rhs)
                }
            }

            trait $vecnum_trait<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: VecN<N, T>, rhs: VecN<N, Rhs>) -> VecN<N, <T as $std_trait<Rhs>>::Output>;
            }
            impl $vecnum_trait<2> for MaybeVecNum<2> {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: VecN<2, T>, rhs: VecN<2, Rhs>) -> VecN<2, <T as $std_trait<Rhs>>::Output> {
                    T::$vec_fn(vec, rhs)
                }
            }
            impl $vecnum_trait<3> for MaybeVecNum<3> {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: VecN<3, T>, rhs: VecN<3, Rhs>) -> VecN<3, <T as $std_trait<Rhs>>::Output> {
                    T::$vec_fn(vec, rhs)
                }
            }
            impl $vecnum_trait<4> for MaybeVecNum<4> {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: VecN<4, T>, rhs: VecN<4, Rhs>) -> VecN<4, <T as $std_trait<Rhs>>::Output> {
                    T::$vec_fn(vec, rhs)
                }
            }
        )*

        pub(super) trait VecNumRhsOps<const N: usize>: $($vecnum_trait<N> + )*
        where
            MaybeVecNum<N>: VecNum<N>,
        {
        }
        impl VecNumRhsOps<2> for MaybeVecNum<2> {}
        impl VecNumRhsOps<3> for MaybeVecNum<3> {}
        impl VecNumRhsOps<4> for MaybeVecNum<4> {}
    }
);
rhs_ops!(declare_rhs_ops);
