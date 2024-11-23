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
    )*
    $(ggmath_proc_macros::vector_interface!(
        $scalar_trait<Rhs: Scalar>: Scalar + $std_trait<Rhs, Output: Scalar>;

        impl<ARhs: VecAlignment> $std_trait<Vector<N, Rhs, ARhs>>:

        type Output = Vector<N, <T as $std_trait<Rhs>>::Output, A>;

        fn $std_fn(self, rhs: Vector<N, Rhs, ARhs>) -> Vector<N, <T as $std_trait<Rhs>>::Output, A> {
            Vector::from_fn(|i| self[i].$std_fn(rhs[i]))
        }
    );)*
);
for_assign_ops!(
    $(ggmath_proc_macros::vector_interface!(
        $scalar_trait<Rhs: Scalar>: Scalar + $std_trait<Rhs>;

        impl<ARhs: VecAlignment> $std_trait<Vector<N, Rhs, ARhs>>:

        fn $std_fn(&mut self, rhs: Vector<N, Rhs, ARhs>) {
            for i in 0..N {
                self[i].$std_fn(rhs[i])
            }
        }
    );)*
);
