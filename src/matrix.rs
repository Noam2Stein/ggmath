use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, SupportedLength, Unaligned, Vector,
    constants::{Nan, One, Zero},
    utils::{Repr3, Repr4, transmute_generic, transmute_mut, transmute_ref},
};

mod float;
#[cfg(feature = "wide")]
mod wide;
#[cfg(feature = "wide")]
mod wide_float;

/// An `N`x`N` column-major matrix of type `T`.
///
/// `A` controls SIMD alignment and is either [`Aligned`] or [`Unaligned`]. See
/// [`Alignment`] for more details.
///
/// Most constructors are dimension specific. See [`from_columns`] for raw
/// construction.
///
/// # Type aliases
///
/// - [`Mat2<T>`] for `Matrix<2, T, Aligned>`.
/// - [`Mat3<T>`] for `Matrix<3, T, Aligned>`.
/// - [`Mat4<T>`] for `Matrix<4, T, Aligned>`.
/// - [`Mat2U<T>`] for `Matrix<2, T, Unaligned>`.
/// - [`Mat3U<T>`] for `Matrix<3, T, Unaligned>`.
/// - [`Mat4U<T>`] for `Matrix<4, T, Unaligned>`.
///
/// # Fields
///
/// `x_axis: Vector<N, T, Aligned>` (for lengths `2`, `3`, `4`)
///
/// The first column of the matrix.
///
/// This represents the result of multiplying the matrix by `Vector::X`.
///
/// `y_axis: Vector<N, T, Aligned>` (for lengths `2`, `3`, `4`)
///
/// The second column of the matrix.
///
/// This represents the result of multiplying the matrix by `Vector::Y`.
///
/// `z_axis: Vector<N, T, Aligned>` (for lengths `3`, `4`)
///
/// The third column of the matrix.
///
/// This represents the result of multiplying the matrix by `Vector::Z`.
///
/// `w_axis: Vector<N, T, Aligned>` (for lengths `4`)
///
/// The fourth column of the matrix.
///
/// This represents the result of multiplying the matrix by `Vector::W`.
///
/// # Memory layout
///
/// `Matrix<N, T, A>` contains `N` consecutive values of [`Vector<N, T, A>`]
/// with no additional padding.
///
/// For `N = 2` this type has the size and alignment of [`Vector<4, T, A>`].
///
/// For `N = 3` and `N = 4` this type has the size and alignment of
/// `[Vector<N, T, A>; N]`.
///
/// [`from_columns`]: Self::from_columns
/// [`Mat2<T>`]: crate::Mat2
/// [`Mat3<T>`]: crate::Mat3
/// [`Mat4<T>`]: crate::Mat4
/// [`Mat2U<T>`]: crate::Mat2U
/// [`Mat3U<T>`]: crate::Mat3U
/// [`Mat4U<T>`]: crate::Mat4U
/// [`Vec4<T>`]: crate::Vec4
#[repr(transparent)]
pub struct Matrix<const N: usize, T, A: Alignment>(
    #[expect(clippy::type_complexity)]
    <Length<N> as SupportedLength>::Select<
        Vector<4, T, A>,
        Repr3<Vector<3, T, A>>,
        Repr4<Vector<4, T, A>>,
    >,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/// A 2x2 column-major matrix.
///
/// Linear transformations including 2D rotation and scale can be created using
/// functions [`from_diagonal`], [`from_angle`] and [`from_scale_angle`].
///
/// The resulting matrices can be used to transform 2D vectors using vector
/// multiplication.
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
/// [`from_diagonal`]: Mat2::from_diagonal
/// [`from_angle`]: Mat2::from_angle
/// [`from_scale_angle`]: Mat2::from_scale_angle
/// [`Alignment`]: crate::Alignment
pub type Mat2<T> = Matrix<2, T, Aligned>;

/// A 3x3 column-major matrix.
///
/// `Mat3` can be used for both 3D linear transformations and 2D affine
/// transformations. For 2D affine transformations, the [`Affine2`] type results
/// in better performance for some operations (note that benchmarks are
/// currently missing).
///
/// Linear transformations including 3D rotation and scale can be created using
/// functions [`from_diagonal`], [`from_rotation_x`], [`from_rotation_y`],
/// [`from_rotation_z`], [`from_quat`], [`from_axis_angle`] and [`from_euler`].
///
/// The resulting matrices can be used to transform 3D vectors using regular
/// vector multiplication.
///
/// Affine transformations including 2D translation, rotation and scale can be
/// created using functions [`from_scale`], [`from_translation`],
/// [`from_submatrix`], [`from_submatrix_translation`], [`from_angle`],
/// [`from_scale_angle`] and [`from_scale_angle_translation`].
///
/// The resulting matrices can be used to transform 2D vectors using
/// [`transform_point`] and [`transform_vector`].
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
/// [`Affine2`]: crate::Affine2
/// [`from_diagonal`]: Mat3::from_diagonal
/// [`from_rotation_x`]: Mat3::from_rotation_x
/// [`from_rotation_y`]: Mat3::from_rotation_y
/// [`from_rotation_z`]: Mat3::from_rotation_z
/// [`from_quat`]: Mat3::from_quat
/// [`from_axis_angle`]: Mat3::from_axis_angle
/// [`from_euler`]: Mat3::from_euler
/// [`from_scale`]: Mat3::from_scale
/// [`from_translation`]: Mat3::from_translation
/// [`from_submatrix`]: Mat3::from_submatrix
/// [`from_submatrix_translation`]: Mat3::from_submatrix_translation
/// [`from_angle`]: Mat3::from_angle
/// [`from_scale_angle`]: Mat3::from_scale_angle
/// [`from_scale_angle_translation`]: Mat3::from_scale_angle_translation
/// [`transform_point`]: Mat3::transform_point
/// [`transform_vector`]: Mat3::transform_vector
/// [`Alignment`]: crate::Alignment
pub type Mat3<T> = Matrix<3, T, Aligned>;

/// A 4x4 column-major matrix.
///
/// `Mat4` can be used for both linear transformations and perspective
/// projections. For affine transformations, the [`Affine3`] type results in
/// better performance for some operations (note that benchmarks are currently
/// missing).
///
/// Affine transformations including 3D translation, rotation and scale can be
/// created using functions such as [`from_scale`], [`from_translation`],
/// [`from_rotation_x`], [`from_rotation_y`] and [`from_rotation_z`],
/// [`from_quat`], and [`from_scale_rotation_translation`].
///
/// The resulting matrices can be used to transform 3D vectors using
/// [`transform_point`] and [`transform_vector`].
///
/// Left-handed projections can be created using functions such as
/// [`orthographic_lh`], [`perspective_lh`] and [`perspective_infinite_lh`].
/// Right-handed projections can be created using functions such as
/// [`orthographic_rh`], [`perspective_rh`] and [`perspective_infinite_rh`].
///
/// The resulting projections can be used to transform 3D vectors as points
/// using [`project_point`].
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
/// [`Affine3`]: crate::Affine3
/// [`from_scale`]: Mat4::from_scale
/// [`from_translation`]: Mat4::from_translation
/// [`from_rotation_x`]: Mat4::from_rotation_x
/// [`from_rotation_y`]: Mat4::from_rotation_y
/// [`from_rotation_z`]: Mat4::from_rotation_z
/// [`from_quat`]: Mat4::from_quat
/// [`from_scale_rotation_translation`]: Mat4::from_scale_rotation_translation
/// [`transform_point`]: Mat4::transform_point
/// [`transform_vector`]: Mat4::transform_vector
/// [`orthographic_lh`]: Mat4::orthographic_lh
/// [`perspective_lh`]: Mat4::perspective_lh
/// [`perspective_infinite_lh`]: Mat4::perspective_infinite_lh
/// [`orthographic_rh`]: Mat4::orthographic_rh
/// [`perspective_rh`]: Mat4::perspective_rh
/// [`perspective_infinite_rh`]: Mat4::perspective_infinite_rh
/// [`project_point`]: Mat4::project_point
/// [`Alignment`]: crate::Alignment
pub type Mat4<T> = Matrix<4, T, Aligned>;

/// A 2x2 column-major matrix.
///
/// Linear transformations including 2D rotation and scale can be created using
/// functions [`from_diagonal`], [`from_angle`] and [`from_scale_angle`].
///
/// The resulting matrices can be used to transform 2D vectors using vector
/// multiplication.
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
/// [`from_diagonal`]: Mat2U::from_diagonal
/// [`from_angle`]: Mat2U::from_angle
/// [`from_scale_angle`]: Mat2U::from_scale_angle
/// [`Alignment`]: crate::Alignment
pub type Mat2U<T> = Matrix<2, T, Unaligned>;

/// A 3x3 column-major matrix.
///
/// `Mat3U` can be used for both 3D linear transformations and 2D affine
/// transformations. For 2D affine transformations, the [`Affine2U`] type
/// results in better performance for some operations (note that benchmarks are
/// currently missing).
///
/// Linear transformations including 3D rotation and scale can be created using
/// functions [`from_diagonal`], [`from_rotation_x`], [`from_rotation_y`],
/// [`from_rotation_z`], [`from_quat`], [`from_axis_angle`] and [`from_euler`].
///
/// The resulting matrices can be used to transform 3D vectors using regular
/// vector multiplication.
///
/// Affine transformations including 2D translation, rotation and scale can be
/// created using functions [`from_scale`], [`from_translation`],
/// [`from_submatrix`], [`from_submatrix_translation`], [`from_angle`],
/// [`from_scale_angle`] and [`from_scale_angle_translation`].
///
/// The resulting matrices can be used to transform 2D vectors using
/// [`transform_point`] and [`transform_vector`].
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
/// [`Affine2U`]: crate::Affine2U
/// [`from_diagonal`]: Mat3U::from_diagonal
/// [`from_rotation_x`]: Mat3U::from_rotation_x
/// [`from_rotation_y`]: Mat3U::from_rotation_y
/// [`from_rotation_z`]: Mat3U::from_rotation_z
/// [`from_quat`]: Mat3U::from_quat
/// [`from_axis_angle`]: Mat3U::from_axis_angle
/// [`from_euler`]: Mat3U::from_euler
/// [`from_scale`]: Mat3U::from_scale
/// [`from_translation`]: Mat3U::from_translation
/// [`from_submatrix`]: Mat3U::from_submatrix
/// [`from_submatrix_translation`]: Mat3U::from_submatrix_translation
/// [`from_angle`]: Mat3U::from_angle
/// [`from_scale_angle`]: Mat3U::from_scale_angle
/// [`from_scale_angle_translation`]: Mat3U::from_scale_angle_translation
/// [`transform_point`]: Mat3U::transform_point
/// [`transform_vector`]: Mat3U::transform_vector
/// [`Alignment`]: crate::Alignment
pub type Mat3U<T> = Matrix<3, T, Unaligned>;

/// A 4x4 column-major matrix.
///
/// `Mat4U` can be used for both linear transformations and perspective
/// projections. For affine transformations, the [`Affine3U`] type results in
/// better performance for some operations (note that benchmarks are currently
/// missing).
///
/// Affine transformations including 3D translation, rotation and scale can be
/// created using functions such as [`from_scale`], [`from_translation`],
/// [`from_rotation_x`], [`from_rotation_y`] and [`from_rotation_z`],
/// [`from_quat`], and [`from_scale_rotation_translation`].
///
/// The resulting matrices can be used to transform 3D vectors using
/// [`transform_point`] and [`transform_vector`].
///
/// Left-handed projections can be created using functions such as
/// [`orthographic_lh`], [`perspective_lh`] and [`perspective_infinite_lh`].
/// Right-handed projections can be created using functions such as
/// [`orthographic_rh`], [`perspective_rh`] and [`perspective_infinite_rh`].
///
/// The resulting projections can be used to transform 3D vectors as points
/// using [`project_point`].
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
/// [`Affine3U`]: crate::Affine3U
/// [`from_scale`]: Mat4U::from_scale
/// [`from_translation`]: Mat4U::from_translation
/// [`from_rotation_x`]: Mat4U::from_rotation_x
/// [`from_rotation_y`]: Mat4U::from_rotation_y
/// [`from_rotation_z`]: Mat4U::from_rotation_z
/// [`from_quat`]: Mat4U::from_quat
/// [`from_scale_rotation_translation`]: Mat4U::from_scale_rotation_translation
/// [`transform_point`]: Mat4U::transform_point
/// [`transform_vector`]: Mat4U::transform_vector
/// [`orthographic_lh`]: Mat4U::orthographic_lh
/// [`perspective_lh`]: Mat4U::perspective_lh
/// [`perspective_infinite_lh`]: Mat4U::perspective_infinite_lh
/// [`orthographic_rh`]: Mat4U::orthographic_rh
/// [`perspective_rh`]: Mat4U::perspective_rh
/// [`perspective_infinite_rh`]: Mat4U::perspective_infinite_rh
/// [`project_point`]: Mat4U::project_point
/// [`Alignment`]: crate::Alignment
pub type Mat4U<T> = Matrix<4, T, Unaligned>;

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// A matrix with all elements set to `0`.
    ///
    /// This transforms all vectors to a zero vector. See [`IDENTITY`] for a
    /// matrix with no transformation.
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    pub const ZERO: Self = Self::from_columns(&[Vector::ZERO; N]);
}

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero + One,
{
    /// A matrix with no transformation.
    ///
    /// `IDENTITY` diagonal elements are `1` and all other elements are `0`.
    pub const IDENTITY: Self = Self::from_diagonal(Vector::ONE);
}

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// A matrix with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_columns(&[Vector::NAN; N]);
}

impl<const N: usize, T, A: Alignment> Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates a matrix from an array of column vectors.
    #[inline]
    #[must_use]
    pub const fn from_columns(columns: &[Vector<N, T, A>; N]) -> Self {
        // SAFETY: `Matrix<N, T, A>` contains `N` consecutive values of
        // `Vector<N, T, A>` with no additional padding.
        unsafe { transmute_generic::<[Vector<N, T, A>; N], Matrix<N, T, A>>(*columns) }
    }

    /// Creates a matrix by calling function `f` for each column index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a column
    /// vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_column_fn(|i| Vec4::new(i, i, i, 0));
    ///
    /// assert_eq!(matrix.column(0), Vec4::new(0, 0, 0, 0));
    /// assert_eq!(matrix.column(1), Vec4::new(1, 1, 1, 0));
    /// assert_eq!(matrix.column(2), Vec4::new(2, 2, 2, 0));
    /// assert_eq!(matrix.column(3), Vec4::new(3, 3, 3, 0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_column_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self::from_columns(&core::array::from_fn(f))
    }

    /// Creates a matrix with the diagonal set to `diagonal` and all other
    /// elements set to `0`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_diagonal(Vec4::new(2, 2, 2, 1));
    ///
    /// assert_eq!(matrix.column(0), Vec4::new(2, 0, 0, 0));
    /// assert_eq!(matrix.column(1), Vec4::new(0, 2, 0, 0));
    /// assert_eq!(matrix.column(2), Vec4::new(0, 0, 2, 0));
    /// assert_eq!(matrix.column(3), Vec4::new(0, 0, 0, 1));
    ///
    /// assert_eq!(matrix.row(0), Vec4::new(2, 0, 0, 0));
    /// assert_eq!(matrix.row(1), Vec4::new(0, 2, 0, 0));
    /// assert_eq!(matrix.row(2), Vec4::new(0, 0, 2, 0));
    /// assert_eq!(matrix.row(3), Vec4::new(0, 0, 0, 1));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_diagonal(diagonal: Vector<N, T, A>) -> Self
    where
        T: Zero,
    {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<2, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Matrix<2, T, A>, Matrix<N, T, A>>(
                    Matrix::<2, T, A>::from_columns(&[
                        Vector::<2, T, A>::new(diagonal.as_array_ref()[0], T::ZERO),
                        Vector::<2, T, A>::new(T::ZERO, diagonal.as_array_ref()[1]),
                    ]),
                )
            },

            // SAFETY: Because `N == 3`, `Matrix<3, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Matrix<3, T, A>, Matrix<N, T, A>>(
                    Matrix::<3, T, A>::from_columns(&[
                        Vector::<3, T, A>::new(diagonal.as_array_ref()[0], T::ZERO, T::ZERO),
                        Vector::<3, T, A>::new(T::ZERO, diagonal.as_array_ref()[1], T::ZERO),
                        Vector::<3, T, A>::new(T::ZERO, T::ZERO, diagonal.as_array_ref()[2]),
                    ]),
                )
            },

            // SAFETY: Because `N == 4`, `Matrix<4, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Matrix<4, T, A>, Matrix<N, T, A>>(
                    Matrix::<4, T, A>::from_columns(&[
                        Vector::<4, T, A>::new(
                            diagonal.as_array_ref()[0],
                            T::ZERO,
                            T::ZERO,
                            T::ZERO,
                        ),
                        Vector::<4, T, A>::new(
                            T::ZERO,
                            diagonal.as_array_ref()[1],
                            T::ZERO,
                            T::ZERO,
                        ),
                        Vector::<4, T, A>::new(
                            T::ZERO,
                            T::ZERO,
                            diagonal.as_array_ref()[2],
                            T::ZERO,
                        ),
                        Vector::<4, T, A>::new(
                            T::ZERO,
                            T::ZERO,
                            T::ZERO,
                            diagonal.as_array_ref()[3],
                        ),
                    ]),
                )
            },

            _ => unreachable!(),
        }
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
    /// # use ggmath::{Aligned, Mat2, Mat2U, Unaligned, Vec2, Vec2U};
    /// #
    /// let aligned = Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Mat2U::from_columns(&[Vec2U::new(1, 2), Vec2U::new(3, 4)]));
    ///
    /// let unaligned = Mat2U::from_columns(&[Vec2U::new(1, 2), Vec2U::new(3, 4)]);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)]));
    /// ```
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(&self) -> Matrix<N, T, A2> {
        match (N, A2::IS_ALIGNED == A::IS_ALIGNED) {
            // SAFETY: If `A` is `A2`, the types of the transmute are the same
            // and make it safe. Otherwhise, matrices with length `2` and `4`
            // are guaranteed to be made out of `N * N` consecutive values of
            // `T` with no padding. Meaning they have compatible layouts between
            // alignments.
            (2 | 4, _) | (_, true) => unsafe {
                transmute_generic::<Matrix<N, T, A>, Matrix<N, T, A2>>(*self)
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type, and `Matrix<N, T, A2>` and `Matrix<3, T, A2>`
            // are the same type.
            (3, false) => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                transmute_generic::<Matrix<3, T, A2>, Matrix<N, T, A2>>(
                    Matrix::<3, T, A2>::from_columns(&[
                        matrix.as_columns()[0].to_alignment(),
                        matrix.as_columns()[1].to_alignment(),
                        matrix.as_columns()[2].to_alignment(),
                    ]),
                )
            },

            _ => unreachable!(),
        }
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Mat2U, Vec2, Vec2U};
    /// #
    /// let unaligned = Mat2U::from_columns(&[Vec2U::new(1, 2), Vec2U::new(3, 4)]);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn align(&self) -> Matrix<N, T, Aligned> {
        self.to_alignment()
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Mat2U, Vec2, Vec2U};
    /// #
    /// let aligned = Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Mat2U::from_columns(&[Vec2U::new(1, 2), Vec2U::new(3, 4)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Matrix<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Returns a reference to the matrix's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns(&self) -> &[Vector<N, T, A>; N] {
        // SAFETY: `Matrix<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `Vector<N, T, A>`.
        unsafe { transmute_ref::<Matrix<N, T, A>, [Vector<N, T, A>; N]>(self) }
    }

    /// Returns a mutable reference to the matrix's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns_mut(&mut self) -> &mut [Vector<N, T, A>; N] {
        // SAFETY: `Matrix<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `Vector<N, T, A>`.
        unsafe { transmute_mut::<Matrix<N, T, A>, [Vector<N, T, A>; N]>(self) }
    }

    /// Returns the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the dimension of the
    /// matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn column(&self, index: usize) -> Vector<N, T, A> {
        self.as_columns()[index]
    }

    /// Returns a mutable reference to the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the dimension of the
    /// matrix.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn column_mut(&mut self, index: usize) -> &mut Vector<N, T, A> {
        &mut self.as_columns_mut()[index]
    }

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the dimension of the
    /// matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_columns(&[
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(0, 0, 0, 1),
    /// ]);
    ///
    /// assert_eq!(matrix.row(0), Vec4::new(1, 1, 1, 0));
    /// assert_eq!(matrix.row(1), Vec4::new(2, 2, 2, 0));
    /// assert_eq!(matrix.row(2), Vec4::new(3, 3, 3, 0));
    /// assert_eq!(matrix.row(3), Vec4::new(4, 4, 4, 1));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn row(&self, index: usize) -> Vector<N, T, A> {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type, and `Vector<2, T, A>` and `Vector<N, T, A>`
            // are the same type.
            2 => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    matrix.as_columns()[0].as_array_ref()[index],
                    matrix.as_columns()[1].as_array_ref()[index],
                ))
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type, and `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    matrix.as_columns()[0].as_array_ref()[index],
                    matrix.as_columns()[1].as_array_ref()[index],
                    matrix.as_columns()[2].as_array_ref()[index],
                ))
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type, and `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    matrix.as_columns()[0].as_array_ref()[index],
                    matrix.as_columns()[1].as_array_ref()[index],
                    matrix.as_columns()[2].as_array_ref()[index],
                    matrix.as_columns()[3].as_array_ref()[index],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Sets the row at the given index to the given value.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let mut matrix = Mat4::from_columns(&[
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(0, 0, 0, 1),
    /// ]);
    /// matrix.set_row(1, Vec4::new(5, 5, 5, 0));
    ///
    /// assert_eq!(matrix.column(0), Vec4::new(1, 5, 3, 4));
    /// assert_eq!(matrix.column(1), Vec4::new(1, 5, 3, 4));
    /// assert_eq!(matrix.column(2), Vec4::new(1, 5, 3, 4));
    /// assert_eq!(matrix.column(3), Vec4::new(0, 0, 0, 1));
    /// ```
    #[inline]
    #[track_caller]
    pub const fn set_row(&mut self, index: usize, value: Vector<N, T, A>) {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type.
            2 => unsafe {
                let matrix = transmute_mut::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                matrix.as_columns_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                matrix.as_columns_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type.
            3 => unsafe {
                let matrix = transmute_mut::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                matrix.as_columns_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                matrix.as_columns_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
                matrix.as_columns_mut()[2].as_array_mut()[index] = value.as_array_ref()[2];
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type.
            4 => unsafe {
                let matrix = transmute_mut::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                matrix.as_columns_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                matrix.as_columns_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
                matrix.as_columns_mut()[2].as_array_mut()[index] = value.as_array_ref()[2];
                matrix.as_columns_mut()[3].as_array_mut()[index] = value.as_array_ref()[3];
            },

            _ => unreachable!(),
        }
    }

    /// Returns the transpose of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_columns(&[
    ///     Vec4::new(1, 1, 1, 0),
    ///     Vec4::new(2, 2, 2, 0),
    ///     Vec4::new(3, 3, 3, 0),
    ///     Vec4::new(4, 4, 4, 1),
    /// ]);
    /// assert_eq!(
    ///     matrix.transpose(),
    ///     Mat4::from_columns(&[
    ///         Vec4::new(1, 2, 3, 4),
    ///         Vec4::new(1, 2, 3, 4),
    ///         Vec4::new(1, 2, 3, 4),
    ///         Vec4::new(0, 0, 0, 1),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn transpose(&self) -> Self {
        Self::from_column_fn(|i| self.row(i))
    }

    /// Transforms the vector `rhs` by the transpose of `self`.
    ///
    /// Equivalent to `self.transpose() * rhs` but is faster and may return a
    /// slightly different value.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transpose_mul_vector(&self, rhs: Vector<N, T, A>) -> Vector<N, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Vector::from_fn(|i| self.column(i).dot(rhs))
    }

    /// Returns the diagonal of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_columns(&[
    ///     Vec4::new(1, 1, 1, 0),
    ///     Vec4::new(2, 2, 2, 0),
    ///     Vec4::new(3, 3, 3, 0),
    ///     Vec4::new(4, 4, 4, 1),
    /// ]);
    ///
    /// assert_eq!(matrix.diagonal(), Vec4::new(1, 2, 3, 1));
    /// ```
    #[inline]
    #[must_use]
    pub const fn diagonal(&self) -> Vector<N, T, A> {
        match N {
            // SAFETY: Because `N == 2`, `Vector<2, T, A>` and `Vector<N, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    self.as_columns()[0].as_array_ref()[0],
                    self.as_columns()[1].as_array_ref()[1],
                ))
            },

            // SAFETY: Because `N == 3`, `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    self.as_columns()[0].as_array_ref()[0],
                    self.as_columns()[1].as_array_ref()[1],
                    self.as_columns()[2].as_array_ref()[2],
                ))
            },

            // SAFETY: Because `N == 4`, `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    self.as_columns()[0].as_array_ref()[0],
                    self.as_columns()[1].as_array_ref()[1],
                    self.as_columns()[2].as_array_ref()[2],
                    self.as_columns()[3].as_array_ref()[3],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Multiplies `self` by a scaling vector `scale`.
    ///
    /// Equivalent to `self * Matrix::from_diagonal(scale)` but is faster. This
    /// may be inconsistent for NaNs and `-0.0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn mul_diagonal(&self, scale: Vector<N, T, A>) -> Self
    where
        T: Mul<Output = T>,
    {
        Self::from_column_fn(|i| self.column(i) * scale[i])
    }

    /// Returns the determinant of `self`.
    ///
    /// # Consistency
    ///
    /// Floating-point precision and integer overflow may be inconsistent across
    /// target architectures.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec2};
    /// #
    /// let matrix = Mat3::from_scale(Vec2::new(2, 2));
    ///
    /// assert_eq!(matrix.determinant(), 4);
    /// ```
    #[must_use]
    #[track_caller]
    pub fn determinant(&self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        match N {
            2 => {
                // SAFETY: Because `N == 2`, `Matrix<N, T, A>` is `Matrix<2, T, A>`.
                let matrix = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self) };

                matrix.x_axis.x * matrix.y_axis.y - matrix.x_axis.y * matrix.y_axis.x
            }
            3 => {
                // SAFETY: Because `N == 3`, `Matrix<N, T, A>` is `Matrix<3, T, A>`.
                let matrix = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self) };

                matrix.x_axis.cross(matrix.y_axis).dot(matrix.z_axis)
            }
            4 => {
                // SAFETY: Because `N == 4`, `Matrix<N, T, A>` is `Matrix<4, T, A>`.
                let matrix = unsafe { transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self) };

                let [m00, m01, m02, m03] = matrix.x_axis.to_array();
                let [m10, m11, m12, m13] = matrix.y_axis.to_array();
                let [m20, m21, m22, m23] = matrix.z_axis.to_array();
                let [m30, m31, m32, m33] = matrix.w_axis.to_array();

                let a2323 = m22 * m33 - m23 * m32;
                let a1323 = m21 * m33 - m23 * m31;
                let a1223 = m21 * m32 - m22 * m31;
                let a0323 = m20 * m33 - m23 * m30;
                let a0223 = m20 * m32 - m22 * m30;
                let a0123 = m20 * m31 - m21 * m30;

                m00 * (m11 * a2323 - m12 * a1323 + m13 * a1223)
                    - m01 * (m10 * a2323 - m12 * a0323 + m13 * a0223)
                    + m02 * (m10 * a1323 - m11 * a0323 + m13 * a0123)
                    - m03 * (m10 * a1223 - m11 * a0223 + m12 * a0123)
            }
            _ => unreachable!(),
        }
    }
}

impl<T, A: Alignment> Matrix<2, T, A>
where
    T: Scalar,
{
    /// Creates a matrix from a column-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let matrix = Mat2::from_column_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix, Mat2::from_columns(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_column_array(array: &[T; 4]) -> Self {
        Self::from_columns(&[
            Vector::<2, T, A>::new(array[0], array[1]),
            Vector::<2, T, A>::new(array[2], array[3]),
        ])
    }
}

impl<T, A: Alignment> Matrix<3, T, A>
where
    T: Scalar,
{
    /// Creates a matrix from a column-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let matrix = Mat2::from_column_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix, Mat2::from_columns(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_column_array(array: &[T; 9]) -> Self {
        Self::from_columns(&[
            Vector::<3, T, A>::new(array[0], array[1], array[2]),
            Vector::<3, T, A>::new(array[3], array[4], array[5]),
            Vector::<3, T, A>::new(array[6], array[7], array[8]),
        ])
    }

    /// Creates an affine transformation matrix from the given 2D `scale`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vector<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<3, T, A>::new(scale.as_array_ref()[0], T::ZERO, T::ZERO),
            Vector::<3, T, A>::new(T::ZERO, scale.as_array_ref()[1], T::ZERO),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates an affine transformation matrix from the given 2D `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vector<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<3, T, A>::X,
            Vector::<3, T, A>::Y,
            Vector::<3, T, A>::new(
                translation.as_array_ref()[0],
                translation.as_array_ref()[1],
                T::ONE,
            ),
        ])
    }

    /// Creates an affine transformation matrix from the given 2x2 matrix.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_submatrix(submatrix: &Matrix<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<3, T, A>::new(
                submatrix.column(0).as_array_ref()[0],
                submatrix.column(0).as_array_ref()[1],
                T::ZERO,
            ),
            Vector::<3, T, A>::new(
                submatrix.column(1).as_array_ref()[0],
                submatrix.column(1).as_array_ref()[1],
                T::ZERO,
            ),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Creates an affine transformation matrix from the given 2x2 matrix and 2D
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 2D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_submatrix_translation(
        submatrix: &Matrix<2, T, A>,
        translation: Vector<2, T, A>,
    ) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<3, T, A>::new(
                submatrix.column(0).as_array_ref()[0],
                submatrix.column(0).as_array_ref()[1],
                T::ZERO,
            ),
            Vector::<3, T, A>::new(
                submatrix.column(1).as_array_ref()[0],
                submatrix.column(1).as_array_ref()[1],
                T::ZERO,
            ),
            Vector::<3, T, A>::new(
                translation.as_array_ref()[0],
                translation.as_array_ref()[1],
                T::ONE,
            ),
        ])
    }

    /// Returns a 2x2 matrix discarding the third column and row.
    #[inline]
    #[must_use]
    pub const fn submatrix(&self) -> Matrix<2, T, A> {
        Matrix::from_columns(&[
            Vector::<2, T, A>::new(
                self.column(0).as_array_ref()[0],
                self.column(0).as_array_ref()[1],
            ),
            Vector::<2, T, A>::new(
                self.column(1).as_array_ref()[0],
                self.column(1).as_array_ref()[1],
            ),
        ])
    }

    /// Returns the translation of an affine transformation matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec2, Vec3};
    /// #
    /// let matrix = Mat3::from_columns(&[
    ///     Vec3::new(1, 0, 0),
    ///     Vec3::new(0, 1, 0),
    ///     Vec3::new(6, 8, 1),
    /// ]);
    /// assert_eq!(matrix.translation(), Vec2::new(6, 8));
    /// ```
    #[inline]
    #[must_use]
    pub fn translation(&self) -> Vector<2, T, A> {
        self.z_axis.truncate()
    }

    /// Returns a 2x2 matrix discarding the given `column` and `row`.
    ///
    /// # Panics
    ///
    /// Panics if `column` or `row` are greater than `2`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn remove(&self, column: usize, row: usize) -> Matrix<2, T, A> {
        match (column, row) {
            (0, 0) => Matrix::from_columns(&[self.y_axis.yz(), self.z_axis.yz()]),
            (0, 1) => Matrix::from_columns(&[self.y_axis.xz(), self.z_axis.xz()]),
            (0, 2) => Matrix::from_columns(&[self.y_axis.xy(), self.z_axis.xy()]),
            (1, 0) => Matrix::from_columns(&[self.x_axis.yz(), self.z_axis.yz()]),
            (1, 1) => Matrix::from_columns(&[self.x_axis.xz(), self.z_axis.xz()]),
            (1, 2) => Matrix::from_columns(&[self.x_axis.xy(), self.z_axis.xy()]),
            (2, 0) => Matrix::from_columns(&[self.x_axis.yz(), self.y_axis.yz()]),
            (2, 1) => Matrix::from_columns(&[self.x_axis.xz(), self.y_axis.xz()]),
            (2, 2) => Matrix::from_columns(&[self.x_axis.xy(), self.y_axis.xy()]),
            _ => panic!("index out of bounds"),
        }
    }

    /// Transforms the given 2D vector as a point.
    ///
    /// Equivalent to `self * (point, 1)` but is faster.
    ///
    /// `self` must contain a valid affine transform, meaning the third row must
    /// be `(0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the third row of `self` is not `(0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_point(&self, point: Vector<2, T, A>) -> Vector<2, T, A>
    where
        T: PartialEq + Add<Output = T> + Mul<Output = T> + Zero + One,
    {
        #[cfg(assertions)]
        assert!(self.row(2) == Vector::<3, T, A>::Z);

        self.x_axis.xy() * point.x + self.y_axis.xy() * point.y + self.z_axis.xy()
    }

    /// Transforms the given 2D vector without applying translation.
    ///
    /// Equivalent to `self * (vector, 0)` but is faster.
    ///
    /// `self` must contain a valid affine transform, meaning the third row must
    /// be `(0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the third row of `self` is not `(0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_vector(&self, vector: Vector<2, T, A>) -> Vector<2, T, A>
    where
        T: PartialEq + Add<Output = T> + Mul<Output = T> + Zero + One,
    {
        #[cfg(assertions)]
        assert!(self.row(2) == Vector::<3, T, A>::Z);

        self.x_axis.xy() * vector.x + self.y_axis.xy() * vector.y
    }
}

impl<T, A: Alignment> Matrix<4, T, A>
where
    T: Scalar,
{
    /// Creates a matrix from a column-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let matrix = Mat2::from_column_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix, Mat2::from_columns(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_column_array(array: &[T; 16]) -> Self {
        Self::from_columns(&[
            Vector::<4, T, A>::new(array[0], array[1], array[2], array[3]),
            Vector::<4, T, A>::new(array[4], array[5], array[6], array[7]),
            Vector::<4, T, A>::new(array[8], array[9], array[10], array[11]),
            Vector::<4, T, A>::new(array[12], array[13], array[14], array[15]),
        ])
    }

    /// Creates an affine transformation matrix from the given 3D `scale`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vector<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<4, T, A>::new(scale.as_array_ref()[0], T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, scale.as_array_ref()[1], T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, scale.as_array_ref()[2], T::ZERO),
            Vector::<4, T, A>::W,
        ])
    }

    /// Creates an affine transformation matrix from the given 3D `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vector<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<4, T, A>::X,
            Vector::<4, T, A>::Y,
            Vector::<4, T, A>::Z,
            Vector::<4, T, A>::new(
                translation.as_array_ref()[0],
                translation.as_array_ref()[1],
                translation.as_array_ref()[2],
                T::ONE,
            ),
        ])
    }

    /// Creates an affine transformation matrix from the given 3x3 matrix.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_submatrix(submatrix: &Matrix<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<4, T, A>::new(
                submatrix.column(0).as_array_ref()[0],
                submatrix.column(0).as_array_ref()[1],
                submatrix.column(0).as_array_ref()[2],
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                submatrix.column(1).as_array_ref()[0],
                submatrix.column(1).as_array_ref()[1],
                submatrix.column(1).as_array_ref()[2],
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                submatrix.column(2).as_array_ref()[0],
                submatrix.column(2).as_array_ref()[1],
                submatrix.column(2).as_array_ref()[2],
                T::ZERO,
            ),
            Vector::<4, T, A>::W,
        ])
    }

    /// Creates an affine transformation matrix from the given 3x3 matrix and 3D
    /// `translation`.
    ///
    /// The resulting matrix can be used to transform 3D points and vectors. See
    /// [`transform_point`] and [`transform_vector`].
    ///
    /// [`transform_point`]: Self::transform_point
    /// [`transform_vector`]: Self::transform_vector
    #[inline]
    #[must_use]
    pub const fn from_submatrix_translation(
        submatrix: &Matrix<3, T, A>,
        translation: Vector<3, T, A>,
    ) -> Self
    where
        T: Zero + One,
    {
        Self::from_columns(&[
            Vector::<4, T, A>::new(
                submatrix.column(0).as_array_ref()[0],
                submatrix.column(0).as_array_ref()[1],
                submatrix.column(0).as_array_ref()[2],
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                submatrix.column(1).as_array_ref()[0],
                submatrix.column(1).as_array_ref()[1],
                submatrix.column(1).as_array_ref()[2],
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                submatrix.column(2).as_array_ref()[0],
                submatrix.column(2).as_array_ref()[1],
                submatrix.column(2).as_array_ref()[2],
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                translation.as_array_ref()[0],
                translation.as_array_ref()[1],
                translation.as_array_ref()[2],
                T::ONE,
            ),
        ])
    }

    /// Returns a 3x3 matrix discarding the fourth column and row.
    #[inline]
    #[must_use]
    pub const fn submatrix(&self) -> Matrix<3, T, A> {
        Matrix::from_columns(&[
            Vector::<3, T, A>::new(
                self.column(0).as_array_ref()[0],
                self.column(0).as_array_ref()[1],
                self.column(0).as_array_ref()[2],
            ),
            Vector::<3, T, A>::new(
                self.column(1).as_array_ref()[0],
                self.column(1).as_array_ref()[1],
                self.column(1).as_array_ref()[2],
            ),
            Vector::<3, T, A>::new(
                self.column(2).as_array_ref()[0],
                self.column(2).as_array_ref()[1],
                self.column(2).as_array_ref()[2],
            ),
        ])
    }

    /// Returns the translation of an affine transformation matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat3, Vec2, Vec3};
    /// #
    /// let matrix = Mat3::from_columns(&[
    ///     Vec3::new(1, 0, 0),
    ///     Vec3::new(0, 1, 0),
    ///     Vec3::new(6, 8, 1),
    /// ]);
    /// assert_eq!(matrix.translation(), Vec2::new(6, 8));
    /// ```
    #[inline]
    #[must_use]
    pub fn translation(&self) -> Vector<3, T, A> {
        self.w_axis.truncate()
    }

    /// Returns a 3x3 matrix discarding the given `column` and `row`.
    ///
    /// # Panics
    ///
    /// Panics if `column` or `row` are greater than `3`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn remove(&self, column: usize, row: usize) -> Matrix<3, T, A> {
        match (column, row) {
            (0, 0) => {
                Matrix::from_columns(&[self.y_axis.yzw(), self.z_axis.yzw(), self.w_axis.yzw()])
            }
            (0, 1) => {
                Matrix::from_columns(&[self.y_axis.xzw(), self.z_axis.xzw(), self.w_axis.xzw()])
            }
            (0, 2) => {
                Matrix::from_columns(&[self.y_axis.xyw(), self.z_axis.xyw(), self.w_axis.xyw()])
            }
            (0, 3) => {
                Matrix::from_columns(&[self.y_axis.xyz(), self.z_axis.xyz(), self.w_axis.xyz()])
            }
            (1, 0) => {
                Matrix::from_columns(&[self.x_axis.yzw(), self.z_axis.yzw(), self.w_axis.yzw()])
            }
            (1, 1) => {
                Matrix::from_columns(&[self.x_axis.xzw(), self.z_axis.xzw(), self.w_axis.xzw()])
            }
            (1, 2) => {
                Matrix::from_columns(&[self.x_axis.xyw(), self.z_axis.xyw(), self.w_axis.xyw()])
            }
            (1, 3) => {
                Matrix::from_columns(&[self.x_axis.xyz(), self.z_axis.xyz(), self.w_axis.xyz()])
            }
            (2, 0) => {
                Matrix::from_columns(&[self.x_axis.yzw(), self.y_axis.yzw(), self.w_axis.yzw()])
            }
            (2, 1) => {
                Matrix::from_columns(&[self.x_axis.xzw(), self.y_axis.xzw(), self.w_axis.xzw()])
            }
            (2, 2) => {
                Matrix::from_columns(&[self.x_axis.xyw(), self.y_axis.xyw(), self.w_axis.xyw()])
            }
            (2, 3) => {
                Matrix::from_columns(&[self.x_axis.xyz(), self.y_axis.xyz(), self.w_axis.xyz()])
            }
            (3, 0) => {
                Matrix::from_columns(&[self.x_axis.yzw(), self.y_axis.yzw(), self.z_axis.yzw()])
            }
            (3, 1) => {
                Matrix::from_columns(&[self.x_axis.xzw(), self.y_axis.xzw(), self.z_axis.xzw()])
            }
            (3, 2) => {
                Matrix::from_columns(&[self.x_axis.xyw(), self.y_axis.xyw(), self.z_axis.xyw()])
            }
            (3, 3) => {
                Matrix::from_columns(&[self.x_axis.xyz(), self.y_axis.xyz(), self.z_axis.xyz()])
            }
            _ => panic!("index out of bounds"),
        }
    }

    /// Transforms the given 3D vector as a point.
    ///
    /// Equivalent to `self * (point, 1)` but is faster. This does not perform a
    /// perspective divide.
    ///
    /// `self` must contain a valid affine transform, meaning the fourth row
    /// must be `(0, 0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the fourth row of `self` is not `(0, 0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_point(&self, point: Vector<3, T, A>) -> Vector<3, T, A>
    where
        T: PartialEq + Add<Output = T> + Mul<Output = T> + Zero + One,
    {
        #[cfg(assertions)]
        assert!(self.row(3) == Vector::<4, T, A>::W);

        self.x_axis.xyz() * point.x
            + self.y_axis.xyz() * point.y
            + self.z_axis.xyz() * point.z
            + self.w_axis.xyz()
    }

    /// Transforms the given 3D vector without applying translation.
    ///
    /// Equivalent to `self * (vector, 0)` but is faster.
    ///
    /// `self` must contain a valid affine transform, meaning the fourth row
    /// must be `(0, 0, 0, 1)`.
    ///
    /// # Panics
    ///
    /// When assertions are enabled (see the crate documentation):
    ///
    /// Panics if the fourth row of `self` is not `(0, 0, 0, 1)`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_vector(&self, vector: Vector<3, T, A>) -> Vector<3, T, A>
    where
        T: PartialEq + Add<Output = T> + Mul<Output = T> + Zero + One,
    {
        #[cfg(assertions)]
        assert!(self.row(3) == Vector::<4, T, A>::W);

        self.x_axis.xyz() * vector.x + self.y_axis.xyz() * vector.y + self.z_axis.xyz() * vector.z
    }
}

impl<const N: usize, T, A: Alignment> Clone for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat2Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(1, 0)`.
    pub x_axis: Vector<2, T, A>,
    /// The second column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(0, 1)`.
    pub y_axis: Vector<2, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<2, T, A>
where
    T: Scalar,
{
    type Target = Mat2Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `Vector<2, T, A>`, and so begin with `Xy<T, A>`.
        unsafe { transmute_ref::<Matrix<2, T, A>, Mat2Fields<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `Vector<2, T, A>`, and so begin with `Xy<T, A>`.
        unsafe { transmute_mut::<Matrix<2, T, A>, Mat2Fields<T, A>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat3Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(1, 0, 0)`.
    pub x_axis: Vector<3, T, A>,
    /// The second column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(0, 1, 0)`.
    pub y_axis: Vector<3, T, A>,
    /// The third column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(0, 0, 1)`.
    pub z_axis: Vector<3, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<3, T, A>
where
    T: Scalar,
{
    type Target = Mat3Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`, and so begin with `Xyz<T, A>`.
        unsafe { transmute_ref::<Matrix<3, T, A>, Mat3Fields<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`, and so begin with `Xyz<T, A>`.
        unsafe { transmute_mut::<Matrix<3, T, A>, Mat3Fields<T, A>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat4Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(1, 0, 0, 0)`.
    pub x_axis: Vector<4, T, A>,
    /// The second column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(0, 1, 0, 0)`.
    pub y_axis: Vector<4, T, A>,
    /// The third column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(0, 0, 1, 0)`.
    pub z_axis: Vector<4, T, A>,
    /// The fourth column of the matrix.
    ///
    /// This represents the result of multiplying the matrix by `(0, 0, 0, 1)`.
    pub w_axis: Vector<4, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<4, T, A>
where
    T: Scalar,
{
    type Target = Mat4Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`, and so begin with `Xyzw<T, A>`.
        unsafe { transmute_ref::<Matrix<4, T, A>, Mat4Fields<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`, and so begin with `Xyzw<T, A>`.
        unsafe { transmute_mut::<Matrix<4, T, A>, Mat4Fields<T, A>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_columns())
    }
}

impl<const N: usize, T, A: Alignment> Display for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "[{}, {}]", self.column(0), self.column(1)),
            3 => write!(
                f,
                "[{}, {}, {}]",
                self.column(0),
                self.column(1),
                self.column(2)
            ),
            4 => write!(
                f,
                "[{}, {}, {}, {}]",
                self.column(0),
                self.column(1),
                self.column(2),
                self.column(3)
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        (0..N).all(|i| self.column(i) == other.column(i))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_columns().hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
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
        impl<const N: usize, T, A: Alignment> Neg for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                -(&self)
            }
        }

        impl<const N: usize, T, A: Alignment> Neg for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                Matrix::from_column_fn(|i| -self.column(i))
            }
        }
    };
}
impl_neg!(
    /// Performs the unary `-` operation for each element.
    ///
    /// Equivalent to `[-self.x_axis, -self.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Add for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                (&self) + (&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Add<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: &Self) -> Self::Output {
                (&self) + rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Add<Matrix<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Matrix<N, T, A>) -> Self::Output {
                self + (&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Add for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                Matrix::from_column_fn(|i| self.column(i) + rhs.column(i))
            }
        }
    };
}
impl_add!(
    /// Performs the `+` operation for each element.
    ///
    /// Equivalent to
    /// `[self.x_axis + rhs.x_axis, self.y_axis + rhs.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_add_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> AddAssign for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = &*self + rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: &Self) {
                *self = &*self + rhs;
            }
        }
    };
}
impl_add_assign!(
    /// Performs the `+=` operation for each element.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis += rhs.x_axis;
    /// self.y_axis += rhs.y_axis;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix + matrix`.
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                (&self) - (&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: &Self) -> Self::Output {
                (&self) - rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Sub<Matrix<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Matrix<N, T, A>) -> Self::Output {
                self - (&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Sub for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                Matrix::from_column_fn(|i| self.column(i) - rhs.column(i))
            }
        }
    };
}
impl_sub!(
    /// Performs the `-` operation for each element.
    ///
    /// Equivalent to
    /// `[self.x_axis - rhs.x_axis, self.y_axis - rhs.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_sub_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> SubAssign for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = &*self - rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: &Self) {
                *self = &*self - rhs;
            }
        }
    };
}
impl_sub_assign!(
    /// Performs the `-=` operation for each element.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis -= rhs.x_axis;
    /// self.y_axis -= rhs.y_axis;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix - matrix`.
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                &self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                Matrix::from_column_fn(|i| self.column(i) * rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                self * *rhs
            }
        }
    };
}
impl_mul_scalar!(
    /// Matrix-scalar multiplication.
    ///
    /// Equivalent to `[self.x_axis * rhs, self.y_axis * rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_mul_assign_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = &*self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = &*self * *rhs
            }
        }
    };
}
impl_mul_assign_scalar!(
    /// Matrix-scalar multiplication.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis *= rhs;
    /// self.y_axis *= rhs;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix * scalar`.
);

macro_rules! impl_mul_vector {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Vector<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Vector<N, T, A>) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Vector<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Vector<N, T, A>) -> Self::Output {
                &self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Vector<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Vector<N, T, A>) -> Self::Output {
                match N {
                    2 => self.column(0) * rhs[0] + self.column(1) * rhs[1],
                    3 => {
                        self.column(0) * rhs[0] + self.column(1) * rhs[1] + self.column(2) * rhs[2]
                    }
                    4 => {
                        self.column(0) * rhs[0]
                            + self.column(1) * rhs[1]
                            + self.column(2) * rhs[2]
                            + self.column(3) * rhs[3]
                    }
                    _ => unreachable!(),
                }
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Vector<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Vector<N, T, A>) -> Self::Output {
                self * *rhs
            }
        }
    };
}
impl_mul_vector!(
    /// Matrix-vector multiplication.
    ///
    /// Because vectors are treated as column matrices, they always go on the
    /// right-hand side.
    ///
    /// Equivalent to `self.x_axis * rhs.x + self.y_axis * rhs.y + ...`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Self) -> Self::Output {
                &self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Matrix<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<N, T, A>) -> Self::Output {
                self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                Matrix::from_column_fn(|i| self * rhs.column(i))
            }
        }
    };
}
impl_mul!(
    /// Matrix multiplication.
    ///
    /// Because vectors are treated as column matrices, matrix multiplication
    /// first applies the right-hand side matrix, then the left-hand side
    /// matrix.
    ///
    /// Equivalent to `[self * rhs.x_axis, self * rhs.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = &*self * &rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Matrix<N, T, A>) {
                *self = &*self * rhs;
            }
        }
    };
}
impl_mul_assign!(
    /// Matrix multiplication.
    ///
    /// Because vectors are treated as column matrices, matrix multiplication
    /// first applies the right-hand side matrix, then the left-hand side
    /// matrix.
    ///
    /// Equivalent to `self = [self * rhs.x_axis, self * rhs.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix * matrix`.
);

macro_rules! impl_div_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Div<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                &self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                &self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                Matrix::from_column_fn(|i| self.column(i) / rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                self / *rhs
            }
        }
    };
}
impl_div_scalar!(
    /// Matrix-scalar division.
    ///
    /// Equivalent to `[self.x_axis / rhs, self.y_axis / rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_div_assign_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> DivAssign<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: T) {
                *self = &*self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: &T) {
                *self = &*self / *rhs
            }
        }
    };
}
impl_div_assign_scalar!(
    /// Matrix-scalar division.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis /= rhs;
    /// self.y_axis /= rhs;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix / scalar`.
);

// SAFETY: Matrices are equivalent to values of `T` mixed with padding.
// Because `T` is `Send` and padding is `Send`, the matrix is too.
unsafe impl<const N: usize, T, A: Alignment> Send for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Send,
{
}

// SAFETY: Matrices are equivalent to values of `T` mixed with padding.
// Because `T` is `Sync` and padding is `Sync`, the matrix is too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Aligned, Mat2, Mat2U, Mat3, Mat3U, Mat4, Mat4U, Matrix, Unaligned, Vec2, Vec3, Vec4,
        Vector,
        utils::{assert_assertions_panic, assert_float_eq, assert_panic, for_parameters},
    };

    #[test]
    fn test_layout() {
        for_parameters!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Mat2<T>>(), size_of::<Vec4<T>>());
            assert_eq!(align_of::<Mat2<T>>(), align_of::<Vec4<T>>());

            assert!(
                size_of::<Mat3<T>>() == size_of::<Vec3<T>>() * 3
                    && align_of::<Mat3<T>>() == align_of::<Vec3<T>>()
                    || size_of::<Mat3<T>>() == size_of::<Vec3<T>>() * 4
                        && align_of::<Mat3<T>>() == size_of::<Vec3<T>>() * 4
            );

            assert_eq!(size_of::<Mat4<T>>(), size_of::<Vec4<T>>() * 4);
            assert!(
                align_of::<Mat4<T>>() == align_of::<Vec4<T>>()
                    || align_of::<Mat4<T>>() == size_of::<Vec4<T>>() * 4
            );

            assert_eq!(size_of::<Mat2U<T>>(), size_of::<T>() * 4);
            assert_eq!(align_of::<Mat2U<T>>(), align_of::<T>());

            assert_eq!(size_of::<Mat3U<T>>(), size_of::<T>() * 9);
            assert_eq!(align_of::<Mat3U<T>>(), align_of::<T>());

            assert_eq!(size_of::<Mat4U<T>>(), size_of::<T>() * 16);
            assert_eq!(align_of::<Mat4U<T>>(), align_of::<T>());
        });
    }

    #[test]
    fn test_zero() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Matrix::<2, T, A>::ZERO,
                Matrix::from_columns(&[Vector::ZERO; 2])
            );
            assert_eq!(
                Matrix::<3, T, A>::ZERO,
                Matrix::from_columns(&[Vector::ZERO; 3])
            );
            assert_eq!(
                Matrix::<4, T, A>::ZERO,
                Matrix::from_columns(&[Vector::ZERO; 4])
            );
        });
    }

    #[test]
    fn test_identity() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Matrix::<2, T, A>::IDENTITY,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(T::as_from(1), T::as_from(0)),
                    Vector::<2, T, A>::new(T::as_from(0), T::as_from(1))
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::IDENTITY,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(T::as_from(1), T::as_from(0), T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), T::as_from(1), T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(1))
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::IDENTITY,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(
                        T::as_from(1),
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(0)
                    ),
                    Vector::<4, T, A>::new(
                        T::as_from(0),
                        T::as_from(1),
                        T::as_from(0),
                        T::as_from(0)
                    ),
                    Vector::<4, T, A>::new(
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(1),
                        T::as_from(0)
                    ),
                    Vector::<4, T, A>::new(
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(1)
                    )
                ])
            );
        });
    }

    #[test]
    fn test_nan() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Matrix::<2, T, A>::NAN,
                Matrix::from_columns(&[Vector::NAN; 2])
            );
            assert_float_eq!(
                Matrix::<3, T, A>::NAN,
                Matrix::from_columns(&[Vector::NAN; 3])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::NAN,
                Matrix::from_columns(&[Vector::NAN; 4])
            );
        });
    }

    #[test]
    fn test_from_column_fn() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_column_fn(|i| [
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ][i]),
                Matrix::from_columns(&[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, w)])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_column_fn(|i| [
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ][i]),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_column_fn(|idx| [
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ][idx]),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
            );
        });
    }

    #[test]
    fn test_from_diagonal() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [_, x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_diagonal(Vector::<2, T, A>::new(x, y)),
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x, T::as_from(0)),
                    Vector::<2, T, A>::new(T::as_from(0), y)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_diagonal(Vector::<3, T, A>::new(x, y, z)),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x, T::as_from(0), T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), y, T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), T::as_from(0), z)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_diagonal(Vector::<4, T, A>::new(x, y, z, w)),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x, T::as_from(0), T::as_from(0), T::as_from(0)),
                    Vector::<4, T, A>::new(T::as_from(0), y, T::as_from(0), T::as_from(0)),
                    Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), z, T::as_from(0)),
                    Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(0), w)
                ])
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .to_alignment(),
                Matrix::<2, T, Aligned>::from_columns(&[
                    Vector::<2, T, Aligned>::new(x, y),
                    Vector::<2, T, Aligned>::new(z, w)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .to_alignment(),
                Matrix::<3, T, Aligned>::from_columns(&[
                    Vector::<3, T, Aligned>::new(x, y, z),
                    Vector::<3, T, Aligned>::new(w, a, b),
                    Vector::<3, T, Aligned>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .to_alignment(),
                Matrix::<4, T, Aligned>::from_columns(&[
                    Vector::<4, T, Aligned>::new(x, y, z, w),
                    Vector::<4, T, Aligned>::new(a, b, c, d),
                    Vector::<4, T, Aligned>::new(e, f, g, h),
                    Vector::<4, T, Aligned>::new(i, j, k, l)
                ])
            );

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .to_alignment(),
                Matrix::<2, T, Unaligned>::from_columns(&[
                    Vector::<2, T, Unaligned>::new(x, y),
                    Vector::<2, T, Unaligned>::new(z, w)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .to_alignment(),
                Matrix::<3, T, Unaligned>::from_columns(&[
                    Vector::<3, T, Unaligned>::new(x, y, z),
                    Vector::<3, T, Unaligned>::new(w, a, b),
                    Vector::<3, T, Unaligned>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .to_alignment(),
                Matrix::<4, T, Unaligned>::from_columns(&[
                    Vector::<4, T, Unaligned>::new(x, y, z, w),
                    Vector::<4, T, Unaligned>::new(a, b, c, d),
                    Vector::<4, T, Unaligned>::new(e, f, g, h),
                    Vector::<4, T, Unaligned>::new(i, j, k, l)
                ])
            );
        });
    }

    #[test]
    fn test_align() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .align(),
                Matrix::<2, T, Aligned>::from_columns(&[
                    Vector::<2, T, Aligned>::new(x, y),
                    Vector::<2, T, Aligned>::new(z, w)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .align(),
                Matrix::<3, T, Aligned>::from_columns(&[
                    Vector::<3, T, Aligned>::new(x, y, z),
                    Vector::<3, T, Aligned>::new(w, a, b),
                    Vector::<3, T, Aligned>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .align(),
                Matrix::<4, T, Aligned>::from_columns(&[
                    Vector::<4, T, Aligned>::new(x, y, z, w),
                    Vector::<4, T, Aligned>::new(a, b, c, d),
                    Vector::<4, T, Aligned>::new(e, f, g, h),
                    Vector::<4, T, Aligned>::new(i, j, k, l)
                ])
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .unalign(),
                Matrix::<2, T, Unaligned>::from_columns(&[
                    Vector::<2, T, Unaligned>::new(x, y),
                    Vector::<2, T, Unaligned>::new(z, w)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .unalign(),
                Matrix::<3, T, Unaligned>::from_columns(&[
                    Vector::<3, T, Unaligned>::new(x, y, z),
                    Vector::<3, T, Unaligned>::new(w, a, b),
                    Vector::<3, T, Unaligned>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .unalign(),
                Matrix::<4, T, Unaligned>::from_columns(&[
                    Vector::<4, T, Unaligned>::new(x, y, z, w),
                    Vector::<4, T, Unaligned>::new(a, b, c, d),
                    Vector::<4, T, Unaligned>::new(e, f, g, h),
                    Vector::<4, T, Unaligned>::new(i, j, k, l)
                ])
            );
        });
    }

    #[test]
    fn test_as_columns() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .as_columns(),
                &[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, w)]
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .as_columns(),
                &[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ]
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .as_columns(),
                &[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ]
            );
        });
    }

    #[test]
    fn test_as_columns_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .as_columns_mut(),
                &mut [Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, w)]
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .as_columns_mut(),
                &mut [
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ]
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .as_columns_mut(),
                &mut [
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ]
            );
        });
    }

    #[test]
    fn test_column() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(matrix.column(0), Vector::<2, T, A>::new(x, y));
            assert_eq!(matrix.column(1), Vector::<2, T, A>::new(z, w));
            assert_panic!(matrix.column(2));

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(matrix.column(0), Vector::<3, T, A>::new(x, y, z));
            assert_eq!(matrix.column(1), Vector::<3, T, A>::new(w, a, b));
            assert_eq!(matrix.column(2), Vector::<3, T, A>::new(c, d, e));
            assert_panic!(matrix.column(3));

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(matrix.column(0), Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(matrix.column(1), Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(matrix.column(2), Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(matrix.column(3), Vector::<4, T, A>::new(i, j, k, l));
            assert_panic!(matrix.column(4));
        });
    }

    #[test]
    fn test_column_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(matrix.column_mut(0), &mut Vector::<2, T, A>::new(x, y));
            assert_eq!(matrix.column_mut(1), &mut Vector::<2, T, A>::new(z, w));
            assert_panic!(matrix.clone().column_mut(2));

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(matrix.column_mut(0), &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(matrix.column_mut(1), &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(matrix.column_mut(2), &mut Vector::<3, T, A>::new(c, d, e));
            assert_panic!(matrix.clone().column_mut(3));

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(
                matrix.column_mut(0),
                &mut Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                matrix.column_mut(1),
                &mut Vector::<4, T, A>::new(a, b, c, d)
            );
            assert_eq!(
                matrix.column_mut(2),
                &mut Vector::<4, T, A>::new(e, f, g, h)
            );
            assert_eq!(
                matrix.column_mut(3),
                &mut Vector::<4, T, A>::new(i, j, k, l)
            );
            assert_panic!(matrix.clone().column_mut(4));
        });
    }

    #[test]
    fn test_row() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(matrix.row(0), Vector::<2, T, A>::new(x, z));
            assert_eq!(matrix.row(1), Vector::<2, T, A>::new(y, w));
            assert_panic!(matrix.row(2));

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(matrix.row(0), Vector::<3, T, A>::new(x, w, c));
            assert_eq!(matrix.row(1), Vector::<3, T, A>::new(y, a, d));
            assert_eq!(matrix.row(2), Vector::<3, T, A>::new(z, b, e));
            assert_panic!(matrix.row(3));

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(matrix.row(0), Vector::<4, T, A>::new(x, a, e, i));
            assert_eq!(matrix.row(1), Vector::<4, T, A>::new(y, b, f, j));
            assert_eq!(matrix.row(2), Vector::<4, T, A>::new(z, c, g, k));
            assert_eq!(matrix.row(3), Vector::<4, T, A>::new(w, d, h, l));
            assert_panic!(matrix.row(4));
        });
    }

    #[test]
    fn test_set_row() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            matrix.set_row(0, Vector::<2, T, A>::new(a, b));
            assert_eq!(
                matrix,
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(a, y),
                    Vector::<2, T, A>::new(b, w)
                ])
            );
            matrix.set_row(1, Vector::<2, T, A>::new(c, d));
            assert_eq!(
                matrix,
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(a, c),
                    Vector::<2, T, A>::new(b, d)
                ])
            );
            assert_panic!(matrix.clone().set_row(2, Vector::ZERO));

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            matrix.set_row(0, Vector::<3, T, A>::new(a, b, d));
            assert_eq!(
                matrix,
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(a, y, z),
                    Vector::<3, T, A>::new(b, a, b),
                    Vector::<3, T, A>::new(d, d, e)
                ])
            );
            matrix.set_row(1, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(
                matrix,
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(a, x, z),
                    Vector::<3, T, A>::new(b, y, b),
                    Vector::<3, T, A>::new(d, z, e)
                ])
            );
            matrix.set_row(2, Vector::<3, T, A>::new(e, f, g));
            assert_eq!(
                matrix,
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(a, x, e),
                    Vector::<3, T, A>::new(b, y, f),
                    Vector::<3, T, A>::new(d, z, g)
                ])
            );
            assert_panic!(matrix.clone().set_row(3, Vector::ZERO));

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            matrix.set_row(0, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, y, z, w),
                    Vector::<4, T, A>::new(b, b, c, d),
                    Vector::<4, T, A>::new(c, f, g, h),
                    Vector::<4, T, A>::new(d, j, k, l)
                ])
            );
            matrix.set_row(1, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, x, z, w),
                    Vector::<4, T, A>::new(b, y, c, d),
                    Vector::<4, T, A>::new(c, z, g, h),
                    Vector::<4, T, A>::new(d, w, k, l)
                ])
            );
            matrix.set_row(2, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, x, a, w),
                    Vector::<4, T, A>::new(b, y, b, d),
                    Vector::<4, T, A>::new(c, z, c, h),
                    Vector::<4, T, A>::new(d, w, d, l)
                ])
            );
            matrix.set_row(3, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, x, a, e),
                    Vector::<4, T, A>::new(b, y, b, f),
                    Vector::<4, T, A>::new(c, z, c, g),
                    Vector::<4, T, A>::new(d, w, d, h)
                ])
            );
            assert_panic!(matrix.clone().set_row(4, Vector::ZERO));
        });
    }

    #[test]
    fn test_transpose() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .transpose(),
                Matrix::from_columns(&[Vector::<2, T, A>::new(x, z), Vector::<2, T, A>::new(y, w)])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .transpose(),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x, w, c),
                    Vector::<3, T, A>::new(y, a, d),
                    Vector::<3, T, A>::new(z, b, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .transpose(),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x, a, e, i),
                    Vector::<4, T, A>::new(y, b, f, j),
                    Vector::<4, T, A>::new(z, c, g, k),
                    Vector::<4, T, A>::new(w, d, h, l)
                ])
            );
        });
    }

    #[test]
    fn test_transpose_mul_vector() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x * x) || !T::is_finite(y * y) || !T::is_finite(z * z) {
                return;
            }

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            let vector = Vector::<2, T, A>::new(x, z);
            assert_float_eq!(
                matrix.transpose_mul_vector(vector),
                matrix.transpose() * vector,
                abs <= Vector::splat((x * x).max(y * y).max(z * z)) * 1e-6
            );

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, z, y),
            ]);
            let vector = Vector::<3, T, A>::new(z, y, w);
            assert_float_eq!(
                matrix.transpose_mul_vector(vector),
                matrix.transpose() * vector,
                abs <= Vector::splat((x * x).max(y * y).max(z * z)) * 1e-6
            );

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, z, y, z),
                Vector::<4, T, A>::new(y, y, z, x),
            ]);
            let vector = Vector::<4, T, A>::new(z, y, w, z);
            assert_float_eq!(
                matrix.transpose_mul_vector(vector),
                matrix.transpose() * vector,
                abs <= Vector::splat((x * x).max(y * y).max(z * z)) * 1e-6
            );
        });
    }

    #[test]
    fn test_diagonal() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .diagonal(),
                Vector::<2, T, A>::new(x, w)
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .diagonal(),
                Vector::<3, T, A>::new(x, a, e)
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
                .diagonal(),
                Vector::<4, T, A>::new(x, b, g, l)
            );
        });
    }

    #[test]
    fn test_mul_diagonal() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x * x) || !T::is_finite(y * y) || !T::is_finite(z * z) {
                return;
            }

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            let scale = Vector::<2, T, A>::new(x, z);
            assert_float_eq!(
                matrix.mul_diagonal(scale),
                matrix * Matrix::from_diagonal(scale),
                0.0 = -0.0
            );

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, z, y),
            ]);
            let scale = Vector::<3, T, A>::new(z, y, w);
            assert_float_eq!(
                matrix.mul_diagonal(scale),
                matrix * Matrix::from_diagonal(scale),
                0.0 = -0.0
            );

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, z, y, z),
                Vector::<4, T, A>::new(y, y, z, x),
            ]);
            let scale = Vector::<4, T, A>::new(z, y, w, z);
            assert_float_eq!(
                matrix.mul_diagonal(scale),
                matrix * Matrix::from_diagonal(scale),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_determinant() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let [w, a, b] = [x + y, x + z, y + z];
            let [c, d, e] = [x * 1.3, y * 0.7, z * 1.1];
            let [f, g, h] = [w * 2.1, a * 1.9, b * 1.6];
            let [i, j, k, l] = [c + d, c + e, d + f, f + g];

            assert_float_eq!(
                Matrix::<2, T, A>::from_column_array(&[x, y, z, w]).determinant(),
                x * w - y * z
            );

            if !x.is_finite()
                || !y.is_finite()
                || !z.is_finite()
                || x.abs() > 1000.0
                || y.abs() > 1000.0
                || z.abs() > 1000.0
            {
                return;
            }

            let matrix = Matrix::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e]);
            assert_float_eq!(
                matrix.determinant(),
                x * matrix.remove(0, 0).determinant() - y * matrix.remove(0, 1).determinant()
                    + z * matrix.remove(0, 2).determinant(),
                abs <= matrix.determinant().abs() * 1e-4 + 1e-4,
                0.0 = -0.0
            );

            let matrix = Matrix::<4, T, A>::from_column_array(&[
                x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l,
            ]);
            assert_float_eq!(
                matrix.determinant(),
                x * matrix.remove(0, 0).determinant() - y * matrix.remove(0, 1).determinant()
                    + z * matrix.remove(0, 2).determinant()
                    - w * matrix.remove(0, 3).determinant(),
                abs <= matrix.determinant().abs() * 1e-4 + 1e-4,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_from_column_array() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Matrix::<2, T, A>::from_column_array(&[x, y, z, w]),
                Matrix::from_columns(&[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, w)])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_column_array(&[x, y, z, w, a, b, c, d, e]),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_column_array(&[
                    x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l
                ]),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
            );
        });
    }

    #[test]
    fn test_from_scale() {
        assert_eq!(
            Mat3::from_scale(Vec2::new(2, 3)).transform_point(Vec2::new(4, 5)),
            Vec2::new(8, 15)
        );
        assert_eq!(
            Mat3::from_scale(Vec2::new(2, 3)).transform_vector(Vec2::new(4, 5)),
            Vec2::new(8, 15)
        );

        assert_eq!(
            Mat4::from_scale(Vec3::new(2, 3, 4)).transform_point(Vec3::new(5, 6, 7)),
            Vec3::new(10, 18, 28)
        );
        assert_eq!(
            Mat4::from_scale(Vec3::new(2, 3, 4)).transform_vector(Vec3::new(5, 6, 7)),
            Vec3::new(10, 18, 28)
        );
    }

    #[test]
    fn test_from_translation() {
        assert_eq!(
            Mat3::from_translation(Vec2::new(1, 2)).transform_point(Vec2::new(3, 4)),
            Vec2::new(4, 6)
        );
        assert_eq!(
            Mat3::from_translation(Vec2::new(1, 2)).transform_vector(Vec2::new(3, 4)),
            Vec2::new(3, 4)
        );

        assert_eq!(
            Mat4::from_translation(Vec3::new(1, 2, 3)).transform_point(Vec3::new(4, 5, 6)),
            Vec3::new(5, 7, 9)
        );
        assert_eq!(
            Mat4::from_translation(Vec3::new(1, 2, 3)).transform_vector(Vec3::new(4, 5, 6)),
            Vec3::new(4, 5, 6)
        );
    }

    #[test]
    fn test_from_submatrix() {
        assert_eq!(
            Mat3::from_submatrix(&Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)])),
            Mat3::from_columns(&[Vec3::new(1, 2, 0), Vec3::new(3, 4, 0), Vec3::new(0, 0, 1)])
        );
        assert_eq!(
            Mat4::from_submatrix(&Mat3::from_columns(&[
                Vec3::new(1, 2, 3),
                Vec3::new(4, 5, 6),
                Vec3::new(7, 8, 9)
            ])),
            Mat4::from_columns(&[
                Vec4::new(1, 2, 3, 0),
                Vec4::new(4, 5, 6, 0),
                Vec4::new(7, 8, 9, 0),
                Vec4::new(0, 0, 0, 1)
            ])
        );
    }

    #[test]
    fn test_from_submatrix_translation() {
        assert_eq!(
            Mat3::from_submatrix_translation(
                &Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)]),
                Vec2::new(5, 6)
            ),
            Mat3::from_columns(&[Vec3::new(1, 2, 0), Vec3::new(3, 4, 0), Vec3::new(5, 6, 1)])
        );
        assert_eq!(
            Mat4::from_submatrix_translation(
                &Mat3::from_columns(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)]),
                Vec3::new(10, 11, 12)
            ),
            Mat4::from_columns(&[
                Vec4::new(1, 2, 3, 0),
                Vec4::new(4, 5, 6, 0),
                Vec4::new(7, 8, 9, 0),
                Vec4::new(10, 11, 12, 1)
            ])
        );
    }

    #[test]
    fn test_submatrix() {
        assert_eq!(
            Mat3::from_columns(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)])
                .submatrix(),
            Mat2::from_columns(&[Vec2::new(1, 2), Vec2::new(4, 5)])
        );
        assert_eq!(
            Mat4::from_columns(&[
                Vec4::new(1, 2, 3, 4),
                Vec4::new(5, 6, 7, 8),
                Vec4::new(9, 10, 11, 12),
                Vec4::new(13, 14, 15, 16)
            ])
            .submatrix(),
            Mat3::from_columns(&[Vec3::new(1, 2, 3), Vec3::new(5, 6, 7), Vec3::new(9, 10, 11)])
        );
    }

    #[test]
    fn test_translation() {
        assert_eq!(
            Mat3::from_columns(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)])
                .translation(),
            Vec2::new(7, 8)
        );
        assert_eq!(
            Mat4::from_columns(&[
                Vec4::new(1, 2, 3, 4),
                Vec4::new(5, 6, 7, 8),
                Vec4::new(9, 10, 11, 12),
                Vec4::new(13, 14, 15, 16)
            ])
            .translation(),
            Vec3::new(13, 14, 15)
        );
    }

    #[test]
    fn test_remove() {
        let matrix =
            Mat3::from_columns(&[Vec3::new(1, 2, 3), Vec3::new(4, 5, 6), Vec3::new(7, 8, 9)]);

        assert_panic!(matrix.remove(1, 3));
        assert_panic!(matrix.remove(3, 1));

        for column in 0..3 {
            for row in 0..3 {
                let columns = match column {
                    0 => [matrix.column(1), matrix.column(2)],
                    1 => [matrix.column(0), matrix.column(2)],
                    2 => [matrix.column(0), matrix.column(1)],
                    _ => unreachable!(),
                };
                let columns = match row {
                    0 => columns.map(|c| c.yz()),
                    1 => columns.map(|c| c.xz()),
                    2 => columns.map(|c| c.xy()),
                    _ => unreachable!(),
                };

                assert_eq!(matrix.remove(column, row), Mat2::from_columns(&columns));
            }
        }

        let matrix = Mat4::from_columns(&[
            Vec4::new(1, 2, 3, 4),
            Vec4::new(5, 6, 7, 8),
            Vec4::new(9, 10, 11, 12),
            Vec4::new(13, 14, 15, 16),
        ]);

        assert_panic!(matrix.remove(1, 4));
        assert_panic!(matrix.remove(4, 1));

        for column in 0..4 {
            for row in 0..4 {
                let columns = match column {
                    0 => [matrix.column(1), matrix.column(2), matrix.column(3)],
                    1 => [matrix.column(0), matrix.column(2), matrix.column(3)],
                    2 => [matrix.column(0), matrix.column(1), matrix.column(3)],
                    3 => [matrix.column(0), matrix.column(1), matrix.column(2)],
                    _ => unreachable!(),
                };
                let columns = match row {
                    0 => columns.map(|c| c.yzw()),
                    1 => columns.map(|c| c.xzw()),
                    2 => columns.map(|c| c.xyw()),
                    3 => columns.map(|c| c.xyz()),
                    _ => unreachable!(),
                };

                assert_eq!(matrix.remove(column, row), Mat3::from_columns(&columns));
            }
        }
    }

    #[test]
    fn test_transform_point() {
        assert_eq!(
            Mat3::from_columns(&[Vec3::new(2, 3, 0), Vec3::new(4, 5, 0), Vec3::new(6, 7, 1)])
                .transform_point(Vec2::new(-1, -2)),
            Vec2::new(-4, -6)
        );
        assert_eq!(
            Mat4::from_columns(&[
                Vec4::new(2, 3, 4, 0),
                Vec4::new(5, 6, 7, 0),
                Vec4::new(8, 9, 10, 0),
                Vec4::new(11, 12, 13, 1)
            ])
            .transform_point(Vec3::new(-1, -2, -3)),
            Vec3::new(-25, -30, -35)
        );

        assert_assertions_panic!(
            Mat3::from_columns(&[Vec3::new(2, 3, 0), Vec3::new(4, 5, 1), Vec3::new(6, 7, 1)])
                .transform_point(Vec2::new(-1, -2))
        );
        assert_assertions_panic!(
            Mat4::from_columns(&[
                Vec4::new(2, 3, 4, 0),
                Vec4::new(5, 6, 7, 0),
                Vec4::new(8, 9, 10, 1),
                Vec4::new(11, 12, 13, 1)
            ])
            .transform_point(Vec3::new(-1, -2, -3))
        );
    }

    #[test]
    fn test_transform_vector() {
        assert_eq!(
            Mat3::from_columns(&[Vec3::new(2, 3, 0), Vec3::new(4, 5, 0), Vec3::new(6, 7, 1)])
                .transform_vector(Vec2::new(-1, -2)),
            Vec2::new(-10, -13)
        );
        assert_eq!(
            Mat4::from_columns(&[
                Vec4::new(2, 3, 4, 0),
                Vec4::new(5, 6, 7, 0),
                Vec4::new(8, 9, 10, 0),
                Vec4::new(11, 12, 13, 1)
            ])
            .transform_vector(Vec3::new(-1, -2, -3)),
            Vec3::new(-36, -42, -48)
        );

        assert_assertions_panic!(
            Mat3::from_columns(&[Vec3::new(2, 3, 0), Vec3::new(4, 5, 1), Vec3::new(6, 7, 1)])
                .transform_vector(Vec2::new(-1, -2))
        );
        assert_assertions_panic!(
            Mat4::from_columns(&[
                Vec4::new(2, 3, 4, 0),
                Vec4::new(5, 6, 7, 0),
                Vec4::new(8, 9, 10, 1),
                Vec4::new(11, 12, 13, 1)
            ])
            .transform_vector(Vec3::new(-1, -2, -3))
        );
    }

    #[test]
    fn test_deref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(matrix.x_axis, Vector::<2, T, A>::new(x, y));
            assert_eq!(matrix.y_axis, Vector::<2, T, A>::new(z, w));

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(matrix.x_axis, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(matrix.y_axis, Vector::<3, T, A>::new(w, a, b));
            assert_eq!(matrix.z_axis, Vector::<3, T, A>::new(c, d, e));

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(matrix.x_axis, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(matrix.y_axis, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(matrix.z_axis, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(matrix.w_axis, Vector::<4, T, A>::new(i, j, k, l));
        });
    }

    #[test]
    fn test_deref_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(&mut matrix.x_axis, &mut Vector::<2, T, A>::new(x, y));
            assert_eq!(&mut matrix.y_axis, &mut Vector::<2, T, A>::new(z, w));

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(&mut matrix.x_axis, &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(&mut matrix.y_axis, &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(&mut matrix.z_axis, &mut Vector::<3, T, A>::new(c, d, e));

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(&mut matrix.x_axis, &mut Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(&mut matrix.y_axis, &mut Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(&mut matrix.z_axis, &mut Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(&mut matrix.w_axis, &mut Vector::<4, T, A>::new(i, j, k, l));
        });
    }

    #[test]
    fn test_debug() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!(
                    "{:?}",
                    Matrix::<2, T, A>::from_columns(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ])
                ),
                format!("[({x:?}, {y:?}), ({z:?}, {w:?})]")
            );
            assert_eq!(
                format!(
                    "{:?}",
                    Matrix::<3, T, A>::from_columns(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ])
                ),
                format!("[({x:?}, {y:?}, {z:?}), ({w:?}, {a:?}, {b:?}), ({c:?}, {d:?}, {e:?})]")
            );
            assert_eq!(
                format!(
                    "{:?}",
                    Matrix::<4, T, A>::from_columns(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ])
                ),
                format!(
                    "[({x:?}, {y:?}, {z:?}, {w:?}), ({a:?}, {b:?}, {c:?}, {d:?}), ({e:?}, {f:?}, {g:?}, {h:?}), ({i:?}, {j:?}, {k:?}, {l:?})]"
                )
            );
        });
    }

    #[test]
    fn test_display() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                format!(
                    "{}",
                    Matrix::<2, T, A>::from_columns(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ])
                ),
                format!("[({x}, {y}), ({z}, {w})]")
            );
            assert_eq!(
                format!(
                    "{}",
                    Matrix::<3, T, A>::from_columns(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ])
                ),
                format!("[({x}, {y}, {z}), ({w}, {a}, {b}), ({c}, {d}, {e})]")
            );
            assert_eq!(
                format!(
                    "{}",
                    Matrix::<4, T, A>::from_columns(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ])
                ),
                format!(
                    "[({x}, {y}, {z}, {w}), ({a}, {b}, {c}, {d}), ({e}, {f}, {g}, {h}), ({i}, {j}, {k}, {l})]"
                )
            );
        });
    }

    #[test]
    fn test_eq() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]) == Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, y),
                    Vector::<2, T, A>::new(z, w),
                ]),
                x == z && y == y && z == z && w == w
            );
            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]) == Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]),
                x == z && y == w
            );

            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(x, y, w),
                    Vector::<3, T, A>::new(x, y, z),
                ]) == Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, w),
                    Vector::<3, T, A>::new(x, y, w),
                    Vector::<3, T, A>::new(x, y, z),
                ]),
                x == x && y == y && z == w && w == w && z == z
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                ]) == Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                ]),
                x == z && y == w && z == y
            );

            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, y),
                    Vector::<4, T, A>::new(x, y, y, w),
                    Vector::<4, T, A>::new(x, y, z, x),
                ]) == Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(w, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, y),
                    Vector::<4, T, A>::new(x, y, y, w),
                    Vector::<4, T, A>::new(x, y, z, x),
                ]),
                x == w && y == y && z == z && w == w
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                ]) == Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                ]),
                x == z && y == w && z == y && w == x
            );
        });
    }

    #[test]
    fn test_ne() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]) != Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, y),
                    Vector::<2, T, A>::new(z, w),
                ]),
                x != z || y != y || z != z || w != w
            );
            assert_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]) != Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]),
                x != z || y != w
            );

            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(x, y, w),
                    Vector::<3, T, A>::new(x, y, z),
                ]) != Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, w),
                    Vector::<3, T, A>::new(x, y, w),
                    Vector::<3, T, A>::new(x, y, z),
                ]),
                x != x || y != y || z != w || w != w || z != z
            );
            assert_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                ]) != Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                ]),
                x != z || y != w || z != y
            );

            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, w),
                ]) != Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(w, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(x, y, z, w),
                ]),
                x != w || y != y || z != z || w != w
            );
            assert_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                ]) != Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                ]),
                x != z || y != w || z != y || w != x
            );
        });
    }

    #[test]
    fn test_default() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Matrix::<2, T, A>::default(), Matrix::IDENTITY);
            assert_eq!(Matrix::<3, T, A>::default(), Matrix::IDENTITY);
            assert_eq!(Matrix::<4, T, A>::default(), Matrix::IDENTITY);
        });
    }

    #[test]
    fn test_neg() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                -Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]),
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(-x, -y),
                    Vector::<2, T, A>::new(-z, -w),
                ])
            );
            assert_float_eq!(
                -Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, x, y),
                    Vector::<3, T, A>::new(z, w, x),
                ]),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(-x, -y, -z),
                    Vector::<3, T, A>::new(-w, -x, -y),
                    Vector::<3, T, A>::new(-z, -w, -x),
                ])
            );
            assert_float_eq!(
                -Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, x, w, y),
                    Vector::<4, T, A>::new(y, w, x, z),
                    Vector::<4, T, A>::new(w, z, y, x),
                ]),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(-x, -y, -z, -w),
                    Vector::<4, T, A>::new(-z, -x, -w, -y),
                    Vector::<4, T, A>::new(-y, -w, -x, -z),
                    Vector::<4, T, A>::new(-w, -z, -y, -x),
                ])
            );
        });
    }

    #[test]
    fn test_add() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]) + Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]),
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x + z, y + w),
                    Vector::<2, T, A>::new(z + x, w + y),
                ])
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                ]) + Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                ]),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x + z, y + w, z + y),
                    Vector::<3, T, A>::new(z + x, w + y, y + z),
                    Vector::<3, T, A>::new(x + z, y + w, z + y),
                ])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(y, x, w, z),
                    Vector::<4, T, A>::new(w, z, x, y),
                ]) + Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(w, z, x, y),
                    Vector::<4, T, A>::new(y, x, w, z),
                ]),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x + z, y + w, z + y, w + x),
                    Vector::<4, T, A>::new(z + x, w + y, y + z, x + w),
                    Vector::<4, T, A>::new(y + w, x + z, w + x, z + y),
                    Vector::<4, T, A>::new(w + y, z + x, x + w, y + z),
                ])
            );
        });
    }

    #[test]
    fn test_add_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            matrix += Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x + z, y + w),
                    Vector::<2, T, A>::new(z + x, w + y),
                ])
            );

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            matrix += Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
            ]);
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x + z, y + w, z + y),
                    Vector::<3, T, A>::new(z + x, w + y, y + z),
                    Vector::<3, T, A>::new(x + z, y + w, z + y),
                ])
            );

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            matrix += Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(w, z, x, y),
                Vector::<4, T, A>::new(y, x, w, z),
            ]);
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x + z, y + w, z + y, w + x),
                    Vector::<4, T, A>::new(z + x, w + y, y + z, x + w),
                    Vector::<4, T, A>::new(y + w, x + z, w + x, z + y),
                    Vector::<4, T, A>::new(w + y, z + x, x + w, y + z),
                ])
            );
        });
    }

    #[test]
    fn test_sub() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                ]) - Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]),
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x - z, y - w),
                    Vector::<2, T, A>::new(z - x, w - y),
                ])
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                ]) - Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                ]),
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x - z, y - w, z - y),
                    Vector::<3, T, A>::new(z - x, w - y, y - z),
                    Vector::<3, T, A>::new(x - z, y - w, z - y),
                ])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(y, x, w, z),
                    Vector::<4, T, A>::new(w, z, x, y),
                ]) - Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(w, z, x, y),
                    Vector::<4, T, A>::new(y, x, w, z),
                ]),
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x - z, y - w, z - y, w - x),
                    Vector::<4, T, A>::new(z - x, w - y, y - z, x - w),
                    Vector::<4, T, A>::new(y - w, x - z, w - x, z - y),
                    Vector::<4, T, A>::new(w - y, z - x, x - w, y - z),
                ])
            );
        });
    }

    #[test]
    fn test_sub_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            matrix -= Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x - z, y - w),
                    Vector::<2, T, A>::new(z - x, w - y),
                ])
            );

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            matrix -= Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
            ]);
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x - z, y - w, z - y),
                    Vector::<3, T, A>::new(z - x, w - y, y - z),
                    Vector::<3, T, A>::new(x - z, y - w, z - y),
                ])
            );

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            matrix -= Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(w, z, x, y),
                Vector::<4, T, A>::new(y, x, w, z),
            ]);
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x - z, y - w, z - y, w - x),
                    Vector::<4, T, A>::new(z - x, w - y, y - z, x - w),
                    Vector::<4, T, A>::new(y - w, x - z, w - x, z - y),
                    Vector::<4, T, A>::new(w - y, z - x, x - w, y - z),
                ])
            );
        });
    }

    #[test]
    fn test_mul_scalar() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]) * w,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(z * w, w * w),
                    Vector::<2, T, A>::new(x * w, y * w),
                ])
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                ]) * w,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x * w, y * w, z * w),
                    Vector::<3, T, A>::new(z * w, w * w, y * w),
                    Vector::<3, T, A>::new(x * w, y * w, z * w),
                ])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(y, x, w, z),
                    Vector::<4, T, A>::new(w, z, x, y),
                ]) * w,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x * w, y * w, z * w, w * w),
                    Vector::<4, T, A>::new(z * w, w * w, y * w, x * w),
                    Vector::<4, T, A>::new(y * w, x * w, w * w, z * w),
                    Vector::<4, T, A>::new(w * w, z * w, x * w, y * w),
                ])
            );
        });
    }

    #[test]
    fn test_mul_assign_scalar() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            matrix *= w;
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(z * w, w * w),
                    Vector::<2, T, A>::new(x * w, y * w),
                ])
            );

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            matrix *= w;
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x * w, y * w, z * w),
                    Vector::<3, T, A>::new(z * w, w * w, y * w),
                    Vector::<3, T, A>::new(x * w, y * w, z * w),
                ])
            );

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            matrix *= w;
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x * w, y * w, z * w, w * w),
                    Vector::<4, T, A>::new(z * w, w * w, y * w, x * w),
                    Vector::<4, T, A>::new(y * w, x * w, w * w, z * w),
                    Vector::<4, T, A>::new(w * w, z * w, x * w, y * w),
                ])
            );
        });
    }

    #[test]
    fn test_mul_vector() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]) * Vector::<2, T, A>::new(x, z),
                Vector::<2, T, A>::new(x * z + z * x, x * w + z * y)
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, w),
                ]) * Vector::<3, T, A>::new(y, z, x),
                Vector::<3, T, A>::new(
                    y * x + z * z + x * x,
                    y * y + z * w + x * y,
                    y * z + z * y + x * w
                )
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(y, x, w, z),
                    Vector::<4, T, A>::new(w, z, x, y),
                ]) * Vector::<4, T, A>::new(w, x, w, y),
                Vector::<4, T, A>::new(
                    w * x + x * z + w * y + y * w,
                    w * y + x * w + w * x + y * z,
                    w * z + x * y + w * w + y * x,
                    w * w + x * x + w * z + y * y
                )
            );
        });
    }

    #[test]
    fn test_mul() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x * x * x * x + y * y * y * y + z * z * z * z) {
                return;
            }

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            let matrix2 = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(y, x),
                Vector::<2, T, A>::new(z, w),
            ]);
            let vector = Vector::<2, T, A>::new(x, w);
            assert_float_eq!(
                matrix * matrix2 * vector,
                matrix * (matrix2 * vector),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, w),
            ]);
            let matrix2 = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, y),
                Vector::<3, T, A>::new(z, x, z),
                Vector::<3, T, A>::new(y, z, x),
            ]);
            let vector = Vector::<3, T, A>::new(x, w, y);
            assert_float_eq!(
                matrix * matrix2 * vector,
                matrix * (matrix2 * vector),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            let matrix2 = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, z, y, x),
                Vector::<4, T, A>::new(z, w, x, w),
                Vector::<4, T, A>::new(y, y, x, w),
                Vector::<4, T, A>::new(w, x, y, y),
            ]);
            let vector = Vector::<4, T, A>::new(x, w, y, z);
            assert_float_eq!(
                matrix * matrix2 * vector,
                matrix * (matrix2 * vector),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_mul_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x * x * x * x + y * y * y * y + z * z * z * z) {
                return;
            }

            let matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            let matrix2 = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(y, x),
                Vector::<2, T, A>::new(z, w),
            ]);
            let vector = Vector::<2, T, A>::new(x, w);

            let mut matrix12 = matrix;
            matrix12 *= matrix2;
            assert_float_eq!(
                matrix12 * vector,
                matrix * (matrix2 * vector),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, w),
            ]);
            let matrix2 = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, y),
                Vector::<3, T, A>::new(z, x, z),
                Vector::<3, T, A>::new(y, z, x),
            ]);
            let vector = Vector::<3, T, A>::new(x, w, y);

            let mut matrix12 = matrix;
            matrix12 *= matrix2;
            assert_float_eq!(
                matrix12 * vector,
                matrix * (matrix2 * vector),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            let matrix2 = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, z, y, x),
                Vector::<4, T, A>::new(z, w, x, w),
                Vector::<4, T, A>::new(y, y, x, w),
                Vector::<4, T, A>::new(w, x, y, y),
            ]);
            let vector = Vector::<4, T, A>::new(x, w, y, z);

            let mut matrix12 = matrix;
            matrix12 *= matrix2;
            assert_float_eq!(
                matrix12 * vector,
                matrix * (matrix2 * vector),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_div_scalar() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            assert_float_eq!(
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(x, y),
                ]) / w,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(z / w, w / w),
                    Vector::<2, T, A>::new(x / w, y / w),
                ])
            );
            assert_float_eq!(
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(z, w, y),
                    Vector::<3, T, A>::new(x, y, z),
                ]) / w,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x / w, y / w, z / w),
                    Vector::<3, T, A>::new(z / w, w / w, y / w),
                    Vector::<3, T, A>::new(x / w, y / w, z / w),
                ])
            );
            assert_float_eq!(
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(z, w, y, x),
                    Vector::<4, T, A>::new(y, x, w, z),
                    Vector::<4, T, A>::new(w, z, x, y),
                ]) / w,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x / w, y / w, z / w, w / w),
                    Vector::<4, T, A>::new(z / w, w / w, y / w, x / w),
                    Vector::<4, T, A>::new(y / w, x / w, w / w, z / w),
                    Vector::<4, T, A>::new(w / w, z / w, x / w, y / w),
                ])
            );
        });
    }

    #[test]
    fn test_div_assign_scalar() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            let mut matrix = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            matrix /= w;
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(z / w, w / w),
                    Vector::<2, T, A>::new(x / w, y / w),
                ])
            );

            let mut matrix = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            matrix /= w;
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x / w, y / w, z / w),
                    Vector::<3, T, A>::new(z / w, w / w, y / w),
                    Vector::<3, T, A>::new(x / w, y / w, z / w),
                ])
            );

            let mut matrix = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            matrix /= w;
            assert_float_eq!(
                matrix,
                Matrix::from_columns(&[
                    Vector::<4, T, A>::new(x / w, y / w, z / w, w / w),
                    Vector::<4, T, A>::new(z / w, w / w, y / w, x / w),
                    Vector::<4, T, A>::new(y / w, x / w, w / w, z / w),
                    Vector::<4, T, A>::new(w / w, z / w, x / w, y / w),
                ])
            );
        });
    }
}
