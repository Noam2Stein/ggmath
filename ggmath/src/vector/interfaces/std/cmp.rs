use std::cmp::Ordering;

ggmath_proc_macros::vec_interface!(
    ScalarPartialEq<Rhs: Scalar>: Scalar + PartialEq<Rhs>;

    impl PartialEq<Vector<N, Rhs, A>>:

    fn eq(&self, rhs: &Vector<N, Rhs, A>) -> bool {
        (0..N).all(|i| self[i].eq(&rhs[i]))
    }
);

impl<const N: usize, T: ScalarPartialEq<T> + Eq, A: VecAlignment> Eq for Vector<N, T, A> where
    ScalarCount<N>: VecLen<N>
{
}
