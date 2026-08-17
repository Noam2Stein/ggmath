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
    /// Affine-transform matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, multiplication first
    /// applies the left-hand side transform, then the right-hand side matrix.
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
    /// Matrix affine transform multiplication.
    ///
    /// Because vectors are treated as row matrices, multiplication first
    /// applies the left-hand side matrix, then the right-hand side transform.
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
    /// Matrix affine-transform multiplication.
    ///
    /// Because vectors are treated as row matrices, affine transform
    /// multiplication first applies the left-hand side matrix, then the
    /// right-hand side transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);
