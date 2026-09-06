use core::ops::{Add, Mul, Neg, Sub};

use crate::{Alignment, One, Rot2, Rot2A, Rotation2, Scalar, Vector, Zero};

impl<T, A: Alignment> Rotation2<T, A>
where
    T: Scalar + Zero + One,
{
    /// A 2D rotation that keeps vectors unchanged (an angle equal to zero).
    pub const IDENTITY: Self = Self::from_cos_sin(T::ONE, T::ZERO);
}

impl<T, A: Alignment> Rotation2<T, A>
where
    T: Scalar,
{
    /// Creates a 2D rotation from raw elements, the cosine and sine of an
    /// angle.
    ///
    /// This is how the rotation is stored, so this function is akin to
    /// [`Vec2::new`]. Note that the rotation is meant to be normalized, but
    /// this function does not enforce that.
    ///
    /// [`Vec2::new`]: crate::Vec2::new
    #[inline]
    #[must_use]
    pub const fn from_cos_sin(cos: T, sin: T) -> Self {
        Self(Vector::<2, T, A>::new(cos, sin))
    }

    /// Negates the sine element.
    ///
    /// This affectively inverts the rotation, though consider using [`inverse`]
    /// for that.
    ///
    /// [`inverse`]: Rotation2::inverse
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn conjugate(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::from_cos_sin(self.cos, -self.sin)
    }

    /// Rotates a complex number by a quarter of a turn, adding 90 degrees to
    /// the rotation.
    ///
    /// This rotates `+X` to `+Y`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perp(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(self.0.perp())
    }

    /// Computes the dot product of two rotations.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.dot(rhs.0)
    }

    /// Computes `self.perp().dot(rhs)`.
    ///
    /// Also reffered to as the wedge/outer product, the 2D cross product, the
    /// determinant and the signed area.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn perp_dot(self, rhs: Self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.0.perp_dot(rhs.0)
    }

    /// Computes the squared length/magnitude of `self`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.length_squared()
    }

    /// Creates a 2D rotation from raw elements `[cos(a), sin(a)]`.
    ///
    /// This is how the rotation is stored, so this function is akin to
    /// [`Vec2::from_array`]. Note that the rotation is meant to be normalized,
    /// but this function does not enforce that.
    ///
    /// [`Vec2::from_array`]: crate::Vec2::from_array
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 2]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Returns the raw elements of a 2D rotation `[cos(a), sin(a)]`.
    ///
    /// This is how the rotation is stored, so this function is akin to
    /// [`Vec2::to_array`].
    ///
    /// [`Vec2::to_array`]: crate::Vec2::to_array
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 2] {
        self.0.to_array()
    }

    /// Returns a reference to the raw elements of a 2D rotation
    /// `[cos(a), sin(a)]`.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 2] {
        self.0.as_array()
    }

    /// Returns a mutable reference to the raw elements of a 2D rotation
    /// `[cos(a), sin(a)]`.
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 2] {
        self.0.as_mut_array()
    }

    /// Creates a 2D rotation from a raw element vector `(cos(a), sin(a))`.
    ///
    /// This is how the rotation is stored, so this function is akin to
    /// [`Vec2::from_array`]. Note that the rotation is meant to be normalized,
    /// but this function does not enforce that.
    ///
    /// [`Vec2::from_array`]: crate::Vec2::from_array
    #[inline]
    #[must_use]
    pub const fn from_vector(vector: Vector<2, T, A>) -> Self {
        Self(vector)
    }

    /// Returns the raw elements of a 2D rotation `(cos(a), sin(a))`.
    ///
    /// This is how the rotation is stored, so this function is akin to
    /// [`Vec2::to_array`].
    ///
    /// [`Vec2::to_array`]: crate::Vec2::to_array
    #[inline]
    #[must_use]
    pub const fn to_vector(self) -> Vector<2, T, A> {
        self.0
    }

    /// Returns a reference to the raw elements of a 2D rotation
    /// `(cos(a), sin(a))`.
    #[inline]
    #[must_use]
    pub const fn as_vector(&self) -> &Vector<2, T, A> {
        &self.0
    }

    /// Returns a mutable reference to the raw elements of a 2D rotation
    /// `(cos(a), sin(a))`.
    #[inline]
    #[must_use]
    pub const fn as_mut_vector(&mut self) -> &mut Vector<2, T, A> {
        &mut self.0
    }

    /// Conversion between [`Aligned`] and [`Unaligned`] storage.
    ///
    /// See [`align`] and [`unalign`] for scenarios where the output alignment
    /// is known.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// [`Aligned`]: crate::Aligned
    /// [`Unaligned`]: crate::Unaligned
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Rotation2<T, A2> {
        Rotation2(self.0.to_alignment())
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// [`Aligned`]: crate::Aligned
    #[inline]
    #[must_use]
    pub const fn align(self) -> Rot2A<T> {
        Rotation2(self.0.align())
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// [`Unaligned`]: crate::Unaligned
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Rot2<T> {
        Rotation2(self.0.unalign())
    }
}

// Tests are located at `src/rotation2.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
