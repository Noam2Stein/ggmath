use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Mul, MulAssign, Neg, Sub, SubAssign},
};

use crate::{
    Aligned, Alignment, Scalar, SignedInteger, Unaligned, Vector,
    constants::{Nan, One, Zero},
    utils::{transmute_mut, transmute_ref},
};

mod float;

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

/// A quaternion representing an orientation.
///
/// Quaternions are currently missing most functionality.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # SIMD alignment
///
/// `Quat<T>` has SIMD alignment for appropriate scalar types. See [`QuatU<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
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
/// [`Alignment`]: crate::Alignment
pub type Quat<T> = Quaternion<T, Aligned>;

/// A quaternion representing an orientation.
///
/// Quaternions are currently missing most functionality.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # No SIMD alignment
///
/// `QuatU<T>` does not have SIMD alignment. See [`Quat<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
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
/// [`Alignment`]: crate::Alignment
pub type QuatU<T> = Quaternion<T, Unaligned>;

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
    pub const ZERO: Self = Self::from_vector(Vector::ZERO);
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
    pub const NAN: Self = Self::from_vector(Vector::NAN);
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
    pub const fn as_array_ref(&self) -> &[T; 4] {
        self.0.as_array_ref()
    }

    /// Returns a mutable reference to the quaternion's elements.
    ///
    /// The first 3 elements `x`, `y` and `z` are the imaginary parts and the
    /// last element `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn as_array_mut(&mut self) -> &mut [T; 4] {
        self.0.as_array_mut()
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
    pub const fn as_vector_ref(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to the quaternion `self` as a 4-dimensional
    /// vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[inline]
    #[must_use]
    pub const fn as_vector_mut(&mut self) -> &mut Vector<4, T, A> {
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
    #[deprecated(since = "0.16.7", note = "renamed to `from_xyzw`")]
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self::from_xyzw(x, y, z, w)
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
    #[deprecated(since = "0.16.7", note = "renamed to `from_vector`")]
    pub const fn from_vec(vector: Vector<4, T, A>) -> Self {
        Self::from_vector(vector)
    }

    /// Converts the quaternion `self` to a 4-dimensional vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[inline]
    #[must_use]
    #[deprecated(since = "0.16.7", note = "renamed to `to_vector`")]
    pub const fn to_vec(self) -> Vector<4, T, A> {
        self.0
    }

    /// Returns a reference to the quaternion `self` as a 4-dimensional vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[inline]
    #[must_use]
    #[deprecated(since = "0.16.7", note = "renamed to `as_vector_ref`")]
    pub const fn as_vec_ref(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to the quaternion `self` as a 4-dimensional
    /// vector.
    ///
    /// `x`, `y` and `z` are the imaginary parts and `w` is the real part.
    #[deprecated(since = "0.16.7", note = "renamed to `as_vector_mut`")]
    #[inline]
    #[must_use]
    pub const fn as_vec_mut(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
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

#[cfg(test)]
mod tests {
    use crate::{
        Aligned, Quaternion, Unaligned, Vector,
        utils::{assert_float_eq, for_parameters},
    };

    #[test]
    fn test_layout() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(size_of::<Quaternion<T, A>>(), size_of::<Vector<4, T, A>>());
            assert_eq!(
                align_of::<Quaternion<T, A>>(),
                align_of::<Vector<4, T, A>>()
            );
        });
    }

    #[test]
    fn test_zero() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Quaternion::<T, A>::ZERO,
                Quaternion::from_xyzw(T::as_from(0), T::as_from(0), T::as_from(0), T::as_from(0))
            );
        });
    }

    #[test]
    fn test_identity() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Quaternion::<T, A>::IDENTITY,
                Quaternion::from_xyzw(T::as_from(0), T::as_from(0), T::as_from(0), T::as_from(1))
            );
        });
    }

    #[test]
    fn test_nan() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Quaternion::<T, A>::NAN,
                Quaternion::from_xyzw(T::NAN, T::NAN, T::NAN, T::NAN)
            );
        });
    }

    #[test]
    fn test_from_array() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_array([x, y, z, w]),
                Quaternion::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_from_vector() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::from_vector(Vector::<4, T, A>::new(x, y, z, w)),
                Quaternion::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

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
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).align(),
                Quaternion::<T, Aligned>::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).unalign(),
                Quaternion::<T, Unaligned>::from_xyzw(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_array() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).to_array(),
                [x, y, z, w]
            );
        });
    }

    #[test]
    fn test_as_array_ref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_array_ref(),
                &[x, y, z, w]
            );
        });
    }

    #[test]
    fn test_as_array_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_array_mut(),
                &mut [x, y, z, w]
            );
        });
    }

    #[test]
    fn test_to_vector() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).to_vector(),
                Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_as_vector_ref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_vector_ref(),
                &Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_as_vector_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w).as_vector_mut(),
                &mut Vector::<4, T, A>::new(x, y, z, w)
            );
        });
    }

    #[test]
    fn test_to_repr() {
        for_parameters!(|A| {
            assert_eq!(
                // SAFETY: `u32` accepts all bit patterns.
                unsafe { Quaternion::<i32, A>::from_xyzw(0, 1, 2, 3).to_repr() },
                Quaternion::<u32, A>::from_xyzw(0, 1, 2, 3)
            );
        });
    }

    #[test]
    fn test_deref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).x, x);
            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).y, y);
            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).z, z);
            assert_eq!(Quaternion::<T, A>::from_xyzw(x, y, z, w).w, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(T::as_from);

            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).x, &mut x);
            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).y, &mut y);
            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).z, &mut z);
            assert_eq!(&mut Quaternion::<T, A>::from_xyzw(x, y, z, w).w, &mut w);
        });
    }

    #[test]
    fn test_debug() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{:?}", Quaternion::<T, A>::from_xyzw(x, y, z, w)),
                format!("Quat({x:?}, {y:?}, {z:?}, {w:?})")
            );
        });
    }

    #[test]
    fn test_display() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!("{}", Quaternion::<T, A>::from_xyzw(x, y, z, w)),
                format!("({x}, {y}, {z}, {w})")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w)
                    == Quaternion::<T, A>::from_xyzw(z, w, y, x),
                x == z && y == w && z == y && w == x
            );
        });
    }

    #[test]
    fn test_ne() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w)
                    != Quaternion::<T, A>::from_xyzw(z, w, y, x),
                x != z || y != w || z != y || w != x
            );
        });
    }

    #[test]
    fn test_default() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Quaternion::<T, A>::default(), Quaternion::IDENTITY);
        });
    }

    #[test]
    fn test_neg() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                -Quaternion::<T, A>::from_xyzw(x, y, z, w),
                Quaternion::from_xyzw(-x, -y, -z, -w)
            );
        });
    }

    #[test]
    fn test_add() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w)
                    + Quaternion::<T, A>::from_xyzw(z, w, y, x),
                Quaternion::from_xyzw(x + z, y + w, z + y, w + x)
            );
        });
    }

    #[test]
    fn test_sub() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w)
                    - Quaternion::<T, A>::from_xyzw(z, w, y, x),
                Quaternion::from_xyzw(x - z, y - w, z - y, w - x)
            );
        });
    }

    #[test]
    fn test_mul() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Quaternion::<T, A>::from_xyzw(x, y, z, w) * w,
                Quaternion::from_xyzw(x * w, y * w, z * w, w * w)
            );
        });
    }

    #[test]
    fn test_add_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut quat = Quaternion::<T, A>::from_xyzw(x, y, z, w);
            quat += Quaternion::<T, A>::from_xyzw(z, w, y, x);
            assert_float_eq!(quat, Quaternion::from_xyzw(x + z, y + w, z + y, w + x));
        });
    }

    #[test]
    fn test_sub_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut quat = Quaternion::<T, A>::from_xyzw(x, y, z, w);
            quat -= Quaternion::<T, A>::from_xyzw(z, w, y, x);
            assert_float_eq!(quat, Quaternion::from_xyzw(x - z, y - w, z - y, w - x));
        });
    }

    #[test]
    fn test_mul_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut quat = Quaternion::<T, A>::from_xyzw(x, y, z, w);
            quat *= w;
            assert_float_eq!(quat, Quaternion::from_xyzw(x * w, y * w, z * w, w * w));
        });
    }
}
