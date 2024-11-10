use std::ops::*;

ggmath_proc_macros::vec_interface!(
    impl<Rhs: Scalar> Add<Vector<N, Rhs, A>>:

    ScalarAdd: Scalar + Add<Rhs, Output: Scalar>,

    type Output = Vector<N, <T as Add<Rhs>>::Output, A>;

    fn add(self, rhs: Vector<N, Rhs, A>) -> Self::Output {
        for index in 0..N {
            self.set(index, self.get(index).unwrap().add(rhs.get(index)))
        }
    }
);
