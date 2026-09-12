use core::{
    mem::MaybeUninit,
    ops::{Add, Mul},
};

use crate::{
    Affine, Aligned, Alignment, Dim, Element, Matrix, One, TwoThreeOrFour, Unaligned, Vector, Zero,
    affine::AffineFields,
    utils::{transmute_generic, transmute_mut, transmute_ref},
};

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Dim<N>: TwoThreeOrFour,
    T: Element + Zero,
{
    /// An affine transform with all elements set to `0`.
    ///
    /// This transforms all vectors to a zero vector. See [`IDENTITY`] for
    /// an affine transform with no transformation.
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    pub const ZERO: Self = Self::from_matrix_translation(&Matrix::ZERO, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Dim<N>: TwoThreeOrFour,
    T: Element + Zero + One,
{
    /// An affine transform with no transformation.
    pub const IDENTITY: Self = Self::from_matrix_translation(&Matrix::IDENTITY, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Dim<N>: TwoThreeOrFour,
    T: Element,
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
        Self::from_matrix_translation(&Matrix::from_row_fn(&mut f), f(N))
    }

    /// Creates an affine transform from a non-uniform `scale`.
    #[inline]
    #[must_use]
    pub const fn from_scale(scale: Vector<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self::from_matrix(&Matrix::from_scale(scale))
    }

    /// Creates an affine transform from a `translation` vector.
    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_matrix_translation(&Matrix::IDENTITY, translation)
    }

    /// Creates an affine transform from a non-uniform scale and a translation
    /// vector.
    #[inline]
    #[must_use]
    pub const fn from_scale_translation(
        scale: Vector<N, T, A>,
        translation: Vector<N, T, A>,
    ) -> Self
    where
        T: Zero + One,
    {
        Self::from_matrix_translation(&Matrix::from_scale(scale), translation)
    }

    /// Creates an affine transform from `matrix` expressing rotation and
    /// scale, but not translation.
    #[inline]
    #[must_use]
    pub const fn from_matrix(matrix: &Matrix<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self::from_matrix_translation(matrix, Vector::ZERO)
    }

    /// Creates an affine transform from `translation` and `matrix`
    /// expressing rotation and scale.
    #[inline]
    #[must_use]
    pub const fn from_matrix_translation(
        matrix: &Matrix<N, T, A>,
        translation: Vector<N, T, A>,
    ) -> Self {
        if const {
            size_of::<Affine<N, T, A>>()
                == size_of::<Matrix<N, T, A>>() + size_of::<Vector<N, T, A>>()
        } {
            #[repr(C)]
            struct Inner<const N: usize, T, A: Alignment>(Matrix<N, T, A>, Vector<N, T, A>)
            where
                Dim<N>: TwoThreeOrFour,
                T: Element;

            // SAFETY: We checked that there is no padding that needs to be
            // initialized. These types are guaranteed to simply consist of
            // six values of `T`.
            unsafe {
                transmute_generic::<Inner<N, T, A>, Affine<N, T, A>>(Inner(*matrix, translation))
            }
        } else if const { N == 2 && A::IS_ALIGNED && size_of::<Affine<N, T, A>>() == size_of::<T>() * 8 }
        {
            #[repr(C)]
            struct Inner<const N: usize, T, A: Alignment>(
                Matrix<N, T, A>,
                Vector<N, T, A>,
                MaybeUninit<Vector<N, T, A>>,
            )
            where
                Dim<N>: TwoThreeOrFour,
                T: Element;

            // SAFETY: We checked that `Affine` "contains" exactly eight
            // elements of `T` (including padding). We zeroed the padding, which
            // is guaranteed to accept all bit-patterns.
            unsafe {
                transmute_generic::<Inner<N, T, A>, Affine<N, T, A>>(Inner(
                    *matrix,
                    translation,
                    MaybeUninit::zeroed(),
                ))
            }
        } else {
            unreachable!()
        }
    }

    /// Converts an affine transform to a matrix and a translation vector.
    ///
    /// This is a no-op.
    #[inline]
    #[must_use]
    pub fn to_matrix_translation(&self) -> (Matrix<N, T, A>, Vector<N, T, A>) {
        (self.matrix, self.translation)
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

    /// Converts `self` to SIMD-aligned storage.
    ///
    /// See [`Alignment`] for more information about SIMD-aligned types.
    #[inline]
    #[must_use]
    pub const fn align(&self) -> Affine<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts `self` to non-SIMD-aligned storage.
    ///
    /// See [`Alignment`] for more information about SIMD-aligned types.
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Affine<N, T, Unaligned> {
        self.to_alignment()
    }

    /// Converts `self` to the specified SIMD-alignment mode.
    ///
    /// If the output mode is known to always be [`Aligned`] or always be
    /// [`Unaligned`], use methods [`align`] and [`unalign`] instead.
    ///
    /// See [`Alignment`] for more information about SIMD-aligned types.
    ///
    /// [`align`]: Self::align
    /// [`unalign`]: Self::unalign
    #[inline]
    #[must_use]
    pub const fn to_alignment<A2: Alignment>(&self) -> Affine<N, T, A2> {
        // SAFETY: Just like in `Deref`, this operation is sound.
        let fields = unsafe { transmute_ref::<Affine<N, T, A>, AffineFields<N, T, A>>(self) };

        Affine::from_matrix_translation(
            &fields.matrix.to_alignment(),
            fields.translation.to_alignment(),
        )
    }
}

impl<T, A: Alignment> Affine<2, T, A>
where
    T: Element,
{
    /// Creates a 2D affine transform from three row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<2, T, A>; 3]) -> Self {
        Self::from_matrix_translation(&Matrix::from_rows(&[rows[0], rows[1]]), rows[2])
    }

    /// Converts a row-major affine transform to an array of row vectors.
    #[inline]
    #[must_use]
    pub const fn to_rows(&self) -> [Vector<2, T, A>; 3] {
        *self.as_rows()
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

    /// Converts a row-major affine transform to a row-major array of elements.
    #[inline]
    #[must_use]
    pub const fn to_row_array(&self) -> [T; 6] {
        // SAFETY: Because 2 is a power of two, there is no padding, so elements
        // are consecutive
        unsafe { *transmute_ref::<Affine<2, T, A>, [T; 6]>(self) }
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
    ///     Mat3::from_rows(&[
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
}

impl<T, A: Alignment> Affine<3, T, A>
where
    T: Element,
{
    /// Creates a 3D affine transform from four row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<3, T, A>; 4]) -> Self {
        Self::from_matrix_translation(&Matrix::from_rows(&[rows[0], rows[1], rows[2]]), rows[3])
    }

    /// Converts a row-major affine transform to an array of row vectors.
    #[inline]
    #[must_use]
    pub const fn to_rows(&self) -> [Vector<3, T, A>; 4] {
        *self.as_rows()
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

    /// Converts a row-major affine transform to a row-major array of elements.
    #[inline]
    #[must_use]
    pub const fn to_row_array(&self) -> [T; 12] {
        if const {
            let there_is_padding = size_of::<Vector<3, T, A>>() > size_of::<[T; 3]>();
            there_is_padding
        } {
            [
                self.as_rows()[0].as_array()[0],
                self.as_rows()[0].as_array()[1],
                self.as_rows()[0].as_array()[2],
                self.as_rows()[1].as_array()[0],
                self.as_rows()[1].as_array()[1],
                self.as_rows()[1].as_array()[2],
                self.as_rows()[2].as_array()[0],
                self.as_rows()[2].as_array()[1],
                self.as_rows()[2].as_array()[2],
                self.as_rows()[3].as_array()[0],
                self.as_rows()[3].as_array()[1],
                self.as_rows()[3].as_array()[2],
            ]
        } else {
            // SAFETY: This only runs if there is no padding, in which case
            // elements are consecutive
            unsafe { *transmute_ref::<Affine<3, T, A>, [T; 12]>(self) }
        }
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
    ///     Mat3::from_rows(&[
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
}

impl<T, A: Alignment> Affine<4, T, A>
where
    T: Element,
{
    /// Creates a 4D affine transform from five row vectors.
    #[inline]
    #[must_use]
    pub const fn from_rows(rows: &[Vector<4, T, A>; 5]) -> Self {
        Self::from_matrix_translation(
            &Matrix::from_rows(&[rows[0], rows[1], rows[2], rows[3]]),
            rows[4],
        )
    }

    /// Converts a row-major affine transform to an array of row vectors.
    #[inline]
    #[must_use]
    pub const fn to_rows(&self) -> [Vector<4, T, A>; 5] {
        *self.as_rows()
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

    /// Converts a row-major affine transform to a row-major array of elements.
    #[inline]
    #[must_use]
    pub const fn to_row_array(&self) -> [T; 20] {
        // SAFETY: Because 4 is a power of two, there is no padding, so elements
        // are consecutive
        unsafe { *transmute_ref::<Affine<4, T, A>, [T; 20]>(self) }
    }
}

// Tests are located at `src/affine.rs`. This module's contents are separated
// into this `generic` module as a workaround for a rustdoc bug, so no reason to
// also move the tests.
