use std::ops::*;

use super::*;

use ggmath_proc_macros::{for_assign_ops, for_rhs_ops, for_self_ops};

for_self_ops!($(
    pub trait $scalar_trait: Scalar + $std_trait<Output: Scalar> {
        fn $scalar_fn<const N: usize, A: VecAlignment>
            (vec: Vector<N, Self, A>) -> Vector<N, Self::Output, A>
        where
            ScalarCount<N>: VecLen,
        {
            vec.map(|c| c.$std_fn())
        }
    }

    impl<const N: usize, T: $scalar_trait, A: VecAlignment>
        $std_trait for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen
    {
        type Output = Vector<N, <T as $std_trait>::Output, A>;

        fn $std_fn(self) -> Vector<N, <T as $std_trait>::Output, A> {
            T::$scalar_fn(self)
        }
    }
)*);

for_rhs_ops!($(
    pub trait $scalar_trait<Rhs: Scalar>: Scalar + $std_trait<Rhs, Output: Scalar> {
        fn $scalar_fn<const N: usize, A: VecAlignment>
            (vec: Vector<N, Self, A>, rhs: Vector<N, Rhs, impl VecAlignment>) -> Vector<N, Self::Output, A>
        where
            ScalarCount<N>: VecLen,
        {
            Vector::from_fn(|i| vec[i].$std_fn(rhs[i]))
        }
    }

    impl<const N: usize, T: $scalar_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
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
    pub trait $scalar_trait<Rhs: Scalar>: Scalar + $std_trait<Rhs> {
        fn $scalar_fn<const N: usize, A: VecAlignment>
            (vec: &mut Vector<N, Self, A>, rhs: Vector<N, Rhs, impl VecAlignment>)
        where
            ScalarCount<N>: VecLen,
        {
            for i in 0..N {
                vec[i].$std_fn(rhs[i])
            }
        }
    }

    impl<const N: usize, T: $scalar_trait<TRhs>, A: VecAlignment, TRhs: Scalar, ARhs: VecAlignment>
        $std_trait<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
    where
        ScalarCount<N>: VecLen,
    {
        fn $std_fn(&mut self, rhs: Vector<N, TRhs, ARhs>) {
            T::$scalar_fn(self, rhs)
        }
    }
)*);
