use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    Aligned, Alignment, One, Scalar, Unaligned, Vector, Zero,
    backend::QuaternionBackend,
    utils::{specialize, transmute_mut, transmute_ref},
};

mod float;
#[cfg(feature = "wide")]
mod wide;
#[cfg(feature = "wide")]
mod wide_float;

/// A quaternion representing a rotation.
///
/// `A` controls SIMD alignment and is either [`Unaligned`] or [`Aligned`]. See
/// [`Alignment`] for more details.
///
/// This quaternion is intended to be normalized, but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # Type aliases
///
/// - [`Quat<T>`] for [`Quaternion<T, Unaligned>`].
/// - [`QuatA<T>`] for [`Quaternion<T, Aligned>`].
///
/// # Fields
///
/// - `x: T` (rotates `+Y` to `+Z`)
/// - `y: T` (rotates `+Z` to `+X`)
/// - `z: T` (rotates `+X` to `+Y`)
/// - `w: T` (the scalar part)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Quaternion<T, A>`] is a transparent wrapper around [`Vector<4, T, A>`].
#[repr(transparent)]
pub struct Quaternion<T, A: Alignment>(pub(crate) Vector<4, T, A>)
where
    T: Scalar;

/// A quaternion representing a rotation.
///
/// This quaternion is intended to be normalized, but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # No SIMD alignment
///
/// [`Quat<T>`] does not have SIMD alignment, for that use [`QuatA<T>`].
///
/// # Fields
///
/// - `x: T` (rotates `+Y` to `+Z`)
/// - `y: T` (rotates `+Z` to `+X`)
/// - `z: T` (rotates `+X` to `+Y`)
/// - `w: T` (the scalar part)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Quat<T> = Quaternion<T, Unaligned>;

/// A quaternion representing a rotation.
///
/// This quaternion is intended to be normalized, but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`QuatA<T>`] has SIMD alignment. For no SIMD use
/// [`Quat<T>`].
///
/// # Fields
///
/// - `x: T` (rotates `+Y` to `+Z`)
/// - `y: T` (rotates `+Z` to `+X`)
/// - `z: T` (rotates `+X` to `+Y`)
/// - `w: T` (the scalar part)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type QuatA<T> = Quaternion<T, Aligned>;

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

impl<T, A: Alignment> Clone for Quaternion<T, A>
where
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<T, A: Alignment> Copy for Quaternion<T, A> where T: Scalar {}

#[doc(hidden)]
#[repr(C)]
pub struct QuatFields<T> {
    /// The first imaginary component of the quaternion.
    pub x: T,
    /// The second imaginary component of the quaternion.
    pub y: T,
    /// The third imaginary component of the quaternion.
    pub z: T,
    /// The real part of the quaternion.
    pub w: T,
}

impl<T, A: Alignment> Deref for Quaternion<T, A>
where
    T: Scalar,
{
    type Target = QuatFields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Quaternion<T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_ref::<Quaternion<T, A>, QuatFields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Quaternion<T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Quaternion<T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Xyzw<T>`.
        unsafe { transmute_mut::<Quaternion<T, A>, QuatFields<T>>(self) }
    }
}

impl<T, A: Alignment> Debug for Quaternion<T, A>
where
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_tuple("Quat")
            .field(&self.x)
            .field(&self.y)
            .field(&self.z)
            .field(&self.w)
            .finish()
    }
}

impl<T, A: Alignment> Display for Quaternion<T, A>
where
    T: Scalar + Display,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {}, {}, {})", self.x, self.y, self.z, self.w)
    }
}

impl<T, A: Alignment> PartialEq for Quaternion<T, A>
where
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        self.0 != other.0
    }
}

impl<T, A: Alignment> Eq for Quaternion<T, A> where T: Scalar + Eq {}

impl<T, A: Alignment> Hash for Quaternion<T, A>
where
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_array().hash(state);
    }
}

impl<T, A: Alignment> Default for Quaternion<T, A>
where
    T: Scalar + Zero + One,
{
    /// Returns [`IDENTITY`].
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    #[inline]
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl<T, A: Alignment> Neg for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T>,
{
    type Output = Self;

    #[inline]
    #[track_caller]
    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<T, A: Alignment> Add for Quaternion<T, A>
where
    T: Scalar + Add<Output = T>,
{
    type Output = Self;

    /// Adds two quaternions.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations
    /// represented by the two quaternions. That corresponds to multiplication
    /// (not implemented yet).
    #[inline]
    #[track_caller]
    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl<T, A: Alignment> Sub for Quaternion<T, A>
where
    T: Scalar + Sub<Output = T>,
{
    type Output = Self;

    /// Subtracts the quaternion `rhs` from `self`.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    #[track_caller]
    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl<T, A: Alignment> Mul<T> for Quaternion<T, A>
where
    T: Scalar + Mul<Output = T>,
{
    type Output = Self;

    /// Multiplies a quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    #[track_caller]
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
    }
}

impl<T, A: Alignment> Mul for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    /// Quaternion multiplication.
    ///
    /// Returns a quaternion that first applies the left-hand side quaternion,
    /// then the right-hand side quaternion.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
    #[inline]
    #[track_caller]
    fn mul(self, rhs: Self) -> Self::Output {
        specialize!(<T as QuaternionBackend<A>>::quat_mul(self, rhs))
    }
}

impl<T, A: Alignment> Mul<Quaternion<T, A>> for Vector<3, T, A>
where
    T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    type Output = Self;

    /// 3D vector quaternion multiplication. Returns the rotated vector.
    #[inline]
    #[track_caller]
    fn mul(self, rhs: Quaternion<T, A>) -> Self::Output {
        let w = rhs.w;
        let b = rhs.0.xyz();
        let b2 = b.dot(b);
        let self_dot_b = self.dot(b);
        (self * (w * w - b2)) + (b * (self_dot_b + self_dot_b)) + (b.cross(self) * (w + w))
    }
}

impl<T, A: Alignment> Div<T> for Quaternion<T, A>
where
    T: Scalar + Div<Output = T>,
{
    type Output = Self;

    /// Divides a quaternion by a scalar value.
    ///
    /// The division is not guaranteed to be normalized.
    #[inline]
    #[track_caller]
    fn div(self, rhs: T) -> Self::Output {
        Self(self.0 / rhs)
    }
}

impl<T, A: Alignment> AddAssign for Quaternion<T, A>
where
    T: Scalar + Add<Output = T>,
{
    /// Adds the quaternion `rhs` to `self`.
    ///
    /// The sum is not guaranteed to be normalized.
    ///
    /// Note that addition is not the same as combining the rotations
    /// represented by the two quaternions. That corresponds to multiplication
    /// (not implemented yet).
    #[inline]
    #[track_caller]
    fn add_assign(&mut self, rhs: Self) {
        *self = Self(self.0 + rhs.0);
    }
}

impl<T, A: Alignment> SubAssign for Quaternion<T, A>
where
    T: Scalar + Sub<Output = T>,
{
    /// Subtracts the quaternion `rhs` from `self`.
    ///
    /// The difference is not guaranteed to be normalized.
    #[inline]
    #[track_caller]
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self(self.0 - rhs.0);
    }
}

impl<T, A: Alignment> MulAssign<T> for Quaternion<T, A>
where
    T: Scalar + Mul<Output = T>,
{
    /// Multiplies the quaternion by a scalar value.
    ///
    /// The product is not guaranteed to be normalized.
    #[inline]
    #[track_caller]
    fn mul_assign(&mut self, rhs: T) {
        *self = Self(self.0 * rhs);
    }
}

impl<T, A: Alignment> MulAssign for Quaternion<T, A>
where
    T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// Quaternion multiplication.
    ///
    /// Returns a quaternion that first applies the left-hand side quaternion,
    /// then the right-hand side quaternion.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
    #[inline]
    #[track_caller]
    fn mul_assign(&mut self, rhs: Self) {
        *self = *self * rhs;
    }
}

impl<T, A: Alignment> MulAssign<Quaternion<T, A>> for Vector<3, T, A>
where
    T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    /// 3D vector quaternion multiplication. Returns the rotated vector.
    #[inline]
    #[track_caller]
    fn mul_assign(&mut self, rhs: Quaternion<T, A>) {
        *self = *self * rhs;
    }
}

impl<T, A: Alignment> DivAssign<T> for Quaternion<T, A>
where
    T: Scalar + Div<Output = T>,
{
    /// Divides the quaternion by a scalar value.
    ///
    /// The division is not guaranteed to be normalized.
    #[inline]
    #[track_caller]
    fn div_assign(&mut self, rhs: T) {
        *self = Self(self.0 / rhs);
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Aligned, Matrix, Quaternion, Unaligned, Vector,
        test_utils::{assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(size_of::<Quaternion<T, A>>(), size_of::<Vector<4, T, A>>());
            assert_eq!(
                align_of::<Quaternion<T, A>>(),
                align_of::<Vector<4, T, A>>()
            );
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Quaternion::<T, A>::IDENTITY,
                Quaternion::from_xyzw(T::as_from(0), T::as_from(0), T::as_from(0), T::as_from(1))
            );
        });
    }

    #[test]
    fn test_from_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_array([x, y, z, w]),
                Quaternion::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_from_vector() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::from_vector(Vector::<4, T, A>::new(x, y, z, w)),
                Quaternion::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).to_alignment(),
                Quaternion::<T, Aligned>::from_xyzw(x, y, z, w)
            );
            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).to_alignment(),
                Quaternion::<T, Unaligned>::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).align(),
                Quaternion::<T, Aligned>::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).unalign(),
                Quaternion::<T, Unaligned>::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).to_array(),
                [x, y, z, w]
            );
        });
    }

    #[test]
    fn test_as_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_array(),
                &[x, y, z, w]
            );
        });
    }

    #[test]
    fn test_as_mut_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_mut_array(),
                &mut [x, y, z, w]
            );
        });
    }

    #[test]
    fn test_to_vector() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).to_vector(),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_as_vector() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_vector(),
                &Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_as_mut_vector() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_mut_vector(),
                &mut Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_xyz() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).xyz(),
                Vector::<3, T, A>::new(x, y, z)
            );
        });
    }

    #[test]
    fn test_conjugate() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_test_eq!(
                    quat.conjugate(),
                    Quaternion::from_xyzw(-quat.x, -quat.y, -quat.z, quat.w)
                );
            }
        });
    }

    #[test]
    fn test_dot() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>() {
                assert_test_eq!(quat_1.dot(quat_2), quat_1.0.dot(quat_2.0));
            }
        });
    }

    #[test]
    fn test_length_squared() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_test_eq!(quat.length_squared(), quat.0.length_squared());
            }
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).x, x);
            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).y, y);
            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).z, z);
            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).w, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).x, &mut x);
            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).y, &mut y);
            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).z, &mut z);
            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).w, &mut w);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{:?}", Quaternion::<T, A>::from_xyzw(x, y, z, w)),
                format!("Quat({x:?}, {y:?}, {z:?}, {w:?})")
            );
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{}", Quaternion::<T, A>::from_xyzw(x, y, z, w)),
                format!("({x}, {y}, {z}, {w})")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>()
                .take(10)
                .chain(random_iter().take(10).map(|quat| [quat; 2]))
                .chain(random_iter::<[T; 4]>().take(10).map(|[x, y, z, w]| {
                    [
                        Quaternion::from_xyzw(x, y, z, w),
                        Quaternion::from_xyzw(x.max(y), y.max(z), z.max(w), w.max(x)),
                    ]
                }))
            {
                assert_eq!(
                    quat_1 == quat_2,
                    quat_1.x == quat_2.x
                        && quat_1.y == quat_2.y
                        && quat_1.z == quat_2.z
                        && quat_1.w == quat_2.w
                );
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>()
                .take(10)
                .chain(random_iter().take(10).map(|quat| [quat; 2]))
                .chain(random_iter::<[T; 4]>().take(10).map(|[x, y, z, w]| {
                    [
                        Quaternion::from_xyzw(x, y, z, w),
                        Quaternion::from_xyzw(x.max(y), y.max(z), z.max(w), w.max(x)),
                    ]
                }))
            {
                assert_eq!(
                    quat_1 != quat_2,
                    quat_1.x != quat_2.x
                        || quat_1.y != quat_2.y
                        || quat_1.z != quat_2.z
                        || quat_1.w != quat_2.w
                );
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(Quaternion::<T, A>::default(), Quaternion::IDENTITY);
        });
    }

    #[test]
    fn test_neg() {
        for_types!(|T: PrimitiveFloat, A| {
            for quat in random_iter::<Quaternion<T, A>>() {
                assert_test_eq!(
                    -quat,
                    Quaternion::from_xyzw(-quat.x, -quat.y, -quat.z, -quat.w)
                );
            }
        });
    }

    #[test]
    fn test_add() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>() {
                assert_test_eq!(
                    quat_1 + quat_2,
                    Quaternion::from_xyzw(
                        quat_1.x + quat_2.x,
                        quat_1.y + quat_2.y,
                        quat_1.z + quat_2.z,
                        quat_1.w + quat_2.w
                    )
                );
            }
        });
    }

    #[test]
    fn test_sub() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>() {
                assert_test_eq!(
                    quat_1 - quat_2,
                    Quaternion::from_xyzw(
                        quat_1.x - quat_2.x,
                        quat_1.y - quat_2.y,
                        quat_1.z - quat_2.z,
                        quat_1.w - quat_2.w
                    )
                );
            }
        });
    }

    #[test]
    fn test_mul_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (quat, scalar) in random_iter::<(Quaternion<T, A>, T)>() {
                assert_test_eq!(
                    quat * scalar,
                    Quaternion::from_xyzw(
                        quat.x * scalar,
                        quat.y * scalar,
                        quat.z * scalar,
                        quat.w * scalar
                    )
                );
            }
        });
    }

    #[test]
    fn test_mul() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, quat_1, quat_2) in [
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Quaternion::<T, A>::from_xyzw(0.8, 0.4, 0.3, 0.1),
                    Quaternion::<T, A>::from_xyzw(0.3, 0.8, 0.1, 0.4),
                ),
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Quaternion::<T, A>::IDENTITY,
                    Quaternion::<T, A>::IDENTITY,
                ),
            ]
            .into_iter()
            .chain(random_iter())
            {
                if !vector.is_finite() {
                    continue;
                }

                let [quat_1, quat_2] =
                    [quat_1, quat_2].map(|q| q.normalize_or(Quaternion::IDENTITY).normalize());

                assert_test_eq!(
                    vector * (quat_1 * quat_2),
                    vector * quat_1 * quat_2,
                    abs <= vector.abs().max_element() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_vector_mul() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, quat) in [
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Quaternion::<T, A>::from_xyzw(0.8, 0.4, 0.3, 0.1),
                ),
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Quaternion::<T, A>::IDENTITY,
                ),
            ]
            .into_iter()
            .chain(random_iter())
            {
                if !vector.is_finite() {
                    continue;
                }

                let quat = quat.normalize_or(Quaternion::IDENTITY).normalize();

                assert_test_eq!(
                    vector * quat,
                    vector * Matrix::<3, T, A>::from_quat(quat),
                    abs <= vector.abs().max_element() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_div_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (quat, scalar) in random_iter::<(Quaternion<T, A>, T)>() {
                assert_test_eq!(
                    quat / scalar,
                    Quaternion::from_xyzw(
                        quat.x / scalar,
                        quat.y / scalar,
                        quat.z / scalar,
                        quat.w / scalar
                    )
                );
            }
        });
    }

    #[test]
    fn test_add_assign() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>() {
                let mut result = quat_1;
                result += quat_2;

                assert_test_eq!(result, quat_1 + quat_2);
            }
        });
    }

    #[test]
    fn test_sub_assign() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_1, quat_2] in random_iter::<[Quaternion<T, A>; 2]>() {
                let mut result = quat_1;
                result -= quat_2;

                assert_test_eq!(result, quat_1 - quat_2);
            }
        });
    }

    #[test]
    fn test_mul_assign_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (quat, scalar) in random_iter::<(Quaternion<T, A>, T)>() {
                let mut result = quat;
                result *= scalar;

                assert_test_eq!(result, quat * scalar);
            }
        });
    }

    #[test]
    fn test_mul_assign() {
        for_types!(|T: PrimitiveFloat, A| {
            for [quat_a, quat_b] in random_iter::<[Quaternion<T, A>; 2]>() {
                let mut result = quat_a;
                result *= quat_b;

                assert_test_eq!(result, quat_a * quat_b);
            }
        });
    }

    #[test]
    fn test_vector_mul_assign() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, quat) in random_iter::<(Vector<3, T, A>, Quaternion<T, A>)>() {
                let mut result = vector;
                result *= quat;

                assert_test_eq!(result, vector * quat);
            }
        });
    }

    #[test]
    fn test_div_assign_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (quat, scalar) in random_iter::<(Quaternion<T, A>, T)>() {
                let mut result = quat;
                result /= scalar;

                assert_test_eq!(result, quat / scalar);
            }
        });
    }
}
