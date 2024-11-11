use std::ops::*;

ggmath_proc_macros::self_ops!(
    $(ggmath_proc_macros::vec_interface!(
        impl $std_trait:

        $scalar_trait: Scalar + $std_trait<Output: Scalar>,

        type Output = Vector<N, <T as $std_trait>::Output, A>;

        fn $std_fn(self) -> Vector<N, <T as $std_trait>::Output, A> {
            Vector::from_array(self.into_array().map(|c| c.$std_fn()))
        }
    );)*
);
ggmath_proc_macros::rhs_ops!(
    $(ggmath_proc_macros::vec_interface!(
        impl<Rhs: Scalar> $std_trait<Vector<N, Rhs, A>>:

        $scalar_trait: Scalar + $std_trait<Rhs, Output: Scalar>,

        type Output = Vector<N, <T as $std_trait<Rhs>>::Output, A>;

        fn $std_fn(self, rhs: Vector<N, Rhs, T>) -> Vector<N, <T as $std_trait<Rhs>>::Output, A> {
            Vector::from_array(self.into_array().map(|c| c.$std_fn()))
        }
    );)*
);
