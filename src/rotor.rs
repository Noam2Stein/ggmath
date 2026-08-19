#[expect(unused_imports)]
use core::ops::{Deref, DerefMut};

use crate::{
    Alignment, Length, One, Scalar, Unaligned, Vector, Zero, length::TwoOrThree,
    utils::transmute_generic,
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
/// Unless you are familiar with rotor/quaternion math, avoid using these fields
/// directly, and instead use higher level helper functions.
///
/// > You may have an easier time reading documentation specific to
/// > [2D](Rot2#representation-and-fields) and
/// > [3D](Rot3#representation-and-fields) first. This section explains fields
/// > in a dimension agnostic manner.
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
    <Length<N> as TwoOrThree>::Select<Vector<2, T, A>, Vector<4, T, A>>,
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
/// - `xy: T = plane_of_rotation.xy * sin(angle * 0.5)`
/// - `xz: T = plane_of_rotation.xz * sin(angle * 0.5)`
/// - `yz: T = plane_of_rotation.yz * sin(angle * 0.5)`
/// - `s: T = cos(angle * 0.5)`
///
/// This representation uses lexicographical ordering `xy, xz, yz`, which
/// differs from right-hand rule conventions that might expect `yz, zx, xy`.
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
/// - `xy: T = plane_of_rotation.xy * sin(angle * 0.5)`
/// - `xz: T = plane_of_rotation.xz * sin(angle * 0.5)`
/// - `yz: T = plane_of_rotation.yz * sin(angle * 0.5)`
/// - `s: T = cos(angle * 0.5)`
///
/// This representation uses lexicographical ordering `xy, xz, yz`, which
/// differs from right-hand rule conventions that might expect `yz, zx, xy`.
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

impl<T, A: Alignment> Rotor<2, T, A>
where
    T: Scalar,
{
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
}

impl<T, A: Alignment> Rotor<3, T, A>
where
    T: Scalar,
{
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
}
