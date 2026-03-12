use crate::{Affine, Aligned, Mask, Matrix, Quaternion, Unaligned, Vector};

/// A 2-dimensional vector.
///
/// # SIMD alignment
///
/// `Vec2<T>` has SIMD alignment for appropriate scalar types. See [`Vec2U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec2<T> = Vector<2, T, Aligned>;

/// A 3-dimensional vector.
///
/// # SIMD alignment
///
/// `Vec3<T>` has SIMD alignment for appropriate scalar types. See [`Vec3U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec3<T> = Vector<3, T, Aligned>;

/// A 4-dimensional vector.
///
/// # SIMD alignment
///
/// `Vec4<T>` has SIMD alignment for appropriate scalar types. See [`Vec4U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// `w: T`
///
/// The fourth element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec4<T> = Vector<4, T, Aligned>;

/// A 2-dimensional vector.
///
/// # No SIMD alignment
///
/// `Vec2U<T>` does not have SIMD alignment. See [`Vec2<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec2U<T> = Vector<2, T, Unaligned>;

/// A 3-dimensional vector.
///
/// # No SIMD alignment
///
/// `Vec3U<T>` does not have SIMD alignment. See [`Vec3<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec3U<T> = Vector<3, T, Unaligned>;

/// A 4-dimensional vector.
///
/// # No SIMD alignment
///
/// `Vec4U<T>` does not have SIMD alignment. See [`Vec4<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x: T`
///
/// The first element of the vector.
///
/// `y: T`
///
/// The second element of the vector.
///
/// `z: T`
///
/// The third element of the vector.
///
/// `w: T`
///
/// The fourth element of the vector.
///
/// [`Alignment`]: crate::Alignment
pub type Vec4U<T> = Vector<4, T, Unaligned>;

/// A 2x2 column-major matrix.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Mat2<T>` has SIMD alignment for appropriate scalar types. See [`Mat2U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x_axis: Vec2<T>`
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `(1, 0)`.
///
/// `y_axis: Vec2<T>`
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 1)`.
///
/// [`from_columns`]: Matrix::from_columns
/// [`Alignment`]: crate::Alignment
pub type Mat2<T> = Matrix<2, T, Aligned>;

/// A 3x3 column-major matrix.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Mat3<T>` has SIMD alignment for appropriate scalar types. See [`Mat3U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x_axis: Vec3<T>`
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `(1, 0, 0)`.
///
/// `y_axis: Vec3<T>`
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 1, 0)`.
///
/// `z_axis: Vec3<T>`
///
/// The third column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 0, 1)`.
///
/// [`from_columns`]: Matrix::from_columns
/// [`Alignment`]: crate::Alignment
pub type Mat3<T> = Matrix<3, T, Aligned>;

/// A 4x4 column-major matrix.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Mat4<T>` has SIMD alignment for appropriate scalar types. See [`Mat4U<T>`]
/// for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x_axis: Vec4<T>`
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `(1, 0, 0, 0)`.
///
/// `y_axis: Vec4<T>`
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 1, 0, 0)`.
///
/// `z_axis: Vec4<T>`
///
/// The third column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 0, 1, 0)`.
///
/// `w_axis: Vec4<T>`
///
/// The fourth column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 0, 0, 1)`.
///
/// [`from_columns`]: Matrix::from_columns
/// [`Alignment`]: crate::Alignment
pub type Mat4<T> = Matrix<4, T, Aligned>;

/// A 2x2 column-major matrix.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # No SIMD alignment
///
/// `Mat2U<T>` does not have SIMD alignment. See [`Mat2<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x_axis: Vec2U<T>`
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `(1, 0)`.
///
/// `y_axis: Vec2U<T>`
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 1)`.
///
/// [`from_columns`]: Matrix::from_columns
/// [`Alignment`]: crate::Alignment
pub type Mat2U<T> = Matrix<2, T, Unaligned>;

/// A 3x3 column-major matrix.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # No SIMD alignment
///
/// `Mat3U<T>` does not have SIMD alignment. See [`Mat3<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x_axis: Vec3U<T>`
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `(1, 0, 0)`.
///
/// `y_axis: Vec3U<T>`
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 1, 0)`.
///
/// `z_axis: Vec3U<T>`
///
/// The third column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 0, 1)`.
///
/// [`from_columns`]: Matrix::from_columns
/// [`Alignment`]: crate::Alignment
pub type Mat3U<T> = Matrix<3, T, Unaligned>;

/// A 4x4 column-major matrix.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # No SIMD alignment
///
/// `Mat4U<T>` does not have SIMD alignment. See [`Mat4<T>`] for a SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `x_axis: Vec4U<T>`
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `(1, 0, 0, 0)`.
///
/// `y_axis: Vec4U<T>`
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 1, 0, 0)`.
///
/// `z_axis: Vec4U<T>`
///
/// The third column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 0, 1, 0)`.
///
/// `w_axis: Vec4U<T>`
///
/// The fourth column of the matrix.
///
/// This represents the result of multiplying the matrix by `(0, 0, 0, 1)`.
///
/// [`from_columns`]: Matrix::from_columns
/// [`Alignment`]: crate::Alignment
pub type Mat4U<T> = Matrix<4, T, Unaligned>;

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

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Affines are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Affine2<T>` has SIMD alignment for appropriate scalar types. See
/// [`Affine2U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `matrix: Mat2<T>`
///
/// The part representing rotation, scaling and shear.
///
/// `translation: Vec2<T>`
///
/// The part representing translation.
///
/// [`from_columns`]: Affine::from_columns
/// [`Alignment`]: crate::Alignment
pub type Affine2<T> = Affine<2, T, Aligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Affines are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Affine3<T>` has SIMD alignment for appropriate scalar types. See
/// [`Affine3U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `matrix: Mat3<T>`
///
/// The part representing rotation, scaling and shear.
///
/// `translation: Vec3<T>`
///
/// The part representing translation.
///
/// [`from_columns`]: Affine::from_columns
/// [`Alignment`]: crate::Alignment
pub type Affine3<T> = Affine<3, T, Aligned>;

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Affines are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Affine2U<T>` does not have SIMD alignment. See [`Affine2<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `matrix: Mat2U<T>`
///
/// The part representing rotation, scaling and shear.
///
/// `translation: Vec2U<T>`
///
/// The part representing translation.
///
/// [`from_columns`]: Affine::from_columns
/// [`Alignment`]: crate::Alignment
pub type Affine2U<T> = Affine<2, T, Unaligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Affines are currently missing most functionality. See [`from_columns`] for
/// raw construction.
///
/// # SIMD alignment
///
/// `Affine3U<T>` does not have SIMD alignment. See [`Affine3<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// # Fields
///
/// `matrix: Mat3U<T>`
///
/// The part representing rotation, scaling and shear.
///
/// `translation: Vec3U<T>`
///
/// The part representing translation.
///
/// [`from_columns`]: Affine::from_columns
/// [`Alignment`]: crate::Alignment
pub type Affine3U<T> = Affine<3, T, Unaligned>;

/// A 2-element vector mask.
///
/// # SIMD alignment
///
/// `Mask2<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask2U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask2<T> = Mask<2, T, Aligned>;

/// A 3-element vector mask.
///
/// # SIMD alignment
///
/// `Mask3<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask3U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask3<T> = Mask<3, T, Aligned>;

/// A 4-element vector mask.
///
/// # SIMD alignment
///
/// `Mask4<T>` has SIMD alignment for appropriate scalar types. See
/// [`Mask4U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask4<T> = Mask<4, T, Aligned>;

/// A 2-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask2U<T>` does not have SIMD alignment. See [`Mask2<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask2U<T> = Mask<2, T, Unaligned>;

/// A 3-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask3U<T>` does not have SIMD alignment. See [`Mask3<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask3U<T> = Mask<3, T, Unaligned>;

/// A 4-element vector mask.
///
/// # No SIMD alignment
///
/// `Mask4U<T>` does not have SIMD alignment. See [`Mask4<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Alignment`]: crate::Alignment
pub type Mask4U<T> = Mask<4, T, Unaligned>;
