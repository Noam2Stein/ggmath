use core::ops::{Add, Mul, Neg, Sub};

use crate::{Alignment, One, Rot2, Rot2A, Rotation2, Scalar, Vector, Zero};

impl<T, A: Alignment> Rotation2<T, A>
where
    T: Scalar + Zero + One,
{
    /// TODO
    pub const IDENTITY: Self = Self::from_cos_sin(T::ONE, T::ZERO);
}

impl<T, A: Alignment> Rotation2<T, A>
where
    T: Scalar,
{
    /// TODO
    #[inline]
    #[must_use]
    pub const fn from_cos_sin(cos: T, sin: T) -> Self {
        Self(Vector::<2, T, A>::new(cos, sin))
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn conjugate(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::from_cos_sin(self.cos, -self.sin)
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perp(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(self.0.perp())
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.dot(rhs.0)
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn wedge(self, rhs: Self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.0.wedge(rhs.0)
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.length_squared()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 2]) -> Self {
        Self(Vector::from_array(array))
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 2] {
        self.0.to_array()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 2] {
        self.0.as_array()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 2] {
        self.0.as_mut_array()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn from_vector(vector: Vector<2, T, A>) -> Self {
        Self(vector)
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn to_vector(self) -> Vector<2, T, A> {
        self.0
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_vector(&self) -> &Vector<2, T, A> {
        &self.0
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_mut_vector(&mut self) -> &mut Vector<2, T, A> {
        &mut self.0
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Rotation2<T, A2> {
        Rotation2(self.0.to_alignment())
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn align(self) -> Rot2A<T> {
        Rotation2(self.0.align())
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Rot2<T> {
        Rotation2(self.0.unalign())
    }
}

// Tests are located at `src/rotation2.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
