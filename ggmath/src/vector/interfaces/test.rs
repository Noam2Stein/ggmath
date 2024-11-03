use std::ops::*;

ggmath_proc_macros::vec_interface!(
    impl Add<Rhs: Scalar>:

    ScalarAdd: Scalar + Add<Rhs, Output: Scalar>,

    fn add(self, rhs: Vector<N, Rhs, A>) -> Vector<N, <T as Add<Rhs>>::Output, A> {

    }
);
