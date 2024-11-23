use super::*;

use std::{array, cmp::Ordering};

ggmath_proc_macros::vector_interface!(
    ScalarPartialEq<Rhs: Scalar>: Scalar + PartialEq<Rhs>;

    impl<ARhs: VecAlignment> PartialEq<Vector<N, Rhs, ARhs>>:

    fn eq(&self, rhs: &Vector<N, Rhs, ARhs>) -> bool {
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

    fn cmin(self) -> T {
        self.iter().min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(self[0])
    }
    fn cmax(self) -> T {
        self.iter().max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal)).unwrap_or(self[0])
    }
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
