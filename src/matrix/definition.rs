use core::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Scalar, ScalarBackend, ScalarRepr, SignedInteger, SupportedLength,
    Unaligned, Vector,
    constants::Zero,
    utils::{Repr2, Repr3, Repr4, specialize, transmute_generic, transmute_mut, transmute_ref},
};

/// A square column major matrix.
///
/// `Matrix` is the generic form of:
///
/// - [`Mat2<T>`](crate::Mat2)
/// - [`Mat3<T>`](crate::Mat3)
/// - [`Mat4<T>`](crate::Mat4)
/// - [`Mat2U<T>`](crate::Mat2U)
/// - [`Mat3U<T>`](crate::Mat3U)
/// - [`Mat4U<T>`](crate::Mat4U)
///
/// `Matrix` is generic over:
///
/// - `N`: Length (2, 3, or 4)
/// - `T`: Scalar type (see [`Scalar`])
/// - `A`: Alignment (see [`Alignment`])
///
/// To initialize matrices, use the macros [`mat2`](crate::mat2),
/// [`mat3`](crate::mat3), [`mat4`](crate::mat4). To initialize a matrix of an
/// unknown length, use [`Matrix::from_col_array`].
///
/// # Guarantees
///
/// `Matrix<N, T, A>` is guaranteed to be made out of `N` consecutive values of
/// `Vector<N, T, A>` without any padding in the middle, followed by optional
/// padding that is made out of initialized bytes.
///
/// `Matrix<N, T, Unaligned>` is guaranteed to have the size and alignment of
/// `[T; N * N]`.
///
/// `Matrix<2, T, Aligned>` is guaranteed to be a transparent wrapper around
/// `Vector<4, T, Aligned>`.
///
/// `Matrix<3, T, Aligned>` is guaranteed to have the size of either
/// `[Vec3<T>; 3]` or `[Vec3<T>; 4]`, and may have additional alignment. When
/// the size is of `[Vec3<T>; 4]`, the padding vector is guaranteed to have
/// initialized bytes, but may not contain initialized values of `T` if it
/// doesn't accept all bit-patterns.
///
/// `Matrix<4, T, Aligned>` is guaranteed to have the size of `[T; 16]`, and may
/// have additional alignment.
///
/// Types containing `Matrix` are not guaranteed to have the same memory layout
/// as types containing `[Vector<N, T, A>; N]`. For example, `Option<Mat2U<T>>`
/// is not guaranteed to have the same size as `Option<[Vec2U<T>; 2]>`.
///
/// Matrices of scalars with the same [`Scalar::Repr`] are guaranteed to have
/// compatible memory layouts if `Repr` is a signed integer. The alignment of
/// the matrices is not guaranteed to be the same, but their size and element
/// positions do.
///
/// Keep in mind that scalars that have the same `Repr` today might silently
/// change their `Repr` in the future.
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
    T: Scalar,
{
    /// Creates a matrix from an array of columns.
    ///
    /// The preferable way to create matrices is using the macros
    /// [`mat2`](crate::mat2), [`mat3`](crate::mat3), [`mat4`](crate::mat4).
    ///
    /// `Matrix::from_array` should only be used when the length of the matrix
    /// is unknown or when directly converting from an array.
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<N, T, A>; N]) -> Self {
        match N {
            // SAFETY: Because `N == 2`, `[Vector<N, T, A>; N]` and
            // `[Vector<2, T, A>; 2]` are the same type, and `Matrix<N, T, A>`
            // and `Matrix<2, T, A>` are the same type.
            2 => unsafe {
                let array = transmute_generic::<[Vector<N, T, A>; N], [Vector<2, T, A>; 2]>(*array);
                transmute_generic::<Matrix<2, T, A>, Matrix<N, T, A>>(Matrix::<2, T, A>::new(
                    array[0], array[1],
                ))
            },

            // SAFETY: Because `N == 3`, `[Vector<N, T, A>; N]` and
            // `[Vector<3, T, A>; 3]` are the same type, and `Matrix<N, T, A>`
            // and `Matrix<3, T, A>` are the same type.
            3 => unsafe {
                let array = transmute_generic::<[Vector<N, T, A>; N], [Vector<3, T, A>; 3]>(*array);
                transmute_generic::<Matrix<3, T, A>, Matrix<N, T, A>>(Matrix::<3, T, A>::new(
                    array[0], array[1], array[2],
                ))
            },

            // SAFETY: Because `N == 4`, `[Vector<N, T, A>; N]` and
            // `[Vector<4, T, A>; 4]` are the same type, and `Matrix<N, T, A>`
            // and `Matrix<4, T, A>` are the same type.
            4 => unsafe {
                let array = transmute_generic::<[Vector<N, T, A>; N], [Vector<4, T, A>; 4]>(*array);
                transmute_generic::<Matrix<4, T, A>, Matrix<N, T, A>>(Matrix::<4, T, A>::new(
                    array[0], array[1], array[2], array[3],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Creates a matrix by calling function `f` for each column index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a column.
    #[inline]
    #[must_use]
    pub fn from_col_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self::from_col_array(&core::array::from_fn(f))
    }

    /// Creates a matrix with its diagonal set to `diagonal` and all other entries set to 0.
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
                transmute_generic::<Matrix<2, T, A>, Matrix<N, T, A>>(Matrix::<2, T, A>::new(
                    Vector::<2, T, A>::new(diagonal.as_array_ref()[0], T::ZERO),
                    Vector::<2, T, A>::new(T::ZERO, diagonal.as_array_ref()[1]),
                ))
            },

            // SAFETY: Because `N == 3`, `Matrix<3, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Matrix<3, T, A>, Matrix<N, T, A>>(Matrix::<3, T, A>::new(
                    Vector::<3, T, A>::new(diagonal.as_array_ref()[0], T::ZERO, T::ZERO),
                    Vector::<3, T, A>::new(T::ZERO, diagonal.as_array_ref()[1], T::ZERO),
                    Vector::<3, T, A>::new(T::ZERO, T::ZERO, diagonal.as_array_ref()[2]),
                ))
            },

            // SAFETY: Because `N == 4`, `Matrix<4, T, A>` and `Matrix<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Matrix<4, T, A>, Matrix<N, T, A>>(Matrix::<4, T, A>::new(
                    Vector::<4, T, A>::new(diagonal.as_array_ref()[0], T::ZERO, T::ZERO, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, diagonal.as_array_ref()[1], T::ZERO, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, diagonal.as_array_ref()[2], T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, diagonal.as_array_ref()[3]),
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Converts the matrix to the specified alignment.
    ///
    /// See [`Alignment`] for more information.
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
            // are the same type, and `Matrix<N, T, A2>` and `Matrix<3, T, A2`
            // are the same type.
            (3, false) => unsafe {
                let mat = transmute_generic::<Matrix<N, T, A>, Matrix<3, T, A>>(*self);
                transmute_generic::<Matrix<3, T, A2>, Matrix<N, T, A2>>(Matrix::<3, T, A2>::new(
                    mat.as_col_array_ref()[0].to_alignment(),
                    mat.as_col_array_ref()[1].to_alignment(),
                    mat.as_col_array_ref()[2].to_alignment(),
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Converts the matrix to [`Aligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(&self) -> Matrix<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the matrix to [`Unaligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Matrix<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts the matrix to an array of columns.
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<N, T, A>; N] {
        *self.as_col_array_ref()
    }

    /// Returns a reference to the matrix's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<N, T, A>; N] {
        // SAFETY: `Matrix<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `Vector<N, T, A>`.
        unsafe { transmute_ref::<Matrix<N, T, A>, [Vector<N, T, A>; N]>(self) }
    }

    /// Returns a mutable reference to the matrix's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<N, T, A>; N] {
        // SAFETY: `Matrix<N, T, A>` is guaranteed to begin with `N` consecutive
        // values of `Vector<N, T, A>`.
        unsafe { transmute_mut::<Matrix<N, T, A>, [Vector<N, T, A>; N]>(self) }
    }

    /// Returns the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    #[must_use]
    pub const fn col(&self, index: usize) -> Vector<N, T, A> {
        self.as_col_array_ref()[index]
    }

    /// Returns a mutable reference to the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    #[must_use]
    pub const fn col_mut(&mut self, index: usize) -> &mut Vector<N, T, A> {
        &mut self.as_col_array_mut()[index]
    }

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of bounds.
    #[inline]
    #[must_use]
    pub const fn row(&self, index: usize) -> Vector<N, T, A> {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type, and `Vector<2, T, A>` and `Vector<N, T, A>`
            // are the same type.
            2 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    mat.as_col_array_ref()[0].as_array_ref()[index],
                    mat.as_col_array_ref()[1].as_array_ref()[index],
                ))
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type, and `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    mat.as_col_array_ref()[0].as_array_ref()[index],
                    mat.as_col_array_ref()[1].as_array_ref()[index],
                    mat.as_col_array_ref()[2].as_array_ref()[index],
                ))
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type, and `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                let mat = transmute_ref::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    mat.as_col_array_ref()[0].as_array_ref()[index],
                    mat.as_col_array_ref()[1].as_array_ref()[index],
                    mat.as_col_array_ref()[2].as_array_ref()[index],
                    mat.as_col_array_ref()[3].as_array_ref()[index],
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
    #[inline]
    pub const fn set_row(&mut self, index: usize, value: Vector<N, T, A>) {
        match N {
            // SAFETY: Because `N == 2`, `Matrix<N, T, A>` and `Matrix<2, T, A>`
            // are the same type.
            2 => unsafe {
                let mat = transmute_mut::<Matrix<N, T, A>, Matrix<2, T, A>>(self);
                mat.as_col_array_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                mat.as_col_array_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
            },

            // SAFETY: Because `N == 3`, `Matrix<N, T, A>` and `Matrix<3, T, A>`
            // are the same type.
            3 => unsafe {
                let mat = transmute_mut::<Matrix<N, T, A>, Matrix<3, T, A>>(self);
                mat.as_col_array_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                mat.as_col_array_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
                mat.as_col_array_mut()[2].as_array_mut()[index] = value.as_array_ref()[2];
            },

            // SAFETY: Because `N == 4`, `Matrix<N, T, A>` and `Matrix<4, T, A>`
            // are the same type.
            4 => unsafe {
                let mat = transmute_mut::<Matrix<N, T, A>, Matrix<4, T, A>>(self);
                mat.as_col_array_mut()[0].as_array_mut()[index] = value.as_array_ref()[0];
                mat.as_col_array_mut()[1].as_array_mut()[index] = value.as_array_ref()[1];
                mat.as_col_array_mut()[2].as_array_mut()[index] = value.as_array_ref()[2];
                mat.as_col_array_mut()[3].as_array_mut()[index] = value.as_array_ref()[3];
            },

            _ => unreachable!(),
        }
    }

    /// Returns the diagonal of the matrix.
    #[inline]
    #[must_use]
    pub const fn diagonal(&self) -> Vector<N, T, A> {
        match N {
            // SAFETY: Because `N == 2`, `Vector<2, T, A>` and `Vector<N, T, A>`
            // are the same type.
            2 => unsafe {
                transmute_generic::<Vector<2, T, A>, Vector<N, T, A>>(Vector::<2, T, A>::new(
                    self.as_col_array_ref()[0].as_array_ref()[0],
                    self.as_col_array_ref()[1].as_array_ref()[1],
                ))
            },

            // SAFETY: Because `N == 3`, `Vector<3, T, A>` and `Vector<N, T, A>`
            // are the same type.
            3 => unsafe {
                transmute_generic::<Vector<3, T, A>, Vector<N, T, A>>(Vector::<3, T, A>::new(
                    self.as_col_array_ref()[0].as_array_ref()[0],
                    self.as_col_array_ref()[1].as_array_ref()[1],
                    self.as_col_array_ref()[2].as_array_ref()[2],
                ))
            },

            // SAFETY: Because `N == 4`, `Vector<4, T, A>` and `Vector<N, T, A>`
            // are the same type.
            4 => unsafe {
                transmute_generic::<Vector<4, T, A>, Vector<N, T, A>>(Vector::<4, T, A>::new(
                    self.as_col_array_ref()[0].as_array_ref()[0],
                    self.as_col_array_ref()[1].as_array_ref()[1],
                    self.as_col_array_ref()[2].as_array_ref()[2],
                    self.as_col_array_ref()[3].as_array_ref()[3],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Reinterprets the bits of the matrix to a different scalar type.
    ///
    /// The two scalar types must have compatible memory layouts. This is
    /// enforced via trait bounds in this function's signature.
    ///
    /// This function is used to make SIMD optimizations in implementations of [`Scalar`].
    ///
    /// # Safety
    ///
    /// The components of the input must be valid for the output matrix type.
    ///
    /// For example, when converting matrices from `u8` to `bool` the
    /// input components must be either `0` or `1`.
    ///
    /// The optional padding does not need to be a valid value of `T2`.
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
}

impl<T, A: Alignment> Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    pub(crate) const fn new(x: Vector<2, T, A>, y: Vector<2, T, A>) -> Self {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to be made out of 2
        // consecutive values of `Vector<2, T, A>`, with no additional padding.
        unsafe { transmute_generic::<Repr2<Vector<2, T, A>>, Matrix<2, T, A>>(Repr2(x, y)) }
    }
}

impl<T, A: Alignment> Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    pub(crate) const fn new(x: Vector<3, T, A>, y: Vector<3, T, A>, z: Vector<3, T, A>) -> Self {
        match size_of::<Matrix<3, T, A>>() / size_of::<Vector<3, T, A>>() {
            // SAFETY: Because the matrix has 3 values of `Vector<3, T, A>` and
            // no padding, its equivalent to `Repr3<Vector<3, T, A>>`.
            3 => unsafe {
                transmute_generic::<Repr3<Vector<3, T, A>>, Matrix<3, T, A>>(Repr3(x, y, z))
            },

            // SAFETY: Because the vector has 4 values of `Vector<3, T, A>` plus
            // 1 padding vector, its equivalent to `Repr4<Vector<3, T, A>>`.
            4 => unsafe {
                transmute_generic::<Repr4<Vector<3, T, A>>, Matrix<3, T, A>>(Repr4(x, y, z, z))
            },

            _ => unreachable!(),
        }
    }
}

impl<T, A: Alignment> Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    pub(crate) const fn new(
        x: Vector<4, T, A>,
        y: Vector<4, T, A>,
        z: Vector<4, T, A>,
        w: Vector<4, T, A>,
    ) -> Self {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to be made out of 4
        // consecutive values of `Vector<4, T, A>`, with no additional padding.
        unsafe { transmute_generic::<Repr4<Vector<4, T, A>>, Matrix<4, T, A>>(Repr4(x, y, z, w)) }
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

impl<const N: usize, T, A: Alignment> Debug for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.to_col_array())
    }
}

impl<const N: usize, T, A: Alignment> Display for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "[{}, {}]", self.col(0), self.col(1)),
            3 => write!(f, "[{}, {}, {}]", self.col(0), self.col(1), self.col(2)),
            4 => write!(
                f,
                "[{}, {}, {}, {}]",
                self.col(0),
                self.col(1),
                self.col(2),
                self.col(3)
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

impl<const N: usize, T, A: Alignment> Neg for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Neg<Output = T>,
{
    type Output = Self;

    #[inline]
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

    #[inline]
    fn neg(self) -> Self::Output {
        specialize!(<T as ScalarBackend<N, A>>::mat_neg(self))
    }
}

impl<const N: usize, T, A: Alignment> Add for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Add<Output = T>,
{
    type Output = Self;

    #[expect(clippy::op_ref)]
    #[inline]
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

    #[expect(clippy::op_ref)]
    #[inline]
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

    #[expect(clippy::op_ref)]
    #[inline]
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

    #[inline]
    fn add(self, rhs: Self) -> Self::Output {
        specialize!(<T as ScalarBackend<N, A>>::mat_add(self, rhs))
    }
}

impl<const N: usize, T, A: Alignment> AddAssign for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Add<Output = T>,
{
    #[expect(clippy::op_ref)]
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        *self = &*self + rhs;
    }
}

impl<const N: usize, T, A: Alignment> AddAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Add<Output = T>,
{
    #[expect(clippy::op_ref)]
    #[inline]
    fn add_assign(&mut self, rhs: &Self) {
        *self = &*self + rhs;
    }
}

impl<const N: usize, T, A: Alignment> Sub for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sub<Output = T>,
{
    type Output = Self;

    #[expect(clippy::op_ref)]
    #[inline]
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

    #[expect(clippy::op_ref)]
    #[inline]
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

    #[expect(clippy::op_ref)]
    #[inline]
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

    #[inline]
    fn sub(self, rhs: Self) -> Self::Output {
        specialize!(<T as ScalarBackend<N, A>>::mat_sub(self, rhs))
    }
}

impl<const N: usize, T, A: Alignment> SubAssign for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sub<Output = T>,
{
    #[expect(clippy::op_ref)]
    #[inline]
    fn sub_assign(&mut self, rhs: Self) {
        *self = &*self - rhs;
    }
}

impl<const N: usize, T, A: Alignment> SubAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sub<Output = T>,
{
    #[expect(clippy::op_ref)]
    #[inline]
    fn sub_assign(&mut self, rhs: &Self) {
        *self = &*self - rhs;
    }
}

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
