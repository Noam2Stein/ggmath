use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    Aligned, Alignment, Scalar, SignedInteger, Unaligned, Vector,
    constants::{Nan, One, Zero},
    transmute::{transmute_mut, transmute_ref},
};

/// A quaternion representing an orientation of type `T`.
///
/// `A` controls SIMD alignment and is either [`Aligned`] or [`Unaligned`]. See
/// [`Alignment`] for more details.
///
/// Quaternions are currently missing most functionality.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # Type aliases
///
/// - [`Quat<T>`] for `Quaternion<T, Aligned>`.
/// - [`QuatU<T>`] for `Quaternion<T, Unaligned>`.
///
/// # Fields
///
/// `x: T`
///
/// The first imaginary component of the quaternion.
///
/// `y: T`
///
/// The second imaginary component of the quaternion.
///
/// `z: T`
///
/// The third imaginary component of the quaternion.
///
/// `w: T`
///
/// The real part of the quaternion.
///
/// # Memory layout
///
/// `Quaternion<T, A>` is a transparent wrapper around `Vector<4, T, A>`.
///
/// [`Quat<T>`]: crate::Quat
/// [`QuatU<T>`]: crate::QuatU
#[repr(transparent)]
pub struct Quaternion<T, A: Alignment>(Vector<4, T, A>)
where
    T: Scalar;

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Zero,
{
    /// A quaternion with all elements set to `0`.
    ///
    /// This does not represent a valid rotation. See [`IDENTITY`] for a
    /// quaternion with no rotation.
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    pub const ZERO: Self = Self::from_vec(Vector::ZERO);
}

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Zero + One,
{
    /// A quaternion with no rotation.
    pub const IDENTITY: Self = Self::from_array([T::ZERO, T::ZERO, T::ZERO, T::ONE]);
}

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar + Nan,
{
    /// A quaternion with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_vec(Vector::NAN);
}

impl<T, A: Alignment> Quaternion<T, A>
where
    T: Scalar,
{
    /// Creates a rotation quaternion from its `x`, `y`, `z` and `w` elements.
    ///
    /// # Unchecked
    ///
    /// This does not check that the input is normalized. It is up to the user
    /// to provide normalized input or to normalize the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self(Vector::<4, T, A>::new(x, y, z, w))
    }

    /// Creates a rotation quaternion from an array containing `x`, `y`, `z` and
    /// `w`.
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

    /// Creates a rotation quaternion from a 4-dimensional vector.
    ///
    /// # Unchecked
    ///
    /// This does not check that the input is normalized. It is up to the user
    /// to provide normalized input or to normalize the resulting quaternion.
    #[inline]
    #[must_use]
    pub const fn from_vec(vec: Vector<4, T, A>) -> Self {
        Self(vec)
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
    /// # use ggmath::{Aligned, Quat, QuatU, Unaligned};
    /// #
    /// let aligned = Quat::new(0.5, 0.5, 0.5, 0.5);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, QuatU::new(0.5, 0.5, 0.5, 0.5));
    ///
    /// let unaligned = QuatU::new(0.5, 0.5, 0.5, 0.5);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Quat::new(0.5, 0.5, 0.5, 0.5));
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
    /// # use ggmath::{Aligned, Quat, QuatU, Unaligned};
    /// #
    /// let unaligned = QuatU::new(0.5, 0.5, 0.5, 0.5);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Quat::new(0.5, 0.5, 0.5, 0.5));
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
    /// # use ggmath::{Aligned, Quat, QuatU, Unaligned};
    /// #
    /// let aligned = Quat::new(0.5, 0.5, 0.5, 0.5);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, QuatU::new(0.5, 0.5, 0.5, 0.5));
    /// ```
    #[inline]
    #[must_use]
    pub const fn unalign(self) -> Quaternion<T, Unaligned> {
        Quaternion(self.0.unalign())
    }

    /// Converts the quaternion to an array containing `x`, `y`, `z` and `w`.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// Returns a reference to the quaternion's elements.
    #[inline]
    #[must_use]
    pub const fn as_array_ref(&self) -> &[T; 4] {
        self.0.as_array_ref()
    }

    /// Returns a mutable reference to the quaternion's elements.
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

    /// Raw transmutation between scalar types.
    ///
    /// This function's signature staticly guarantees that the types have
    /// compatible memory layouts.
    ///
    /// This function is used to make SIMD optimizations in implementations of
    /// [`Scalar`].
    ///
    /// # Safety
    ///
    /// The elements of `self` must contain bit patterns that are valid for the
    /// output type. For example, when converting quaternions from `u8` to
    /// `bool`, the input elements must be either `0` or `1` (that example is
    /// unconventional, but the rule applies for any scalar that does not accept
    /// all bit patterns).
    ///
    /// The padding does not need to contain valid values of the output type.
    ///
    /// # Examples
    ///
    /// Correct usage:
    ///
    /// ```
    /// # use ggmath::Quat;
    /// #
    /// let bits = Quat::<u8>::new(1, 0, 0, 1);
    ///
    /// // SAFETY: `bool` accepts both the `0` and `1` bit patterns.
    /// let bools = unsafe { bits.to_repr::<bool>() };
    ///
    /// assert_eq!(bools, Quat::new(true, false, false, true));
    /// ```
    ///
    /// Incorrect usage:
    ///
    /// ```compile_fail
    /// # use ggmath::Quat;
    /// #
    /// let a = Quat::<i32>::new(1, 2, 3, 4);
    ///
    /// // This does not compile since `i32` and `i64` are not compatible.
    /// let _ = unsafe { a.to_repr::<i64>() };
    /// ```
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
