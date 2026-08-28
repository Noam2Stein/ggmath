use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Div, Mul, Neg, Sub},
};

use crate::{
    Aligned, Alignment, Length, One, Rotor, Scalar, Unaligned, Vector, Zero,
    backend::RotorBackend,
    length::TwoOrThree,
    utils::{specialize, specialize_23, transmute_generic},
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zero,
{
    /// A rotor with all elements set to zero.
    ///
    /// This is intentionally not exposed to the public API, as it does not
    /// represent a valid rotation.
    pub(crate) const ZERO: Self = match N {
        // SAFETY: We are transmuting a type to itself
        2 => unsafe {
            transmute_generic::<Rotor<2, T, A>, Rotor<N, T, A>>(Rotor::<2, T, A>(
                Vector::<2, T, A>::ZERO,
            ))
        },
        // SAFETY: We are transmuting a type to itself
        3 => unsafe {
            transmute_generic::<Rotor<3, T, A>, Rotor<N, T, A>>(Rotor::<3, T, A>(
                Vector::<4, T, A>::ZERO,
            ))
        },
        _ => unreachable!(),
    };
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zero + One,
{
    /// A rotor that keeps all vectors unchanged.
    ///
    /// This sets `s` to one and all plane fields to zero.
    pub const IDENTITY: Self = Self::IDENTITY_INTERNAL_IMPL;

    /// The implementation of [`Self::IDENTITY`].
    ///
    /// Because of type system limitations, this implementation looks crazy. Use
    /// a separate constant so that IDEs do not show the implementation.
    const IDENTITY_INTERNAL_IMPL: Self = match N {
        // SAFETY: We are transmuting a type to itself
        2 => unsafe {
            transmute_generic::<Rotor<2, T, A>, Rotor<N, T, A>>(Rotor::<2, T, A>(
                Vector::<2, T, A>::Y,
            ))
        },
        // SAFETY: We are transmuting a type to itself
        3 => unsafe {
            transmute_generic::<Rotor<3, T, A>, Rotor<N, T, A>>(Rotor::<3, T, A>(
                Vector::<4, T, A>::W,
            ))
        },
        _ => unreachable!(),
    };
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
    /// Returns the conjugate of a rotor.
    ///
    /// Equivalent to the inverse if `self` is normalized.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn conjugate(self) -> Self
    where
        T: Neg<Output = T>,
    {
        specialize_23!(Rotor::<N, T, A>::conjugate_backend(self))
    }

    /// Computes the dot product of rotors `self` and `rhs`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn dot(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        specialize_23!(Rotor::<N, T, A>::dot_backend(self, rhs))
    }

    /// Computes the squared length/magnitude of a rotor.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn length_squared(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        specialize_23!(Rotor::<N, T, A>::length_squared_backend(self))
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
    // would be nice to have `Rot2 { xy: ..., s: ... }` syntax, however that is
    // impossible to do with a const generic struct.

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

    #[inline(always)]
    #[track_caller]
    fn conjugate_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::from_raw_elements(-self.xy, self.s)
    }

    #[inline(always)]
    #[track_caller]
    fn dot_backend(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.dot(rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn length_squared_backend(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.length_squared()
    }

    #[inline]
    pub(in crate::rotor) fn debug_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Debug,
    {
        self.0.fmt(f)
    }

    #[inline]
    pub(in crate::rotor) fn display_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Display,
    {
        self.0.fmt(f)
    }

    #[inline(always)]
    pub(in crate::rotor) fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    pub(in crate::rotor) fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn neg_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn add_backend(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn sub_backend(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn mul_scalar_backend(self, rhs: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn vector_mul_backend(
        vector: Vector<2, T, A>,
        rhs: Self,
    ) -> Vector<2, T, A>
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let fx = rhs.s * rhs.s - rhs.xy * rhs.xy;
        let fy = rhs.xy * rhs.s;
        let fy = fy + fy;

        Vector::<2, T, A>::new(fx * vector.x - fy * vector.y, fy * vector.x + fx * vector.y)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn mul_backend(self, rhs: Self) -> Self
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        Self::from_raw_elements(
            self.xy * rhs.s + self.s * rhs.xy,
            self.s * rhs.s - self.xy * rhs.xy,
        )
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn div_scalar_backend(self, rhs: T) -> Self
    where
        T: Div<Output = T>,
    {
        Self(self.0 / rhs)
    }
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: Scalar,
{
    // Note about `from_raw_elements`: Because rotor fields are not obvious, it
    // would be nice to have `Rot3 { xy: ..., xz: ..., yz: ..., s: ... }`
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

    #[inline(always)]
    #[track_caller]
    fn conjugate_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::from_raw_elements(-self.xy, -self.xz, -self.yz, self.s)
    }

    #[inline(always)]
    #[track_caller]
    fn dot_backend(self, rhs: Self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.dot(rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn length_squared_backend(self) -> T
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        self.0.length_squared()
    }

    #[inline]
    pub(in crate::rotor) fn debug_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Debug,
    {
        self.0.fmt(f)
    }

    #[inline]
    pub(in crate::rotor) fn display_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Display,
    {
        self.0.fmt(f)
    }

    #[inline(always)]
    pub(in crate::rotor) fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    pub(in crate::rotor) fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn neg_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn add_backend(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn sub_backend(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn mul_scalar_backend(self, rhs: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn vector_mul_backend(
        vector: Vector<3, T, A>,
        rhs: Self,
    ) -> Vector<3, T, A>
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        specialize!(<T as RotorBackend<3, A>>::rotor_vector_mul(vector, rhs))
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn mul_backend(self, rhs: Self) -> Self
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        specialize!(<T as RotorBackend<3, A>>::rotor_mul(self, rhs))
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::rotor) fn div_scalar_backend(self, rhs: T) -> Self
    where
        T: Div<Output = T>,
    {
        Self(self.0 / rhs)
    }
}

// Tests are located at `src/rotor.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
