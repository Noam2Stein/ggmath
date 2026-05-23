use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Index, IndexMut, Mul, MulAssign},
};

use crate::{
    Aligned, Alignment, Length, Matrix, Scalar, SupportedLength, Unaligned, Vector,
    constants::{Nan, One, Zero},
    utils::{transmute_mut, transmute_ref},
};

mod float;
#[cfg(feature = "wide")]
mod wide;
#[cfg(feature = "wide")]
mod wide_float;

/// An `N`-dimensional affine transform which can represent translation,
/// rotation, scaling and shear of type `T`.
///
/// Equivalent to a [`Matrix`] containing an affine transformation, but results
/// in better performance for some operations.
///
/// Note that currently both [`Matrix`] and `Affine` are missing benchmarks and
/// possible optimizations. Still, the performance advantages of `Affine` over
/// [`Matrix`] have been proved by [`glam`].
///
/// `A` controls SIMD alignment and is either [`Aligned`] or [`Unaligned`]. See
/// [`Alignment`] for more details.
///
/// # Type aliases
///
/// - [`Affine2<T>`] for `Affine<2, T, Aligned>`.
/// - [`Affine3<T>`] for `Affine<3, T, Aligned>`.
/// - [`Affine2U<T>`] for `Affine<2, T, Unaligned>`.
/// - [`Affine3U<T>`] for `Affine<3, T, Unaligned>`.
///
/// [`glam`]: https://docs.rs/glam
/// [`Affine2<T>`]: crate::Affine2
/// [`Affine3<T>`]: crate::Affine3
/// [`Affine2U<T>`]: crate::Affine2U
/// [`Affine3U<T>`]: crate::Affine3U
/// [`Mat2<T>`]: crate::Mat2
/// [`Mat3<T>`]: crate::Mat3
/// [`Vec3<T>`]: crate::Vec3
/// [`Mat4<T>`]: crate::Mat4
/// [`from_rows`]: Self::from_rows
#[repr(C)]
pub struct Affine<const N: usize, T, A: Alignment>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// The part representing rotation, scaling and shear.
    pub submatrix: Matrix<N, T, A>,
    /// The part representing translation.
    pub translation: Vector<N, T, A>,
}

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Equivalent to a [`Mat3`] containing a 2D affine transformation, but results
/// in better performance for some operations.
///
/// Note that currently both [`Mat3`] and `Affine2` are missing benchmarks and
/// possible optimizations. Still, the performance advantages of `Affine2` over
/// [`Mat3`] have been proved by [`glam`].
///
/// # SIMD alignment
///
/// `Affine2<T>` has SIMD alignment for appropriate scalar types. See
/// [`Affine2U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Mat3`]: crate::Mat3
/// [`glam`]: https://docs.rs/glam
/// [`from_rows`]: Affine::from_rows
/// [`Alignment`]: crate::Alignment
pub type Affine2<T> = Affine<2, T, Aligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Equivalent to a [`Mat4`] containing a 3D affine transformation, but results
/// in better performance for some operations.
///
/// Note that currently both [`Mat4`] and `Affine3` are missing benchmarks and
/// possible optimizations. Still, the performance advantages of `Affine3` over
/// [`Mat4`] have been proved by [`glam`].
///
/// # SIMD alignment
///
/// `Affine3<T>` has SIMD alignment for appropriate scalar types. See
/// [`Affine3U<T>`] for a non-SIMD variant.
///
/// See [`Alignment`] for more details.
///
/// [`Mat4`]: crate::Mat4
/// [`glam`]: https://docs.rs/glam
/// [`from_rows`]: Affine::from_rows
/// [`Alignment`]: crate::Alignment
pub type Affine3<T> = Affine<3, T, Aligned>;

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Equivalent to a [`Mat3U`] containing a 2D affine transformation, but results
/// in better performance for some operations.
///
/// Note that currently both [`Mat3U`] and `Affine2U` are missing benchmarks and
/// possible optimizations. Still, the performance advantages of [`Affine`] over
/// [`Matrix`] have been proved by [`glam`].
///
/// # SIMD alignment
///
/// `Affine2U<T>` does not have SIMD alignment. See [`Affine2<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Mat3U`]: crate::Mat3U
/// [`glam`]: https://docs.rs/glam
/// [`from_rows`]: Affine::from_rows
/// [`Alignment`]: crate::Alignment
pub type Affine2U<T> = Affine<2, T, Unaligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Equivalent to a [`Mat4U`] containing a 3D affine transformation, but results
/// in better performance for some operations.
///
/// Note that currently both [`Mat4U`] and `Affine3U` are missing benchmarks and
/// possible optimizations. Still, the performance advantages of [`Affine`] over
/// [`Matrix`] have been proved by [`glam`].
///
/// # SIMD alignment
///
/// `Affine3U<T>` does not have SIMD alignment. See [`Affine3<T>`] for a SIMD
/// variant.
///
/// See [`Alignment`] for more details.
///
/// [`Mat4U`]: crate::Mat4U
/// [`glam`]: https://docs.rs/glam
/// [`from_rows`]: Affine::from_rows
/// [`Alignment`]: crate::Alignment
pub type Affine3U<T> = Affine<3, T, Unaligned>;

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
    pub const ZERO: Self = Self::from_submatrix_translation(Matrix::ZERO, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero + One,
{
    /// An affine transform with no transformation.
    pub const IDENTITY: Self = Self::from_submatrix_translation(Matrix::IDENTITY, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// An affine transform with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_submatrix_translation(Matrix::NAN, Vector::NAN);
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
            submatrix: Matrix::from_row_fn(&mut f),
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
            submatrix: Matrix::from_diagonal(scale),
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
            submatrix: Matrix::IDENTITY,
            translation,
        }
    }

    /// Creates an affine transform from `submatrix` expressing rotation and
    /// scale, but not translation.
    #[inline]
    #[must_use]
    pub const fn from_submatrix(submatrix: Matrix<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self {
            submatrix,
            translation: Vector::ZERO,
        }
    }

    /// Creates an affine transform from `translation` and `submatrix`
    /// expressing rotation and scale.
    #[inline]
    #[must_use]
    pub const fn from_submatrix_translation(
        submatrix: Matrix<N, T, A>,
        translation: Vector<N, T, A>,
    ) -> Self {
        Self {
            submatrix,
            translation,
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
    /// # use ggmath::{Aligned, Affine2, Affine2U, Unaligned};
    /// #
    /// let aligned = Affine2::<f32>::IDENTITY;
    /// let unaligned = aligned.to_alignment::<Unaligned>();
    /// assert_eq!(unaligned, Affine2U::IDENTITY);
    ///
    /// let unaligned = Affine2U::<f32>::IDENTITY;
    /// let aligned = unaligned.to_alignment::<Aligned>();
    /// assert_eq!(aligned, Affine2::IDENTITY);
    /// ```
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(&self) -> Affine<N, T, A2> {
        Affine::from_submatrix_translation(
            self.submatrix.to_alignment(),
            self.translation.to_alignment(),
        )
    }

    /// Conversion to [`Aligned`] storage.
    ///
    /// See [`Alignment`] for more details.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Affine2U};
    /// #
    /// let unaligned = Affine2U::<f32>::IDENTITY;
    /// let aligned = unaligned.align();
    /// assert_eq!(aligned, Affine2::IDENTITY);
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
    /// # use ggmath::{Affine2, Affine2U};
    /// #
    /// let aligned = Affine2::<f32>::IDENTITY;
    /// let unaligned = aligned.unalign();
    /// assert_eq!(unaligned, Affine2U::IDENTITY);
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
        point * self.submatrix + self.translation
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
        vector * self.submatrix
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
            submatrix: Matrix::from_rows(&[rows[0], rows[1]]),
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

    /// Creates an affine transform from an affine transformation matrix,
    /// discarding the last column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let matrix = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 2.0, 0.0),
    ///     Vec3::new(3.0, 4.0, 0.0),
    ///     Vec3::new(5.0, 6.0, 1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     Affine2::from_matrix(matrix),
    ///     Affine2::from_rows(&[
    ///         Vec2::new(1.0, 2.0),
    ///         Vec2::new(3.0, 4.0),
    ///         Vec2::new(5.0, 6.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_matrix(matrix: Matrix<3, T, A>) -> Self {
        Self::from_rows(&[matrix[0].xy(), matrix[1].xy(), matrix[2].xy()])
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
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_mut::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }

    /// Converts `self` to an affine transformation matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let affine = Affine2::from_rows(&[
    ///     Vec2::new(1.0, 2.0),
    ///     Vec2::new(3.0, 4.0),
    ///     Vec2::new(5.0, 6.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     affine.to_matrix(),
    ///     Mat3::from_rows(&[
    ///         Vec3::new(1.0, 2.0, 0.0),
    ///         Vec3::new(3.0, 4.0, 0.0),
    ///         Vec3::new(5.0, 6.0, 1.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<3, T, A>
    where
        T: Zero + One,
    {
        Matrix::from_rows(&[
            Vector::<3, T, A>::new(self.submatrix.x_axis.x, self.submatrix.x_axis.y, T::ZERO),
            Vector::<3, T, A>::new(self.submatrix.y_axis.x, self.submatrix.y_axis.y, T::ZERO),
            Vector::<3, T, A>::new(self.translation.x, self.translation.y, T::ONE),
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
            submatrix: Matrix::from_rows(&[rows[0], rows[1], rows[2]]),
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

    /// Creates an affine transform from an affine transformation matrix,
    /// discarding the last column.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let matrix = Mat3::from_rows(&[
    ///     Vec3::new(1.0, 2.0, 0.0),
    ///     Vec3::new(3.0, 4.0, 0.0),
    ///     Vec3::new(5.0, 6.0, 1.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     Affine2::from_matrix(matrix),
    ///     Affine2::from_rows(&[
    ///         Vec2::new(1.0, 2.0),
    ///         Vec2::new(3.0, 4.0),
    ///         Vec2::new(5.0, 6.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_matrix(matrix: Matrix<4, T, A>) -> Self {
        Self::from_rows(&[
            matrix[0].xyz(),
            matrix[1].xyz(),
            matrix[2].xyz(),
            matrix[3].xyz(),
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
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_mut::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }

    /// Converts `self` to an affine transformation matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Mat3, Vec2, Vec3};
    /// #
    /// let affine = Affine2::from_rows(&[
    ///     Vec2::new(1.0, 2.0),
    ///     Vec2::new(3.0, 4.0),
    ///     Vec2::new(5.0, 6.0),
    /// ]);
    ///
    /// assert_eq!(
    ///     affine.to_matrix(),
    ///     Mat3::from_rows(&[
    ///         Vec3::new(1.0, 2.0, 0.0),
    ///         Vec3::new(3.0, 4.0, 0.0),
    ///         Vec3::new(5.0, 6.0, 1.0),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn to_matrix(&self) -> Matrix<4, T, A>
    where
        T: Zero + One,
    {
        Matrix::from_rows(&[
            Vector::<4, T, A>::new(
                self.submatrix.x_axis.x,
                self.submatrix.x_axis.y,
                self.submatrix.x_axis.z,
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                self.submatrix.y_axis.x,
                self.submatrix.y_axis.y,
                self.submatrix.y_axis.z,
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                self.submatrix.z_axis.x,
                self.submatrix.z_axis.y,
                self.submatrix.z_axis.z,
                T::ZERO,
            ),
            Vector::<4, T, A>::new(
                self.translation.x,
                self.translation.y,
                self.translation.z,
                T::ONE,
            ),
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
            submatrix: Matrix::from_rows(&[rows[0], rows[1], rows[2], rows[3]]),
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
    pub const fn as_rows_mut(&mut self) -> &mut [Vector<4, T, A>; 5] {
        // SAFETY: `Affine<4, T, A>` is guaranteed to begin with
        // `Matrix<4, T, A>` (four vectors) then `Vector<4, T, A>`, which is 5
        // vectors in total.
        unsafe { transmute_mut::<Affine<4, T, A>, [Vector<4, T, A>; 5]>(self) }
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
            (2, 0) => &self.submatrix[0],
            (2, 1) => &self.submatrix[1],
            (2, 2) => &self.translation,
            (3, 0) => &self.submatrix[0],
            (3, 1) => &self.submatrix[1],
            (3, 2) => &self.submatrix[2],
            (3, 3) => &self.translation,
            (4, 0) => &self.submatrix[0],
            (4, 1) => &self.submatrix[1],
            (4, 2) => &self.submatrix[2],
            (4, 3) => &self.submatrix[3],
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
            (2, 0) => &mut self.submatrix[0],
            (2, 1) => &mut self.submatrix[1],
            (2, 2) => &mut self.translation,
            (3, 0) => &mut self.submatrix[0],
            (3, 1) => &mut self.submatrix[1],
            (3, 2) => &mut self.submatrix[2],
            (3, 3) => &mut self.translation,
            (4, 0) => &mut self.submatrix[0],
            (4, 1) => &mut self.submatrix[1],
            (4, 2) => &mut self.submatrix[2],
            (4, 3) => &mut self.submatrix[3],
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
                self.submatrix[0], self.submatrix[1], self.translation
            ),
            3 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}]",
                self.submatrix[0], self.submatrix[1], self.submatrix[2], self.translation
            ),
            4 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}, {:?}]",
                self.submatrix[0],
                self.submatrix[1],
                self.submatrix[2],
                self.submatrix[3],
                self.translation
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
                self.submatrix[0], self.submatrix[1], self.translation
            ),
            3 => write!(
                f,
                "[{}, {}, {}, {}]",
                self.submatrix[0], self.submatrix[1], self.submatrix[2], self.translation
            ),
            4 => write!(
                f,
                "[{}, {}, {}, {}, {}]",
                self.submatrix[0],
                self.submatrix[1],
                self.submatrix[2],
                self.submatrix[3],
                self.translation
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
        self.submatrix == other.submatrix && self.translation == other.translation
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
        self.submatrix.hash(state);
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
                Affine::from_submatrix_translation(
                    self.submatrix * rhs.submatrix,
                    self.translation * rhs.submatrix + rhs.translation,
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

macro_rules! impl_mul_matrix {
    ($N:literal, $N2:literal, $(#[$doc:meta])*) => {
        impl<T, A: Alignment> Mul<Matrix<$N2, T, A>> for Affine<$N, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Matrix<$N2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<$N2, T, A>) -> Self::Output {
                &self.to_matrix() * &rhs
            }
        }

        impl<T, A: Alignment> Mul<&Matrix<$N2, T, A>> for Affine<$N, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Matrix<$N2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<$N2, T, A>) -> Self::Output {
                &self.to_matrix() * rhs
            }
        }

        impl<T, A: Alignment> Mul<Matrix<$N2, T, A>> for &Affine<$N, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Matrix<$N2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<$N2, T, A>) -> Self::Output {
                &self.to_matrix() * &rhs
            }
        }

        impl<T, A: Alignment> Mul<&Matrix<$N2, T, A>> for &Affine<$N, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Matrix<$N2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<$N2, T, A>) -> Self::Output {
                &self.to_matrix() * rhs
            }
        }
    };
}
impl_mul_matrix!(
    2,
    3,
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
impl_mul_matrix!(
    3,
    4,
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

macro_rules! impl_matrix_mul {
    ($N:literal, $N2:literal, $(#[$doc:meta])*) => {
        impl<T, A: Alignment> Mul<Affine<$N, T, A>> for Matrix<$N2, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Affine<$N, T, A>) -> Self::Output {
                &self * &rhs.to_matrix()
            }
        }

        impl<T, A: Alignment> Mul<&Affine<$N, T, A>> for Matrix<$N2, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Affine<$N, T, A>) -> Self::Output {
                &self * &rhs.to_matrix()
            }
        }

        impl<T, A: Alignment> Mul<Affine<$N, T, A>> for &Matrix<$N2, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Matrix<$N2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Affine<$N, T, A>) -> Self::Output {
                self * &rhs.to_matrix()
            }
        }

        impl<T, A: Alignment> Mul<&Affine<$N, T, A>> for &Matrix<$N2, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            type Output = Matrix<$N2, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Affine<$N, T, A>) -> Self::Output {
                self * &rhs.to_matrix()
            }
        }
    };
}
impl_matrix_mul!(
    2,
    3,
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
impl_matrix_mul!(
    3,
    4,
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

macro_rules! impl_matrix_mul_assign {
    ($N:literal, $N2:literal, $(#[$doc:meta])*) => {
        impl<T, A: Alignment> MulAssign<Affine<$N, T, A>> for Matrix<$N2, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Affine<$N, T, A>) {
                *self = &*self * &rhs
            }
        }

        impl<T, A: Alignment> MulAssign<&Affine<$N, T, A>> for Matrix<$N2, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T> + Zero + One,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Affine<$N, T, A>) {
                *self = &*self * rhs
            }
        }
    };
}
impl_matrix_mul_assign!(
    2,
    3,
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
impl_matrix_mul_assign!(
    3,
    4,
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Affine, Aligned, Matrix, Unaligned, Vector,
        utils::{assert_float_eq, assert_panic, for_parameters},
    };

    #[test]
    fn test_zero() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Affine::<2, T, A>::ZERO,
                Affine::from_submatrix_translation(Matrix::ZERO, Vector::ZERO)
            );
            assert_eq!(
                Affine::<3, T, A>::ZERO,
                Affine::from_submatrix_translation(Matrix::ZERO, Vector::ZERO)
            );
            assert_eq!(
                Affine::<4, T, A>::ZERO,
                Affine::from_submatrix_translation(Matrix::ZERO, Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_identity() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Affine::<2, T, A>::IDENTITY,
                Affine::from_submatrix_translation(Matrix::IDENTITY, Vector::ZERO)
            );
            assert_eq!(
                Affine::<3, T, A>::IDENTITY,
                Affine::from_submatrix_translation(Matrix::IDENTITY, Vector::ZERO)
            );
            assert_eq!(
                Affine::<4, T, A>::IDENTITY,
                Affine::from_submatrix_translation(Matrix::IDENTITY, Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_nan() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_float_eq!(
                Affine::<2, T, A>::NAN,
                Affine::from_submatrix_translation(Matrix::NAN, Vector::NAN)
            );
            assert_float_eq!(
                Affine::<3, T, A>::NAN,
                Affine::from_submatrix_translation(Matrix::NAN, Vector::NAN)
            );
            assert_float_eq!(
                Affine::<4, T, A>::NAN,
                Affine::from_submatrix_translation(Matrix::NAN, Vector::NAN)
            );
        });
    }

    #[test]
    fn test_from_row_fn() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_row_fn(|i| [
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ][i]),
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ])
            );
            assert_eq!(
                Affine::<3, T, A>::from_row_fn(|i| [
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ][i]),
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ])
            );
            assert_eq!(
                Affine::<4, T, A>::from_row_fn(|index| [
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l),
                    Vector::<4, T, A>::new(m, n, o, p)
                ][index]),
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
    fn test_from_scale() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [_, _, x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_scale(Vector::<2, T, A>::new(x, y)),
                Affine::from_submatrix(Matrix::from_diagonal(Vector::<2, T, A>::new(x, y)))
            );
            assert_eq!(
                Affine::<3, T, A>::from_scale(Vector::<3, T, A>::new(x, y, z)),
                Affine::from_submatrix(Matrix::from_diagonal(Vector::<3, T, A>::new(x, y, z)))
            );
            assert_eq!(
                Affine::<4, T, A>::from_scale(Vector::<4, T, A>::new(x, y, z, w)),
                Affine::from_submatrix(Matrix::from_diagonal(Vector::<4, T, A>::new(x, y, z, w)))
            );
        });
    }

    #[test]
    fn test_from_translation() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [_, _, x, y, z, w] = std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_translation(Vector::<2, T, A>::new(x, y)),
                Affine::from_submatrix_translation(Matrix::IDENTITY, Vector::<2, T, A>::new(x, y))
            );
            assert_eq!(
                Affine::<3, T, A>::from_translation(Vector::<3, T, A>::new(x, y, z)),
                Affine::from_submatrix_translation(
                    Matrix::IDENTITY,
                    Vector::<3, T, A>::new(x, y, z)
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_translation(Vector::<4, T, A>::new(x, y, z, w)),
                Affine::from_submatrix_translation(
                    Matrix::IDENTITY,
                    Vector::<4, T, A>::new(x, y, z, w)
                )
            );
        });
    }

    #[test]
    fn test_from_submatrix() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [_, x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_submatrix(Matrix::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])),
                Affine::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::ZERO
                )
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix(Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])),
                Affine::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::ZERO
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix(Matrix::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l)
                ])),
                Affine::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::ZERO
                )
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::<2, T, A>::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
                .to_alignment(),
                Affine::<2, T, Aligned>::from_submatrix_translation(
                    Matrix::<2, T, Aligned>::from_rows(&[
                        Vector::<2, T, Aligned>::new(x, y),
                        Vector::<2, T, Aligned>::new(z, w)
                    ]),
                    Vector::<2, T, Aligned>::new(a, b)
                )
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::<3, T, A>::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
                .to_alignment(),
                Affine::<3, T, Aligned>::from_submatrix_translation(
                    Matrix::<3, T, Aligned>::from_rows(&[
                        Vector::<3, T, Aligned>::new(x, y, z),
                        Vector::<3, T, Aligned>::new(w, a, b),
                        Vector::<3, T, Aligned>::new(c, d, e)
                    ]),
                    Vector::<3, T, Aligned>::new(f, g, h)
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::<4, T, A>::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
                .to_alignment(),
                Affine::<4, T, Aligned>::from_submatrix_translation(
                    Matrix::<4, T, Aligned>::from_rows(&[
                        Vector::<4, T, Aligned>::new(x, y, z, w),
                        Vector::<4, T, Aligned>::new(a, b, c, d),
                        Vector::<4, T, Aligned>::new(e, f, g, h),
                        Vector::<4, T, Aligned>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, Aligned>::new(m, n, o, p)
                )
            );

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::<2, T, A>::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
                .to_alignment(),
                Affine::<2, T, Unaligned>::from_submatrix_translation(
                    Matrix::<2, T, Unaligned>::from_rows(&[
                        Vector::<2, T, Unaligned>::new(x, y),
                        Vector::<2, T, Unaligned>::new(z, w)
                    ]),
                    Vector::<2, T, Unaligned>::new(a, b)
                )
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::<3, T, A>::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
                .to_alignment(),
                Affine::<3, T, Unaligned>::from_submatrix_translation(
                    Matrix::<3, T, Unaligned>::from_rows(&[
                        Vector::<3, T, Unaligned>::new(x, y, z),
                        Vector::<3, T, Unaligned>::new(w, a, b),
                        Vector::<3, T, Unaligned>::new(c, d, e)
                    ]),
                    Vector::<3, T, Unaligned>::new(f, g, h)
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::<4, T, A>::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
                .to_alignment(),
                Affine::<4, T, Unaligned>::from_submatrix_translation(
                    Matrix::<4, T, Unaligned>::from_rows(&[
                        Vector::<4, T, Unaligned>::new(x, y, z, w),
                        Vector::<4, T, Unaligned>::new(a, b, c, d),
                        Vector::<4, T, Unaligned>::new(e, f, g, h),
                        Vector::<4, T, Unaligned>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, Unaligned>::new(m, n, o, p)
                )
            );
        });
    }

    #[test]
    fn test_align() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::<2, T, A>::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
                .align(),
                Affine::<2, T, Aligned>::from_submatrix_translation(
                    Matrix::<2, T, Aligned>::from_rows(&[
                        Vector::<2, T, Aligned>::new(x, y),
                        Vector::<2, T, Aligned>::new(z, w)
                    ]),
                    Vector::<2, T, Aligned>::new(a, b)
                )
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::<3, T, A>::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
                .align(),
                Affine::<3, T, Aligned>::from_submatrix_translation(
                    Matrix::<3, T, Aligned>::from_rows(&[
                        Vector::<3, T, Aligned>::new(x, y, z),
                        Vector::<3, T, Aligned>::new(w, a, b),
                        Vector::<3, T, Aligned>::new(c, d, e)
                    ]),
                    Vector::<3, T, Aligned>::new(f, g, h)
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::<4, T, A>::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
                .align(),
                Affine::<4, T, Aligned>::from_submatrix_translation(
                    Matrix::<4, T, Aligned>::from_rows(&[
                        Vector::<4, T, Aligned>::new(x, y, z, w),
                        Vector::<4, T, Aligned>::new(a, b, c, d),
                        Vector::<4, T, Aligned>::new(e, f, g, h),
                        Vector::<4, T, Aligned>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, Aligned>::new(m, n, o, p)
                )
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::<2, T, A>::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
                .unalign(),
                Affine::<2, T, Unaligned>::from_submatrix_translation(
                    Matrix::<2, T, Unaligned>::from_rows(&[
                        Vector::<2, T, Unaligned>::new(x, y),
                        Vector::<2, T, Unaligned>::new(z, w)
                    ]),
                    Vector::<2, T, Unaligned>::new(a, b)
                )
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::<3, T, A>::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
                .unalign(),
                Affine::<3, T, Unaligned>::from_submatrix_translation(
                    Matrix::<3, T, Unaligned>::from_rows(&[
                        Vector::<3, T, Unaligned>::new(x, y, z),
                        Vector::<3, T, Unaligned>::new(w, a, b),
                        Vector::<3, T, Unaligned>::new(c, d, e)
                    ]),
                    Vector::<3, T, Unaligned>::new(f, g, h)
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::<4, T, A>::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
                .unalign(),
                Affine::<4, T, Unaligned>::from_submatrix_translation(
                    Matrix::<4, T, Unaligned>::from_rows(&[
                        Vector::<4, T, Unaligned>::new(x, y, z, w),
                        Vector::<4, T, Unaligned>::new(a, b, c, d),
                        Vector::<4, T, Unaligned>::new(e, f, g, h),
                        Vector::<4, T, Unaligned>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, Unaligned>::new(m, n, o, p)
                )
            );
        });
    }

    #[test]
    fn test_transform_point() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h] = std::array::from_fn(T::as_from);

            let point = Vector::<2, T, A>::new(x, y);
            let matrix = Matrix::from_rows(&[
                Vector::<3, T, A>::new(z, w, T::as_from(0)),
                Vector::<3, T, A>::new(a, b, T::as_from(0)),
                Vector::<3, T, A>::new(c, d, T::as_from(1)),
            ]);
            assert_eq!(
                Affine::<2, T, A>::from_matrix(matrix).transform_point(point),
                matrix.transform_point(point)
            );

            let point = Vector::<3, T, A>::new(x, y, z);
            let matrix = Matrix::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, T::as_from(0)),
                Vector::<4, T, A>::new(w, a, b, T::as_from(0)),
                Vector::<4, T, A>::new(c, d, e, T::as_from(0)),
                Vector::<4, T, A>::new(f, g, h, T::as_from(1)),
            ]);
            assert_eq!(
                Affine::<3, T, A>::from_matrix(matrix).transform_point(point),
                matrix.transform_point(point)
            );
        });
    }

    #[test]
    fn test_transform_vector() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h] = std::array::from_fn(T::as_from);

            let point = Vector::<2, T, A>::new(x, y);
            let matrix = Matrix::from_rows(&[
                Vector::<3, T, A>::new(z, w, T::as_from(0)),
                Vector::<3, T, A>::new(a, b, T::as_from(0)),
                Vector::<3, T, A>::new(c, d, T::as_from(1)),
            ]);
            assert_eq!(
                Affine::<2, T, A>::from_matrix(matrix).transform_vector(point),
                matrix.transform_vector(point)
            );

            let point = Vector::<3, T, A>::new(x, y, z);
            let matrix = Matrix::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, T::as_from(0)),
                Vector::<4, T, A>::new(w, a, b, T::as_from(0)),
                Vector::<4, T, A>::new(c, d, e, T::as_from(0)),
                Vector::<4, T, A>::new(f, g, h, T::as_from(1)),
            ]);
            assert_eq!(
                Affine::<3, T, A>::from_matrix(matrix).transform_vector(point),
                matrix.transform_vector(point)
            );
        });
    }

    #[test]
    fn test_from_rows() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ]),
                Affine::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
            );
            assert_eq!(
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ]),
                Affine::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
            );
            assert_eq!(
                Affine::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l),
                    Vector::<4, T, A>::new(m, n, o, p)
                ]),
                Affine::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
            );
        });
    }

    #[test]
    fn test_from_row_array() {
        for_parameters!(|T: PrimitiveNumber, A| {
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
    fn test_from_matrix() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] = std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_matrix(Matrix::from_rows(&[
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
                Affine::<3, T, A>::from_matrix(Matrix::from_rows(&[
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
    fn test_as_rows() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::<2, T, A>::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
                .as_rows(),
                &[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ]
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::<3, T, A>::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
                .as_rows(),
                &[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ]
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::<4, T, A>::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
                .as_rows(),
                &[
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l),
                    Vector::<4, T, A>::new(m, n, o, p)
                ]
            );
        });
    }

    #[test]
    fn test_as_rows_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::<2, T, A>::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w)
                    ]),
                    Vector::<2, T, A>::new(a, b)
                )
                .as_rows_mut(),
                &mut [
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ]
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::<3, T, A>::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(w, a, b),
                        Vector::<3, T, A>::new(c, d, e)
                    ]),
                    Vector::<3, T, A>::new(f, g, h)
                )
                .as_rows_mut(),
                &mut [
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ]
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::<4, T, A>::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(a, b, c, d),
                        Vector::<4, T, A>::new(e, f, g, h),
                        Vector::<4, T, A>::new(i, j, k, l)
                    ]),
                    Vector::<4, T, A>::new(m, n, o, p)
                )
                .as_rows_mut(),
                &mut [
                    Vector::<4, T, A>::new(x, y, z, w),
                    Vector::<4, T, A>::new(a, b, c, d),
                    Vector::<4, T, A>::new(e, f, g, h),
                    Vector::<4, T, A>::new(i, j, k, l),
                    Vector::<4, T, A>::new(m, n, o, p)
                ]
            );
        });
    }

    #[test]
    fn test_to_matrix() {
        for_parameters!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(1.0, 2.0),
                    Vector::<2, T, A>::new(3.0, 4.0),
                    Vector::<2, T, A>::new(5.0, 6.0)
                ])
                .to_matrix(),
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(1.0, 2.0, 0.0),
                    Vector::<3, T, A>::new(3.0, 4.0, 0.0),
                    Vector::<3, T, A>::new(5.0, 6.0, 1.0)
                ])
            );
            assert_eq!(
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(1.0, 2.0, 3.0),
                    Vector::<3, T, A>::new(4.0, 5.0, 6.0),
                    Vector::<3, T, A>::new(7.0, 8.0, 9.0),
                    Vector::<3, T, A>::new(10.0, 11.0, 12.0)
                ])
                .to_matrix(),
                Matrix::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(1.0, 2.0, 3.0, 0.0),
                    Vector::<4, T, A>::new(4.0, 5.0, 6.0, 0.0),
                    Vector::<4, T, A>::new(7.0, 8.0, 9.0, 0.0),
                    Vector::<4, T, A>::new(10.0, 11.0, 12.0, 1.0)
                ])
            );
        });
    }

    #[test]
    fn test_index() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            let affine = Affine::<2, T, A>::from_rows(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(a, b),
            ]);
            assert_eq!(affine[0], Vector::<2, T, A>::new(x, y));
            assert_eq!(affine[1], Vector::<2, T, A>::new(z, w));
            assert_eq!(affine[2], Vector::<2, T, A>::new(a, b));
            assert_panic!(affine[3]);

            let affine = Affine::<3, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
                Vector::<3, T, A>::new(f, g, h),
            ]);
            assert_eq!(affine[0], Vector::<3, T, A>::new(x, y, z));
            assert_eq!(affine[1], Vector::<3, T, A>::new(w, a, b));
            assert_eq!(affine[2], Vector::<3, T, A>::new(c, d, e));
            assert_eq!(affine[3], Vector::<3, T, A>::new(f, g, h));
            assert_panic!(affine[4]);

            let affine = Affine::<4, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
                Vector::<4, T, A>::new(m, n, o, p),
            ]);
            assert_eq!(affine[0], Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(affine[1], Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(affine[2], Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(affine[3], Vector::<4, T, A>::new(i, j, k, l));
            assert_eq!(affine[4], Vector::<4, T, A>::new(m, n, o, p));
            assert_panic!(affine[5]);
        });
    }

    #[test]
    fn test_index_mut() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            let mut affine = Affine::<2, T, A>::from_rows(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
                Vector::<2, T, A>::new(a, b),
            ]);
            assert_eq!(&mut affine[0], &mut Vector::<2, T, A>::new(x, y));
            assert_eq!(&mut affine[1], &mut Vector::<2, T, A>::new(z, w));
            assert_eq!(&mut affine[2], &mut Vector::<2, T, A>::new(a, b));
            assert_panic!({
                #[expect(clippy::clone_on_copy)]
                &mut affine.clone()[3]
            });

            let mut affine = Affine::<3, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
                Vector::<3, T, A>::new(f, g, h),
            ]);
            assert_eq!(&mut affine[0], &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(&mut affine[1], &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(&mut affine[2], &mut Vector::<3, T, A>::new(c, d, e));
            assert_eq!(&mut affine[3], &mut Vector::<3, T, A>::new(f, g, h));
            assert_panic!({
                #[expect(clippy::clone_on_copy)]
                &mut affine.clone()[4]
            });

            let mut affine = Affine::<4, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
                Vector::<4, T, A>::new(m, n, o, p),
            ]);
            assert_eq!(&mut affine[0], &mut Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(&mut affine[1], &mut Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(&mut affine[2], &mut Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(&mut affine[3], &mut Vector::<4, T, A>::new(i, j, k, l));
            assert_eq!(&mut affine[4], &mut Vector::<4, T, A>::new(m, n, o, p));
            assert_panic!({
                #[expect(clippy::clone_on_copy)]
                &mut affine.clone()[5]
            });
        });
    }

    #[test]
    fn test_debug() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                format!(
                    "{:?}",
                    Affine::<2, T, A>::from_submatrix_translation(
                        Matrix::from_rows(&[
                            Vector::<2, T, A>::new(x, y),
                            Vector::<2, T, A>::new(z, w)
                        ]),
                        Vector::<2, T, A>::new(a, b)
                    )
                ),
                format!("[({x:?}, {y:?}), ({z:?}, {w:?}), ({a:?}, {b:?})]")
            );
            assert_eq!(
                format!(
                    "{:?}",
                    Affine::<3, T, A>::from_submatrix_translation(
                        Matrix::from_rows(&[
                            Vector::<3, T, A>::new(x, y, z),
                            Vector::<3, T, A>::new(w, a, b),
                            Vector::<3, T, A>::new(c, d, e)
                        ]),
                        Vector::<3, T, A>::new(f, g, h)
                    )
                ),
                format!(
                    "[({x:?}, {y:?}, {z:?}), ({w:?}, {a:?}, {b:?}), ({c:?}, {d:?}, {e:?}), ({f:?}, {g:?}, {h:?})]"
                )
            );
            assert_eq!(
                format!(
                    "{:?}",
                    Affine::<4, T, A>::from_submatrix_translation(
                        Matrix::from_rows(&[
                            Vector::<4, T, A>::new(x, y, z, w),
                            Vector::<4, T, A>::new(a, b, c, d),
                            Vector::<4, T, A>::new(e, f, g, h),
                            Vector::<4, T, A>::new(i, j, k, l)
                        ]),
                        Vector::<4, T, A>::new(m, n, o, p)
                    )
                ),
                format!(
                    "[({x:?}, {y:?}, {z:?}, {w:?}), ({a:?}, {b:?}, {c:?}, {d:?}), ({e:?}, {f:?}, {g:?}, {h:?}), ({i:?}, {j:?}, {k:?}, {l:?}), ({m:?}, {n:?}, {o:?}, {p:?})]"
                )
            );
        });
    }

    #[test]
    fn test_display() {
        for_parameters!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p] =
                std::array::from_fn(T::as_from);

            assert_eq!(
                format!(
                    "{}",
                    Affine::<2, T, A>::from_submatrix_translation(
                        Matrix::from_rows(&[
                            Vector::<2, T, A>::new(x, y),
                            Vector::<2, T, A>::new(z, w)
                        ]),
                        Vector::<2, T, A>::new(a, b)
                    )
                ),
                format!("[({x}, {y}), ({z}, {w}), ({a}, {b})]")
            );
            assert_eq!(
                format!(
                    "{}",
                    Affine::<3, T, A>::from_submatrix_translation(
                        Matrix::from_rows(&[
                            Vector::<3, T, A>::new(x, y, z),
                            Vector::<3, T, A>::new(w, a, b),
                            Vector::<3, T, A>::new(c, d, e)
                        ]),
                        Vector::<3, T, A>::new(f, g, h)
                    )
                ),
                format!("[({x}, {y}, {z}), ({w}, {a}, {b}), ({c}, {d}, {e}), ({f}, {g}, {h})]")
            );
            assert_eq!(
                format!(
                    "{}",
                    Affine::<4, T, A>::from_submatrix_translation(
                        Matrix::from_rows(&[
                            Vector::<4, T, A>::new(x, y, z, w),
                            Vector::<4, T, A>::new(a, b, c, d),
                            Vector::<4, T, A>::new(e, f, g, h),
                            Vector::<4, T, A>::new(i, j, k, l)
                        ]),
                        Vector::<4, T, A>::new(m, n, o, p)
                    )
                ),
                format!(
                    "[({x}, {y}, {z}, {w}), ({a}, {b}, {c}, {d}), ({e}, {f}, {g}, {h}), ({i}, {j}, {k}, {l}), ({m}, {n}, {o}, {p})]"
                )
            );
        });
    }

    #[test]
    fn test_eq() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w),
                    ]),
                    Vector::<2, T, A>::new(x, z),
                ) == Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(z, y),
                        Vector::<2, T, A>::new(z, w),
                    ]),
                    Vector::<2, T, A>::new(x, z),
                ),
                x == z && y == y && z == z && w == w && x == x
            );
            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w),
                    ]),
                    Vector::<2, T, A>::new(x, z),
                ) == Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(z, w),
                        Vector::<2, T, A>::new(x, y),
                    ]),
                    Vector::<2, T, A>::new(z, x),
                ),
                x == z && y == w
            );

            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(x, y, w),
                        Vector::<3, T, A>::new(x, y, z),
                    ]),
                    Vector::<3, T, A>::new(x, y, z),
                ) == Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, w),
                        Vector::<3, T, A>::new(x, y, w),
                        Vector::<3, T, A>::new(x, y, z),
                    ]),
                    Vector::<3, T, A>::new(x, y, z),
                ),
                x == x && y == y && z == w && w == w
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(z, w, y),
                        Vector::<3, T, A>::new(x, y, z),
                    ]),
                    Vector::<3, T, A>::new(z, x, y),
                ) == Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(z, w, y),
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(z, w, y),
                    ]),
                    Vector::<3, T, A>::new(y, y, w),
                ),
                x == z && y == w && z == y
            );

            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, y, x, w),
                        Vector::<4, T, A>::new(x, y, z, y),
                    ]),
                    Vector::<4, T, A>::new(x, y, z, w),
                ) == Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(w, y, z, w),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, y, x, w),
                        Vector::<4, T, A>::new(x, y, z, y),
                    ]),
                    Vector::<4, T, A>::new(x, y, z, w),
                ),
                x == w && y == y && z == z && w == w
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, w, y, x),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, w, y, x),
                    ]),
                    Vector::<4, T, A>::new(z, y, x, w),
                ) == Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(z, w, y, x),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, w, y, x),
                        Vector::<4, T, A>::new(x, y, z, w),
                    ]),
                    Vector::<4, T, A>::new(x, y, z, x),
                ),
                x == z && y == w && z == y
            );
        });
    }

    #[test]
    fn test_ne() {
        for_parameters!(|T: PrimitiveNumber, A, x, y, z| {
            let w = if x > y { x } else { y };

            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w),
                    ]),
                    Vector::<2, T, A>::new(x, z),
                ) != Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(z, y),
                        Vector::<2, T, A>::new(z, w),
                    ]),
                    Vector::<2, T, A>::new(x, z),
                ),
                x != z || y != y || z != z || w != w || x != x
            );
            assert_eq!(
                Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(x, y),
                        Vector::<2, T, A>::new(z, w),
                    ]),
                    Vector::<2, T, A>::new(x, z),
                ) != Affine::<2, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<2, T, A>::new(z, w),
                        Vector::<2, T, A>::new(x, y),
                    ]),
                    Vector::<2, T, A>::new(z, x),
                ),
                x != z || y != w
            );

            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(x, y, w),
                        Vector::<3, T, A>::new(x, y, z),
                    ]),
                    Vector::<3, T, A>::new(x, y, z),
                ) != Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, w),
                        Vector::<3, T, A>::new(x, y, w),
                        Vector::<3, T, A>::new(x, y, z),
                    ]),
                    Vector::<3, T, A>::new(x, y, z),
                ),
                x != x || y != y || z != w || w != w
            );
            assert_eq!(
                Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(z, w, y),
                        Vector::<3, T, A>::new(x, y, z),
                    ]),
                    Vector::<3, T, A>::new(z, x, y),
                ) != Affine::<3, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<3, T, A>::new(z, w, y),
                        Vector::<3, T, A>::new(x, y, z),
                        Vector::<3, T, A>::new(z, w, y),
                    ]),
                    Vector::<3, T, A>::new(y, y, w),
                ),
                x != z || y != w || z != y
            );

            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, y, x, w),
                        Vector::<4, T, A>::new(x, y, z, y),
                    ]),
                    Vector::<4, T, A>::new(x, y, z, w),
                ) != Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(w, y, z, w),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, y, x, w),
                        Vector::<4, T, A>::new(x, y, z, y),
                    ]),
                    Vector::<4, T, A>::new(x, y, z, w),
                ),
                x != w || y != y || z != z || w != w
            );
            assert_eq!(
                Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, w, y, x),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, w, y, x),
                    ]),
                    Vector::<4, T, A>::new(z, y, x, w),
                ) != Affine::<4, T, A>::from_submatrix_translation(
                    Matrix::from_rows(&[
                        Vector::<4, T, A>::new(z, w, y, x),
                        Vector::<4, T, A>::new(x, y, z, w),
                        Vector::<4, T, A>::new(z, w, y, x),
                        Vector::<4, T, A>::new(x, y, z, w),
                    ]),
                    Vector::<4, T, A>::new(x, y, z, x),
                ),
                x != z || y != w || z != y
            );
        });
    }

    #[test]
    fn test_default() {
        for_parameters!(|T: PrimitiveNumber, A| {
            assert_eq!(Affine::<2, T, A>::default(), Affine::IDENTITY);
            assert_eq!(Affine::<3, T, A>::default(), Affine::IDENTITY);
            assert_eq!(Affine::<4, T, A>::default(), Affine::IDENTITY);
        });
    }

    #[test]
    fn test_mul() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.max(y);

            if !(x.powi(4) + y.powi(4) + z.powi(4)).is_finite() {
                return;
            }

            let affine = Affine::<2, T, A>::from_row_array(&[x, y, z, w, y, z]);
            let affine2 = Affine::<2, T, A>::from_row_array(&[w, x, y, z, w, y]);
            let point = Vector::<2, T, A>::new(x + 1.3, w + 5.4);
            assert_float_eq!(
                (affine * affine2).transform_point(point),
                affine2.transform_point(affine.transform_point(point)),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 1e-5,
                0.0 = -0.0
            );

            let affine = Affine::<3, T, A>::from_row_array(&[x, y, z, w, y, z, x, w, z, w, y, x]);
            let affine2 = Affine::<3, T, A>::from_row_array(&[w, x, y, z, w, y, y, x, w, x, y, z]);
            let point = Vector::<3, T, A>::new(x + 1.3, w + 5.4, y + 4.2);
            assert_float_eq!(
                (affine * affine2).transform_point(point),
                affine2.transform_point(affine.transform_point(point)),
                r2nd <= Vector::splat(x.abs().max(y.abs()).max(z.abs())) * 1e-5,
                0.0 = -0.0
            );
        });
    }

    #[test]
    fn test_mul_matrix() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.max(y);

            let affine = Affine::<2, T, A>::from_row_array(&[x, y, z, w, y, z]);
            let matrix = Matrix::<3, T, A>::from_row_array(&[w, x, y, z, w, y, x, z, w]);
            assert_float_eq!(affine * matrix, affine.to_matrix() * matrix);

            let affine = Affine::<3, T, A>::from_row_array(&[x, y, z, w, y, z, x, w, z, w, y, x]);
            let matrix = Matrix::<4, T, A>::from_row_array(&[
                w, x, y, z, w, y, x, z, w, x, z, y, w, x, z, w,
            ]);
            assert_float_eq!(affine * matrix, affine.to_matrix() * matrix);
        });
    }

    #[test]
    fn test_matrix_mul() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.max(y);

            let matrix = Matrix::<3, T, A>::from_row_array(&[w, x, y, z, w, y, x, z, w]);
            let affine = Affine::<2, T, A>::from_row_array(&[x, y, z, w, y, z]);
            assert_float_eq!(matrix * affine, matrix * affine.to_matrix());

            let matrix = Matrix::<4, T, A>::from_row_array(&[
                w, x, y, z, w, y, x, z, w, x, z, y, w, x, z, w,
            ]);
            let affine = Affine::<3, T, A>::from_row_array(&[x, y, z, w, y, z, x, w, z, w, y, x]);
            assert_float_eq!(matrix * affine, matrix * affine.to_matrix());
        });
    }

    #[test]
    fn test_mul_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.max(y);

            let affine = Affine::<2, T, A>::from_row_array(&[x, y, z, w, y, z]);
            let affine2 = Affine::<2, T, A>::from_row_array(&[w, x, y, z, w, y]);
            let mut result = affine;
            result *= affine2;
            assert_float_eq!(result, affine * affine2);

            let affine = Affine::<3, T, A>::from_row_array(&[x, y, z, w, y, z, x, w, z, w, y, x]);
            let affine2 = Affine::<3, T, A>::from_row_array(&[w, x, y, z, w, y, y, x, w, x, y, z]);
            let mut result = affine;
            result *= affine2;
            assert_float_eq!(result, affine * affine2);
        });
    }

    #[test]
    fn test_matrix_mul_assign() {
        for_parameters!(|T: PrimitiveFloat, A, x, y, z| {
            let _: [T; 3] = [x, y, z];
            let w = x.max(y);

            let matrix = Matrix::<3, T, A>::from_row_array(&[w, x, y, z, w, y, x, z, w]);
            let affine = Affine::<2, T, A>::from_row_array(&[x, y, z, w, y, z]);
            let mut result = matrix;
            result *= affine;
            assert_float_eq!(result, matrix * affine);

            let matrix = Matrix::<4, T, A>::from_row_array(&[
                w, x, y, z, w, y, x, z, w, x, z, y, w, x, z, w,
            ]);
            let affine = Affine::<3, T, A>::from_row_array(&[x, y, z, w, y, z, x, w, z, w, y, x]);
            let mut result = matrix;
            result *= affine;
            assert_float_eq!(result, matrix * affine);
        });
    }
}
