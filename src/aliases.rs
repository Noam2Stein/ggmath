use crate::{Affine, Aligned, Mask, Matrix, Quaternion, Unaligned, Vector};

/// A 2-dimensional vector.
///
/// To initialize this type use the [`vec2`](crate::vec2) macro.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Vec2<T> = Vector<2, T, Aligned>;

/// A 3-dimensional vector.
///
/// To initialize this type use the [`vec3`](crate::vec3) macro.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Vec3<T> = Vector<3, T, Aligned>;

/// A 4-dimensional vector.
///
/// To initialize this type use the [`vec4`](crate::vec4) macro.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Vec4<T> = Vector<4, T, Aligned>;

/// A 2-dimensional vector.
///
/// To initialize this type use the [`vec2`](crate::vec2) macro.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 2]`.
pub type Vec2U<T> = Vector<2, T, Unaligned>;

/// A 3-dimensional vector.
///
/// To initialize this type use the [`vec3`](crate::vec3) macro.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 3]`.
pub type Vec3U<T> = Vector<3, T, Unaligned>;

/// A 4-dimensional vector.
///
/// To initialize this type use the [`vec4`](crate::vec4) macro.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 4]`.
pub type Vec4U<T> = Vector<4, T, Unaligned>;

/// A 2x2 column major matrix.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Mat2<T> = Matrix<2, T, Aligned>;

/// A 3x3 column major matrix.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Mat3<T> = Matrix<3, T, Aligned>;

/// A 4x4 column major matrix.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Mat4<T> = Matrix<4, T, Aligned>;

/// A 2x2 column major matrix.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 4]`.
pub type Mat2U<T> = Matrix<2, T, Unaligned>;

/// A 3x3 column major matrix.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 9]`.
pub type Mat3U<T> = Matrix<3, T, Unaligned>;

/// A 4x4 column major matrix.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 16]`.
pub type Mat4U<T> = Matrix<4, T, Unaligned>;

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Quat<T> = Quaternion<T, Aligned>;

/// A quaternion representing an orientation.
///
/// This quaternion is intended to be of unit length but may denormalize due to
/// floating point "error creep" which can occur when successive quaternion
/// operations are applied.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 4]`.
pub type QuatU<T> = Quaternion<T, Unaligned>;

/// A 2D affine transform, which can represent translation, rotation, scaling
/// and shear.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Affine2<T> = Affine<2, T, Aligned>;

/// A 3D affine transform, which can represent translation, rotation, scaling
/// and shear.
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Affine3<T> = Affine<3, T, Aligned>;

/// A 2D affine transform, which can represent translation, rotation, scaling
/// and shear.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 6]`.
pub type Affine2U<T> = Affine<2, T, Unaligned>;

/// A 3D affine transform, which can represent translation, rotation, scaling
/// and shear.
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[T; 12]`.
pub type Affine3U<T> = Affine<3, T, Unaligned>;

/// A 2-component vector mask.
///
/// To initialize this type use [`Mask2::new`].
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Mask2<T> = Mask<2, T, Aligned>;

/// A 3-component vector mask.
///
/// To initialize this type use [`Mask3::new`].
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Mask3<T> = Mask<3, T, Aligned>;

/// A 4-component vector mask.
///
/// To initialize this type use [`Mask4::new`].
///
/// # SIMD Alignment
///
/// This type may be SIMD-aligned depending on the target architecture.
pub type Mask4<T> = Mask<4, T, Aligned>;

/// A 2-component vector mask.
///
/// To initialize this type use [`Mask2U::new`].
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[bool; 2]`.
pub type Mask2U<T> = Mask<2, T, Unaligned>;

/// A 3-component vector mask.
///
/// To initialize this type use [`Mask3U::new`].
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[bool; 3]`.
pub type Mask3U<T> = Mask<3, T, Unaligned>;

/// A 4-component vector mask.
///
/// To initialize this type use [`Mask4U::new`].
///
/// # No SIMD Alignment
///
/// This type is not SIMD-aligned and has the memory layout of `[bool; 4]`.
pub type Mask4U<T> = Mask<4, T, Unaligned>;
