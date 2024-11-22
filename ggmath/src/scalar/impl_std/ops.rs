use std::ops::*;

use super::*;

use ggmath_proc_macros::{for_assign_ops, for_rhs_ops, for_self_ops};

for_self_ops!(
    $(ggmath_proc_macros::vector_interface!(
        $scalar_trait: Scalar + $std_trait<Output: Scalar>;

        impl $std_trait:

        type Output = Vector<N, <T as $std_trait>::Output, A>;

        fn $std_fn(self) -> Vector<N, <T as $std_trait>::Output, A> {
            Vector::from_array(self.into_array().map(|c| c.$std_fn()))
        }
    );)*
);
for_rhs_ops!(
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
