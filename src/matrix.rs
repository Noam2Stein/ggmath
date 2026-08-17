use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
        SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Projective, Scalar, SupportedLength, Unaligned, Vector, Zero,
    length::TwoOrThree,
    utils::{
        Repr3, Repr4, specialize, specialize_23, transmute_generic, transmute_mut, transmute_ref,
    },
};

mod float;
#[cfg(feature = "wide")]
mod wide;
#[cfg(feature = "wide")]
mod wide_float;

/// An `N`x`N` row-major matrix of type `T`.
///
/// `A` controls SIMD alignment and is either [`Unaligned`] or [`Aligned`]. See
/// [`Alignment`] for more details.
///
/// Most constructors are dimension specific. See [`from_rows`] for raw
/// construction.
///
/// # Type aliases
///
/// - [`Mat2<T>`] for [`Matrix<2, T, Unaligned>`].
/// - [`Mat3<T>`] for [`Matrix<3, T, Unaligned>`].
/// - [`Mat4<T>`] for [`Matrix<4, T, Unaligned>`].
/// - [`Mat2A<T>`] for [`Matrix<2, T, Aligned>`].
/// - [`Mat3A<T>`] for [`Matrix<3, T, Aligned>`].
/// - [`Mat4A<T>`] for [`Matrix<4, T, Aligned>`].
///
/// # Fields
///
/// - `x_axis: Vector<N, T, N>` (the first row of the matrix, represents the
///   result of `+X * matrix`, exists for lengths `2`, `3`, `4`)
///
/// - `y_axis: Vector<N, T, N>` (the second row of the matrix, represents the
///   result of `+Y * matrix`, exists for lengths `2`, `3`, `4`)
///
/// - `z_axis: Vector<N, T, N>` (the third row of the matrix, represents the
///   result of `+Z * matrix`, exists for lengths `3`, `4`)
///
/// - `w_axis: Vector<N, T, N>` (the fourth row of the matrix, represents the
///   result of `+W * matrix`, exists for length `4`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Matrix<N, T, A>`] contains `N` consecutive values of [`Vector<N, T, A>`]
/// with no additional padding.
///
/// For `N = 2` this type has the size and alignment of [`Vector<4, T, A>`].
///
/// For `N = 3` and `N = 4` this type has the size and alignment of
/// `[Vector<N, T, A>; N]`.
///
/// [`from_rows`]: Self::from_rows
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

/// A 2x2 row-major matrix.
///
/// This matrix can be used for 2D linear transformations (applied using
/// `vec2 * self`).
///
/// # No SIMD alignment
///
/// [`Mat2<T>`] does not have SIMD alignment, for that use [`Mat2A<T>`].
///
/// # Fields
///
/// - `x_axis: Vec2<T>` (the first row of the matrix, represents the result of
///   `(1, 0) * self`)
///
/// - `y_axis: Vec2<T>` (the second row of the matrix, represents the result of
///   `(0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Mat2<T> = Matrix<2, T, Unaligned>;

/// A 3x3 row-major matrix.
///
/// This matrix can be used for both 3D linear transformations (applied using
/// `vec3 * self`) and 2D affine transformations (applied using
/// `self.transform_point(vec2)` and `self.transform_vector(vec2)`).
///
/// For 2D affine transformations, consider using the [`Affine2<T>`] type which
/// takes less memory than [`Mat3<T>`] and performs better for select operations
/// (see [benchmark results]).
///
/// # No SIMD alignment
///
/// [`Mat3<T>`] does not have SIMD alignment, for that use [`Mat3A<T>`].
///
/// # Fields
///
/// - `x_axis: Vec3<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0) * self`)
///
/// - `y_axis: Vec3<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0) * self`)
///
/// - `z_axis: Vec3<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine2<T>`]: crate::Affine2
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat3<T> = Matrix<3, T, Unaligned>;

/// A 4x4 row-major matrix.
///
/// This matrix can be used for both 3D affine transformations (applied using
/// `self.transform_point(vec3)` and `self.transform_vector(vec3)`) and 3D
/// projections (applied using `self.project_point(vec3)`).
///
/// For 3D affine transformations, consider using the [`Affine3<T>`] type which
/// takes less memory than [`Mat4<T>`] and performs better for select operations
/// (see [benchmark results]).
///
/// # No SIMD alignment
///
/// [`Mat4<T>`] does not have SIMD alignment, for that use [`Mat4A<T>`].
///
/// # Fields
///
/// - `x_axis: Vec4<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0, 0) * self`)
///
/// - `y_axis: Vec4<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0, 0) * self`)
///
/// - `z_axis: Vec4<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1, 0) * self`)
///
/// - `w_axis: Vec4<T>` (the fourth row of the matrix, represents the result of
///   `(0, 0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine3<T>`]: crate::Affine3
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat4<T> = Matrix<4, T, Unaligned>;

/// A 2x2 row-major matrix.
///
/// This matrix can be used for 2D linear transformations (applied using
/// `vec2 * self`).
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Mat2A<T>`] has SIMD alignment. For no SIMD use
/// [`Mat2<T>`].
///
/// # Fields
///
/// - `x_axis: Vec2A<T>` (the first row of the matrix, represents the result of
///   `(1, 0) * self`)
///
/// - `y_axis: Vec2A<T>` (the second row of the matrix, represents the result of
///   `(0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Mat2A<T> = Matrix<2, T, Aligned>;

/// A 3x3 row-major matrix.
///
/// This matrix can be used for both 3D linear transformations (applied using
/// `vec3 * self`) and 2D affine transformations (applied using
/// `self.transform_point(vec2)` and `self.transform_vector(vec2)`).
///
/// For 2D affine transformations, consider using the [`Affine2A<T>`] type which
/// takes less memory than [`Mat3A<T>`] and performs better for select
/// operations (see [benchmark results]).
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Mat3A<T>`] has SIMD alignment. For no SIMD use
/// [`Mat3<T>`].
///
/// # Fields
///
/// - `x_axis: Vec3A<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0) * self`)
///
/// - `y_axis: Vec3A<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0) * self`)
///
/// - `z_axis: Vec3A<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine2A<T>`]: crate::Affine2A
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat3A<T> = Matrix<3, T, Aligned>;

/// A 4x4 row-major matrix.
///
/// This matrix can be used for both 3D affine transformations (applied using
/// `self.transform_point(vec3)` and `self.transform_vector(vec3)`) and 3D
/// projections (applied using `self.project_point(vec3)`).
///
/// For 3D affine transformations, consider using the [`Affine3A<T>`] type which
/// performs better than [`Mat4A`] for select operations (see
/// [benchmark results]).
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Mat4A<T>`] has SIMD alignment. For no SIMD use
/// [`Mat4<T>`].
///
/// # Fields
///
/// - `x_axis: Vec4A<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0, 0) * self`)
///
/// - `y_axis: Vec4A<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0, 0) * self`)
///
/// - `z_axis: Vec4A<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1, 0) * self`)
///
/// - `w_axis: Vec4A<T>` (the fourth row of the matrix, represents the result of
///   `(0, 0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine3A<T>`]: crate::Affine3A
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat4A<T> = Matrix<4, T, Aligned>;

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
    pub const ZERO: Self = Self::from_rows(&[Vector::ZERO; N]);
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
    T: Scalar,
{
    /// Creates a matrix from an array of row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<N, T, A>; N]) -> Self {
        // SAFETY: `Matrix<N, T, A>` contains `N` consecutive values of
        // `Vector<N, T, A>` with no additional padding.
        unsafe { transmute_generic::<[Vector<N, T, A>; N], Matrix<N, T, A>>(*rows) }
    }

    /// Creates a matrix by calling function `f` for each row index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a row vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_row_fn(|i| Vec4::new(i, i, i, 0));
    ///
    /// assert_eq!(matrix[0], Vec4::new(0, 0, 0, 0));
    /// assert_eq!(matrix[1], Vec4::new(1, 1, 1, 0));
    /// assert_eq!(matrix[2], Vec4::new(2, 2, 2, 0));
    /// assert_eq!(matrix[3], Vec4::new(3, 3, 3, 0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_row_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self::from_rows(&core::array::from_fn(f))
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
    /// assert_eq!(matrix[0], Vec4::new(2, 0, 0, 0));
    /// assert_eq!(matrix[1], Vec4::new(0, 2, 0, 0));
    /// assert_eq!(matrix[2], Vec4::new(0, 0, 2, 0));
    /// assert_eq!(matrix[3], Vec4::new(0, 0, 0, 1));
    ///
    /// assert_eq!(matrix.column(0), Vec4::new(2, 0, 0, 0));
    /// assert_eq!(matrix.column(1), Vec4::new(0, 2, 0, 0));
    /// assert_eq!(matrix.column(2), Vec4::new(0, 0, 2, 0));
    /// assert_eq!(matrix.column(3), Vec4::new(0, 0, 0, 1));
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
                transmute_generic::<Matrix<2, T, A>, Matrix<N, T, A>>(Matrix::<2, T, A>::from_rows(
                    &[
                        Vector::<2, T, A>::new(diagonal.as_array()[0], T::ZERO),
                        Vector::<2, T, A>::new(T::ZERO, diagonal.as_array()[1]),
                    ],
                ))
            },

            // SAFETY: Because `N == 3`, `Matrix<3, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Matrix<3, T, A>, Matrix<N, T, A>>(Matrix::<3, T, A>::from_rows(
                    &[
                        Vector::<3, T, A>::new(diagonal.as_array()[0], T::ZERO, T::ZERO),
                        Vector::<3, T, A>::new(T::ZERO, diagonal.as_array()[1], T::ZERO),
                        Vector::<3, T, A>::new(T::ZERO, T::ZERO, diagonal.as_array()[2]),
                    ],
                ))
            },

            // SAFETY: Because `N == 4`, `Matrix<4, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Matrix<4, T, A>, Matrix<N, T, A>>(Matrix::<4, T, A>::from_rows(
                    &[
                        Vector::<4, T, A>::new(diagonal.as_array()[0], T::ZERO, T::ZERO, T::ZERO),
                        Vector::<4, T, A>::new(T::ZERO, diagonal.as_array()[1], T::ZERO, T::ZERO),
                        Vector::<4, T, A>::new(T::ZERO, T::ZERO, diagonal.as_array()[2], T::ZERO),
                        Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, diagonal.as_array()[3]),
                    ],
                ))
            },

            _ => unreachable!(),
        }
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
    /// assert_eq!(matrix[0], Vec4::new(2, 0, 0, 0));
    /// assert_eq!(matrix[1], Vec4::new(0, 2, 0, 0));
    /// assert_eq!(matrix[2], Vec4::new(0, 0, 2, 0));
    /// assert_eq!(matrix[3], Vec4::new(0, 0, 0, 1));
    ///
    /// assert_eq!(matrix.column(0), Vec4::new(2, 0, 0, 0));
    /// assert_eq!(matrix.column(1), Vec4::new(0, 2, 0, 0));
    /// assert_eq!(matrix.column(2), Vec4::new(0, 0, 2, 0));
    /// assert_eq!(matrix.column(3), Vec4::new(0, 0, 0, 1));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vector<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self::from_diagonal(scale)
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
    /// # use ggmath::{Aligned, Mat2, Mat2A, Unaligned, Vec2, Vec2A};
    /// #
    /// let unaligned = Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Mat2A::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]));
    ///
    /// let aligned = Mat2A::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]);
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]));
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
                    Matrix::<3, T, A2>::from_rows(&[
                        matrix.as_rows()[0].to_alignment(),
                        matrix.as_rows()[1].to_alignment(),
                        matrix.as_rows()[2].to_alignment(),
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
    /// # use ggmath::{Mat2, Mat2A, Vec2, Vec2A};
    /// #
    /// let unaligned = Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Mat2A::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]));
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
    /// # use ggmath::{Mat2, Mat2A, Vec2, Vec2A};
    /// #
    /// let aligned = Mat2A::from_rows(&[Vec2A::new(1, 2), Vec2A::new(3, 4)]);
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Mat2::from_rows(&[Vec2::new(1, 2), Vec2::new(3, 4)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Matrix<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Returns a reference to the matrix's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<N, T, A>; N] {
        // SAFETY: `Matrix<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `Vector<N, T, A>`.
        unsafe { transmute_ref::<Matrix<N, T, A>, [Vector<N, T, A>; N]>(self) }
    }

    /// Returns a mutable reference to the matrix's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<N, T, A>; N] {
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
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_rows(&[
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(0, 0, 0, 1),
    /// ]);
    ///
    /// assert_eq!(matrix.column(0), Vec4::new(1, 1, 1, 0));
    /// assert_eq!(matrix.column(1), Vec4::new(2, 2, 2, 0));
    /// assert_eq!(matrix.column(2), Vec4::new(3, 3, 3, 0));
    /// assert_eq!(matrix.column(3), Vec4::new(4, 4, 4, 1));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn column(&self, index: usize) -> Vector<N, T, A> {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type, and `Vector<2, T, A>` and `Vector<N, T, A>`
            // are the same type.
            2 => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    matrix.as_rows()[0].as_array()[index],
                    matrix.as_rows()[1].as_array()[index],
                ))
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type, and `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    matrix.as_rows()[0].as_array()[index],
                    matrix.as_rows()[1].as_array()[index],
                    matrix.as_rows()[2].as_array()[index],
                ))
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type, and `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                let matrix = transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    matrix.as_rows()[0].as_array()[index],
                    matrix.as_rows()[1].as_array()[index],
                    matrix.as_rows()[2].as_array()[index],
                    matrix.as_rows()[3].as_array()[index],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Sets the column at the given index to the given value.
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
    /// let mut matrix = Mat4::from_rows(&[
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(0, 0, 0, 1),
    /// ]);
    /// matrix.set_column(1, Vec4::new(5, 5, 5, 0));
    ///
    /// assert_eq!(matrix[0], Vec4::new(1, 5, 3, 4));
    /// assert_eq!(matrix[1], Vec4::new(1, 5, 3, 4));
    /// assert_eq!(matrix[2], Vec4::new(1, 5, 3, 4));
    /// assert_eq!(matrix[3], Vec4::new(0, 0, 0, 1));
    /// ```
    #[inline]
    #[track_caller]
    pub const fn set_column(&mut self, index: usize, value: Vector<N, T, A>) {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type.
            2 => unsafe {
                let matrix = transmute_mut::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                matrix.as_mut_rows()[0].as_mut_array()[index] = value.as_array()[0];
                matrix.as_mut_rows()[1].as_mut_array()[index] = value.as_array()[1];
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type.
            3 => unsafe {
                let matrix = transmute_mut::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                matrix.as_mut_rows()[0].as_mut_array()[index] = value.as_array()[0];
                matrix.as_mut_rows()[1].as_mut_array()[index] = value.as_array()[1];
                matrix.as_mut_rows()[2].as_mut_array()[index] = value.as_array()[2];
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type.
            4 => unsafe {
                let matrix = transmute_mut::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                matrix.as_mut_rows()[0].as_mut_array()[index] = value.as_array()[0];
                matrix.as_mut_rows()[1].as_mut_array()[index] = value.as_array()[1];
                matrix.as_mut_rows()[2].as_mut_array()[index] = value.as_array()[2];
                matrix.as_mut_rows()[3].as_mut_array()[index] = value.as_array()[3];
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
    /// let matrix = Mat4::from_rows(&[
    ///     Vec4::new(1, 1, 1, 0),
    ///     Vec4::new(2, 2, 2, 0),
    ///     Vec4::new(3, 3, 3, 0),
    ///     Vec4::new(4, 4, 4, 1),
    /// ]);
    /// assert_eq!(
    ///     matrix.transpose(),
    ///     Mat4::from_rows(&[
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
        specialize!(Matrix::<N, T, A>::transpose_backend(self))
    }

    /// Transforms `vector` by the transpose of `self`.
    ///
    /// Equivalent to `vector * self.transpose()` but is faster and may return a
    /// slightly different value.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transpose_mul_vector(&self, vector: Vector<N, T, A>) -> Vector<N, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        specialize!(Matrix::<N, T, A>::transpose_mul_vector_backend(
            self, vector
        ))
    }

    /// Returns the diagonal of `self`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat4, Vec4};
    /// #
    /// let matrix = Mat4::from_rows(&[
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
                    self.as_rows()[0].as_array()[0],
                    self.as_rows()[1].as_array()[1],
                ))
            },

            // SAFETY: Because `N == 3`, `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    self.as_rows()[0].as_array()[0],
                    self.as_rows()[1].as_array()[1],
                    self.as_rows()[2].as_array()[2],
                ))
            },

            // SAFETY: Because `N == 4`, `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    self.as_rows()[0].as_array()[0],
                    self.as_rows()[1].as_array()[1],
                    self.as_rows()[2].as_array()[2],
                    self.as_rows()[3].as_array()[3],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Returns a matrix that first applies scaling vector `scale` then applies
    /// `self`.
    ///
    /// Equivalent to `Matrix::from_scale(scale) * self` but is faster. This
    /// may be inconsistent for NaNs and `-0.0`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn prepend_scale(&self, scale: Vector<N, T, A>) -> Self
    where
        T: Mul<Output = T>,
    {
        specialize!(Matrix::<N, T, A>::prepend_scale_backend(self, scale))
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
        specialize!(Matrix::<N, T, A>::determinant_backend(self))
    }

    /// TODO
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub fn from_projective(projective: &Projective<N, T, A>) -> Self
    where
        Length<N>: TwoOrThree,
    {
        specialize_23!(Matrix::<N, T, A>::from_projective_backend(projective))
    }

    /// Returns a mutable reference to the matrix's rows.
    ///
    /// This function has been renamed to [`as_mut_rows`].
    ///
    /// [`as_mut_rows`]: Self::as_mut_rows
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_mut_rows`")]
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<N, T, A>; N] {
        self.as_mut_rows()
    }
}

impl<T, A: Alignment> Matrix<2, T, A>
where
    T: Scalar,
{
    /// Creates a matrix from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let matrix = Mat2::from_row_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix, Mat2::from_rows(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 4]) -> Self {
        Self::from_rows(&[
            Vector::<2, T, A>::new(array[0], array[1]),
            Vector::<2, T, A>::new(array[2], array[3]),
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
    pub fn to_homogeneous(&self) -> Matrix<3, T, A>
    where
        T: Zero + One,
    {
        Matrix::from_rows(&[
            self.x_axis.extend(T::ZERO),
            self.y_axis.extend(T::ZERO),
            Vector::<3, T, A>::Z,
        ])
    }

    /// Returns a 2x2 matrix discarding the third row and column.
    #[inline]
    #[must_use]
    pub fn from_homogeneous(homogeneous: &Matrix<3, T, A>) -> Self {
        Self::from_rows(&[homogeneous.x_axis.truncate(), homogeneous.y_axis.truncate()])
    }

    #[inline(always)]
    fn transpose_backend(&self) -> Self {
        Self(self.0.xzyw())
    }

    #[track_caller]
    #[inline(always)]
    fn transpose_mul_vector_backend(&self, vector: Vector<2, T, A>) -> Vector<2, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Vector::<2, T, A>::new(self.x_axis.dot(vector), self.y_axis.dot(vector))
    }

    #[track_caller]
    #[inline(always)]
    fn prepend_scale_backend(&self, scale: Vector<2, T, A>) -> Self
    where
        T: Mul<Output = T>,
    {
        Self(self.0 * scale.xxyy())
    }

    #[track_caller]
    #[inline(always)]
    fn determinant_backend(&self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.x_axis.x * self.y_axis.y - self.x_axis.y * self.y_axis.x
    }

    #[inline(always)]
    fn from_projective_backend(projective: &Projective<2, T, A>) -> Self {
        Self::from_rows(&[projective.x_axis.truncate(), projective.y_axis.truncate()])
    }
}

impl<T, A: Alignment> Matrix<3, T, A>
where
    T: Scalar,
{
    /// Creates a matrix from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let matrix = Mat2::from_row_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix, Mat2::from_rows(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 9]) -> Self {
        Self::from_rows(&[
            Vector::<3, T, A>::new(array[0], array[1], array[2]),
            Vector::<3, T, A>::new(array[3], array[4], array[5]),
            Vector::<3, T, A>::new(array[6], array[7], array[8]),
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
    pub fn to_homogeneous(&self) -> Matrix<4, T, A>
    where
        T: Zero + One,
    {
        Matrix::from_rows(&[
            self.x_axis.extend(T::ZERO),
            self.y_axis.extend(T::ZERO),
            self.z_axis.extend(T::ZERO),
            Vector::<4, T, A>::W,
        ])
    }

    /// Returns a 2x2 matrix discarding the third row and column.
    #[inline]
    #[must_use]
    pub fn from_homogeneous(homogeneous: &Matrix<4, T, A>) -> Self {
        Self::from_rows(&[
            homogeneous.x_axis.truncate(),
            homogeneous.y_axis.truncate(),
            homogeneous.z_axis.truncate(),
        ])
    }

    /// Returns a 2x2 matrix discarding the given `row` and `column`.
    ///
    /// # Panics
    ///
    /// Panics if `row` or `column` are greater than `2`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn remove(&self, row: usize, column: usize) -> Matrix<2, T, A> {
        match (row, column) {
            (0, 0) => Matrix::from_rows(&[self.y_axis.yz(), self.z_axis.yz()]),
            (0, 1) => Matrix::from_rows(&[self.y_axis.xz(), self.z_axis.xz()]),
            (0, 2) => Matrix::from_rows(&[self.y_axis.xy(), self.z_axis.xy()]),
            (1, 0) => Matrix::from_rows(&[self.x_axis.yz(), self.z_axis.yz()]),
            (1, 1) => Matrix::from_rows(&[self.x_axis.xz(), self.z_axis.xz()]),
            (1, 2) => Matrix::from_rows(&[self.x_axis.xy(), self.z_axis.xy()]),
            (2, 0) => Matrix::from_rows(&[self.x_axis.yz(), self.y_axis.yz()]),
            (2, 1) => Matrix::from_rows(&[self.x_axis.xz(), self.y_axis.xz()]),
            (2, 2) => Matrix::from_rows(&[self.x_axis.xy(), self.y_axis.xy()]),
            _ => panic!("index out of bounds"),
        }
    }

    #[inline(always)]
    fn transpose_backend(&self) -> Self {
        Self::from_rows(&[
            Vector::<3, T, A>::new(self.x_axis.x, self.y_axis.x, self.z_axis.x),
            Vector::<3, T, A>::new(self.x_axis.y, self.y_axis.y, self.z_axis.y),
            Vector::<3, T, A>::new(self.x_axis.z, self.y_axis.z, self.z_axis.z),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn transpose_mul_vector_backend(&self, vector: Vector<3, T, A>) -> Vector<3, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Vector::<3, T, A>::new(
            self.x_axis.dot(vector),
            self.y_axis.dot(vector),
            self.z_axis.dot(vector),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn prepend_scale_backend(&self, scale: Vector<3, T, A>) -> Self
    where
        T: Mul<Output = T>,
    {
        Self::from_rows(&[
            self.x_axis * scale.x,
            self.y_axis * scale.y,
            self.z_axis * scale.z,
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn determinant_backend(&self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        self.x_axis.cross(self.y_axis).dot(self.z_axis)
    }

    #[inline(always)]
    fn from_projective_backend(projective: &Projective<3, T, A>) -> Self {
        Self::from_rows(&[
            projective.x_axis.truncate(),
            projective.y_axis.truncate(),
            projective.z_axis.truncate(),
        ])
    }
}

impl<T, A: Alignment> Matrix<4, T, A>
where
    T: Scalar,
{
    /// Creates a matrix from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let matrix = Mat2::from_row_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(matrix, Mat2::from_rows(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 16]) -> Self {
        Self::from_rows(&[
            Vector::<4, T, A>::new(array[0], array[1], array[2], array[3]),
            Vector::<4, T, A>::new(array[4], array[5], array[6], array[7]),
            Vector::<4, T, A>::new(array[8], array[9], array[10], array[11]),
            Vector::<4, T, A>::new(array[12], array[13], array[14], array[15]),
        ])
    }

    /// Returns a 3x3 matrix discarding the given `row` and `column`.
    ///
    /// # Panics
    ///
    /// Panics if `row` or `column` are greater than `3`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn remove(&self, row: usize, column: usize) -> Matrix<3, T, A> {
        match (row, column) {
            (0, 0) => Matrix::from_rows(&[self.y_axis.yzw(), self.z_axis.yzw(), self.w_axis.yzw()]),
            (0, 1) => Matrix::from_rows(&[self.y_axis.xzw(), self.z_axis.xzw(), self.w_axis.xzw()]),
            (0, 2) => Matrix::from_rows(&[self.y_axis.xyw(), self.z_axis.xyw(), self.w_axis.xyw()]),
            (0, 3) => Matrix::from_rows(&[self.y_axis.xyz(), self.z_axis.xyz(), self.w_axis.xyz()]),
            (1, 0) => Matrix::from_rows(&[self.x_axis.yzw(), self.z_axis.yzw(), self.w_axis.yzw()]),
            (1, 1) => Matrix::from_rows(&[self.x_axis.xzw(), self.z_axis.xzw(), self.w_axis.xzw()]),
            (1, 2) => Matrix::from_rows(&[self.x_axis.xyw(), self.z_axis.xyw(), self.w_axis.xyw()]),
            (1, 3) => Matrix::from_rows(&[self.x_axis.xyz(), self.z_axis.xyz(), self.w_axis.xyz()]),
            (2, 0) => Matrix::from_rows(&[self.x_axis.yzw(), self.y_axis.yzw(), self.w_axis.yzw()]),
            (2, 1) => Matrix::from_rows(&[self.x_axis.xzw(), self.y_axis.xzw(), self.w_axis.xzw()]),
            (2, 2) => Matrix::from_rows(&[self.x_axis.xyw(), self.y_axis.xyw(), self.w_axis.xyw()]),
            (2, 3) => Matrix::from_rows(&[self.x_axis.xyz(), self.y_axis.xyz(), self.w_axis.xyz()]),
            (3, 0) => Matrix::from_rows(&[self.x_axis.yzw(), self.y_axis.yzw(), self.z_axis.yzw()]),
            (3, 1) => Matrix::from_rows(&[self.x_axis.xzw(), self.y_axis.xzw(), self.z_axis.xzw()]),
            (3, 2) => Matrix::from_rows(&[self.x_axis.xyw(), self.y_axis.xyw(), self.z_axis.xyw()]),
            (3, 3) => Matrix::from_rows(&[self.x_axis.xyz(), self.y_axis.xyz(), self.z_axis.xyz()]),
            _ => panic!("index out of bounds"),
        }
    }

    #[inline(always)]
    fn transpose_backend(&self) -> Self {
        Self::from_rows(&[
            Vector::<4, T, A>::new(self.x_axis.x, self.y_axis.x, self.z_axis.x, self.w_axis.x),
            Vector::<4, T, A>::new(self.x_axis.y, self.y_axis.y, self.z_axis.y, self.w_axis.y),
            Vector::<4, T, A>::new(self.x_axis.z, self.y_axis.z, self.z_axis.z, self.w_axis.z),
            Vector::<4, T, A>::new(self.x_axis.w, self.y_axis.w, self.z_axis.w, self.w_axis.w),
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn transpose_mul_vector_backend(&self, vector: Vector<4, T, A>) -> Vector<4, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Vector::<4, T, A>::new(
            self.x_axis.dot(vector),
            self.y_axis.dot(vector),
            self.z_axis.dot(vector),
            self.w_axis.dot(vector),
        )
    }

    #[track_caller]
    #[inline(always)]
    fn prepend_scale_backend(&self, scale: Vector<4, T, A>) -> Self
    where
        T: Mul<Output = T>,
    {
        Self::from_rows(&[
            self.x_axis * scale.x,
            self.y_axis * scale.y,
            self.z_axis * scale.z,
            self.w_axis * scale.w,
        ])
    }

    #[track_caller]
    #[inline(always)]
    fn determinant_backend(&self) -> T
    where
        T: Neg<Output = T> + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        if const { align_of::<Vector<4, T, A>>() > align_of::<T>() } {
            // Ported from `https://docs.rs/glam/0.33.1/src/glam/f32/sse2/mat4.rs.html#649-685`.
            // Based on https://github.com/g-truc/glm `glm_mat4_determinant_lowp`.

            // `[det_23_23, det_13_23, det_12_23, det_03_23]`
            let dets_23_1234 =
                self.z_axis.zyyx() * self.w_axis.wwzw() - self.z_axis.wwzw() * self.w_axis.zyyx();

            // `[det_02_23, det_01_23, _, _]`
            let dets_23_56 = {
                // `[m02 * m23, m02 * m13, m22 * m03, m12 * m03]`
                let products = self.z_axis.xxzy() * self.w_axis.zyxx();

                // `[m02 * m23 - m22 * m03, m02 * m13 - m12 * m03, _, _]`
                products - products.zwxx()
            };

            // `[det_123_123, det_023_123, det_013_123, det_012_123]`
            let dets_123 = {
                // `[det_23_23, det_23_23, det_13_23, det_12_23]`
                let dets_23_1123 = dets_23_1234.xxyz();

                // `[det_13_23, det_03_23, det_03_23, det_02_23]`
                let dets_23_2445 = Vector::<4, T, A>::new(
                    dets_23_1234.y,
                    dets_23_1234.w,
                    dets_23_56.x,
                    dets_23_56.x,
                )
                .xyyw();

                // `[det_12_23, det_02_23, det_01_23, det_01_23]`
                let dets_23_3566 = Vector::<4, T, A>::new(
                    dets_23_1234.z,
                    dets_23_1234.z,
                    dets_23_56.x,
                    dets_23_56.y,
                )
                .xzww();

                self.y_axis.yxxx() * dets_23_1123 - self.y_axis.zzyy() * dets_23_2445
                    + self.y_axis.wwwz() * dets_23_3566
            };

            let cofactors = self.x_axis * dets_123;
            let cofactors =
                Vector::<4, T, A>::new(cofactors.x, -cofactors.y, cofactors.z, -cofactors.w);

            cofactors.element_sum()
        } else {
            // Ported from `https://docs.rs/glam/0.33.1/src/glam/f64/dmat4.rs.html#629-646`.

            let [m00, m10, m20, m30] = self.x_axis.to_array();
            let [m01, m11, m21, m31] = self.y_axis.to_array();
            let [m02, m12, m22, m32] = self.z_axis.to_array();
            let [m03, m13, m23, m33] = self.w_axis.to_array();

            let det_23_23 = m22 * m33 - m32 * m23;
            let det_13_23 = m12 * m33 - m32 * m13;
            let det_12_23 = m12 * m23 - m22 * m13;
            let det_03_23 = m02 * m33 - m32 * m03;
            let det_02_23 = m02 * m23 - m22 * m03;
            let det_01_23 = m02 * m13 - m12 * m03;

            let det_123_123 = m11 * det_23_23 - m21 * det_13_23 + m31 * det_12_23;
            let det_023_123 = m01 * det_23_23 - m21 * det_03_23 + m31 * det_02_23;
            let det_013_123 = m01 * det_13_23 - m11 * det_03_23 + m31 * det_01_23;
            let det_012_123 = m01 * det_12_23 - m11 * det_02_23 + m21 * det_01_23;

            m00 * det_123_123 - m10 * det_023_123 + m20 * det_013_123 - m30 * det_012_123
        }
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

impl<const N: usize, T, A: Alignment> Index<usize> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = Vector<N, T, A>;

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to the dimension of the matrix.
    #[inline]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_rows()[index]
    }
}

impl<const N: usize, T, A: Alignment> IndexMut<usize> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Returns a mutable reference to the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to the dimension of the matrix.
    #[inline]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_rows()[index]
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat2Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first row of the matrix.
    ///
    /// This represents the result of multiplying `(1, 0)` by the matrix.
    pub x_axis: Vector<2, T, A>,
    /// The second row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 1)` by the matrix.
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
    /// The first row of the matrix.
    ///
    /// This represents the result of multiplying `(1, 0, 0)` by the matrix.
    pub x_axis: Vector<3, T, A>,
    /// The second row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 1, 0)` by the matrix.
    pub y_axis: Vector<3, T, A>,
    /// The third row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 0, 1)` by the matrix.
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
    /// The first row of the matrix.
    ///
    /// This represents the result of multiplying `(1, 0, 0, 0)` by the matrix.
    pub x_axis: Vector<4, T, A>,
    /// The second row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 1, 0, 0)` by the matrix.
    pub y_axis: Vector<4, T, A>,
    /// The third row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 0, 1, 0)` by the matrix.
    pub z_axis: Vector<4, T, A>,
    /// The fourth row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 0, 0, 1)` by the matrix.
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
        write!(f, "{:?}", self.as_rows())
    }
}

impl<const N: usize, T, A: Alignment> Display for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "[{}, {}]", self[0], self[1]),
            3 => write!(f, "[{}, {}, {}]", self[0], self[1], self[2]),
            4 => write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3]),
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
        (0..N).all(|i| self[i] == other[i])
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
        self.as_rows().hash(state);
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
                Matrix::from_row_fn(|i| -self[i])
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
                Matrix::from_row_fn(|i| self[i] + rhs[i])
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
                Matrix::from_row_fn(|i| self[i] - rhs[i])
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
                Matrix::from_row_fn(|i| self[i] * rhs)
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
                Matrix::from_row_fn(|i| self[i] * rhs)
            }
        }
    };
}
impl_mul!(
    /// Matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, matrix multiplication first
    /// applies the left-hand side matrix, then the right-hand side matrix.
    ///
    /// Equivalent to `[self.x_axis * rhs, self.y_axis * rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<N, T, A>) -> Self::Output {
                self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                match N {
                    2 => rhs[0] * self[0] + rhs[1] * self[1],
                    3 => rhs[0] * self[0] + rhs[1] * self[1] + rhs[2] * self[2],
                    4 => rhs[0] * self[0] + rhs[1] * self[1] + rhs[2] * self[2] + rhs[3] * self[3],
                    _ => unreachable!(),
                }
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Matrix<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<N, T, A>) -> Self::Output {
                *self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }
    };
}
impl_vector_mul!(
    /// Vector-matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, they always go on the
    /// left-hand side.
    ///
    /// Equivalent to `self.x * rhs.x_axis + self.y * rhs.y_axis + ...`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
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
    /// Because vectors are treated as row matrices, matrix multiplication first
    /// applies the left-hand side matrix, then the right-hand side matrix.
    ///
    /// Equivalent to `self = [self.x_axis * rhs, self.y_axis * rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix * matrix`.
);

macro_rules! impl_vector_mul_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign<Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Matrix<N, T, A>) {
                *self = *self * &rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Matrix<N, T, A>) {
                *self = *self * rhs;
            }
        }
    };
}
impl_vector_mul_assign!(
    /// Vector-matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, they always go on the
    /// left-hand side.
    ///
    /// Equivalent to `self.x * rhs.x_axis + self.y * rhs.y_axis + ...`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
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
                Matrix::from_row_fn(|i| self[i] / rhs)
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
