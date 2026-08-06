use crate::{
    Aligned, Alignment, Length, Matrix, Scalar, Unaligned, Vector,
    length::TwoOrThree,
    utils::{transmute_generic, transmute_ref},
};

/// An `N`-dimensional projective transform represented by a homogeneous matrix.
///
/// TODO
#[repr(transparent)]
#[expect(private_bounds)]
pub struct Projective<const N: usize, T, A: Alignment>(
    <Length<N> as TwoOrThree>::Select<Matrix<3, T, A>, Matrix<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D projective transform represented by a homogeneous 3x3 matrix.
///
/// TODO
pub type Proj2<T> = Projective<2, T, Unaligned>;

/// A 3D projective transform represented by a homogeneous 4x4 matrix.
///
/// TODO
pub type Proj3<T> = Projective<3, T, Unaligned>;

/// A 2D projective transform represented by a homogeneous 3x3 matrix.
///
/// TODO
pub type Proj2A<T> = Projective<2, T, Aligned>;

/// A 3D projective transform represented by a homogeneous 4x4 matrix.
///
/// TODO
pub type Proj3A<T> = Projective<3, T, Aligned>;

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
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
    pub const fn to_alignment<A2: Alignment>(&self) -> Projective<N, T, A2> {
        match (N, A2::IS_ALIGNED == A::IS_ALIGNED) {
            // SAFETY: If `A` is `A2`, the types of the transmute are the same
            // and make it safe. Otherwhise, matrices with length `4` are
            // guaranteed to be made out of `N * N` consecutive values of `T`
            // with no padding. Meaning they have compatible layouts between
            // alignments.
            (3, _) | (_, true) => unsafe {
                transmute_generic::<Projective<N, T, A>, Projective<N, T, A2>>(*self)
            },

            // SAFETY: Because `N == 2`, `Projective<N, T, A>` and
            // `Projective<2, T, A>` are the same type, and
            // `Projective<N, T, A2>` and `Projective<2, T, A2>` are the same
            // type.
            (2, false) => unsafe {
                let proj = transmute_ref::<Projective<N, T, A>, Projective<2, T, A>>(self);
                transmute_generic::<Projective<2, T, A2>, Projective<N, T, A2>>(Projective::<
                    2,
                    T,
                    A2,
                >(
                    proj.0.to_alignment(),
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(&self) -> Projective<N, T, Aligned> {
        self.to_alignment()
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Projective<N, T, Unaligned> {
        self.to_alignment()
    }
}

impl<T, A: Alignment> Projective<2, T, A>
where
    T: Scalar,
{
    /// Creates a projective transform from an array of homogeneous row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<3, T, A>; 3]) -> Self {
        Self(Matrix::from_rows(rows))
    }

    /// Creates a projective transform by calling function `f` for each
    /// homogeneous row index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a homogeneous
    /// row vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj3, Vec4};
    /// #
    /// let transform = Proj3::from_row_fn(|i| Vec4::new(i, i, i, 0));
    ///
    /// assert_eq!(transform[0], Vec4::new(0, 0, 0, 0));
    /// assert_eq!(transform[1], Vec4::new(1, 1, 1, 0));
    /// assert_eq!(transform[2], Vec4::new(2, 2, 2, 0));
    /// assert_eq!(transform[3], Vec4::new(3, 3, 3, 0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_row_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<3, T, A>,
    {
        Self(Matrix::from_row_fn(f))
    }

    /// Creates a projective transform from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj2, Vec3};
    /// #
    /// let proj = Proj2::from_row_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(
    ///     proj,
    ///     Proj2::from_rows(&[
    ///         Vec3::new(1, 2, 3),
    ///         Vec3::new(4, 5, 6),
    ///         Vec3::new(7, 8, 9),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 9]) -> Self {
        Self(Matrix::<3, T, A>::from_row_array(array))
    }

    /// Reinterprets a homogeneous 3x3 matrix as a 2D projective transform.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn from_homogeneous(homogeneous: &Matrix<3, T, A>) -> Self {
        Self(*homogeneous)
    }

    /// Reinterprets `self` as a homogeneous 3x3 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn to_homogeneous(&self) -> Matrix<3, T, A> {
        self.0
    }

    /// Reinterprets `self` as a reference to a homogeneous 3x3 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_homogeneous(&self) -> &Matrix<3, T, A> {
        &self.0
    }

    /// Reinterprets `self` as a mutable reference to a homogeneous 3x3 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_mut_homogeneous(&mut self) -> &mut Matrix<3, T, A> {
        &mut self.0
    }

    /// Returns a reference to the transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<3, T, A>; 3] {
        self.0.as_rows()
    }

    /// Returns a mutable reference to the transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<3, T, A>; 3] {
        self.0.as_mut_rows()
    }

    /// Returns the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of rows
    /// `N + 1`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn column(&self, index: usize) -> Vector<3, T, A> {
        self.0.column(index)
    }

    /// Sets the column at the given index to the given value.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of rows
    /// `N + 1`.
    #[inline]
    #[track_caller]
    pub const fn set_column(&mut self, index: usize, value: Vector<3, T, A>) {
        self.0.set_column(index, value);
    }
}

impl<T, A: Alignment> Projective<3, T, A>
where
    T: Scalar,
{
    /// Creates a projective transform from an array of homogeneous row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<4, T, A>; 4]) -> Self {
        Self(Matrix::from_rows(rows))
    }

    /// Creates a projective transform by calling function `f` for each
    /// homogeneous row index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a homogeneous
    /// row vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj3, Vec4};
    /// #
    /// let transform = Proj3::from_row_fn(|i| Vec4::new(i, i, i, 0));
    ///
    /// assert_eq!(transform[0], Vec4::new(0, 0, 0, 0));
    /// assert_eq!(transform[1], Vec4::new(1, 1, 1, 0));
    /// assert_eq!(transform[2], Vec4::new(2, 2, 2, 0));
    /// assert_eq!(transform[3], Vec4::new(3, 3, 3, 0));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_row_fn<F>(f: F) -> Self
    where
        F: FnMut(usize) -> Vector<4, T, A>,
    {
        Self(Matrix::from_row_fn(f))
    }

    /// Creates a projective transform from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj2, Vec3};
    /// #
    /// let proj = Proj2::from_row_array(&[1, 2, 3, 4, 5, 6, 7, 8, 9]);
    /// assert_eq!(
    ///     proj,
    ///     Proj2::from_rows(&[
    ///         Vec3::new(1, 2, 3),
    ///         Vec3::new(4, 5, 6),
    ///         Vec3::new(7, 8, 9),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 16]) -> Self {
        Self(Matrix::<4, T, A>::from_row_array(array))
    }

    /// Reinterprets a homogeneous 4x4 matrix as a 3D projective transform.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn from_homogeneous(homogeneous: &Matrix<4, T, A>) -> Self {
        Self(*homogeneous)
    }

    /// Reinterprets `self` as a homogeneous 4x4 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn to_homogeneous(&self) -> Matrix<4, T, A> {
        self.0
    }

    /// Reinterprets `self` as a reference to a homogeneous 4x4 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_homogeneous(&self) -> &Matrix<4, T, A> {
        &self.0
    }

    /// Reinterprets `self` as a mutable reference to a homogeneous 4x4 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_mut_homogeneous(&mut self) -> &mut Matrix<4, T, A> {
        &mut self.0
    }

    /// Returns a reference to the transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<4, T, A>; 4] {
        self.0.as_rows()
    }

    /// Returns a mutable reference to the transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<4, T, A>; 4] {
        self.0.as_mut_rows()
    }

    /// Returns the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of rows
    /// `N + 1`.
    #[inline]
    #[must_use]
    #[track_caller]
    pub const fn column(&self, index: usize) -> Vector<4, T, A> {
        self.0.column(index)
    }

    /// Sets the column at the given index to the given value.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of rows
    /// `N + 1`.
    #[inline]
    #[track_caller]
    pub const fn set_column(&mut self, index: usize, value: Vector<4, T, A>) {
        self.0.set_column(index, value);
    }
}

impl<const N: usize, T, A: Alignment> Clone for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
}
