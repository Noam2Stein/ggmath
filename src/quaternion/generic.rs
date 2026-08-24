use core::ops::{Add, Mul, Neg};

use crate::{Aligned, Alignment, One, Quaternion, Scalar, Unaligned, Vector, Zero};

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Zero + One,
{
    /// A quaternion with no rotation.
    pub const IDENTITY: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar,
{
    /// Creates a quaternion from 4 components.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    ///
    /// # Unchecked
    ///
    /// This does not check that the input is normalized. It is up to the user
    /// to provide normalized input or to normalize the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_xyzw(x: T, y: T, z: T, w: T) -> Self {
        Self(Vector::<4, T, A>::new(x, y, z, w))
    }

    /// Creates a quaternion from a 4-element array.
    ///
    /// The first 3 elements `x`, `y` and `z` are the imaginary parts and the
    /// last element `w` is the real part.
    ///
    /// # Unchecked
    ///
    /// This does not check that the input is normalized. It is up to the user
    /// to provide normalized input or to normalize the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Creates a quaternion from a 4-dimensional vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    ///
    /// # Unchecked
    ///
    /// This does not check that the input is normalized. It is up to the user
    /// to provide normalized input or to normalize the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_vector(vector: Vector<4, T, A>) -> Self {
        Self(vector)
    }

    /// Conversion between [`Aligned`] and [`Unaligned`] storage.
    ///
    /// See [`align`] and [`unalign`] for scenarios where the output alignment
    /// is known.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Aligned, Quat, QuatA, Unaligned};
    /// #
    /// let unaligned = Quat::from_xyzw(0.5, 0.5, 0.5, 0.5);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, QuatA::from_xyzw(0.5, 0.5, 0.5, 0.5));
    ///
    /// let aligned = QuatA::from_xyzw(0.5, 0.5, 0.5, 0.5);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Quat::from_xyzw(0.5, 0.5, 0.5, 0.5));
    /// ```
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Quaternion<T, A2> {
        Quaternion(self.0.to_alignment())
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Aligned, Quat, QuatA, Unaligned};
    /// #
    /// let unaligned = Quat::from_xyzw(0.5, 0.5, 0.5, 0.5);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, QuatA::from_xyzw(0.5, 0.5, 0.5, 0.5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn align(self) -> Quaternion<T, Aligned> {
        Quaternion(self.0.align())
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Aligned, Quat, QuatA, Unaligned};
    /// #
    /// let aligned = QuatA::from_xyzw(0.5, 0.5, 0.5, 0.5);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Quat::from_xyzw(0.5, 0.5, 0.5, 0.5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Quaternion<T, Unaligned> {
        Quaternion(self.0.unalign())
    }

    /// Converts the quaternion `self` to a 4-element array.
    ///
    /// The first 3 elements `x`, `y` and `z` are the imaginary parts and the
    /// last element `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// Returns a reference to the quaternion's elements.
    ///
    /// The first 3 elements `x`, `y` and `z` are the imaginary parts and the
    /// last element `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 4] {
        self.0.as_array()
    }

    /// Returns a mutable reference to the quaternion's elements.
    ///
    /// The first 3 elements `x`, `y` and `z` are the imaginary parts and the
    /// last element `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 4] {
        self.0.as_mut_array()
    }

    /// Converts the quaternion `self` to a 4-dimensional vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn to_vector(self) -> Vector<4, T, A> {
        self.0
    }

    /// Returns a reference to the quaternion `self` as a 4-dimensional vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn as_vector(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to the quaternion `self` as a 4-dimensional
    /// vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn as_mut_vector(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
    }

    /// Returns the imaginary components `x`, `y` and `z`.
    #[inline]
    #[must_use]
    pub fn xyz(self) -> Vector<3, T, A> {
        self.0.xyz()
    }

    /// Returns the quaternion conjugate of `self`.
    ///
    /// Equivalent to the inverse if `self` is normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn conjugate(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::from_xyzw(-self.x, -self.y, -self.z, self.w)
    }

    /// Computes the dot product of quaternions `self` and `rhs`.
    ///
    /// Equivalent to `self.angle_between(rhs).cos()`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.dot(rhs.0)
    }

    /// Computes the squared length/magnitude of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let quat = Quat::from_xyzw(0, 1, 2, 3);
    /// assert_eq!(quat.length_squared(), 14);
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.length_squared()
    }

    /// Returns a reference to the quaternion's elements.
    ///
    /// This function has been renamed to [`as_array`].
    ///
    /// [`as_array`]: Self::as_array
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_array`")]
    pub const fn as_array_ref(&self) -> &[T; 4] {
        self.as_array()
    }

    /// Returns a mutable reference to the quaternion's elements.
    ///
    /// This function has been renamed to [`as_mut_array`].
    ///
    /// [`as_mut_array`]: Self::as_mut_array
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_mut_array`")]
    pub const fn as_array_mut(&mut self) -> &mut [T; 4] {
        self.as_mut_array()
    }

    /// Returns a reference to the quaternion `self` as a 4-dimensional vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    ///
    /// This function has been renamed to [`as_vector`].
    ///
    /// [`as_vector`]: Self::as_vector
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_vector`")]
    pub const fn as_vector_ref(&self) -> &Vector<4, T, A> {
        self.as_vector()
    }

    /// Returns a mutable reference to the quaternion `self` as a 4-dimensional
    /// vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    ///
    /// This function has been renamed to [`as_mut_vector`].
    ///
    /// [`as_mut_vector`]: Self::as_mut_vector
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_mut_vector`")]
    pub const fn as_vector_mut(&mut self) -> &mut Vector<4, T, A> {
        self.as_mut_vector()
    }
}

// Tests are located at `src/quaternion.rs`. This module's contents are
// separated into this `generic` module as a workaround for a rustdoc bug, so no
// reason to also move the tests.
