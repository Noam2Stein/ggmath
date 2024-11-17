use crate::AbsDiff;

ggmath_proc_macros::vec_interface!(
    ScalarAbsDiff<Rhs: Scalar>: Scalar + AbsDiff<Rhs, Output: Scalar>;
    impl AbsDiff<Vector<N, Rhs, A>>:

    type Output = Vector<N, T::Output, A>;

    fn abs_diff(&self, rhs: &Vector<N, Rhs, A>) -> Vector<N, T::Output, A> {
        Vector::from_fn(|i| self[i].abs_diff(&rhs[i]))
    }
);
