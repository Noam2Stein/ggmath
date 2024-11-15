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

ggmath_proc_macros::vec_interface!(
    ScalarPartialOrd<Rhs: Scalar>: ScalarPartialEq<Rhs> + PartialOrd<Rhs>;

    impl PartialOrd<Vector<N, Rhs, A>>:

    fn partial_cmp(&self, rhs: &Vector<N, Rhs, A>) -> Option<Ordering> {
        for i in 0..N {
            match self[i].partial_cmp(&rhs[i]) {
                Some(Ordering::Equal) => {},
                ordering => return ordering,
            }
        }

        Some(Ordering::Equal)
    }
);

ggmath_proc_macros::vec_interface!(
    ScalarOrd: ScalarPartialEq<T> + Eq + ScalarPartialOrd<T> + Ord;

    impl Ord:

    fn cmp(&self, other: &Self) -> Ordering {
        for i in 0..N {
            match self[i].cmp(&other[i]) {
                Ordering::Equal => {},
                ordering => return ordering,
            }
        }

        Ordering::Equal
    }
);
