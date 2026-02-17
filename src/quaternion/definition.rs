use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    Aligned, Alignment, Scalar, SignedInteger, Unaligned, Vector,
    constants::{One, Zero},
};

/// A quaternion representing an orientation.
///
/// Note: `Quaternion` is currently missing most of its functionality.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// `Quaternion` is the generic form of:
///
/// - [`Quat<T>`](crate::Quat)
/// - [`QuatU<T>`](crate::QuatU)
///
/// `Quaternion` is generic over:
///
/// - `T`: Scalar type (see [`Scalar`])
/// - `A`: Alignment (see [`Alignment`])
///
/// # Guarantees
///
/// `Quaternion<T, A>` is a transparent wrapper around `Vector<4, T, A>`, and
/// thus inherits its guarantees.
#[repr(transparent)]
pub struct Quaternion<T, A: Alignment>(Vector<4, T, A>)
where
    T: Scalar;

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar,
{
    /// Creates a rotation quaternion.
    ///
    /// This is currently the only way to initialize a quaternion. In the future
    /// there will be mathamatical constructors that are recommended over this
    /// one.
    ///
    /// # Unchecked
    ///
    /// This function does not check if the input is normalized. It is up to the
    /// user to provide normalized input or to normalize the resulting
    /// quaternion.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(Vector::<4, T, A>::new(x, y, z, w))
    }

    /// Creates a rotation quaternion from an array.
    ///
    /// # Unchecked
    ///
    /// This function does not check if the input is normalized. It is up to the
    /// user to provide normalized input or to normalize the resulting
    /// quaternion.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Creates a rotation quaternion from a 4-dimensional vector.
    ///
    /// # Unchecked
    ///
    /// This function does not check if the input is normalized. It is up to the
    /// user to provide normalized input or to normalize the resulting
    /// quaternion.
    #[inline]
    #[must_use]
    pub const fn from_vec(vec: Vector<4, T, A>) -> Self {
        Self(vec)
    }

    /// Converts the quaternion to the specified alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(self) -> Quaternion<T, A2> {
        Quaternion(self.0.to_alignment())
    }

    /// Converts the quaternion to [`Aligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(self) -> Quaternion<T, Aligned> {
        Quaternion(self.0.align())
    }

    /// Converts the quaternion to [`Unaligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Quaternion<T, Unaligned> {
        Quaternion(self.0.unalign())
    }

    /// Converts the quaternion to an array.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// Returns a reference to the quaternion's components.
    #[inline]
    #[must_use]
    pub const fn as_array_ref(&self) -> &[T; 4] {
        self.0.as_array_ref()
    }

    /// Returns a mutable reference to the quaternion's components.
    #[inline]
    #[must_use]
    pub const fn as_array_mut(&mut self) -> &mut [T; 4] {
        self.0.as_array_mut()
    }

    /// Converts the quaternion to a 4-dimensional vector.
    #[inline]
    #[must_use]
    pub const fn to_vec(self) -> Vector<4, T, A> {
        self.0
    }

    /// Returns a reference to the quaternion as a 4-dimensional vector.
    #[inline]
    #[must_use]
    pub const fn as_vec_ref(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to the quaternion as a 4-dimensional vector.
    #[inline]
    #[must_use]
    pub const fn as_vec_mut(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
    }

    /// Reinterprets the bits of the quaternion to a different scalar type.
    ///
    /// The two scalar types must have compatible memory layouts. This is
    /// enforced via trait bounds in this function's signature.
    ///
    /// The function `to_repr` for vectors, matrices and affine transformations
    /// is used to make SIMD optimizations in implementations of [`Scalar`]. It
    /// exists for quaternions for consistency's sake.
    ///
    /// # Safety
    ///
    /// The components of the input must be valid for the output quaternion
    /// type.
    ///
    /// For example, when converting quaternions from `u8` to `bool` the input
    /// components must be either `0` or `1`.
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub const unsafe fn to_repr<T2>(self) -> Quaternion<T2, A>
    where
        T2: Scalar<Repr = T::Repr>,
        T::Repr: SignedInteger,
    {
        unsafe { Quaternion(self.0.to_repr()) }
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
        self.as_array_ref().hash(state);
    }
}

impl<T, A: Alignment> Default for Quaternion<T, A>
where
    T: Scalar + Zero + One,
{
    /// Returns `Quaternion::IDENTITY`.
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
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication (not implemented yet).
    #[inline]
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
    fn mul(self, rhs: T) -> Self::Output {
        Self(self.0 * rhs)
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
    /// Note that addition is not the same as combining the rotations represented by the
    /// two quaternions! That corresponds to multiplication (not implemented yet).
    #[inline]
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
    fn mul_assign(&mut self, rhs: T) {
        *self = Self(self.0 * rhs);
    }
}
