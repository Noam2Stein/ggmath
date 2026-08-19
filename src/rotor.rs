#[expect(unused_imports)]
use core::ops::{Deref, DerefMut};

use crate::{Alignment, Length, Scalar, Unaligned, Vector, length::TwoOrThree};

/// A const-generic rotor representing rotation.
///
/// Rotors are an efficient representation of rotations, which make various
/// operations faster and simpler than with matrices. You may already be
/// familiar with quaternions, which are identical to 3D rotors.
///
/// > Rotors come from Geometric Algebra. You do not need to understand any math
/// > in order to use this type, but if you want to anyway, I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy).
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
///
/// # Type aliases
///
/// - [`Rot2<T>`] for [`Rotor<2, T, Unaligned>`].
/// - [`Rot3<T>`] for [`Rotor<3, T, Unaligned>`].
/// - [`Rot2A<T>`] for [`Rotor<2, T, Aligned>`].
/// - [`Rot3A<T>`] for [`Rotor<3, T, Aligned>`].
///
/// # Fields
///
/// Unless you are familiar with rotor/complex-number math, you should avoid
/// using these fields directly, and instead use higher level helper functions.
///
/// > You may have an easier time reading the [2D](Rot2#fields) and
/// > [3D](Rot3#fields) documentation separately. This section explains fields
/// > in a dimension agnostic manner.
///
/// This rotor contains a scalar field `s`, and all bivector fields, with
/// increasing index order signs. For 2D this is: `xy, s`. For 3D, this is
/// `xy, xz, yz, s`. If additional dimensions are ever supported, the pattern
/// would continue, for example, for 4D, `xy, xz, xw, yz, yw, zw, s`.
///
/// For all dimensions but 2D, bivector fields are
/// `sin(angle/2) * plane_of_rotation`. For 2D, the bivector field `xy` is `sin(angle)`, and `s` is
/// `cos(angle)`. This is mathematically incorrect, but is more efficient.
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
///
/// The scalar field `s` is always last in memory. This enables a trick where
/// taking the bivector from a 3D rotor is a no-op with SIMD `vec4.xyz()`.
///
/// The bivector fields use lexicographical ordering.
#[expect(private_bounds)]
pub struct Rotor<const N: usize, T, A: Alignment>(
    <Length<N> as TwoOrThree>::Select<Vector<2, T, A>, Vector<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D rotor representing 2D rotation.
///
/// Rotors are an efficient representation of rotations, which make various
/// operations faster and simpler than with matrices. You may already be
/// familiar with complex numbers, which are identical to this 2D rotor type.
///
/// > Rotors come from Geometric Algebra. You do not need to understand any math
/// > in order to use this type, but if you want to anyway, I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy).
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
///
/// # No SIMD alignment
///
/// [`Rot2<T>`] does not have SIMD alignment, for that use [`Rot2A<T>`].
///
/// # Fields
///
/// Unless you are familiar with rotor/complex-number math, you should avoid
/// using these fields directly, and instead use higher level helper functions.
///
/// - `xy: T = sin(angle)`
/// - `s: T = cos(angle)`
///
/// Note that mathematically, this representation is incorrect. A rotor is
/// typically written as:
///
/// `cos(angle/2) + sin(angle/2) * plane_of_rotation`
///
/// The half-angle is necessary because of the sandwich product, which is used
/// to correctly handle vectors outside the plane of rotation. In 2D, however,
/// the entire vector space is the plane of rotation, so the rotor can be
/// represented directly by `cos(angle), sin(angle)`. This makes many operations
/// faster than with half-angle.
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
/// Rotors are an efficient representation of rotations, which make various
/// operations faster and simpler than with matrices. You may already be
/// familiar with complex numbers, which are identical to this 2D rotor type.
///
/// > Rotors come from Geometric Algebra. You do not need to understand any math
/// > in order to use this type, but if you want to anyway, I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy).
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rot2A<T>`] has SIMD alignment. For no SIMD use
/// [`Rot2<T>`].
///
/// # Fields
///
/// Unless you are familiar with rotor/complex-number math, you should avoid
/// using these fields directly, and instead use higher level helper functions.
///
/// - `xy: T = sin(angle)`
/// - `s: T = cos(angle)`
///
/// Note that mathematically, this representation is incorrect. A rotor is
/// typically written as:
///
/// `cos(angle/2) + sin(angle/2) * plane_of_rotation`
///
/// The half-angle is necessary because of the sandwich product, which is used
/// to correctly handle vectors outside the plane of rotation. In 2D, however,
/// the entire vector space is the plane of rotation, so the rotor can be
/// represented directly by `cos(angle), sin(angle)`. This makes many operations
/// faster than with half-angle.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot2A<T> = Rotor<2, T, Unaligned>;

/// A 3D rotor representing 3D rotation.
///
/// Rotors are an efficient representation of rotations, which make various
/// operations faster and simpler than with matrices. You may already be
/// familiar with quaternions, which are identical to 3D rotors.
///
/// > Rotors come from Geometric Algebra. You do not need to understand any math
/// > in order to use this type, but if you want to anyway, I recommend
/// > [this resource](https://www.youtube.com/playlist?list=PLVuwZXwFua-0Ks3rRS4tIkswgUmDLqqRy).
///
/// This rotor is intended to be normalized, but may denormalize due to floating
/// point "error creep" which can occur when successive operations are applied.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Rot3A<T>`] has SIMD alignment. For no SIMD use
/// [`Rot3<T>`].
///
/// # Fields
///
/// Unless you are familiar with rotor/quaternion math, you should avoid using
/// these fields directly, and instead use higher level helper functions.
///
/// `plane` and `angle` refer to plane-angle/axis-angle rotation.
///
/// - `xy: T = plane.xy * sin(angle * 0.5)`
/// - `xz: T = plane.xz * sin(angle * 0.5)`
/// - `yz: T = plane.yz * sin(angle * 0.5)`
/// - `s: T = cos(angle * 0.5)`
///
/// Note that this does not follow the right-hand rule. This stores
/// `xy, xz, yz`, while the right-hand rule would mean storing `yz, zx, xy`.
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Rot3A<T> = Rotor<3, T, Unaligned>;
