use core::ops::{Add, Mul, Neg};

use crate::{
    Aligned, Alignment, Length, One, Rotor, Scalar, Unaligned, Vector, Zero, length::TwoOrThree,
    utils::transmute_generic,
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zero + One,
{
    /// A rotor that keeps all vectors unchanged.
    ///
    /// This sets `s` to 1 and all other elements to 0.
    pub const IDENTITY: Self = todo!();
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
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
        todo!()
    }

    /// Computes the dot product of two rotors `self` and `rhs`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, _rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        todo!()
    }

    /// Computes the squared length/magnitude of a rotor.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        todo!()
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
        // SAFETY: The vector element count stays the same, and padding accepts
        // all bit patterns and is guaranteed to be initialized.
        unsafe { transmute_generic::<Rotor<N, T, A>, Rotor<N, T, A2>>(self) }
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

impl<T, A: Alignment> Rotor<2, T, A>
where
    T: Scalar,
{
    // Note about `from_raw_elements`: Because rotor fields are not obvious, it
    // would be nice to have `Rotor2 { xy: ..., s: ... }` syntax, however that
    // is impossible to do with a const generic struct.

    /// Creates a 2D rotor from raw elements `xy, s`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_raw_elements(xy: T, s: T) -> Self {
        Self(Vector::<2, T, A>::new(xy, s))
    }

    /// Creates a rotor from a raw-element array `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_raw_array(array: [T; 2]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Converts a rotor to a raw-element array `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn to_raw_array(self) -> [T; 2] {
        self.0.to_array()
    }

    /// Returns a reference to a rotor's raw elements `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_raw_array(&self) -> &[T; 2] {
        self.0.as_array()
    }

    /// Returns a mutable reference to a rotor's raw elements `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_mut_raw_array(&mut self) -> &mut [T; 2] {
        self.0.as_mut_array()
    }

    /// Creates a rotor from a raw-element vector `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_raw_vector(vector: Vector<2, T, A>) -> Self {
        Self(vector)
    }

    /// Converts a rotor to a raw-element vector `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn to_raw_vector(self) -> Vector<2, T, A> {
        self.0
    }

    /// Returns a reference to a rotor's fields as a raw-element vector
    /// `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_raw_vector(&self) -> &Vector<2, T, A> {
        &self.0
    }

    /// Returns a mutable reference to a rotor's fields as a raw-element vector
    /// `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_mut_raw_vector(&mut self) -> &mut Vector<2, T, A> {
        &mut self.0
    }
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: Scalar,
{
    // Note about `from_raw_elements`: Because rotor fields are not obvious, it
    // would be nice to have `Rotor3 { xy: ..., xz: ..., yz: ..., s: ... }`
    // syntax, however that is impossible to do with a const generic struct.

    /// Creates a 3D rotor from raw elements `xy, xz, yz, s`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_raw_elements(xy: T, xz: T, yz: T, s: T) -> Self {
        Self(Vector::<4, T, A>::new(xy, xz, yz, s))
    }

    /// Creates a rotor from a raw-element array `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_raw_array(array: [T; 4]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Converts a rotor to a raw-element array `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn to_raw_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// Returns a reference to a rotor's raw elements `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_raw_array(&self) -> &[T; 4] {
        self.0.as_array()
    }

    /// Returns a mutable reference to a rotor's raw elements `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_mut_raw_array(&mut self) -> &mut [T; 4] {
        self.0.as_mut_array()
    }

    /// Creates a rotor from a raw-element vector `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_raw_vector(vector: Vector<4, T, A>) -> Self {
        Self(vector)
    }

    /// Converts a rotor to a raw-element vector `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn to_raw_vector(self) -> Vector<4, T, A> {
        self.0
    }

    /// Returns a reference to a rotor's fields as a raw-element vector
    /// `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_raw_vector(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to a rotor's fields as a raw-element vector
    /// `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using raw
    /// rotor elements directly. Instead, use higher level helper functions.
    #[inline]
    #[must_use]
    pub const fn as_mut_raw_vector(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
    }
}

// Tests are located at `src/rotor.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
