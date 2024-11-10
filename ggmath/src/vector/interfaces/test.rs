use std::ops::*;

ggmath_proc_macros::vec_interface!(
    impl<Rhs: Scalar> Add<Vector<N, Rhs, A>>:

    ScalarAdd: Scalar + Add<Rhs, Output: Scalar>,

    type Output = Vector<N, <T as Add<Rhs>>::Output, A>;

    fn add(mut self, rhs: Vector<N, Rhs, A>) -> <Self as Add<Vector<N, Rhs, A>>>::Output {
        (0..N).map(|index| self.get(index).unwrap().add(rhs.get(index).unwrap()))
    }
);
