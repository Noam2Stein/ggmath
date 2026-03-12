use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Deref, DerefMut, Neg, Sub, SubAssign},
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
