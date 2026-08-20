use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Scalar, Unaligned, Vector, Zero,
    backend::RotorBackend,
    length::TwoOrThree,
    utils::{specialize, specialize_23, transmute_generic, transmute_mut, transmute_ref},
};

/// A rotor representing rotation.
///
/// Rotors are a compact and efficient alternative to rotation matrices. If you
/// are familiar with complex numbers and quaternions, you already know how to
/// use rotors. This rotor type in 2D is mathematically equivalent to complex
/// numbers, and 3D rotors are mathematically equivalent to quaternions.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # Type aliases
///
/// - [`Rot2<T>`] for [`Rotor<2, T, Unaligned>`].
/// - [`Rot3<T>`] for [`Rotor<3, T, Unaligned>`].
/// - [`Rot2A<T>`] for [`Rotor<2, T, Aligned>`].
/// - [`Rot3A<T>`] for [`Rotor<3, T, Aligned>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using raw rotor
/// elements directly, and instead use higher level helper functions.
///
/// > You may have an easier time reading documentation specific to
/// > [2D](Rot2#representation-and-fields) and
/// > [3D](Rot3#representation-and-fields) first. This section explains the
/// > representation in a dimension agnostic manner.
///
/// A rotor stores one value for each basis plane of rotation, followed by a
/// scalar value `s`, which is always last in memory.
///
/// In 2D there is only one plane of rotation, `xy`. In 3D there are three
/// basis planes of rotation, `xy, xz, yz`. If additional dimensions are ever
/// supported, the pattern would continue with `N choose 2` basis planes, signs
/// determined by increasing index order, and planes stored in lexicographical
/// order. For example, in 4D this would be `xy, xz, xw, yz, yw, zw`.
///
/// In three dimensions or more, plane fields store
/// `plane_of_rotation * sin(angle/2)`, and `s` stores `cos(angle/2)`. The
/// half-angle is necessary to correctly handle vectors outside the plane of
/// rotation. In 2D, however, the entire vector space is the plane of rotation,
/// thus we make 2D a special case and optimize the representation: `xy` stores
/// `sin(angle)` and `s` stores `cos(angle)`.
///
/// In Geometric Algebra terms, this type is precisely defined as `R = e^B` with
/// vector multiplication `vR` for `N <= 2`, and `R = e^(B/2)` with vector
/// multiplication `R~vR` for `N >= 3`. Note that the latter differs from the
/// traditional rotor convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Rotor<2, T, A>`] is a transparent wrapper around [`Vector<2, T, A>`], and
/// [`Rotor<3, T, A>`] is a transparent wrapper around [`Vector<4, T, A>`].
///
/// If additional dimensions are ever supported, [`Rotor<N, T, A>`] would remain
/// a transparent wrapper around [`Vector<rotor_len(N), T, A>`], where
/// `rotor_len(N) = (N choose 2) + 1`.
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    pub(crate) <Length<N> as TwoOrThree>::Select<Vector<2, T, A>, Vector<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D rotor representing 2D rotation.
///
/// Rotors are a compact and efficient alternative to rotation matrices. If you
/// are familiar with complex numbers, you already know how to use rotors. This
/// 2D rotor type is mathematically equivalent to complex numbers.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # No SIMD alignment
///
/// [`Rot2<T>`] does not have SIMD alignment, for that use [`Rot2A<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/complex-number math, avoid using these
/// fields directly, and instead use higher level helper functions.
///
/// This type stores two elements, with this order in memory:
///
/// - `xy: T = sin(angle)`
/// - `s: T = cos(angle)`
///
/// Note that mathematically, this representation is incorrect. The correct
/// representation is `sin(angle/2), cos(angle/2)`. In three dimensions or more,
/// the half-angle is necessary to correctly handle vectors outside the plane of
/// rotation. In 2D, however, the entire vector space is the plane of rotation,
/// so the rotor can be represented directly by `sin(angle), cos(angle)`. This
/// trick makes many operations more efficient.
///
/// In Geometric Algebra terms, this type is precisely defined as `R = e^B` with
/// vector multiplication `vR`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot2<T> = Rotor<2, T, Unaligned>;

/// A 3D rotor representing 3D rotation.
///
/// Rotors are a compact and efficient alternative to rotation matrices. If you
/// are familiar with quaternions, you already know how to use rotors. A 3D
/// rotor is mathematically equivalent to a quaternion.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # No SIMD alignment
///
/// [`Rot3<T>`] does not have SIMD alignment, for that use [`Rot3A<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using these fields
/// directly, and instead use higher level helper functions.
///
/// This type stores four elements, with this order in memory:
///
/// - `xy: T = plane_of_rotation.xy * sin(angle/2)`
/// - `xz: T = plane_of_rotation.xz * sin(angle/2)`
/// - `yz: T = plane_of_rotation.yz * sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Each plane element rotates one axis to another (e.g., `xy` rotates `+X` to
/// `+Y`). This representation uses lexicographical ordering `xy, xz, yz`, which
/// differs from right-hand rule conventions that might expect `yz, zx, xy`
/// (which is `x, y, z` in axis angle notation).
///
/// In Geometric Algebra terms, this type is precisely defined as `R = e^(B/2)`
/// with vector multiplication `R~vR`. This differs from the traditional
/// convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot3<T> = Rotor<3, T, Unaligned>;

/// A 2D rotor representing 2D rotation.
///
/// Rotors are a compact and efficient alternative to rotation matrices. If you
/// are familiar with complex numbers, you already know how to use rotors. This
/// 2D rotor type is mathematically equivalent to complex numbers.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rot2A<T>`] has SIMD alignment. For no SIMD use
/// [`Rot2<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/complex-number math, avoid using these
/// fields directly, and instead use higher level helper functions.
///
/// This type stores two elements, with this order in memory:
///
/// - `xy: T = sin(angle)`
/// - `s: T = cos(angle)`
///
/// Note that mathematically, this representation is incorrect. The correct
/// representation is `sin(angle/2), cos(angle/2)`. In three dimensions or more,
/// the half-angle is necessary to correctly handle vectors outside the plane of
/// rotation. In 2D, however, the entire vector space is the plane of rotation,
/// so the rotor can be represented directly by `sin(angle), cos(angle)`. This
/// trick makes many operations more efficient.
///
/// In Geometric Algebra terms, this type is precisely defined as `R = e^B` with
/// vector multiplication `vR`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot2A<T> = Rotor<2, T, Aligned>;

/// A 3D rotor representing 3D rotation.
///
/// Rotors are a compact and efficient alternative to rotation matrices. If you
/// are familiar with quaternions, you already know how to use rotors. A 3D
/// rotor is mathematically equivalent to a quaternion.
///
/// > If you are curious about the underlying math, rotors come from Geometric
/// > Algebra. I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy)
/// > for learning more.
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
/// Use `rotor.normalize()` to maintain precision.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rot3A<T>`] has SIMD alignment. For no SIMD use
/// [`Rot3<T>`].
///
/// # Representation and Fields
///
/// Unless you are familiar with rotor/quaternion math, avoid using these fields
/// directly, and instead use higher level helper functions.
///
/// This type stores four elements, with this order in memory:
///
/// - `xy: T = plane_of_rotation.xy * sin(angle/2)`
/// - `xz: T = plane_of_rotation.xz * sin(angle/2)`
/// - `yz: T = plane_of_rotation.yz * sin(angle/2)`
/// - `s: T = cos(angle/2)`
///
/// Each plane element rotates one axis to another (e.g., `xy` rotates `+X` to
/// `+Y`). This representation uses lexicographical ordering `xy, xz, yz`, which
/// differs from right-hand rule conventions that might expect `yz, zx, xy`
/// (which is `x, y, z` in axis angle notation).
///
/// In Geometric Algebra terms, this type is precisely defined as `R = e^(B/2)`
/// with vector multiplication `R~vR`. This differs from the traditional
/// convention, `R = e^(-B/2)` and `RvR~`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot3A<T> = Rotor<3, T, Aligned>;

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
    // Note about `new`: Because rotor fields are not obvious, it would be nice
    // to have `Rot2 { xy: ..., s: ... }` syntax, however that is impossible to
    // do with a const generic struct.

    /// Creates a 2D rotor from raw elements `xy, s`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level constructors.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn new(xy: T, s: T) -> Self {
        Self(Vector::<2, T, A>::new(xy, s))
    }

    /// Creates a rotor from a raw-element array `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level constructors.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 2]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Converts a rotor to a raw-element array `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 2] {
        self.0.to_array()
    }

    /// Returns a reference to a rotor's raw elements `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 2] {
        self.0.as_array()
    }

    /// Returns a mutable reference to a rotor's raw elements `[xy, s]`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 2] {
        self.0.as_mut_array()
    }

    /// Creates a rotor from a raw-element vector `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level constructors.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_vector(vector: Vector<2, T, A>) -> Self {
        Self(vector)
    }

    /// Converts a rotor to a raw-element vector `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    #[inline]
    #[must_use]
    pub const fn to_vector(self) -> Vector<2, T, A> {
        self.0
    }

    /// Returns a reference to a rotor's fields as a raw-element vector
    /// `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    #[inline]
    #[must_use]
    pub const fn as_vector(&self) -> &Vector<2, T, A> {
        &self.0
    }

    /// Returns a mutable reference to a rotor's fields as a raw-element vector
    /// `(xy, s)`.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, in 2D, these fields are treated as `sin(angle), cos(angle)`,
    /// not `sin(angle/2), cos(angle/2)`. This special case makes 2D rotors more
    /// efficient. 3D rotors (and higher dimensional rotors, if those will ever
    /// be supported) do use half-angle.
    #[inline]
    #[must_use]
    pub const fn as_mut_vector(&mut self) -> &mut Vector<2, T, A> {
        &mut self.0
    }

    #[inline(always)]
    #[track_caller]
    fn conjugate_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::new(-self.xy, self.s)
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
    fn debug_backend(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        T: Debug,
    {
        self.0.fmt(f)
    }

    #[inline]
    fn display_backend(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        T: Display,
    {
        self.0.fmt(f)
    }

    #[inline(always)]
    fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    fn neg_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    fn add_backend(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn sub_backend(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn mul_scalar_backend(self, rhs: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }

    #[inline(always)]
    #[track_caller]
    fn vector_mul_backend(vector: Vector<2, T, A>, rhs: Self) -> Vector<2, T, A>
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        Vector::<2, T, A>::new(
            vector.x * rhs.s - vector.y * rhs.xy,
            vector.x * rhs.xy + vector.y * rhs.s,
        )
    }

    #[inline(always)]
    #[track_caller]
    fn mul_backend(self, rhs: Self) -> Self
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        Self::new(
            self.xy * rhs.s + self.s * rhs.xy,
            self.s * rhs.s - self.xy * rhs.xy,
        )
    }

    #[inline(always)]
    #[track_caller]
    fn div_scalar_backend(self, rhs: T) -> Self
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
    // Note about `new`: Because rotor fields are not obvious, it would be nice
    // to have `Rot3 { xy: ..., xz: ..., yz: ..., s: ... }` syntax, however that
    // is impossible to do with a const generic struct.

    /// Creates a 3D rotor from raw elements `xy, xz, yz, s`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level constructors.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn new(xy: T, xz: T, yz: T, s: T) -> Self {
        Self(Vector::<4, T, A>::new(xy, xz, yz, s))
    }

    /// Creates a rotor from a raw-element array `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level constructors.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_array(array: [T; 4]) -> Self {
        Self(Vector::from_array(array))
    }

    /// Converts a rotor to a raw-element array `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    #[inline]
    #[must_use]
    pub const fn to_array(self) -> [T; 4] {
        self.0.to_array()
    }

    /// Returns a reference to a rotor's raw elements `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    #[inline]
    #[must_use]
    pub const fn as_array(&self) -> &[T; 4] {
        self.0.as_array()
    }

    /// Returns a mutable reference to a rotor's raw elements `[xy, xz, yz, s]`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    #[inline]
    #[must_use]
    pub const fn as_mut_array(&mut self) -> &mut [T; 4] {
        self.0.as_mut_array()
    }

    /// Creates a rotor from a raw-element vector `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level constructors.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    ///
    /// # Unchecked
    ///
    /// This does not check that the resulting rotor is normalized. It is up to
    /// the caller to provide normalized values or to normalize the resulting
    /// rotor.
    #[inline]
    #[must_use]
    pub const fn from_vector(vector: Vector<4, T, A>) -> Self {
        Self(vector)
    }

    /// Converts a rotor to a raw-element vector `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    #[inline]
    #[must_use]
    pub const fn to_vector(self) -> Vector<4, T, A> {
        self.0
    }

    /// Returns a reference to a rotor's fields as a raw-element vector
    /// `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    #[inline]
    #[must_use]
    pub const fn as_vector(&self) -> &Vector<4, T, A> {
        &self.0
    }

    /// Returns a mutable reference to a rotor's fields as a raw-element vector
    /// `(xy, xz, yz, s)`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// function directly, and instead use higher level methods.
    ///
    /// Remember, plane fields are treated as
    /// `plane_of_rotation * sin(angle/2)`, and `s` is treated as
    /// `cos(angle/2)`.
    #[inline]
    #[must_use]
    pub const fn as_mut_vector(&mut self) -> &mut Vector<4, T, A> {
        &mut self.0
    }

    #[inline(always)]
    #[track_caller]
    fn conjugate_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self::new(-self.xy, -self.xz, -self.yz, self.s)
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
    fn debug_backend(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        T: Debug,
    {
        self.0.fmt(f)
    }

    #[inline]
    fn display_backend(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result
    where
        T: Display,
    {
        self.0.fmt(f)
    }

    #[inline(always)]
    fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    fn neg_backend(self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    fn add_backend(self, rhs: Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn sub_backend(self, rhs: Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    fn mul_scalar_backend(self, rhs: T) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }

    #[inline(always)]
    #[track_caller]
    fn vector_mul_backend(vector: Vector<3, T, A>, rhs: Self) -> Vector<3, T, A>
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        specialize!(<T as RotorBackend<3, A>>::rotor_vector_mul(vector, rhs))
    }

    #[inline(always)]
    #[track_caller]
    fn mul_backend(self, rhs: Self) -> Self
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        specialize!(<T as RotorBackend<3, A>>::rotor_mul(self, rhs))
    }

    #[inline(always)]
    #[track_caller]
    fn div_scalar_backend(self, rhs: T) -> Self
    where
        T: Div<Output = T>,
    {
        Self(self.0 / rhs)
    }
}

impl<const N: usize, T, A: Alignment> Clone for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
}

#[doc(hidden)]
#[repr(C)]
pub struct Rot2Fields<T> {
    /// The first and only basis plane element, rotating `+X` to `+Y`.
    ///
    /// Equal to `sin(angle)`.
    pub xy: T,
    /// The scalar part of a rotor.
    ///
    /// Equal to `cos(angle)`.
    pub s: T,
}

impl<T, A: Alignment> Deref for Rotor<2, T, A>
where
    T: Scalar,
{
    type Target = Rot2Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotor<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Rot2Fields<T>`.
        unsafe { transmute_ref::<Rotor<2, T, A>, Rot2Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotor<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotor<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `T`, and so begin with `Rot2Fields<T>`.
        unsafe { transmute_mut::<Rotor<2, T, A>, Rot2Fields<T>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Rot3Fields<T> {
    /// The first basis plane element, rotating `+X` to `+Y`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// field directly, and instead use higher level methods.
    ///
    /// Equal to `plane_of_rotation.xy * sin(angle/2)`.
    pub xy: T,
    /// The second basis plane element, rotating `+X` to `+Z`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// field directly, and instead use higher level methods.
    ///
    /// Equal to `plane_of_rotation.xz * sin(angle/2)`.
    pub xz: T,
    /// The third basis plane element, rotating `+Y` to `+Z`.
    ///
    /// Unless you are familiar with rotor/quaternion math, avoid using this
    /// field directly, and instead use higher level methods.
    ///
    /// Equal to `plane_of_rotation.yz * sin(angle/2)`.
    pub yz: T,
    /// The scalar part of a rotor.
    ///
    /// Unless you are familiar with rotor/complex-number math, avoid using this
    /// field directly, and instead use higher level methods.
    ///
    /// Equal to `cos(angle/2)`.
    pub s: T,
}

impl<T, A: Alignment> Deref for Rotor<3, T, A>
where
    T: Scalar,
{
    type Target = Rot3Fields<T>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rot3Fields<T>`.
        unsafe { transmute_ref::<Rotor<3, T, A>, Rot3Fields<T>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Rotor<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Rotor<3, T, A>` is guaranteed to begin with 4 consecutive
        // values of `T`, and so begin with `Rot3Fields<T>`.
        unsafe { transmute_mut::<Rotor<3, T, A>, Rot3Fields<T>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        specialize_23!(Rotor::<N, T, A>::debug_backend(self, f))
    }
}

impl<const N: usize, T, A: Alignment> Display for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Display,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        specialize_23!(Rotor::<N, T, A>::display_backend(self, f))
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize_23!(Rotor::<N, T, A>::eq_backend(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        specialize_23!(Rotor::<N, T, A>::hash_backend(self, (state,)))
    }
}

impl<const N: usize, T, A: Alignment> Default for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
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

macro_rules! impl_neg {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Neg for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::neg_backend(self))
            }
        }

        impl<const N: usize, T, A: Alignment> Neg for &Rotor<N, T, A>
        where
        Length<N>: TwoOrThree,
        T: Scalar + Neg<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                -*self
            }
        }
    };
}
impl_neg!(
    /// Negates the sign of each element.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Add for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::add_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Add<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: &Self) -> Self::Output {
                self + *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self + rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                *self + *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = *self + rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: &Self) {
                *self = *self + rhs;
            }
        }
    };
}
impl_add!(
    /// Adds each element of a rotor to another rotor.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::sub_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: &Self) -> Self::Output {
                self - *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self - rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                *self - *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = *self - rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: &Self) {
                *self = *self - rhs;
            }
        }
    };
}
impl_sub!(
    /// Subtracts each element of a rotor from another rotor.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::mul_scalar_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_mul_scalar!(
    /// Multiplies every element by a uniform scalar.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::vector_mul_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                *self * *rhs
            }
        }
    };
}
impl_vector_mul!(
    /// Transforms a vector by a rotor.
    ///
    /// If the rotor is normalized, this just rotates the vector. If the rotor
    /// is not normalized, this also scales the vector by
    /// `rotor.length_squared()`.
    ///
    /// Because the library uses left multiplication, vectors always go on the
    /// left-hand side.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Self) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::mul_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Rotor<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Rotor<N, T, A>> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Rotor<N, T, A>) -> Self::Output {
                *self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = *self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Rotor<N, T, A>> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Rotor<N, T, A>) {
                *self = *self * *rhs;
            }
        }
    };
}
impl_mul!(
    /// Multiplies two rotors.
    ///
    /// The resulting rotor is equivalent to first applying the left rotor, then
    /// the right rotor.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_div_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Div<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                specialize_23!(Rotor::<N, T, A>::div_scalar_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                *self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for &Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            type Output = Rotor<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                *self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: T) {
                *self = *self / rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<&T> for Rotor<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: &T) {
                *self = *self / *rhs;
            }
        }
    };
}
impl_div_scalar!(
    /// Divides every element by a uniform scalar.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Send` the list also is, and the padding is `Send` too.
unsafe impl<const N: usize, T, A: Alignment> Send for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Send,
{
}

// SAFETY: Rotors are equivalent to consecutive values of `T` plus padding.
// Because `T` is `Sync` the list also is, and the padding is `Sync` too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Rotor<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Aligned, Mask, Quaternion, Rot2, Rot2A, Rot3, Rot3A, Rotor, Unaligned, Vec2A, Vec4A,
        Vector,
        test_utils::{assert_test_eq, for_types, random_iter},
        utils::PrimitiveFloatUtils,
    };

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Rot2<T>>(), size_of::<T>() * 2);
            assert_eq!(align_of::<Rot2<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rot3<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Rot3<T>>(), align_of::<T>());

            assert_eq!(size_of::<Rot2A<T>>(), size_of::<Vec2A<T>>());
            assert_eq!(align_of::<Rot2A<T>>(), align_of::<Vec2A<T>>());

            assert_eq!(size_of::<Rot3A<T>>(), size_of::<Vec4A<T>>());
            assert_eq!(align_of::<Rot3A<T>>(), align_of::<Vec4A<T>>());
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Rotor::<2, T, A>::IDENTITY,
                Rotor::<2, T, A>::new(T::ZERO, T::ONE)
            );
            assert_eq!(
                Rotor::<3, T, A>::IDENTITY,
                Rotor::<3, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
            );
        });
    }

    #[test]
    fn test_conjugate() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<2, T, A>>() {
                assert_test_eq!(rotor.conjugate(), Rotor::<2, T, A>::new(-rotor.xy, rotor.s));
            }
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(
                    rotor.conjugate(),
                    Rotor::<3, T, A>::new(-rotor.xy, -rotor.xz, -rotor.yz, rotor.s)
                );
            }
        });
    }

    #[test]
    fn test_dot() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<2, T, A>; 2]>() {
                assert_test_eq!(a.dot(b), a.0.dot(b.0));
            }
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                assert_test_eq!(a.dot(b), a.0.dot(b.0));
            }
        });
    }

    #[test]
    fn test_length_squared() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<2, T, A>>() {
                assert_test_eq!(rotor.length_squared(), rotor.0.length_squared());
            }
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(rotor.length_squared(), rotor.0.length_squared());
            }
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<2, T, Aligned>::from_vector(rotor.0.align())
            );
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<2, T, Unaligned>::from_vector(rotor.0.unalign())
            );

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<3, T, Aligned>::from_vector(rotor.0.align())
            );
            assert_eq!(
                rotor.to_alignment(),
                Rotor::<3, T, Unaligned>::from_vector(rotor.0.unalign())
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.align(),
                Rotor::<2, T, Aligned>::from_vector(rotor.0.align())
            );

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.align(),
                Rotor::<3, T, Aligned>::from_vector(rotor.0.align())
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.unalign(),
                Rotor::<2, T, Unaligned>::from_vector(rotor.0.unalign())
            );

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(
                rotor.unalign(),
                Rotor::<3, T, Unaligned>::from_vector(rotor.0.unalign())
            );
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            let rotor = Rotor::<2, T, A>::new(x, y);
            assert_eq!(rotor.xy, x);
            assert_eq!(rotor.s, y);

            let rotor = Rotor::<3, T, A>::new(x, y, z, w);
            assert_eq!(rotor.xy, x);
            assert_eq!(rotor.xz, y);
            assert_eq!(rotor.yz, z);
            assert_eq!(rotor.s, w);
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [mut x, mut y, mut z, mut w] = std::array::from_fn(|i| T::as_from(i + 1));

            let mut rotor = Rotor::<2, T, A>::new(x, y);
            assert_eq!(&mut rotor.xy, &mut x);
            assert_eq!(&mut rotor.s, &mut y);

            let mut rotor = Rotor::<3, T, A>::new(x, y, z, w);
            assert_eq!(&mut rotor.xy, &mut x);
            assert_eq!(&mut rotor.xz, &mut y);
            assert_eq!(&mut rotor.yz, &mut z);
            assert_eq!(&mut rotor.s, &mut w);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor:?}"), format!("{:?}", rotor.to_vector()));

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor:?}"), format!("{:?}", rotor.to_vector()));
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let rotor = Rotor::<2, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor}"), format!("{}", rotor.to_vector()));

            let rotor = Rotor::<3, T, A>(Vector::from_fn(|i| T::as_from(i + 3)));
            assert_eq!(format!("{rotor}"), format!("{}", rotor.to_vector()));
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|T: PrimitiveNumber, A| {
            for ([rotor, other], mask) in random_iter::<([Rotor<2, T, A>; 2], Mask<2, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor == other, rotor.as_vector() == other.as_vector());
            }
            for ([rotor, other], mask) in random_iter::<([Rotor<3, T, A>; 2], Mask<4, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor == other, rotor.as_vector() == other.as_vector());
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|T: PrimitiveNumber, A| {
            for ([rotor, other], mask) in random_iter::<([Rotor<2, T, A>; 2], Mask<2, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor != other, rotor.as_vector() != other.as_vector());
            }
            for ([rotor, other], mask) in random_iter::<([Rotor<3, T, A>; 2], Mask<4, T, A>)>() {
                let other = Rotor(mask.select(rotor.0, other.0));
                assert_eq!(rotor != other, rotor.as_vector() != other.as_vector());
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N: TwoOrThree, T: PrimitiveNumber, A| {
            assert_eq!(Rotor::<N, T, A>::default(), Rotor::IDENTITY);
        });
    }

    #[test]
    fn test_neg() {
        for_types!(|T: PrimitiveFloat, A| {
            for rotor in random_iter::<Rotor<2, T, A>>() {
                assert_test_eq!(-rotor, Rotor::<2, T, A>::new(-rotor.xy, -rotor.s));
            }
            for rotor in random_iter::<Rotor<3, T, A>>() {
                assert_test_eq!(
                    -rotor,
                    Rotor::<3, T, A>::new(-rotor.xy, -rotor.xz, -rotor.yz, -rotor.s)
                );
            }
        });
    }

    #[test]
    fn test_add() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<2, T, A>; 2]>() {
                assert_test_eq!(a + b, Rotor::<2, T, A>::new(a.xy + b.xy, a.s + b.s));
            }
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                assert_test_eq!(
                    a + b,
                    Rotor::<3, T, A>::new(a.xy + b.xy, a.xz + b.xz, a.yz + b.yz, a.s + b.s)
                );
            }
        });
    }

    #[test]
    fn test_sub() {
        for_types!(|T: PrimitiveFloat, A| {
            for [a, b] in random_iter::<[Rotor<2, T, A>; 2]>() {
                assert_test_eq!(a - b, Rotor::<2, T, A>::new(a.xy - b.xy, a.s - b.s));
            }
            for [a, b] in random_iter::<[Rotor<3, T, A>; 2]>() {
                assert_test_eq!(
                    a - b,
                    Rotor::<3, T, A>::new(a.xy - b.xy, a.xz - b.xz, a.yz - b.yz, a.s - b.s)
                );
            }
        });
    }

    #[test]
    fn test_mul_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotor, scalar) in random_iter::<(Rotor<2, T, A>, T)>() {
                assert_test_eq!(
                    rotor * scalar,
                    Rotor::<2, T, A>::new(rotor.xy * scalar, rotor.s * scalar)
                );
            }
            for (rotor, scalar) in random_iter::<(Rotor<3, T, A>, T)>() {
                assert_test_eq!(
                    rotor * scalar,
                    Rotor::<3, T, A>::new(
                        rotor.xy * scalar,
                        rotor.xz * scalar,
                        rotor.yz * scalar,
                        rotor.s * scalar
                    )
                );
            }
        });
    }

    #[test]
    fn test_vector_mul() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for vector in random_iter::<Vector<N, T, A>>() {
                if vector.is_finite() {
                    assert_test_eq!(vector * Rotor::IDENTITY, vector, 0.0 = -0.0);
                }
            }
        });
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, angle) in random_iter::<(Vector<2, T, A>, T)>() {
                let (sin, cos) = PrimitiveFloatUtils::sin_cos(angle);
                assert_test_eq!(
                    vector * Rotor::<2, T, A>::new(sin, cos),
                    vector.rotate(angle),
                    0.0 = -0.0
                );
            }

            for (vector, angle) in random_iter::<(Vector<3, T, A>, T)>() {
                let (half_sin, half_cos) = PrimitiveFloatUtils::sin_cos(angle / 2.0);
                if vector.is_finite() && half_sin.is_finite() && half_cos.is_finite() {
                    assert_test_eq!(
                        vector * Rotor::<3, T, A>::new(half_sin, 0.0, 0.0, half_cos),
                        vector.rotate_z(angle),
                        abs <= vector.length() * 1e-5,
                        0.0 = -0.0
                    );
                    assert_test_eq!(
                        vector * Rotor::<3, T, A>::new(0.0, half_sin, 0.0, half_cos),
                        vector.rotate_y(-angle),
                        abs <= vector.length() * 1e-5,
                        0.0 = -0.0
                    );
                    assert_test_eq!(
                        vector * Rotor::<3, T, A>::new(0.0, 0.0, half_sin, half_cos),
                        vector.rotate_x(angle),
                        abs <= vector.length() * 1e-5,
                        0.0 = -0.0
                    );
                }
            }

            for (vector, rotor) in [
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Rotor::<3, T, A>::new(0.8, 0.4, 0.3, 0.1),
                ),
                (
                    Vector::<3, T, A>::new(-4.1, 3.3, 10.3),
                    Rotor::<3, T, A>::IDENTITY,
                ),
            ]
            .into_iter()
            .chain(random_iter())
            {
                if !vector.is_finite() {
                    continue;
                }

                // TODO: Replace this with rotor normalize method once it exists
                let rotor = Rotor::<3, T, A>(
                    rotor
                        .0
                        .normalize_or(Rotor::<3, T, A>::IDENTITY.0)
                        .normalize(),
                );

                // TODO: Replace this with a `Matrix::from_rotor` test once it exists
                let quaternion = Quaternion::from_xyzw(rotor.yz, -rotor.xz, rotor.xy, rotor.s);

                assert_test_eq!(
                    vector * rotor,
                    vector * quaternion,
                    abs <= vector.abs().max_element() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_mul() {
        // TODO: Make this generic over `N` when `rotor.normalize` exists
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, [rotor_1, rotor_2]) in
                random_iter::<(Vector<2, T, A>, [Rotor<2, T, A>; 2])>()
            {
                if !vector.is_finite() || vector.length() > 1e5 {
                    continue;
                }

                let [rotor_1, rotor_2] = [rotor_1, rotor_2]
                    .map(|r| Rotor(r.0.normalize_or(Rotor::<2, T, A>::IDENTITY.0).normalize()));

                assert_test_eq!(
                    vector * (rotor_1 * rotor_2),
                    vector * rotor_1 * rotor_2,
                    abs <= vector.length() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
            for (vector, [rotor_1, rotor_2]) in
                random_iter::<(Vector<3, T, A>, [Rotor<3, T, A>; 2])>()
            {
                if !vector.is_finite() || vector.length() > 1e5 {
                    continue;
                }

                let [rotor_1, rotor_2] = [rotor_1, rotor_2]
                    .map(|r| Rotor(r.0.normalize_or(Rotor::<3, T, A>::IDENTITY.0).normalize()));

                assert_test_eq!(
                    vector * (rotor_1 * rotor_2),
                    vector * rotor_1 * rotor_2,
                    abs <= vector.length() * 1e-6 + 1e-4,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_div_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (rotor, scalar) in random_iter::<(Rotor<2, T, A>, T)>() {
                assert_test_eq!(
                    rotor / scalar,
                    Rotor::<2, T, A>::new(rotor.xy / scalar, rotor.s / scalar)
                );
            }
            for (rotor, scalar) in random_iter::<(Rotor<3, T, A>, T)>() {
                assert_test_eq!(
                    rotor / scalar,
                    Rotor::<3, T, A>::new(
                        rotor.xy / scalar,
                        rotor.xz / scalar,
                        rotor.yz / scalar,
                        rotor.s / scalar
                    )
                );
            }
        });
    }
}
