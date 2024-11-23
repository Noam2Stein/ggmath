use super::*;

use std::{array, cmp::Ordering};

// PartialEq, Eq

pub trait ScalarPartialEq<Rhs: Scalar = Self>: Scalar + PartialEq<Rhs> {
    #[inline(always)]
    fn vector_eq<const N: usize, A: VecAlignment>(
        vec: &Vector<N, Self, A>,
        other: &Vector<N, Rhs, impl VecAlignment>,
    ) -> bool
    where
        ScalarCount<N>: VecLen,
    {
        (0..N).all(|i| vec[i].eq(&other[i]))
    }
}

impl<
        const N: usize,
        T: ScalarPartialEq<TRhs>,
        A: VecAlignment,
        TRhs: Scalar,
        ARhs: VecAlignment,
    > PartialEq<Vector<N, TRhs, ARhs>> for Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Vector<N, TRhs, ARhs>) -> bool {
        T::vector_eq(&self, other)
    }
}

impl<const N: usize, T: ScalarPartialEq<T> + Eq, A: VecAlignment> Eq for Vector<N, T, A> where
    ScalarCount<N>: VecLen
{
}

// PartialOrd, Ord

pub trait ScalarPartialOrd: ScalarPartialEq + PartialOrd {
    #[inline(always)]
    fn min(self, other: Self) -> Self {
        if self > other {
            other
        } else {
            self
        }
    }
    #[inline(always)]
    fn max(self, other: Self) -> Self {
        if self > other {
            self
        } else {
            other
        }
    }
    #[inline(always)]
    fn clamp(self, min: Self, max: Self) -> Self {
        if self > max {
            max
        } else if self < min {
            min
        } else {
            self
        }
    }

    fn vector_cmin<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        vec.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(vec[0])
    }
    fn vector_cmax<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        vec.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(vec[0])
    }

    fn vector_min<const N: usize, A: VecAlignment>(
        self,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_array(array::from_fn(|i| match self[i].partial_cmp(&other[i]) {
            None => self[i],
            Some(Ordering::Less) => self[i],
            Some(Ordering::Equal) => self[i],
            Some(Ordering::Greater) => other[i],
        }))
    }
    fn vector_max<const N: usize, A: VecAlignment>(
        self,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_array(array::from_fn(|i| match self[i].partial_cmp(&other[i]) {
            None => self[i],
            Some(Ordering::Less) => other[i],
            Some(Ordering::Equal) => self[i],
            Some(Ordering::Greater) => self[i],
        }))
    }
    fn vector_clamp<const N: usize, A: VecAlignment>(
        self,
        min: Vector<N, Self, impl VecAlignment>,
        max: Vector<N, Self, impl VecAlignment>,
    ) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        self.max(min).min(max)
    }
}

impl<const N: usize, T: ScalarPartialOrd, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    pub fn cmin(self) -> T {
        T::vector_cmin(self)
    }

    pub fn cmax(self) -> T {
        T::vector_cmax(self)
    }

    pub fn min(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_min(self, other)
    }

    pub fn max(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_max(self, other)
    }

    pub fn clamp(
        self,
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        T::vector_clamp(self, min, max)
    }
}
