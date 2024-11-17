use std::{array, cmp::Ordering};

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
    ScalarPartialOrd: ScalarPartialEq<T> + PartialOrd<T>;

    pub impl:

    fn min(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        Vector::from_array(array::from_fn(|i| match self[i].partial_cmp(&other[i]) {
            None => self[i],
            Some(Ordering::Less) => self[i],
            Some(Ordering::Equal) => self[i],
            Some(Ordering::Greater) => other[i],
        }))
    }
    fn max(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        Vector::from_array(array::from_fn(|i| match self[i].partial_cmp(&other[i]) {
            None => self[i],
            Some(Ordering::Less) => other[i],
            Some(Ordering::Equal) => self[i],
            Some(Ordering::Greater) => self[i],
        }))
    }
    fn clamp(self, min: Vector<N, T, impl VecAlignment>, max: Vector<N, T, impl VecAlignment>) -> Self {
        self.max(min).min(max)
    }
);
