use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Index, IndexMut, Mul, MulAssign},
};

use crate::{
    Aligned, Alignment, Length, Matrix, One, Projective, Scalar, SupportedLength, Unaligned,
    Vector, Zero,
    length::TwoOrThree,
    utils::{specialize_23, transmute_mut, transmute_ref},
};

mod float;
#[cfg(feature = "wide")]
mod wide;
#[cfg(feature = "wide")]
mod wide_float;

/// An `N`-dimensional affine transform which can represent translation,
/// rotation, scaling and shear of type `T`.
///
/// `A` controls SIMD alignment and is either [`Unaligned`] or [`Aligned`]. See
/// [`Alignment`] for more details.
///
/// Contains a matrix and a translation vector.
///
/// # Type aliases
///
/// - [`Affine2<T>`] for [`Affine<2, T, Unaligned>`].
/// - [`Affine3<T>`] for [`Affine<3, T, Unaligned>`].
/// - [`Affine2A<T>`] for [`Affine<2, T, Aligned>`].
/// - [`Affine3A<T>`] for [`Affine<3, T, Aligned>`].
#[repr(C)]
pub struct Affine<const N: usize, T, A: Alignment>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// The part representing rotation, scaling and shear.
    pub matrix: Matrix<N, T, A>,
    /// The part representing translation.
    pub translation: Vector<N, T, A>,
}

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Contains a 2x2 matrix and a 2D translation vector.
///
/// # No SIMD alignment
///
/// [`Affine2<T>`] does not have SIMD alignment, for that use [`Affine2A<T>`].
pub type Affine2<T> = Affine<2, T, Unaligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Contains a 3x3 matrix and a 3D translation vector.
///
/// # No SIMD alignment
///
/// [`Affine3<T>`] does not have SIMD alignment, for that use [`Affine3A<T>`].
pub type Affine3<T> = Affine<3, T, Unaligned>;

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Contains a 2x2 matrix and a 2D translation vector.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Affine2A<T>`] has SIMD alignment. For no SIMD
/// use [`Affine2<T>`].
pub type Affine2A<T> = Affine<2, T, Aligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Contains a 3x3 matrix and a 3D translation vector.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Affine3A<T>`] has SIMD alignment. For no SIMD
/// use [`Affine3<T>`].
pub type Affine3A<T> = Affine<3, T, Aligned>;

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero,
{
    /// An affine transform with all elements set to `0`.
    ///
    /// This transforms all vectors to a zero vector. See [`IDENTITY`] for
    /// an affine transform with no transformation.
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    pub const ZERO: Self = Self::from_matrix_translation(Matrix::ZERO, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero + One,
{
    /// An affine transform with no transformation.
    pub const IDENTITY: Self = Self::from_matrix_translation(Matrix::IDENTITY, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates an affine transform by calling function `f` for each row index.
    ///
    /// Equivalent to `[f(0), f(1), f(2), ...]` where each item is a row vector.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine3, Vec3};
    /// #
    /// let affine = Affine3::from_row_fn(|i| Vec3::splat(i));
    ///
    /// assert_eq!(affine[0], Vec3::new(0, 0, 0));
    /// assert_eq!(affine[1], Vec3::new(1, 1, 1));
    /// assert_eq!(affine[2], Vec3::new(2, 2, 2));
    /// assert_eq!(affine.translation, Vec3::new(3, 3, 3));
    /// ```
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn from_row_fn<F>(mut f: F) -> Self
    where
        F: FnMut(usize) -> Vector<N, T, A>,
    {
        Self {
            matrix: Matrix::from_row_fn(&mut f),
            translation: f(N),
        }
    }

    /// Creates an affine transform from a non-uniform `scale`.
    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vector<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self {
            matrix: Matrix::from_diagonal(scale),
            translation: Vector::ZERO,
        }
    }

    /// Creates an affine transform from a `translation` vector.
    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self {
            matrix: Matrix::IDENTITY,
            translation,
        }
    }

    /// Creates an affine transform from `matrix` expressing rotation and
    /// scale, but not translation.
    #[inline]
    #[must_use]
    pub const fn from_matrix(matrix: Matrix<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self {
            matrix,
            translation: Vector::ZERO,
        }
    }

    /// Creates an affine transform from `translation` and `matrix`
    /// expressing rotation and scale.
    #[inline]
    #[must_use]
    pub const fn from_matrix_translation(
        matrix: Matrix<N, T, A>,
        translation: Vector<N, T, A>,
    ) -> Self {
        Self {
            matrix,
            translation,
        }
    }

    /// Creates an affine transform from a projective transform, discarding the
    /// last column.
    ///
    /// The removed column is completely ignored, without checking for identity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Proj2, Vec2, Vec3};
    /// #
    /// let projective = Proj2::from_rows(&[
    ///     Vec3::new(00, 01, 02),
    ///     Vec3::new(10, 11, 12),
    ///     Vec3::new(20, 21, 22),
    /// ]);
    ///
    /// assert_eq!(
    ///     Affine2::from_projective(projective),
    ///     Affine2::from_rows(&[
    ///         Vec2::new(00, 01),
    ///         Vec2::new(10, 11),
    ///         Vec2::new(20, 21),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub fn from_projective(projective: &Projective<N, T, A>) -> Self
    where
        Length<N>: TwoOrThree,
    {
        specialize_23!(Affine::<N, T, A>::from_projective_backend(projective))
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
    /// # use ggmath::{Aligned, Affine2, Affine2A, Unaligned};
    /// #
    /// let unaligned = Affine2::<f32>::IDENTITY;
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Affine2A::IDENTITY);
    ///
    /// let aligned = Affine2A::<f32>::IDENTITY;
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Affine2::IDENTITY);
    /// ```
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(&self) -> Affine<N, T, A2> {
        Affine::from_matrix_translation(self.matrix.to_alignment(), self.translation.to_alignment())
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Affine2A};
    /// #
    /// let unaligned = Affine2::<f32>::IDENTITY;
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Affine2A::IDENTITY);
    /// ```
    #[inline]
    #[must_use]
    pub const fn align(&self) -> Affine<N, T, Aligned> {
        self.to_alignment()
    }

    /// Conversion to [`Unaligned`] storage.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Affine2A};
    /// #
    /// let aligned = Affine2A::<f32>::IDENTITY;
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Affine2::IDENTITY);
    /// ```
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Affine<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Transforms the given vector applying scale, rotation and translation.
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_point(&self, point: Vector<N, T, A>) -> Vector<N, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        point * self.matrix + self.translation
    }

    /// Transforms the given vector applying scale and rotation, but not
    /// translation.
    ///
    /// See [`transform_point`] for also applying translation.
    ///
    /// [`transform_point`]: Self::transform_point
    #[inline]
    #[must_use]
    #[track_caller]
    pub fn transform_vector(&self, vector: Vector<N, T, A>) -> Vector<N, T, A>
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        vector * self.matrix
    }
}

impl<T, A: Alignment> Affine<2, T, A>
where
    T: Scalar,
{
    /// Creates a 2D affine transform from three row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<2, T, A>; 3]) -> Self {
        Self {
            matrix: Matrix::from_rows(&[rows[0], rows[1]]),
            translation: rows[2],
        }
    }

    /// Creates an affine transform from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let affine = Affine2::from_row_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// assert_eq!(
    ///     affine,
    ///     Affine2::from_rows(&[
    ///         Vec2::new(1.0, 2.0),
    ///         Vec2::new(3.0, 4.0),
    ///         Vec2::new(5.0, 6.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 6]) -> Self {
        Self::from_rows(&[
            Vector::<2, T, A>::new(array[0], array[1]),
            Vector::<2, T, A>::new(array[2], array[3]),
            Vector::<2, T, A>::new(array[4], array[5]),
        ])
    }

    /// Returns a reference to the affine transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_ref::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }

    /// Returns a mutable reference to the affine transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_mut::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }

    /// Creates an `N+1`x`N+1` homogeneous transformation matrix from an
    /// `N+1`x`N` affine transform.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let affine = Affine2::from_rows(&[
    ///     Vec2::new(2, 3),
    ///     Vec2::new(4, 5),
    ///     Vec2::new(6, 7),
    /// ]);
    ///
    /// assert_eq!(
    ///     affine.to_homogeneous(),
    ///     Mat3::from_rows([
    ///         Vec3::new(2, 3, 0),
    ///         Vec3::new(4, 5, 0),
    ///         Vec3::new(6, 7, 1),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> Matrix<3, T, A>
    where
        T: Zero + One,
    {
        Matrix::from_rows(&[
            self.matrix.x_axis.extend(T::ZERO),
            self.matrix.y_axis.extend(T::ZERO),
            self.translation.to_homogeneous(),
        ])
    }

    /// Takes the `N+1`x`N` affine transform part of an `N+1`x`N+1` homogeneous
    /// transformation matrix, discarding the last column.
    ///
    /// The removed column is completely ignored, without checking for identity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let homogeneous = Mat3::from_rows(&[
    ///     Vec3::new(00, 01, 02),
    ///     Vec3::new(10, 11, 12),
    ///     Vec3::new(20, 21, 22),
    /// ]);
    ///
    /// assert_eq!(
    ///     Affine2::from_homogeneous(homogeneous),
    ///     Affine2::from_rows(&[
    ///         Vec2::new(00, 01),
    ///         Vec2::new(10, 11),
    ///         Vec2::new(20, 21),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_homogeneous(homogeneous: &Matrix<3, T, A>) -> Self {
        Self::from_rows(&[
            homogeneous.x_axis.truncate(),
            homogeneous.y_axis.truncate(),
            homogeneous.z_axis.truncate(),
        ])
    }

    /// Returns a mutable reference to the affine transform's rows.
    ///
    /// This function has been renamed to [`as_mut_rows`].
    ///
    /// [`as_mut_rows`]: Self::as_mut_rows
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_mut_rows`")]
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<2, T, A>; 3] {
        self.as_mut_rows()
    }

    #[inline]
    fn from_projective_backend(projective: &Projective<2, T, A>) -> Self {
        Self::from_rows(&[
            projective[0].truncate(),
            projective[1].truncate(),
            projective[2].truncate(),
        ])
    }
}

impl<T, A: Alignment> Affine<3, T, A>
where
    T: Scalar,
{
    /// Creates a 3D affine transform from four row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<3, T, A>; 4]) -> Self {
        Self {
            matrix: Matrix::from_rows(&[rows[0], rows[1], rows[2]]),
            translation: rows[3],
        }
    }

    /// Creates an affine transform from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let affine = Affine2::from_row_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// assert_eq!(
    ///     affine,
    ///     Affine2::from_rows(&[
    ///         Vec2::new(1.0, 2.0),
    ///         Vec2::new(3.0, 4.0),
    ///         Vec2::new(5.0, 6.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 12]) -> Self {
        Self::from_rows(&[
            Vector::<3, T, A>::new(array[0], array[1], array[2]),
            Vector::<3, T, A>::new(array[3], array[4], array[5]),
            Vector::<3, T, A>::new(array[6], array[7], array[8]),
            Vector::<3, T, A>::new(array[9], array[10], array[11]),
        ])
    }

    /// Returns a reference to the affine transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_ref::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }

    /// Returns a mutable reference to the affine transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_mut::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }

    /// Creates an `N+1`x`N+1` homogeneous transformation matrix from an
    /// `N+1`x`N` affine transform.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let affine = Affine2::from_rows(&[
    ///     Vec2::new(2, 3),
    ///     Vec2::new(4, 5),
    ///     Vec2::new(6, 7),
    /// ]);
    ///
    /// assert_eq!(
    ///     affine.to_homogeneous(),
    ///     Mat3::from_rows([
    ///         Vec3::new(2, 3, 0),
    ///         Vec3::new(4, 5, 0),
    ///         Vec3::new(6, 7, 1),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_homogeneous(&self) -> Matrix<4, T, A>
    where
        T: Zero + One,
    {
        Matrix::from_rows(&[
            self.matrix.x_axis.extend(T::ZERO),
            self.matrix.y_axis.extend(T::ZERO),
            self.matrix.z_axis.extend(T::ZERO),
            self.translation.to_homogeneous(),
        ])
    }

    /// Takes the `N+1`x`N` affine transform part of an `N+1`x`N+1` homogeneous
    /// transformation matrix, discarding the last column.
    ///
    /// The removed column is completely ignored, without checking for identity.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let homogeneous = Mat3::from_rows(&[
    ///     Vec3::new(00, 01, 02),
    ///     Vec3::new(10, 11, 12),
    ///     Vec3::new(20, 21, 22),
    /// ]);
    ///
    /// assert_eq!(
    ///     Affine2::from_homogeneous(homogeneous),
    ///     Affine2::from_rows(&[
    ///         Vec2::new(00, 01),
    ///         Vec2::new(10, 11),
    ///         Vec2::new(20, 21),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_homogeneous(homogeneous: &Matrix<4, T, A>) -> Self {
        Self::from_rows(&[
            homogeneous.x_axis.truncate(),
            homogeneous.y_axis.truncate(),
            homogeneous.z_axis.truncate(),
            homogeneous.w_axis.truncate(),
        ])
    }

    /// Returns a mutable reference to the affine transform's rows.
    ///
    /// This function has been renamed to [`as_mut_rows`].
    ///
    /// [`as_mut_rows`]: Self::as_mut_rows
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_mut_rows`")]
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<3, T, A>; 4] {
        self.as_mut_rows()
    }

    #[inline]
    fn from_projective_backend(projective: &Projective<3, T, A>) -> Self {
        Self::from_rows(&[
            projective[0].truncate(),
            projective[1].truncate(),
            projective[2].truncate(),
            projective[3].truncate(),
        ])
    }
}

impl<T, A: Alignment> Affine<4, T, A>
where
    T: Scalar,
{
    /// Creates a 4D affine transform from five row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<4, T, A>; 5]) -> Self {
        Self {
            matrix: Matrix::from_rows(&[rows[0], rows[1], rows[2], rows[3]]),
            translation: rows[4],
        }
    }

    /// Creates an affine transform from a row-major array of elements.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let affine = Affine2::from_row_array(&[1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
    /// assert_eq!(
    ///     affine,
    ///     Affine2::from_rows(&[
    ///         Vec2::new(1.0, 2.0),
    ///         Vec2::new(3.0, 4.0),
    ///         Vec2::new(5.0, 6.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub const fn from_row_array(array: &[T; 20]) -> Self {
        Self::from_rows(&[
            Vector::<4, T, A>::new(array[0], array[1], array[2], array[3]),
            Vector::<4, T, A>::new(array[4], array[5], array[6], array[7]),
            Vector::<4, T, A>::new(array[8], array[9], array[10], array[11]),
            Vector::<4, T, A>::new(array[12], array[13], array[14], array[15]),
            Vector::<4, T, A>::new(array[16], array[17], array[18], array[19]),
        ])
    }

    /// Returns a reference to the affine transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<4, T, A>; 5] {
        // SAFETY: `Affine<4, T, A>` is guaranteed to begin with
        // `Matrix<4, T, A>` (four vectors) then `Vector<4, T, A>`, which is 5
        // vectors in total.
        unsafe { transmute_ref::<Affine<4, T, A>, [Vector<4, T, A>; 5]>(self) }
    }

    /// Returns a mutable reference to the affine transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<4, T, A>; 5] {
        // SAFETY: `Affine<4, T, A>` is guaranteed to begin with
        // `Matrix<4, T, A>` (four vectors) then `Vector<4, T, A>`, which is 5
        // vectors in total.
        unsafe { transmute_mut::<Affine<4, T, A>, [Vector<4, T, A>; 5]>(self) }
    }

    /// Returns a mutable reference to the affine transform's rows.
    ///
    /// This function has been renamed to [`as_mut_rows`].
    ///
    /// [`as_mut_rows`]: Self::as_mut_rows
    #[inline]
    #[must_use]
    #[deprecated(since = "0.17.1", note = "renamed to `as_mut_rows`")]
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<4, T, A>; 5] {
        self.as_mut_rows()
    }
}

impl<const N: usize, T, A: Alignment> Clone for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T, A: Alignment> Copy for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
}

impl<const N: usize, T, A: Alignment> Index<usize> for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = Vector<N, T, A>;

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the dimension of the affine transform.
    /// It is fine if `index == N` because of the additional `translation` row.
    #[inline]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        match (N, index) {
            (2, 0) => &self.matrix[0],
            (2, 1) => &self.matrix[1],
            (2, 2) => &self.translation,
            (3, 0) => &self.matrix[0],
            (3, 1) => &self.matrix[1],
            (3, 2) => &self.matrix[2],
            (3, 3) => &self.translation,
            (4, 0) => &self.matrix[0],
            (4, 1) => &self.matrix[1],
            (4, 2) => &self.matrix[2],
            (4, 3) => &self.matrix[3],
            (4, 4) => &self.translation,
            _ => panic!("index out of bounds"),
        }
    }
}

impl<const N: usize, T, A: Alignment> IndexMut<usize> for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Returns a mutable reference to the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than the dimension of the affine transform.
    /// It is fine if `index == N` because of the additional `translation` row.
    #[inline]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match (N, index) {
            (2, 0) => &mut self.matrix[0],
            (2, 1) => &mut self.matrix[1],
            (2, 2) => &mut self.translation,
            (3, 0) => &mut self.matrix[0],
            (3, 1) => &mut self.matrix[1],
            (3, 2) => &mut self.matrix[2],
            (3, 3) => &mut self.translation,
            (4, 0) => &mut self.matrix[0],
            (4, 1) => &mut self.matrix[1],
            (4, 2) => &mut self.matrix[2],
            (4, 3) => &mut self.matrix[3],
            (4, 4) => &mut self.translation,
            _ => panic!("index out of bounds"),
        }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(
                f,
                "[{:?}, {:?}, {:?}]",
                self.matrix[0], self.matrix[1], self.translation
            ),
            3 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}]",
                self.matrix[0], self.matrix[1], self.matrix[2], self.translation
            ),
            4 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}, {:?}]",
                self.matrix[0], self.matrix[1], self.matrix[2], self.matrix[3], self.translation
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> Display for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(
                f,
                "[{}, {}, {}]",
                self.matrix[0], self.matrix[1], self.translation
            ),
            3 => write!(
                f,
                "[{}, {}, {}, {}]",
                self.matrix[0], self.matrix[1], self.matrix[2], self.translation
            ),
            4 => write!(
                f,
                "[{}, {}, {}, {}, {}]",
                self.matrix[0], self.matrix[1], self.matrix[2], self.matrix[3], self.translation
            ),
            _ => unreachable!(),
        }
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix && self.translation == other.translation
    }
}

impl<const N: usize, T, A: Alignment> Eq for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.matrix.hash(state);
        self.translation.hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Affine<N, T, A>
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

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Affine<N, T, A>
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

        impl<const N: usize, T, A: Alignment> Mul<&Affine<N, T, A>> for Affine<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Affine<N, T, A>) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Affine<N, T, A>> for &Affine<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Affine<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Affine<N, T, A>) -> Self::Output {
                self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Affine<N, T, A>> for &Affine<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Affine<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Affine<N, T, A>) -> Self::Output {
                Affine::from_matrix_translation(
                    self.matrix * rhs.matrix,
                    self.translation * rhs.matrix + rhs.translation,
                )
            }
        }
    };
}
impl_mul!(
    /// Affine transform multiplication.
    ///
    /// Because vectors are treated as row matrices, affine transform
    /// multiplication first applies the left-hand side transform, then the
    /// right-hand side transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul_projective {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Projective<N, T, A>> for Affine<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<N, T, A>) -> Self::Output {
                &Projective::from_affine(&self) * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Projective<N, T, A>> for Affine<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<N, T, A>) -> Self::Output {
                &Projective::from_affine(&self) * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Projective<N, T, A>> for &Affine<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<N, T, A>) -> Self::Output {
                &Projective::from_affine(self) * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Projective<N, T, A>> for &Affine<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<N, T, A>) -> Self::Output {
                &Projective::from_affine(self) * rhs
            }
        }
    };
}
impl_mul_projective!(
    /// Affine-transform projective-transform multiplication, resulting in a
    /// projective transform.
    ///
    /// Because vectors are treated as row matrices, multiplication first
    /// applies the left-hand side transform, then the right-hand side
    /// transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_projective_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Affine<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Affine<N, T, A>) -> Self::Output {
                &self * &Projective::from_affine(&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Affine<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Affine<N, T, A>) -> Self::Output {
                &self * &Projective::from_affine(rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Affine<N, T, A>> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Affine<N, T, A>) -> Self::Output {
                self * &Projective::from_affine(&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Affine<N, T, A>> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Affine<N, T, A>) -> Self::Output {
                self * &Projective::from_affine(rhs)
            }
        }
    };
}
impl_projective_mul!(
    /// Projective-transform affine-transform multiplication, resulting in a
    /// projective transform.
    ///
    /// Because vectors are treated as row matrices, multiplication first
    /// applies the left-hand side transform, then the right-hand side
    /// transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign for Affine<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = &*self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Affine<N, T, A>> for Affine<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Affine<N, T, A>) {
                *self = &*self * rhs
            }
        }
    };
}
impl_mul_assign!(
    /// Affine transform multiplication.
    ///
    /// Because vectors are treated as row matrices, affine transform
    /// multiplication first applies the left-hand side transform, then the
    /// right-hand side transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_projective_mul_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign<Affine<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Affine<N, T, A>) {
                *self = &*self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Affine<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Affine<N, T, A>) {
                *self = &*self * rhs
            }
        }
    };
}
impl_projective_mul_assign!(
    /// Projective-transform affine-transform multiplication, then assigned to
    /// the projective transform.
    ///
    /// Because vectors are treated as row matrices, multiplication first
    /// applies the left-hand side transform, then the right-hand side
    /// transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Affine, Aligned, Mask, Matrix, Projective, Unaligned, Vector,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_zero() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(
                Affine::<N, T, A>::ZERO,
                Affine::from_matrix_translation(Matrix::ZERO, Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(
                Affine::<N, T, A>::IDENTITY,
                Affine::from_matrix_translation(Matrix::IDENTITY, Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_from_row_fn() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(
                Affine::<2, T, A>::from_row_fn(|i| rows[i]),
                Affine::<2, T, A>::from_rows(&rows)
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                Affine::<3, T, A>::from_row_fn(|i| rows[i]),
                Affine::<3, T, A>::from_rows(&rows)
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(
                Affine::<4, T, A>::from_row_fn(|i| rows[i]),
                Affine::<4, T, A>::from_rows(&rows)
            );
        });
    }

    #[test]
    fn test_from_scale() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let scale = Vector::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Affine::<N, T, A>::from_scale(scale),
                Affine::from_matrix(Matrix::from_diagonal(scale))
            );
        });
    }

    #[test]
    fn test_from_translation() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let translation = Vector::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Affine::<N, T, A>::from_translation(translation),
                Affine::from_matrix_translation(Matrix::IDENTITY, translation)
            );
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix = Matrix::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                Affine::<N, T, A>::from_matrix(matrix),
                Affine::from_matrix_translation(matrix, Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_from_projective() {
        for_types!(|T: PrimitiveNumber, A| {
            let projective =
                Projective::<2, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(
                Affine::<2, T, A>::from_projective(&projective),
                Affine::<2, T, A>::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                    projective.z_axis.truncate(),
                ])
            );

            let projective =
                Projective::<3, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                Affine::<3, T, A>::from_projective(&projective),
                Affine::<3, T, A>::from_rows(&[
                    projective.x_axis.truncate(),
                    projective.y_axis.truncate(),
                    projective.z_axis.truncate(),
                    projective.w_axis.truncate(),
                ])
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let affine =
                Affine::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                affine.to_alignment(),
                Affine::<N, T, Aligned>::from_matrix_translation(
                    affine.matrix.align(),
                    affine.translation.align()
                )
            );
            assert_eq!(
                affine.to_alignment(),
                Affine::<N, T, Unaligned>::from_matrix_translation(
                    affine.matrix.unalign(),
                    affine.translation.unalign()
                )
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let affine =
                Affine::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                affine.align(),
                Affine::<N, T, Aligned>::from_matrix_translation(
                    affine.matrix.align(),
                    affine.translation.align()
                )
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let affine =
                Affine::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                affine.unalign(),
                Affine::<N, T, Unaligned>::from_matrix_translation(
                    affine.matrix.unalign(),
                    affine.translation.unalign()
                )
            );
        });
    }

    #[test]
    fn test_transform_point() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (point, affine) in random_iter::<(Vector<N, T, A>, Affine<N, T, A>)>() {
                assert_test_eq!(
                    affine.transform_point(point),
                    point * affine.matrix + affine.translation
                );
            }
        });
    }

    #[test]
    fn test_transform_vector() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (point, affine) in random_iter::<(Vector<N, T, A>, Affine<N, T, A>)>() {
                assert_test_eq!(affine.transform_vector(point), point * affine.matrix);
            }
        });
    }

    #[test]
    fn test_from_rows() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(
                Affine::<2, T, A>::from_rows(&rows),
                Affine::<2, T, A>::from_matrix_translation(
                    Matrix::from_rows(&[rows[0], rows[1]]),
                    rows[2]
                )
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                Affine::<3, T, A>::from_rows(&rows),
                Affine::<3, T, A>::from_matrix_translation(
                    Matrix::from_rows(&[rows[0], rows[1], rows[2]]),
                    rows[3]
                )
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(
                Affine::<4, T, A>::from_rows(&rows),
                Affine::<4, T, A>::from_matrix_translation(
                    Matrix::from_rows(&[rows[0], rows[1], rows[2], rows[3]]),
                    rows[4]
                )
            );
        });
    }

    #[test]
    fn test_from_row_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_row_array(&[x, y, z, w, a, b]),
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ])
            );
            assert_eq!(
                Affine::<3, T, A>::from_row_array(&[x, y, z, w, a, b, c, d, e, f, g, h]),
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ])
            );
            assert_eq!(
                Affine::<4, T, A>::from_row_array(&[
                    x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p
                ]),
                Affine::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l),
                    Vector::<4, T, A>::new(m, n, o, p)
                ])
            );
        });
    }

    #[test]
    fn test_as_rows() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(Affine::<2, T, A>::from_rows(&rows).as_rows(), &rows);

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(Affine::<3, T, A>::from_rows(&rows).as_rows(), &rows);

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(Affine::<4, T, A>::from_rows(&rows).as_rows(), &rows);
        });
    }

    #[test]
    fn test_as_mut_rows() {
        for_types!(|T: PrimitiveNumber, A| {
            let mut rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(Affine::<2, T, A>::from_rows(&rows).as_mut_rows(), &mut rows);

            let mut rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(Affine::<3, T, A>::from_rows(&rows).as_mut_rows(), &mut rows);

            let mut rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(Affine::<4, T, A>::from_rows(&rows).as_mut_rows(), &mut rows);
        });
    }

    #[test]
    fn test_to_homogeneous() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(w, a),
                    Vector::<2, T, A>::new(c, d)
                ])
                .to_homogeneous(),
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
            );
            assert_eq!(
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(a, b, c),
                    Vector::<3, T, A>::new(e, f, g),
                    Vector::<3, T, A>::new(i, j, k)
                ])
                .to_homogeneous(),
                Matrix::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])
            );
        });
    }

    #[test]
    fn test_from_homogeneous() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Affine::<2, T, A>::from_homogeneous(&Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])),
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(w, a),
                    Vector::<2, T, A>::new(c, d)
                ])
            );
            assert_eq!(
                Affine::<3, T, A>::from_homogeneous(&Matrix::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])),
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(a, b, c),
                    Vector::<3, T, A>::new(e, f, g),
                    Vector::<3, T, A>::new(i, j, k)
                ])
            );
        });
    }

    #[test]
    fn test_index() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let affine =
                Affine::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            for i in 0..N {
                assert_eq!(affine[i], affine.matrix[i]);
            }
            assert_eq!(affine[N], affine.translation);
            assert_panic!(affine[N + 1]);
            assert_panic!(affine[N + 2]);
        });
    }

    #[test]
    #[expect(clippy::clone_on_copy)]
    fn test_index_mut() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let affine =
                Affine::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            for i in 0..N {
                assert_eq!(&mut affine.clone()[i], &mut affine.clone().matrix[i]);
            }
            assert_eq!(&mut affine.clone()[N], &mut affine.clone().translation);
            assert_panic!(&mut affine.clone()[N + 1]);
            assert_panic!(&mut affine.clone()[N + 2]);
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            let [x_axis, y_axis, translation] = rows;
            assert_eq!(
                format!("{:?}", Affine::<2, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {translation:?}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            let [x_axis, y_axis, z_axis, translation] = rows;
            assert_eq!(
                format!("{:?}", Affine::<3, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {z_axis:?}, {translation:?}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            let [x_axis, y_axis, z_axis, w_axis, translation] = rows;
            assert_eq!(
                format!("{:?}", Affine::<4, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {z_axis:?}, {w_axis:?}, {translation:?}]")
            );
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            let [x_axis, y_axis, translation] = rows;
            assert_eq!(
                format!("{}", Affine::<2, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {translation}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            let [x_axis, y_axis, z_axis, translation] = rows;
            assert_eq!(
                format!("{}", Affine::<3, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {z_axis}, {translation}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            let [x_axis, y_axis, z_axis, w_axis, translation] = rows;
            assert_eq!(
                format!("{}", Affine::<4, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {z_axis}, {w_axis}, {translation}]")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for ([affine, other], mask) in
                random_iter::<([Affine<N, T, A>; 2], [Mask<N, T, A>; 5])>()
            {
                let other = Affine::from_row_fn(|r| mask[r].select(affine[r], other[r]));

                assert_eq!(
                    affine == other,
                    affine.matrix == other.matrix && affine.translation == other.translation
                );
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for ([affine, other], mask) in
                random_iter::<([Affine<N, T, A>; 2], [Mask<N, T, A>; 5])>()
            {
                let other = Affine::from_row_fn(|r| mask[r].select(affine[r], other[r]));

                assert_eq!(
                    affine != other,
                    affine.matrix != other.matrix || affine.translation != other.translation
                );
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(Affine::<N, T, A>::default(), Affine::IDENTITY);
        });
    }

    #[test]
    fn test_mul() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (vector, [affine_1, affine_2]) in
                random_iter::<(Vector<N, T, A>, [Affine<N, T, A>; 2])>()
            {
                if !vector.is_finite()
                    || !affine_1.is_finite()
                    || !affine_2.is_finite()
                    || vector.iter().any(|x| x.abs() > 1e10)
                    || affine_1
                        .matrix
                        .as_rows()
                        .iter()
                        .chain([&affine_1.translation])
                        .flatten()
                        .any(|x| x.abs() > 1e10)
                    || affine_2
                        .matrix
                        .as_rows()
                        .iter()
                        .chain([&affine_2.translation])
                        .flatten()
                        .any(|x| x.abs() > 1e10)
                {
                    continue;
                }

                assert_test_eq!(
                    (affine_1 * affine_2).transform_point(vector),
                    affine_2.transform_point(affine_1.transform_point(vector)),
                    abs <= (affine_1 * affine_2).transform_point(vector).abs() * 1e-5 + 1e-3,
                    0.0 = -0.0,
                    INFINITY = NAN
                );
                assert_test_eq!(
                    (affine_1 * affine_2).transform_vector(vector),
                    affine_2.transform_vector(affine_1.transform_vector(vector)),
                    abs <= (affine_1 * affine_2).transform_vector(vector).abs() * 1e-5 + 1e-3,
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_mul_projective() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for (affine, projective) in random_iter::<(Affine<N, T, A>, Projective<N, T, A>)>() {
                assert_test_eq!(
                    affine * projective,
                    Projective::from_affine(&affine) * projective
                );
            }
        });
    }

    #[test]
    fn test_projective_mul() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for (projective, affine) in random_iter::<(Projective<N, T, A>, Affine<N, T, A>)>() {
                assert_test_eq!(
                    projective * affine,
                    projective * Projective::from_affine(&affine)
                );
            }
        });
    }

    #[test]
    fn test_mul_assign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Affine<N, T, A>; 2]>() {
                let mut result = left;
                result *= right;

                assert_test_eq!(result, left * right);
            }
        });
    }

    #[test]
    fn test_projective_mul_assign() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for (projective, affine) in random_iter::<(Projective<N, T, A>, Affine<N, T, A>)>() {
                let mut result = projective;
                result *= affine;

                assert_test_eq!(result, projective * affine);
            }
        });
    }
}
