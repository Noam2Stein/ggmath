use gomath_proc_macros::{assign_ops, ops_macro};

use super::*;

ops_macro!(
    declare_rhs_ops {
        $(
            pub trait $vec_trait<const N: usize, Rhs: Element>: Element + $std_trait<Rhs>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn $vec_fn(vec: &mut VecN<N, Self>, rhs: VecN<N, Rhs>);
            }

            trait $vecnum_trait<const N: usize>
            where
                MaybeVecNum<N>: VecNum<N>,
            {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: &mut VecN<N, T>, rhs: VecN<N, Rhs>);
            }
            impl $vecnum_trait<2> for MaybeVecNum<2> {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: &mut VecN<2, T>, rhs: VecN<2, Rhs>) {
                    T::$vec_fn(vec, rhs)
                }
            }
            impl $vecnum_trait<3> for MaybeVecNum<3> {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: &mut VecN<3, T>, rhs: VecN<3, Rhs>) {
                    T::$vec_fn(vec, rhs)
                }
            }
            impl $vecnum_trait<4> for MaybeVecNum<4> {
                fn $std_fn<Rhs: Element, T: $element_trait<Rhs>>(vec: &mut VecN<4, T>, rhs: VecN<4, Rhs>) {
                    T::$vec_fn(vec, rhs)
                }
            }
        )*

        pub(super) trait VecNumAssignOps<const N: usize>: $($vecnum_trait<N> + )*
        where
            MaybeVecNum<N>: VecNum<N>,
        {
        }
        impl VecNumAssignOps<2> for MaybeVecNum<2> {}
        impl VecNumAssignOps<3> for MaybeVecNum<3> {}
        impl VecNumAssignOps<4> for MaybeVecNum<4> {}
    }
);
assign_ops!(declare_rhs_ops);
