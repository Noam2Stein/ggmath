use core::ops::{Add, Mul, Neg};

use crate::{
    Aligned, Alignment, Length, One, Rotor, Scalar, Unaligned, Vector, Zero, backend::RotorBackend,
    length::Three, utils::specialize_3,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar + Zero + One,
{
    /// A rotor that keeps all vectors unchanged.
    ///
    /// This sets `s` to 1 and all other elements to 0.
    pub const IDENTITY: Self = Self::IDENTITY_INTERNAL_IMPL;

    /// The implementation of [`Self::IDENTITY`].
    ///
    /// We use this helper constant so that IDEs do not show the implementation
    /// of the constant.
    const IDENTITY_INTERNAL_IMPL: Self = Self(Vector::<4, T, A>::W);
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: Three,
    T: Scalar,
{
    /// Returns the conjugate of a rotor.
    ///
    /// This performs the same operation as [`inverse`]. Use whichever function
    /// makes your intentions clearer.
    ///
    /// [`inverse`]: Rotor#method.inverse
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn conjugate(self) -> Self
    where
        T: Neg<Output = T>,
    {
        specialize_3!(<T as RotorBackend<N, A>>::rotor_conjugate(self))
    }

    /// Computes the dot product of two rotors.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.dot(rhs.0)
    }

    /// Computes the squared length/magnitude of a rotor.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.length_squared()
    }

    /// Conversion between [`Aligned`] and [`Unaligned`] storage.
    ///
    /// See [`align`] and [`unalign`] for scenarios where the output alignment
    /// is known.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Rotor<N, T, A2> {
        Rotor(self.0.to_alignment())
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(self) -> Rotor<N, T, Aligned> {
        self.to_alignment()
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more details.
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

    /// Creates a 3D rotor from elements `yz, zx, xy, s`.
    ///
    /// Note that a rotor is meant to be normalized, but this function does not
    /// enforce that.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn from_elements(yz: T, zx: T, xy: T, s: T) -> Self {
        Self(Vector::<4, T, A>::new(yz, zx, xy, s))
    }

    /// Creates a 3D rotor from an element array `[yz, zx, xy, s]`.
    ///
    /// Note that a rotor is meant to be normalized, but this function does not
    /// enforce that.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Converts a 3D rotor to an element array `[yz, zx, xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// Returns a reference to a 3D rotor element array `[yz, zx, xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 4] {
        self.0.as_array()
    }

    /// Returns a mutable reference to a 3D rotor element array
    /// `[yz, zx, xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 4] {
        self.0.as_mut_array()
    }

    /// Creates a 3D rotor from an element vector `(yz, zx, xy, s)`.
    ///
    /// Note that a rotor is meant to be normalized, but this function does not
    /// enforce that.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn from_raw_vector(vector: Vector<4, T, A>) -> Self {
        Self(vector)
    }

    /// Converts a 3D rotor to an element vector `(yz, zx, xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn to_raw_vector(self) -> Vector<4, T, A> {
        self.0
    }

    /// Returns a reference to a 3D rotor element vector `(yz, zx, xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_raw_vector(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to a 3D rotor element vector
    /// `(yz, zx, xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using rotor
    /// elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_mut_raw_vector(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
    }
}

// Tests are located at `src/rotor.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
