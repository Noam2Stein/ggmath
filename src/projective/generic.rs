use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Mul, Neg, Sub},
};

use crate::{
    Affine, Aligned, Alignment, Length, Matrix, One, Projective, Scalar, Unaligned, Vector, Zero,
    length::TwoOrThree,
    utils::{specialize_23, transmute_generic, transmute_ref},
};

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zero,
{
    /// A projective transform with all elements set to `0`.
    ///
    /// This transforms all vectors to the zero vector. See [`IDENTITY`] for a
    /// transform that keeps all vectors unchanged.
    ///
    /// [`IDENTITY`]: Self::IDENTITY
    pub const ZERO: Self = Self::ZERO_INTERNAL_IMPL;

    /// The internal implementation of [`Self::ZERO`].
    ///
    /// Because of type system limitations, this implementation looks crazy. Use
    /// a separate constant so that IDEs do not show the implementation.
    const ZERO_INTERNAL_IMPL: Self = match N {
        // SAFETY: We are transmuting a type to itself
        2 => unsafe {
            transmute_generic::<Projective<2, T, A>, Projective<N, T, A>>(Projective::<2, T, A>(
                Matrix::<3, T, A>::ZERO,
            ))
        },
        // SAFETY: We are transmuting a type to itself
        3 => unsafe {
            transmute_generic::<Projective<3, T, A>, Projective<N, T, A>>(Projective::<3, T, A>(
                Matrix::<4, T, A>::ZERO,
            ))
        },
        _ => unreachable!(),
    };
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Zero + One,
{
    /// A projective transform that keeps all vectors unchanged.
    ///
    /// Diagonal elements are `1` and the rest are `0`.
    pub const IDENTITY: Self = Self::IDENTITY_INTERNAL_IMPL;

    /// The implementation of [`Self::IDENTITY`].
    ///
    /// Because of type system limitations, this implementation looks crazy. Use
    /// a separate constant so that IDEs do not show the implementation.
    const IDENTITY_INTERNAL_IMPL: Self = match N {
        // SAFETY: We are transmuting a type to itself
        2 => unsafe {
            transmute_generic::<Projective<2, T, A>, Projective<N, T, A>>(Projective::<2, T, A>(
                Matrix::<3, T, A>::IDENTITY,
            ))
        },
        // SAFETY: We are transmuting a type to itself
        3 => unsafe {
            transmute_generic::<Projective<3, T, A>, Projective<N, T, A>>(Projective::<3, T, A>(
                Matrix::<4, T, A>::IDENTITY,
            ))
        },
        _ => unreachable!(),
    };
}

#[expect(private_bounds)]
impl<const N: usize, T, A: Alignment> Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar,
{
    /// Creates a projective transform from a non-uniform `scale`.
    #[inline]
    #[must_use]
    pub fn from_scale(scale: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        specialize_23!(Projective::<N, T, A>::from_scale_backend(scale))
    }

    /// Creates a projective transform from a `translation` vector.
    #[inline]
    #[must_use]
    pub fn from_translation(translation: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        specialize_23!(Projective::<N, T, A>::from_translation_backend(translation))
    }

    /// Creates a projective transform from a non-uniform `scale` and a
    /// `translation` vector.
    #[inline]
    #[must_use]
    pub fn from_scale_translation(scale: Vector<N, T, A>, translation: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        specialize_23!(Projective::<N, T, A>::from_scale_translation_backend(
            scale,
            translation
        ))
    }

    /// Creates a projective transform from a linear transformation matrix.
    #[inline]
    #[must_use]
    pub fn from_matrix(matrix: &Matrix<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        specialize_23!(Projective::<N, T, A>::from_matrix_backend(matrix))
    }

    /// Creates a projective transform from a linear transformation `matrix` and
    /// a `translation` vector.
    #[inline]
    #[must_use]
    pub fn from_matrix_translation(matrix: &Matrix<N, T, A>, translation: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        specialize_23!(Projective::<N, T, A>::from_matrix_translation_backend(
            matrix,
            translation
        ))
    }

    /// Creates a projective transform from an affine transform.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Affine2, Proj2, Vec2, Vec3};
    /// #
    /// let affine = Affine2::from_rows(&[
    ///     Vec2::new(1, 2),
    ///     Vec2::new(3, 4),
    ///     Vec2::new(5, 6),
    /// ]);
    ///
    /// assert_eq!(
    ///     Proj2::from_affine(&affine),
    ///     Proj2::from_rows(&[
    ///         Vec3::new(1, 2, 0),
    ///         Vec3::new(3, 4, 0),
    ///         Vec3::new(5, 6, 1),
    ///     ]),
    /// );
    /// ```
    #[inline]
    #[must_use]
    pub fn from_affine(affine: &Affine<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        specialize_23!(Projective::<N, T, A>::from_affine_backend(affine))
    }

    /// Returns the translation part of a projective transform.
    ///
    /// Even if `self` contains a projection, non-translation cells are
    /// completely ignored.
    ///
    /// # Examples
    ///
    /// ```
    /// # use ggmath::{Proj2, Vec2, Vec3};
    /// #
    /// let proj = Proj2::from_rows(&[
    ///     Vec3::new(1, 0, 0),
    ///     Vec3::new(0, 1, 0),
    ///     Vec3::new(6, 8, 1),
    /// ]);
    /// assert_eq!(proj.translation(), Vec2::new(6, 8));
    /// ```
    #[inline]
    #[must_use]
    pub fn translation(&self) -> Vector<N, T, A> {
        specialize_23!(Projective::<N, T, A>::translation_backend(self))
    }

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

    /// Reinterprets a homogeneous matrix as a projective transform.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn from_homogeneous(homogeneous: &Matrix<3, T, A>) -> Self {
        Self(*homogeneous)
    }

    /// Reinterprets a projective transform as a homogeneous matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn to_homogeneous(&self) -> Matrix<3, T, A> {
        self.0
    }

    /// Reinterprets a projective transform as a reference to a homogeneous
    /// matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_homogeneous(&self) -> &Matrix<3, T, A> {
        &self.0
    }

    /// Reinterprets a projective transform as a mutable reference to a
    /// homogeneous matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_mut_homogeneous(&mut self) -> &mut Matrix<3, T, A> {
        &mut self.0
    }

    /// Returns a reference to the projective transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<3, T, A>; 3] {
        self.0.as_rows()
    }

    /// Returns a mutable reference to the projective transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<3, T, A>; 3] {
        self.0.as_mut_rows()
    }

    /// Returns the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of columns
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
    /// Panics if `index` is greater than or equal to the number of columns
    /// `N + 1`.
    #[inline]
    #[track_caller]
    pub const fn set_column(&mut self, index: usize, value: Vector<3, T, A>) {
        self.0.set_column(index, value);
    }

    #[inline(always)]
    fn from_scale_backend(scale: Vector<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self(Matrix::from_diagonal(scale.to_homogeneous()))
    }

    #[inline(always)]
    fn from_translation_backend(translation: Vector<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            Vector::<3, T, A>::X,
            Vector::<3, T, A>::Y,
            translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn from_scale_translation_backend(scale: Vector<2, T, A>, translation: Vector<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            Vector::<3, T, A>::new(scale.x, T::ZERO, T::ZERO),
            Vector::<3, T, A>::new(T::ZERO, scale.y, T::ZERO),
            translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn from_matrix_backend(matrix: &Matrix<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            matrix.x_axis.extend(T::ZERO),
            matrix.y_axis.extend(T::ZERO),
            Vector::<3, T, A>::new(T::ZERO, T::ZERO, T::ONE),
        ])
    }

    #[inline(always)]
    fn from_matrix_translation_backend(
        matrix: &Matrix<2, T, A>,
        translation: Vector<2, T, A>,
    ) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            matrix.x_axis.extend(T::ZERO),
            matrix.y_axis.extend(T::ZERO),
            translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn from_affine_backend(affine: &Affine<2, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            affine.matrix.x_axis.extend(T::ZERO),
            affine.matrix.y_axis.extend(T::ZERO),
            affine.translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn translation_backend(&self) -> Vector<2, T, A> {
        self.0.z_axis.truncate()
    }

    #[inline(always)]
    pub(in crate::projective) fn debug_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Debug,
    {
        write!(f, "{:?}", self.as_homogeneous())
    }

    #[inline(always)]
    pub(in crate::projective) fn display_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Display,
    {
        write!(f, "{}", self.as_homogeneous())
    }

    #[inline(always)]
    pub(in crate::projective) fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    pub(in crate::projective) fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn neg_backend(&self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn add_backend(&self, rhs: &Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn sub_backend(&self, rhs: &Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn mul_backend(&self, rhs: &Self) -> Self
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Self(self.0 * rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn mul_scalar_backend(&self, rhs: T) -> Self
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Self(self.0 * rhs)
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

    /// Reinterprets a homogeneous matrix as a projective transform.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn from_homogeneous(homogeneous: &Matrix<4, T, A>) -> Self {
        Self(*homogeneous)
    }

    /// Reinterprets a projective transform as a homogeneous matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn to_homogeneous(&self) -> Matrix<4, T, A> {
        self.0
    }

    /// Reinterprets a projective transform as a reference to a homogeneous
    /// matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_homogeneous(&self) -> &Matrix<4, T, A> {
        &self.0
    }

    /// Reinterprets a projective transform as a mutable reference to a
    /// homogeneous matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_mut_homogeneous(&mut self) -> &mut Matrix<4, T, A> {
        &mut self.0
    }

    /// Returns a reference to the projective transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_rows(&self) -> &[Vector<4, T, A>; 4] {
        self.0.as_rows()
    }

    /// Returns a mutable reference to the projective transform's rows.
    #[inline]
    #[must_use]
    pub const fn as_mut_rows(&mut self) -> &mut [Vector<4, T, A>; 4] {
        self.0.as_mut_rows()
    }

    /// Returns the column at the given index.
    ///
    /// # Panics
    ///
    /// Panics if `index` is greater than or equal to the number of columns
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
    /// Panics if `index` is greater than or equal to the number of columns
    /// `N + 1`.
    #[inline]
    #[track_caller]
    pub const fn set_column(&mut self, index: usize, value: Vector<4, T, A>) {
        self.0.set_column(index, value);
    }

    #[inline(always)]
    fn from_scale_backend(scale: Vector<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self(Matrix::from_diagonal(scale.to_homogeneous()))
    }

    #[inline(always)]
    fn from_translation_backend(translation: Vector<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            Vector::<4, T, A>::X,
            Vector::<4, T, A>::Y,
            Vector::<4, T, A>::Z,
            translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn from_scale_translation_backend(scale: Vector<3, T, A>, translation: Vector<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            Vector::<4, T, A>::new(scale.x, T::ZERO, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, scale.y, T::ZERO, T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, scale.z, T::ZERO),
            translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn from_matrix_backend(matrix: &Matrix<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            matrix.x_axis.extend(T::ZERO),
            matrix.y_axis.extend(T::ZERO),
            matrix.z_axis.extend(T::ZERO),
            Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE),
        ])
    }

    #[inline(always)]
    fn from_matrix_translation_backend(
        matrix: &Matrix<3, T, A>,
        translation: Vector<3, T, A>,
    ) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            matrix.x_axis.extend(T::ZERO),
            matrix.y_axis.extend(T::ZERO),
            matrix.z_axis.extend(T::ZERO),
            translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn from_affine_backend(affine: &Affine<3, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_rows(&[
            affine.matrix.x_axis.extend(T::ZERO),
            affine.matrix.y_axis.extend(T::ZERO),
            affine.matrix.z_axis.extend(T::ZERO),
            affine.translation.to_homogeneous(),
        ])
    }

    #[inline(always)]
    fn translation_backend(&self) -> Vector<3, T, A> {
        self.0.w_axis.truncate()
    }

    #[inline(always)]
    pub(in crate::projective) fn debug_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Debug,
    {
        write!(f, "{:?}", self.as_homogeneous())
    }

    #[inline(always)]
    pub(in crate::projective) fn display_backend(
        &self,
        f: &mut core::fmt::Formatter<'_>,
    ) -> core::fmt::Result
    where
        T: Display,
    {
        write!(f, "{}", self.as_homogeneous())
    }

    #[inline(always)]
    pub(in crate::projective) fn eq_backend(&self, other: &Self) -> bool
    where
        T: PartialEq,
    {
        self.0 == other.0
    }

    #[inline(always)]
    pub(in crate::projective) fn hash_backend<H: core::hash::Hasher>(&self, (state,): (&mut H,))
    where
        T: Hash,
    {
        self.0.hash(state);
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn neg_backend(&self) -> Self
    where
        T: Neg<Output = T>,
    {
        Self(-self.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn add_backend(&self, rhs: &Self) -> Self
    where
        T: Add<Output = T>,
    {
        Self(self.0 + rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn sub_backend(&self, rhs: &Self) -> Self
    where
        T: Sub<Output = T>,
    {
        Self(self.0 - rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn mul_backend(&self, rhs: &Self) -> Self
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Self(self.0 * rhs.0)
    }

    #[inline(always)]
    #[track_caller]
    pub(in crate::projective) fn mul_scalar_backend(&self, rhs: T) -> Self
    where
        T: Add<Output = T> + Mul<Output = T>,
    {
        Self(self.0 * rhs)
    }
}

// Tests are located at `src/projective.rs`. This module's contents are
// separated into this `generic` module as a workaround for a rustdoc bug, so no
// reason to also move the tests.
