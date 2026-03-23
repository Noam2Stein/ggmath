use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, ScalarBackend, ScalarRepr, SignedInteger, SupportedLength,
    Unaligned, Vector,
    constants::{Nan, One, Zero},
    repr::{Repr2, Repr3, Repr4},
    specialize::specialize,
    transmute::{transmute_generic, transmute_mut, transmute_ref},
};

mod constructor;
mod float;

/// An `N`x`N` column-major matrix of type `T`.
///
/// `A` controls SIMD alignment and is either [`Aligned`] or [`Unaligned`]. See
/// [`Alignment`] for more details.
///
/// Matrices are currently missing most functionality. See [`from_columns`] for
/// raw construction.
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
/// followed by optional padding.
///
/// `Matrix<N, T, Unaligned>` has the alignment of `T` and has no padding.
/// `Matrix<N, T, Aligned>` may have higher alignment than
/// [`Vector<N, T, Aligned>`]. [`Mat2<T>`] has the exact layout of [`Vec4<T>`].
/// [`Mat3<T>`] may have one padding vector. [`Mat4<T>`] has no padding.
///
/// Padding is fully initialized and accepts all bit patterns. Unless `T`
/// accepts all bit patterns, it is not sound to assume padding contains valid
/// values of `T`.
///
/// Matrices of compatible [`Scalar::Repr`] types have the same size. This means
/// that they are transmutable, but can still have different alignments (see
/// [`to_repr`]).
///
/// Types containing compatible matrices, vectors and arrays may not have
/// compatible layouts themselves. For example, even though [`Mat2<T>`] and
/// [`Vec4<T>`] have compatible layouts, [`Option<Mat2<T>>`] and
/// [`Option<Vec4<T>>`] may not.
///
/// [`from_columns`]: Self::from_columns
/// [`Mat2<T>`]: crate::Mat2
/// [`Mat3<T>`]: crate::Mat3
/// [`Mat4<T>`]: crate::Mat4
/// [`Mat2U<T>`]: crate::Mat2U
/// [`Mat3U<T>`]: crate::Mat3U
/// [`Mat4U<T>`]: crate::Mat4U
/// [`Vec4<T>`]: crate::Vec4
/// [`to_repr`]: Self::to_repr
#[repr(transparent)]
pub struct Matrix<const N: usize, T, A: Alignment>(
    pub(crate) <T::Repr as ScalarRepr>::MatrixRepr<N, T, A>,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

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
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` is the same type as
            // `Matrix<2, T, A>` which contains 2 values of `Vector<2, T, A>`.
            2 => unsafe {
                transmute_generic::<Repr2<Vector<N, T, A>>, Matrix<N, T, A>>(Repr2(
                    columns[0], columns[1],
                ))
            },

            3 => {
                match size_of::<Matrix<3, T, A>>() / size_of::<Vector<3, T, A>>() {
                    // SAFETY: Because `N == 3`, `Matrix<N, T, A>` is the same
                    // type as `Matrix<3, T, A>` which is checked to contain
                    // exactly 3 values of `Vector<3, T, A>`.
                    3 => unsafe {
                        transmute_generic::<Repr3<Vector<N, T, A>>, Matrix<N, T, A>>(Repr3(
                            columns[0], columns[1], columns[2],
                        ))
                    },

                    // SAFETY: Because `N == 3`, `Matrix<N, T, A>` is the same
                    // type as `Matrix<3, T, A>` which is checked to contain
                    // exactly 4 values of `Vector<3, T, A>`.
                    4 => unsafe {
                        transmute_generic::<Repr4<Vector<N, T, A>>, Matrix<N, T, A>>(Repr4(
                            columns[0], columns[1], columns[2], columns[2],
                        ))
                    },

                    _ => unreachable!(),
                }
            }

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` is the same type as
            // `Matrix<4, T, A>` which contains 4 values of `Vector<4, T, A>`.
            4 => unsafe {
                transmute_generic::<Repr4<Vector<N, T, A>>, Matrix<N, T, A>>(Repr4(
                    columns[0], columns[1], columns[2], columns[3],
                ))
            },

            _ => unreachable!(),
        }
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
    /// let mat = Mat4::from_column_fn(|i| Vec4::new(i, i, i, 0));
    ///
    /// assert_eq!(mat.column(0), Vec4::new(0, 0, 0, 0));
    /// assert_eq!(mat.column(1), Vec4::new(1, 1, 1, 0));
    /// assert_eq!(mat.column(2), Vec4::new(2, 2, 2, 0));
    /// assert_eq!(mat.column(3), Vec4::new(3, 3, 3, 0));
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
    /// let mat = Mat4::from_diagonal(Vec4::new(2, 2, 2, 1));
    ///
    /// assert_eq!(mat.column(0), Vec4::new(2, 0, 0, 0));
    /// assert_eq!(mat.column(1), Vec4::new(0, 2, 0, 0));
    /// assert_eq!(mat.column(2), Vec4::new(0, 0, 2, 0));
    /// assert_eq!(mat.column(3), Vec4::new(0, 0, 0, 1));
    ///
    /// assert_eq!(mat.row(0), Vec4::new(2, 0, 0, 0));
    /// assert_eq!(mat.row(1), Vec4::new(0, 2, 0, 0));
    /// assert_eq!(mat.row(2), Vec4::new(0, 0, 2, 0));
    /// assert_eq!(mat.row(3), Vec4::new(0, 0, 0, 1));
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
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                transmute_generic::<Matrix<3, T, A2>, Matrix<N, T, A2>>(
                    Matrix::<3, T, A2>::from_columns(&[
                        mat.as_columns()[0].to_alignment(),
                        mat.as_columns()[1].to_alignment(),
                        mat.as_columns()[2].to_alignment(),
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
    /// let mat = Mat4::from_columns(&[
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(0, 0, 0, 1),
    /// ]);
    ///
    /// assert_eq!(mat.row(0), Vec4::new(1, 1, 1, 0));
    /// assert_eq!(mat.row(1), Vec4::new(2, 2, 2, 0));
    /// assert_eq!(mat.row(2), Vec4::new(3, 3, 3, 0));
    /// assert_eq!(mat.row(3), Vec4::new(4, 4, 4, 1));
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
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    mat.as_columns()[0].as_array_ref()[index],
                    mat.as_columns()[1].as_array_ref()[index],
                ))
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type, and `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    mat.as_columns()[0].as_array_ref()[index],
                    mat.as_columns()[1].as_array_ref()[index],
                    mat.as_columns()[2].as_array_ref()[index],
                ))
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type, and `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    mat.as_columns()[0].as_array_ref()[index],
                    mat.as_columns()[1].as_array_ref()[index],
                    mat.as_columns()[2].as_array_ref()[index],
                    mat.as_columns()[3].as_array_ref()[index],
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
    /// let mut mat = Mat4::from_columns(&[
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(1, 2, 3, 4),
    ///     Vec4::new(0, 0, 0, 1),
    /// ]);
    /// mat.set_row(1, Vec4::new(5, 5, 5, 0));
    ///
    /// assert_eq!(mat.column(0), Vec4::new(1, 5, 3, 4));
    /// assert_eq!(mat.column(1), Vec4::new(1, 5, 3, 4));
    /// assert_eq!(mat.column(2), Vec4::new(1, 5, 3, 4));
    /// assert_eq!(mat.column(3), Vec4::new(0, 0, 0, 1));
    /// ```
    #[inline]
    #[track_caller]
    pub const fn set_row(&mut self, index: usize, value: Vector<N, T, A>) {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type.
            2 => unsafe {
                let mat = transmute_mut::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                mat.as_columns_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                mat.as_columns_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type.
            3 => unsafe {
                let mat = transmute_mut::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                mat.as_columns_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                mat.as_columns_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
                mat.as_columns_mut()[2].as_array_mut()[index] = value.as_array_ref()[2];
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type.
            4 => unsafe {
                let mat = transmute_mut::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                mat.as_columns_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                mat.as_columns_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
                mat.as_columns_mut()[2].as_array_mut()[index] = value.as_array_ref()[2];
                mat.as_columns_mut()[3].as_array_mut()[index] = value.as_array_ref()[3];
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
    /// let mat = Mat4::from_columns(&[
    ///     Vec4::new(1, 1, 1, 0),
    ///     Vec4::new(2, 2, 2, 0),
    ///     Vec4::new(3, 3, 3, 0),
    ///     Vec4::new(4, 4, 4, 1),
    /// ]);
    /// assert_eq!(
    ///     mat.transpose(),
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
    pub fn transpose(self) -> Self {
        // TODO: Replace `self` with `&self` for `v0.17.0`.
        Self::from_column_fn(|i| self.row(i))
    }

    /// Transforms the vector `rhs` by the transpose of `self`.
    ///
    /// Equivalent to `self.transpose() * rhs` but is faster and may return a
    /// slightly different value.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transpose_mul_vec(&self, rhs: Vector<N, T, A>) -> Vector<N, T, A>
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
    /// let mat = Mat4::from_columns(&[
    ///     Vec4::new(1, 1, 1, 0),
    ///     Vec4::new(2, 2, 2, 0),
    ///     Vec4::new(3, 3, 3, 0),
    ///     Vec4::new(4, 4, 4, 1),
    /// ]);
    ///
    /// assert_eq!(mat.diagonal(), Vec4::new(1, 2, 3, 1));
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
    /// output type. For example, when converting matrices from `u8` to `bool`,
    /// the input elements must be either `0` or `1` (that example is
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
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let bits = Mat2::<u8>::from_columns(&[Vec2::new(1, 0), Vec2::new(0, 1)]);
    ///
    /// // SAFETY: `bool` accepts both the `0` and `1` bit patterns.
    /// let bools = unsafe { bits.to_repr::<bool>() };
    ///
    /// assert_eq!(bools, Mat2::from_columns(&[Vec2::new(true, false), Vec2::new(false, true)]));
    /// ```
    ///
    /// Incorrect usage:
    ///
    /// ```compile_fail
    /// # use ggmath::{Mat2, Vec2};
    /// #
    /// let a = Mat2::<i32>::from_columns(&[Vec2::new(1, 2), Vec2::new(3, 4)]);
    ///
    /// // This does not compile since `i32` and `i64` are not compatible.
    /// let _ = unsafe { a.to_repr::<i64>() };
    /// ```
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub const unsafe fn to_repr<T2>(self) -> Matrix<N, T2, A>
    where
        T2: Scalar<Repr = T::Repr>,
        T::Repr: SignedInteger,
    {
        // SAFETY: Matrices of scalars with the same `Scalar::Repr` are
        // guaranteed to have compatible memory layouts if `Repr` is a signed
        // integer.
        unsafe { transmute_generic::<Matrix<N, T, A>, Matrix<N, T2, A>>(self) }
    }

    /// Creates a matrix from an array of column vectors.
    ///
    /// This function has been renamed to [`from_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "renamed to `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<N, T, A>; N]) -> Self {
        Self::from_columns(array)
    }

    /// Creates a matrix by calling function `f` for each column index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a column
    /// vectors.
    ///
    /// This function has been renamed to [`from_column_fn`]. This name will be
    /// removed in a future version.
    ///
    /// [`from_column_fn`]: Self::from_column_fn
    #[deprecated(since = "0.16.3", note = "renamed to `from_column_fn`")]
    #[inline]
    #[must_use]
    pub fn from_col_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self::from_column_fn(f)
    }

    /// Converts the matrix to an array of column vectors.
    ///
    /// This function has been replaced by [`as_columns`] and will be removed in
    /// a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "replaced by `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<N, T, A>; N] {
        *self.as_columns()
    }

    /// Returns a reference to the matrix's columns.
    ///
    /// This function has been renamed to [`as_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<N, T, A>; N] {
        self.as_columns()
    }

    /// Returns a mutable reference to the matrix's columns.
    ///
    /// This function has been renamed to [`as_columns_mut`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns_mut`]: Self::as_columns_mut
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns_mut`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<N, T, A>; N] {
        self.as_columns_mut()
    }

    /// Returns the column at the given index.
    ///
    /// This function has been renamed to [`column`]. This name will be removed
    /// in a future version.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the dimension of the
    /// matrix.
    ///
    /// [`column`]: Self::column
    #[deprecated(since = "0.16.3", note = "renamed to `column`")]
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn col(&self, index: usize) -> Vector<N, T, A> {
        self.column(index)
    }

    /// Returns a mutable reference to the column at the given index.
    ///
    /// This function has been renamed to [`column_mut`]. This name will be
    /// removed in a future version.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the dimension of the
    /// matrix.
    ///
    /// [`column_mut`]: Self::column_mut
    #[deprecated(since = "0.16.3", note = "renamed to `column_mut`")]
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn col_mut(&mut self, index: usize) -> &mut Vector<N, T, A> {
        self.column_mut(index)
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
    /// let mat = Mat2::from_column_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(mat, Mat2::from_columns(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
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
    /// let mat = Mat2::from_column_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(mat, Mat2::from_columns(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
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
    /// let mat = Mat2::from_column_array(&[1.0, 2.0, 3.0, 4.0]);
    /// assert_eq!(mat, Mat2::from_columns(&[Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)]));
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
        specialize!(<T as ScalarBackend<N, A>>::mat_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::mat_ne(self, other))
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
                specialize!(<T as ScalarBackend<N, A>>::mat_neg(self))
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
                specialize!(<T as ScalarBackend<N, A>>::mat_add(self, rhs))
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
    /// This operation is fully consistent with `mat + mat`.
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
                specialize!(<T as ScalarBackend<N, A>>::mat_sub(self, rhs))
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
    /// This operation is fully consistent with `mat - mat`.
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
                specialize!(<T as ScalarBackend<N, A>>::mat_mul_scalar(self, rhs))
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

macro_rules! impl_mul_vec {
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
                specialize!(<T as ScalarBackend<N, A>>::mat_mul_vec(self, rhs))
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
impl_mul_vec!(
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
                specialize!(<T as ScalarBackend<N, A>>::mat_mul(self, rhs))
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
                specialize!(<T as ScalarBackend<N, A>>::mat_div_scalar(self, rhs))
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
    use crate::{
        Aligned, Mat2, Mat2U, Mat3, Mat3U, Mat4, Mat4U, Matrix, Unaligned, Vec2, Vec3, Vec4,
        Vector,
        test_utils::{assert_float_eq, assert_panic, for_parameters},
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

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(mat.column(0), Vector::<2, T, A>::new(x, y));
            assert_eq!(mat.column(1), Vector::<2, T, A>::new(z, w));
            assert_panic!(mat.column(2));

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(mat.column(0), Vector::<3, T, A>::new(x, y, z));
            assert_eq!(mat.column(1), Vector::<3, T, A>::new(w, a, b));
            assert_eq!(mat.column(2), Vector::<3, T, A>::new(c, d, e));
            assert_panic!(mat.column(3));

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(mat.column(0), Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(mat.column(1), Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(mat.column(2), Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(mat.column(3), Vector::<4, T, A>::new(i, j, k, l));
            assert_panic!(mat.column(4));
        });
    }

    #[test]
    fn test_column_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(mat.column_mut(0), &mut Vector::<2, T, A>::new(x, y));
            assert_eq!(mat.column_mut(1), &mut Vector::<2, T, A>::new(z, w));
            assert_panic!(mat.clone().column_mut(2));

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(mat.column_mut(0), &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(mat.column_mut(1), &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(mat.column_mut(2), &mut Vector::<3, T, A>::new(c, d, e));
            assert_panic!(mat.clone().column_mut(3));

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(mat.column_mut(0), &mut Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(mat.column_mut(1), &mut Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(mat.column_mut(2), &mut Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(mat.column_mut(3), &mut Vector::<4, T, A>::new(i, j, k, l));
            assert_panic!(mat.clone().column_mut(4));
        });
    }

    #[test]
    fn test_row() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(mat.row(0), Vector::<2, T, A>::new(x, z));
            assert_eq!(mat.row(1), Vector::<2, T, A>::new(y, w));
            assert_panic!(mat.row(2));

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(mat.row(0), Vector::<3, T, A>::new(x, w, c));
            assert_eq!(mat.row(1), Vector::<3, T, A>::new(y, a, d));
            assert_eq!(mat.row(2), Vector::<3, T, A>::new(z, b, e));
            assert_panic!(mat.row(3));

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(mat.row(0), Vector::<4, T, A>::new(x, a, e, i));
            assert_eq!(mat.row(1), Vector::<4, T, A>::new(y, b, f, j));
            assert_eq!(mat.row(2), Vector::<4, T, A>::new(z, c, g, k));
            assert_eq!(mat.row(3), Vector::<4, T, A>::new(w, d, h, l));
            assert_panic!(mat.row(4));
        });
    }

    #[test]
    fn test_set_row() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            mat.set_row(0, Vector::<2, T, A>::new(a, b));
            assert_eq!(
                mat,
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(a, y),
                    Vector::<2, T, A>::new(b, w)
                ])
            );
            mat.set_row(1, Vector::<2, T, A>::new(c, d));
            assert_eq!(
                mat,
                Matrix::<2, T, A>::from_columns(&[
                    Vector::<2, T, A>::new(a, c),
                    Vector::<2, T, A>::new(b, d)
                ])
            );
            assert_panic!(mat.clone().set_row(2, Vector::ZERO));

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            mat.set_row(0, Vector::<3, T, A>::new(a, b, d));
            assert_eq!(
                mat,
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(a, y, z),
                    Vector::<3, T, A>::new(b, a, b),
                    Vector::<3, T, A>::new(d, d, e)
                ])
            );
            mat.set_row(1, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(
                mat,
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(a, x, z),
                    Vector::<3, T, A>::new(b, y, b),
                    Vector::<3, T, A>::new(d, z, e)
                ])
            );
            mat.set_row(2, Vector::<3, T, A>::new(e, f, g));
            assert_eq!(
                mat,
                Matrix::<3, T, A>::from_columns(&[
                    Vector::<3, T, A>::new(a, x, e),
                    Vector::<3, T, A>::new(b, y, f),
                    Vector::<3, T, A>::new(d, z, g)
                ])
            );
            assert_panic!(mat.clone().set_row(3, Vector::ZERO));

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            mat.set_row(0, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(
                mat,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, y, z, w),
                    Vector::<4, T, A>::new(b, b, c, d),
                    Vector::<4, T, A>::new(c, f, g, h),
                    Vector::<4, T, A>::new(d, j, k, l)
                ])
            );
            mat.set_row(1, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(
                mat,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, x, z, w),
                    Vector::<4, T, A>::new(b, y, c, d),
                    Vector::<4, T, A>::new(c, z, g, h),
                    Vector::<4, T, A>::new(d, w, k, l)
                ])
            );
            mat.set_row(2, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(
                mat,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, x, a, w),
                    Vector::<4, T, A>::new(b, y, b, d),
                    Vector::<4, T, A>::new(c, z, c, h),
                    Vector::<4, T, A>::new(d, w, d, l)
                ])
            );
            mat.set_row(3, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(
                mat,
                Matrix::<4, T, A>::from_columns(&[
                    Vector::<4, T, A>::new(a, x, a, e),
                    Vector::<4, T, A>::new(b, y, b, f),
                    Vector::<4, T, A>::new(c, z, c, g),
                    Vector::<4, T, A>::new(d, w, d, h)
                ])
            );
            assert_panic!(mat.clone().set_row(4, Vector::ZERO));
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
    fn test_transpose_mul_vec() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let w = T::max(x, y);

            if !T::is_finite(x * x) || !T::is_finite(y * y) || !T::is_finite(z * z) {
                return;
            }

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            let vec = Vector::<2, T, A>::new(x, z);
            assert_float_eq!(
                mat.transpose_mul_vec(vec),
                mat.transpose() * vec,
                abs <= Vector::splat((x * x).max(y * y).max(z * z)) * 1e-6
            );

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, z, y),
            ]);
            let vec = Vector::<3, T, A>::new(z, y, w);
            assert_float_eq!(
                mat.transpose_mul_vec(vec),
                mat.transpose() * vec,
                abs <= Vector::splat((x * x).max(y * y).max(z * z)) * 1e-6
            );

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, z, y, z),
                Vector::<4, T, A>::new(y, y, z, x),
            ]);
            let vec = Vector::<4, T, A>::new(z, y, w, z);
            assert_float_eq!(
                mat.transpose_mul_vec(vec),
                mat.transpose() * vec,
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

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            let scale = Vector::<2, T, A>::new(x, z);
            assert_float_eq!(
                mat.mul_diagonal(scale),
                mat * Matrix::from_diagonal(scale),
                0.0 = -0.0
            );

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, z, y),
            ]);
            let scale = Vector::<3, T, A>::new(z, y, w);
            assert_float_eq!(
                mat.mul_diagonal(scale),
                mat * Matrix::from_diagonal(scale),
                0.0 = -0.0
            );

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, z, y, z),
                Vector::<4, T, A>::new(y, y, z, x),
            ]);
            let scale = Vector::<4, T, A>::new(z, y, w, z);
            assert_float_eq!(
                mat.mul_diagonal(scale),
                mat * Matrix::from_diagonal(scale),
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_to_repr() {
        for_parameters!(|A| {
            assert_eq!(
                // SAFETY: `u32` accepts all bit patterns.
                unsafe {
                    Matrix::<2, i32, A>::from_columns(&[
                        Vector::<2, i32, A>::new(0, 1),
                        Vector::<2, i32, A>::new(2, 3),
                    ])
                    .to_repr()
                },
                Matrix::<2, u32, A>::from_columns(&[
                    Vector::<2, u32, A>::new(0, 1),
                    Vector::<2, u32, A>::new(2, 3)
                ])
            );
            assert_eq!(
                // SAFETY: `u32` accepts all bit patterns.
                unsafe {
                    Matrix::<3, i32, A>::from_columns(&[
                        Vector::<3, i32, A>::new(0, 1, 2),
                        Vector::<3, i32, A>::new(3, 4, 5),
                        Vector::<3, i32, A>::new(6, 7, 8),
                    ])
                    .to_repr()
                },
                Matrix::<3, u32, A>::from_columns(&[
                    Vector::<3, u32, A>::new(0, 1, 2),
                    Vector::<3, u32, A>::new(3, 4, 5),
                    Vector::<3, u32, A>::new(6, 7, 8)
                ])
            );
            assert_eq!(
                // SAFETY: `u32` accepts all bit patterns.
                unsafe {
                    Matrix::<4, i32, A>::from_columns(&[
                        Vector::<4, i32, A>::new(0, 1, 2, 3),
                        Vector::<4, i32, A>::new(4, 5, 6, 7),
                        Vector::<4, i32, A>::new(8, 9, 10, 11),
                        Vector::<4, i32, A>::new(12, 13, 14, 15),
                    ])
                    .to_repr()
                },
                Matrix::<4, u32, A>::from_columns(&[
                    Vector::<4, u32, A>::new(0, 1, 2, 3),
                    Vector::<4, u32, A>::new(4, 5, 6, 7),
                    Vector::<4, u32, A>::new(8, 9, 10, 11),
                    Vector::<4, u32, A>::new(12, 13, 14, 15)
                ])
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

        if cfg!(assertions) {
            assert_panic!(
                Mat3::from_columns(&[Vec3::new(2, 3, 0), Vec3::new(4, 5, 1), Vec3::new(6, 7, 1)])
                    .transform_point(Vec2::new(-1, -2))
            );
            assert_panic!(
                Mat4::from_columns(&[
                    Vec4::new(2, 3, 4, 0),
                    Vec4::new(5, 6, 7, 0),
                    Vec4::new(8, 9, 10, 1),
                    Vec4::new(11, 12, 13, 1)
                ])
                .transform_point(Vec3::new(-1, -2, -3))
            );
        }
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

        if cfg!(assertions) {
            assert_panic!(
                Mat3::from_columns(&[Vec3::new(2, 3, 0), Vec3::new(4, 5, 1), Vec3::new(6, 7, 1)])
                    .transform_vector(Vec2::new(-1, -2))
            );
            assert_panic!(
                Mat4::from_columns(&[
                    Vec4::new(2, 3, 4, 0),
                    Vec4::new(5, 6, 7, 0),
                    Vec4::new(8, 9, 10, 1),
                    Vec4::new(11, 12, 13, 1)
                ])
                .transform_vector(Vec3::new(-1, -2, -3))
            );
        }
    }

    #[test]
    fn test_deref() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(mat.x_axis, Vector::<2, T, A>::new(x, y));
            assert_eq!(mat.y_axis, Vector::<2, T, A>::new(z, w));

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(mat.x_axis, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(mat.y_axis, Vector::<3, T, A>::new(w, a, b));
            assert_eq!(mat.z_axis, Vector::<3, T, A>::new(c, d, e));

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(mat.x_axis, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(mat.y_axis, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(mat.z_axis, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(mat.w_axis, Vector::<4, T, A>::new(i, j, k, l));
        });
    }

    #[test]
    fn test_deref_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(&mut mat.x_axis, &mut Vector::<2, T, A>::new(x, y));
            assert_eq!(&mut mat.y_axis, &mut Vector::<2, T, A>::new(z, w));

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(&mut mat.x_axis, &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(&mut mat.y_axis, &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(&mut mat.z_axis, &mut Vector::<3, T, A>::new(c, d, e));

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(&mut mat.x_axis, &mut Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(&mut mat.y_axis, &mut Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(&mut mat.z_axis, &mut Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(&mut mat.w_axis, &mut Vector::<4, T, A>::new(i, j, k, l));
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

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            mat += Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x + z, y + w),
                    Vector::<2, T, A>::new(z + x, w + y),
                ])
            );

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            mat += Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
            ]);
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x + z, y + w, z + y),
                    Vector::<3, T, A>::new(z + x, w + y, y + z),
                    Vector::<3, T, A>::new(x + z, y + w, z + y),
                ])
            );

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            mat += Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(w, z, x, y),
                Vector::<4, T, A>::new(y, x, w, z),
            ]);
            assert_float_eq!(
                mat,
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

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            mat -= Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(x - z, y - w),
                    Vector::<2, T, A>::new(z - x, w - y),
                ])
            );

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            mat -= Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
            ]);
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x - z, y - w, z - y),
                    Vector::<3, T, A>::new(z - x, w - y, y - z),
                    Vector::<3, T, A>::new(x - z, y - w, z - y),
                ])
            );

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            mat -= Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(w, z, x, y),
                Vector::<4, T, A>::new(y, x, w, z),
            ]);
            assert_float_eq!(
                mat,
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

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            mat *= w;
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(z * w, w * w),
                    Vector::<2, T, A>::new(x * w, y * w),
                ])
            );

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            mat *= w;
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x * w, y * w, z * w),
                    Vector::<3, T, A>::new(z * w, w * w, y * w),
                    Vector::<3, T, A>::new(x * w, y * w, z * w),
                ])
            );

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            mat *= w;
            assert_float_eq!(
                mat,
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
    fn test_mul_vec() {
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

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            let mat2 = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(y, x),
                Vector::<2, T, A>::new(z, w),
            ]);
            let vec = Vector::<2, T, A>::new(x, w);
            assert_float_eq!(
                mat * mat2 * vec,
                mat * (mat2 * vec),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, w),
            ]);
            let mat2 = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, y),
                Vector::<3, T, A>::new(z, x, z),
                Vector::<3, T, A>::new(y, z, x),
            ]);
            let vec = Vector::<3, T, A>::new(x, w, y);
            assert_float_eq!(
                mat * mat2 * vec,
                mat * (mat2 * vec),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            let mat2 = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, z, y, x),
                Vector::<4, T, A>::new(z, w, x, w),
                Vector::<4, T, A>::new(y, y, x, w),
                Vector::<4, T, A>::new(w, x, y, y),
            ]);
            let vec = Vector::<4, T, A>::new(x, w, y, z);
            assert_float_eq!(
                mat * mat2 * vec,
                mat * (mat2 * vec),
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

            let mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            let mat2 = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(y, x),
                Vector::<2, T, A>::new(z, w),
            ]);
            let vec = Vector::<2, T, A>::new(x, w);

            let mut mat12 = mat;
            mat12 *= mat2;
            assert_float_eq!(
                mat12 * vec,
                mat * (mat2 * vec),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, w),
            ]);
            let mat2 = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, y),
                Vector::<3, T, A>::new(z, x, z),
                Vector::<3, T, A>::new(y, z, x),
            ]);
            let vec = Vector::<3, T, A>::new(x, w, y);

            let mut mat12 = mat;
            mat12 *= mat2;
            assert_float_eq!(
                mat12 * vec,
                mat * (mat2 * vec),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 0.00001,
                0.0 = -0.0
            );

            let mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            let mat2 = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, z, y, x),
                Vector::<4, T, A>::new(z, w, x, w),
                Vector::<4, T, A>::new(y, y, x, w),
                Vector::<4, T, A>::new(w, x, y, y),
            ]);
            let vec = Vector::<4, T, A>::new(x, w, y, z);

            let mut mat12 = mat;
            mat12 *= mat2;
            assert_float_eq!(
                mat12 * vec,
                mat * (mat2 * vec),
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

            let mut mat = Matrix::<2, T, A>::from_columns(&[
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(x, y),
            ]);
            mat /= w;
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<2, T, A>::new(z / w, w / w),
                    Vector::<2, T, A>::new(x / w, y / w),
                ])
            );

            let mut mat = Matrix::<3, T, A>::from_columns(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(z, w, y),
                Vector::<3, T, A>::new(x, y, z),
            ]);
            mat /= w;
            assert_float_eq!(
                mat,
                Matrix::from_columns(&[
                    Vector::<3, T, A>::new(x / w, y / w, z / w),
                    Vector::<3, T, A>::new(z / w, w / w, y / w),
                    Vector::<3, T, A>::new(x / w, y / w, z / w),
                ])
            );

            let mut mat = Matrix::<4, T, A>::from_columns(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(z, w, y, x),
                Vector::<4, T, A>::new(y, x, w, z),
                Vector::<4, T, A>::new(w, z, x, y),
            ]);
            mat /= w;
            assert_float_eq!(
                mat,
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
