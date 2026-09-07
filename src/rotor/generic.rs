use core::ops::{Add, Mul, Neg};

use crate::{
    Aligned, Alignment, Length, One, Rotor, Scalar, Unaligned, Vector, Zero, length::Three,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Zero + One,
{
    /// TODO
    pub const IDENTITY: Self = todo!();
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar,
{
    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn conjugate(self) -> Self
    where
        T: Neg<Output = T>,
    {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, _rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Rotor<N, T, A2> {
        todo!()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn align(self) -> Rotor<N, T, Aligned> {
        self.to_alignment()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Rotor<N, T, Unaligned> {
        self.to_alignment()
    }
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: Scalar,
{
    // Note about `from_elements`: Because rotor fields are not obvious, it
    // would be nice to have `Rotor3 { yz: ..., zx: ..., xy: ..., s: ... }`
    // syntax, however that is impossible to do with a const generic struct.

    /// TODO
    #[inline]
    #[must_use]
    pub const fn from_elements(yz: T, zx: T, xy: T, s: T) -> Self {
        Self(Vector::<4, T, A>::new(yz, zx, xy, s))
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Self {
        Self(Vector::from_array(array))
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 4] {
        self.0.as_array()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 4] {
        self.0.as_mut_array()
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn from_vector(vector: Vector<4, T, A>) -> Self {
        Self(vector)
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn to_vector(self) -> Vector<4, T, A> {
        self.0
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_vector(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// TODO
    #[inline]
    #[must_use]
    pub const fn as_mut_vector(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
    }
}

// Tests are located at `src/rotor.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
