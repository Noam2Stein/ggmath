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

impl<const N: usize, T: ScalarPartialEq + Eq, A: VecAlignment> Eq for Vector<N, T, A> where
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

    #[inline(always)]
    fn vector_cmin<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        vec.iter()
            .min_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(vec[0])
    }
    #[inline(always)]
    fn vector_cmax<const N: usize, A: VecAlignment>(vec: Vector<N, Self, A>) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        vec.iter()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal))
            .unwrap_or(vec[0])
    }

    #[inline(always)]
    fn vector_min<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_fn(|i| match vec[i].partial_cmp(&other[i]) {
            None => vec[i],
            Some(Ordering::Less) => vec[i],
            Some(Ordering::Equal) => vec[i],
            Some(Ordering::Greater) => other[i],
        })
    }
    #[inline(always)]
    fn vector_max<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        other: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        Vector::from_array(array::from_fn(|i| match vec[i].partial_cmp(&other[i]) {
            None => vec[i],
            Some(Ordering::Less) => other[i],
            Some(Ordering::Equal) => vec[i],
            Some(Ordering::Greater) => vec[i],
        }))
    }
    #[inline(always)]
    fn vector_clamp<const N: usize, A: VecAlignment>(
        vec: Vector<N, Self, A>,
        min: Vector<N, Self, impl VecAlignment>,
        max: Vector<N, Self, impl VecAlignment>,
    ) -> Vector<N, Self, A>
    where
        ScalarCount<N>: VecLen,
    {
        vec.max(min).min(max)
    }
}

impl<const N: usize, T: ScalarPartialOrd, A: VecAlignment> Vector<N, T, A>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    pub fn cmin(self) -> T {
        T::vector_cmin(self)
    }

    #[inline(always)]
    pub fn cmax(self) -> T {
        T::vector_cmax(self)
    }

    #[inline(always)]
    pub fn min(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_min(self, other)
    }

    #[inline(always)]
    pub fn max(self, other: Vector<N, T, impl VecAlignment>) -> Self {
        T::vector_max(self, other)
    }

    #[inline(always)]
    pub fn clamp(
        self,
        min: Vector<N, T, impl VecAlignment>,
        max: Vector<N, T, impl VecAlignment>,
    ) -> Self {
        T::vector_clamp(self, min, max)
    }
}
