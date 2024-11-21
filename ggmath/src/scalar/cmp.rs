use super::*;

use std::{array, cmp::Ordering};

ggmath_proc_macros::vector_interface!(
    ScalarPartialEq<Rhs: Scalar>: Scalar + PartialEq<Rhs>;

    impl PartialEq<Vector<N, Rhs, A>>:

    fn eq(&self, rhs: &Vector<N, Rhs, A>) -> bool {
        (0..N).all(|i| self[i].eq(&rhs[i]))
    }
);

impl<const N: usize, T: ScalarPartialEq<T> + Eq, A: VecAlignment> Eq for Vector<N, T, A> where
    ScalarCount<N>: VecLen
{
}

ggmath_proc_macros::vector_interface!(
    ScalarPartialOrd: ScalarPartialEq<T> + PartialOrd<T>;

    pub impl:

    fn min<MinA: VecAlignment>(self, other: Vector<N, T, MinA>) -> Self {
        Vector::from_array(array::from_fn(|i| match self[i].partial_cmp(&other[i]) {
            None => self[i],
            Some(Ordering::Less) => self[i],
            Some(Ordering::Equal) => self[i],
            Some(Ordering::Greater) => other[i],
        }))
    }
    fn max<MaxA: VecAlignment>(self, other: Vector<N, T, MaxA>) -> Self {
        Vector::from_array(array::from_fn(|i| match self[i].partial_cmp(&other[i]) {
            None => self[i],
            Some(Ordering::Less) => other[i],
            Some(Ordering::Equal) => self[i],
            Some(Ordering::Greater) => self[i],
        }))
    }
    fn clamp<MinA: VecAlignment, MaxA: VecAlignment>(self, min: Vector<N, T, MinA>, max: Vector<N, T, MaxA>) -> Self {
        self.max(min).min(max)
    }
);
