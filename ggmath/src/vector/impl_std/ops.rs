use std::ops::*;

use super::*;

use ggmath_proc_macros::{for_assign_ops, for_rhs_ops, for_self_ops};

for_self_ops!($(
    impl<const N: usize, T: Scalar + $std_trait<Output: Scalar>, A: VecAlignment>
        $std_trait for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen
    {
        type Output = Vector<N, T::Output, A>;

        fn $std_fn(self) -> Vector<N, <T as $std_trait>::Output, A> {
            T::$scalar_fn(self)
        }
    }
)*);

for_rhs_ops!($(
    impl<const N: usize, T: Scalar + $std_trait<TRhs, Output: Scalar>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        $std_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen,
    {
        type Output = Vector<N, T::Output, A>;

        fn $std_fn(self, rhs: Vector<N, TRhs, ARhs>) -> Vector<N, T::Output, A> {
            T::$scalar_fn(self, rhs)
        }
    }
)*);

for_assign_ops!($(
    impl<const N: usize, T: Scalar + $std_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        $std_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen,
    {
        fn $std_fn(&mut self, rhs: Vector<N, TRhs, ARhs>) {
            T::$scalar_fn(self, rhs)
        }
    }
)*);
