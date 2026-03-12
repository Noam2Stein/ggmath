use core::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Matrix, Scalar, ScalarBackend, ScalarRepr, SignedInteger,
    SupportedLength, Unaligned, Vector,
    constants::{Nan, One, Zero},
    repr::{Repr3, Repr4, Repr5},
    specialize::specialize,
    transmute::{transmute_generic, transmute_mut, transmute_ref},
};

/// An `N`-dimensional affine transform which can represent translation,
/// rotation, scaling and shear of type `T`.
///
/// Affines are currently missing most functionality. See [`from_columns`] for
/// raw construction.
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
/// # Fields
///
/// `matrix: Matrix<N, T, A>`
///
/// The part representing rotation, scaling and shear.
///
/// `translation: Vector<N, T, A>`
///
/// The part representing translation.
///
/// # Memory layout
///
/// `Affine<N, T, A>` contains [`Matrix<N, T, A>`] followed by
/// [`Vector<N, T, A>`] followed by optional padding.
///
/// `Affine<N, T, Unaligned>` has the alignment of `T` and has no padding.
/// [`Affine2<T>`] may have one padding vector and may have higher alignment
/// than [`Mat2<T>`]. [`Affine3<T>`] may have four padding elements if both
/// [`Mat3<T>`] and [`Vec3<T>`] have no padding, and may have higher alignment
/// than [`Mat3<T>`]. [`Affine<4, T, Aligned>`] has the alignment of [`Mat4<T>`]
/// and has no padding.
///
/// Padding is fully initialized and accepts all bit patterns. Unless `T`
/// accepts all bit patterns, it is not sound to assume padding contains valid
/// values of `T`.
///
/// Affines of compatible [`Scalar::Repr`] types have the same size. This means
/// that they are transmutable, but can still have different alignments (see
/// [`to_repr`]).
///
/// Types containing compatible affines, matrices, vectors and arrays may not
/// have compatible layouts themselves. For example, even though [`Affine2<T>`]
/// and `[T; 6]` have compatible layouts, [`Option<Affine2<T>>`] and
/// `Option<[T; 6]>` may not.
///
/// [`Affine2<T>`]: crate::Affine2
/// [`Affine3<T>`]: crate::Affine3
/// [`Affine2U<T>`]: crate::Affine2U
/// [`Affine3U<T>`]: crate::Affine3U
/// [`Mat2<T>`]: crate::Mat2
/// [`Mat3<T>`]: crate::Mat3
/// [`Vec3<T>`]: crate::Vec3
/// [`Mat4<T>`]: crate::Mat4
/// [`from_columns`]: Self::from_columns
/// [`to_repr`]: Self::to_repr
#[repr(transparent)]
pub struct Affine<const N: usize, T, A: Alignment>(
    pub(crate) <T::Repr as ScalarRepr>::AffineRepr<N, T, A>,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

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
    pub const ZERO: Self = Self::from_mat_translation(Matrix::ZERO, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Zero + One,
{
    /// An affine transform with no transformation.
    pub const IDENTITY: Self = Self::from_mat_translation(Matrix::IDENTITY, Vector::ZERO);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Nan,
{
    /// An affine transform with all elements set to NaN (Not a Number).
    pub const NAN: Self = Self::from_mat_translation(Matrix::NAN, Vector::NAN);
}

impl<const N: usize, T, A: Alignment> Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Creates an affine transform from a matrix expressing rotation, scaling
    /// and shear.
    #[inline]
    #[must_use]
    pub const fn from_mat(mat: Matrix<N, T, A>) -> Self
    where
        T: Zero,
    {
        Self::from_mat_translation(mat, Vector::ZERO)
    }

    /// Creates an affine transform from a translation vector.
    #[inline]
    #[must_use]
    pub const fn from_translation(translation: Vector<N, T, A>) -> Self
    where
        T: Zero + One,
    {
        Self::from_mat_translation(Matrix::IDENTITY, translation)
    }

    /// Creates an affine transform from a translation vector, and a matrix
    /// expressing rotation, scaling and shear.
    #[inline]
    #[must_use]
    pub const fn from_mat_translation(mat: Matrix<N, T, A>, translation: Vector<N, T, A>) -> Self {
        if const {
            size_of::<Affine<N, T, A>>()
                == size_of::<Matrix<N, T, A>>() + size_of::<Vector<N, T, A>>()
        } {
            // SAFETY: `AffineRepr` is a matrix then a vector, like `Affine`,
            // with no padding because the size was checked to have no padding.
            unsafe {
                #[repr(C)]
                struct AffineRepr<const N: usize, T, A: Alignment>
                where
                    Length<N>: SupportedLength,
                    T: Scalar,
                {
                    mat: Matrix<N, T, A>,
                    translation: Vector<N, T, A>,
                }

                transmute_generic::<AffineRepr<N, T, A>, Affine<N, T, A>>(AffineRepr {
                    mat,
                    translation,
                })
            }
        } else if const { N == 2 && size_of::<Affine<2, T, A>>() / size_of::<Vector<2, T, A>>() == 4 }
        {
            // SAFETY: `AffineRepr` is a matrix then a vector, like `Affine`,
            // then another padding vector because it was checked that there is
            // exactly one padding vector.
            unsafe {
                #[repr(C)]
                struct AffineRepr<const N: usize, T, A: Alignment>
                where
                    Length<N>: SupportedLength,
                    T: Scalar,
                {
                    mat: Matrix<N, T, A>,
                    translation: Vector<N, T, A>,
                    padding: Vector<N, T, A>,
                }

                transmute_generic::<AffineRepr<N, T, A>, Affine<N, T, A>>(AffineRepr {
                    mat,
                    translation,
                    padding: translation,
                })
            }
        } else if const {
            N == 3
                && size_of::<Affine<3, T, A>>()
                    - size_of::<Matrix<3, T, A>>()
                    - size_of::<Vector<3, T, A>>()
                    == size_of::<T>() * 4
        } {
            // SAFETY: `AffineRepr` is a matrix then a vector, like `Affine`,
            // then 4 padding elements because it was checked that there are
            // exactly 4 padding elements.
            unsafe {
                #[repr(C)]
                struct AffineRepr<const N: usize, T, A: Alignment>
                where
                    Length<N>: SupportedLength,
                    T: Scalar,
                {
                    mat: Matrix<N, T, A>,
                    translation: Vector<N, T, A>,
                    padding: Repr4<T>,
                }

                transmute_generic::<AffineRepr<N, T, A>, Affine<N, T, A>>(AffineRepr {
                    mat,
                    translation,
                    padding: Repr4(
                        translation.as_array_ref()[0],
                        translation.as_array_ref()[1],
                        translation.as_array_ref()[2],
                        translation.as_array_ref()[2],
                    ),
                })
            }
        } else {
            unreachable!()
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
        // SAFETY: This operation is identical to the one used in the
        // implementation of `Deref`.
        let deref = unsafe { transmute_ref::<Affine<N, T, A>, AffineDeref<N, T, A>>(self) };

        Affine::from_mat_translation(
            deref.matrix.to_alignment(),
            deref.translation.to_alignment(),
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
    /// output type. For example, when converting affines from `u8` to `bool`,
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
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let bits = Affine2::<u8>::from_columns(&[
    ///     Vec2::new(1, 0),
    ///     Vec2::new(0, 1),
    ///     Vec2::new(0, 1),
    /// ]);
    ///
    /// // SAFETY: `bool` accepts both the `0` and `1` bit patterns.
    /// let bools = unsafe { bits.to_repr::<bool>() };
    ///
    /// assert_eq!(bools, Affine2::from_columns(&[
    ///     Vec2::new(true, false),
    ///     Vec2::new(false, true),
    ///     Vec2::new(false, true),
    /// ]));
    /// ```
    ///
    /// Incorrect usage:
    ///
    /// ```compile_fail
    /// # use ggmath::{Affine2, Vec2};
    /// #
    /// let a = Affine2::<i32>::from_columns(&[
    ///     Vec2::new(1, 2),
    ///     Vec2::new(3, 4),
    ///     Vec2::new(5, 6),
    /// ]);
    ///
    /// // This does not compile since `i32` and `i64` are not compatible.
    /// let _ = unsafe { a.to_repr::<i64>() };
    /// ```
    #[inline]
    #[must_use]
    #[expect(private_bounds)]
    pub const unsafe fn to_repr<T2>(self) -> Affine<N, T2, A>
    where
        T2: Scalar<Repr = T::Repr>,
        T::Repr: SignedInteger,
    {
        // SAFETY: Affines of scalars with the same `Scalar::Repr` are
        // guaranteed to have compatible memory layouts if `Repr` is a signed
        // integer.
        unsafe { transmute_generic::<Affine<N, T, A>, Affine<N, T2, A>>(self) }
    }
}

impl<T, A: Alignment> Affine<2, T, A>
where
    T: Scalar,
{
    /// Creates a 2D affine transform from three column vectors.
    #[inline]
    #[must_use]
    pub const fn from_columns(array: &[Vector<2, T, A>; 3]) -> Self {
        match size_of::<Affine<2, T, A>>() / size_of::<Vector<2, T, A>>() {
            // SAFETY: `Repr3<Vector<2, T, A>>` is `Matrix<2, T, A>` (two vectors)
            // followed by `Vector<2, T, A>` followed by no padding, because
            // the size has been checked to have no padding.
            3 => unsafe {
                transmute_generic::<Repr3<Vector<2, T, A>>, Affine<2, T, A>>(Repr3(
                    array[0], array[1], array[2],
                ))
            },

            // SAFETY: `Repr4<Vector<2, T, A>>` is `Matrix<2, T, A>` (two vectors)
            // followed by `Vector<2, T, A>` followed by one padding vector,
            // because the size has been checked to have exactly one padding
            // vector.
            4 => unsafe {
                transmute_generic::<Repr4<Vector<2, T, A>>, Affine<2, T, A>>(Repr4(
                    array[0], array[1], array[2], array[2],
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Returns a reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns(&self) -> &[Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_ref::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }

    /// Returns a mutable reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns_mut(&mut self) -> &mut [Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_mut::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }

    /// Creates a 2D affine transform from three column vectors.
    ///
    /// This function has been replaced by [`from_columns`] and will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "replaced by `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_cols(
        x_axis: Vector<2, T, A>,
        y_axis: Vector<2, T, A>,
        translation: Vector<2, T, A>,
    ) -> Self {
        Self::from_columns(&[x_axis, y_axis, translation])
    }

    /// Creates a 2D affine transform from three column vectors.
    ///
    /// This function has been renamed to [`from_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "renamed to `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<2, T, A>; 3]) -> Self {
        Self::from_columns(array)
    }

    /// Converts the affine transform to three column vectors.
    ///
    /// This function has been replaced by [`as_columns`] and will be removed in
    /// a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "replaced by `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<2, T, A>; 3] {
        *self.as_columns()
    }

    /// Returns a reference to the affine transform's columns.
    ///
    /// This function has been renamed to [`as_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<2, T, A>; 3] {
        self.as_columns()
    }

    /// Returns a mutable reference to the affine transform's columns.
    ///
    /// This function has been renamed to [`as_columns_mut`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns_mut`]: Self::as_columns_mut
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns_mut`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<2, T, A>; 3] {
        self.as_columns_mut()
    }
}

impl<T, A: Alignment> Affine<3, T, A>
where
    T: Scalar,
{
    /// Creates a 3D affine transform from four column vectors.
    #[inline]
    #[must_use]
    pub const fn from_columns(array: &[Vector<3, T, A>; 4]) -> Self {
        match const {
            (size_of::<Affine<3, T, A>>()
                - size_of::<Matrix<3, T, A>>()
                - size_of::<Vector<3, T, A>>())
                / size_of::<T>()
        } {
            // SAFETY: `Repr4<Vector<3, T, A>>` is `Matrix<3, T, A>` (three
            // vectors) followed by `Vector<3, T, A>` followed by no padding,
            // because the number of padding elements has been checked to be 0.
            0 => unsafe {
                transmute_generic::<Repr4<Vector<3, T, A>>, Affine<3, T, A>>(Repr4(
                    array[0], array[1], array[2], array[3],
                ))
            },

            // SAFETY: `AffineRepr` is `Matrix<3, T, A>` (three vectors)
            // followed by `Vector<3, T, A>` followed by four padding elements,
            // because the number of padding elements has been checked to be 4.
            4 => unsafe {
                #[repr(C)]
                struct AffineRepr<T, A: Alignment>
                where
                    T: Scalar,
                {
                    x_axis: Vector<3, T, A>,
                    y_axis: Vector<3, T, A>,
                    z_axis: Vector<3, T, A>,
                    w_axis: Vector<3, T, A>,
                    padding: Repr4<T>,
                }

                transmute_generic::<AffineRepr<T, A>, Affine<3, T, A>>(AffineRepr {
                    x_axis: array[0],
                    y_axis: array[1],
                    z_axis: array[2],
                    w_axis: array[3],
                    padding: Repr4(
                        array[0].as_array_ref()[0],
                        array[0].as_array_ref()[1],
                        array[0].as_array_ref()[2],
                        array[0].as_array_ref()[2],
                    ),
                })
            },

            _ => unreachable!(),
        }
    }

    /// Returns a reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns(&self) -> &[Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_ref::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }

    /// Returns a mutable reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns_mut(&mut self) -> &mut [Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_mut::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }

    /// Creates a 3D affine transform from four column vectors.
    ///
    /// This function has been replaced by [`from_columns`] and will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "replaced by `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_cols(
        x_axis: Vector<3, T, A>,
        y_axis: Vector<3, T, A>,
        z_axis: Vector<3, T, A>,
        translation: Vector<3, T, A>,
    ) -> Self {
        Self::from_columns(&[x_axis, y_axis, z_axis, translation])
    }

    /// Creates a 3D affine transform from four column vectors.
    ///
    /// This function has been renamed to [`from_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "renamed to `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<3, T, A>; 4]) -> Self {
        Self::from_columns(array)
    }

    /// Converts the affine transform to four column vectors.
    ///
    /// This function has been replaced by [`as_columns`] and will be removed in
    /// a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "replaced by `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<3, T, A>; 4] {
        *self.as_columns()
    }

    /// Returns a reference to the affine transform's columns.
    ///
    /// This function has been renamed to [`as_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<3, T, A>; 4] {
        self.as_columns()
    }

    /// Returns a mutable reference to the affine transform's columns.
    ///
    /// This function has been renamed to [`as_columns_mut`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns_mut`]: Self::as_columns_mut
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns_mut`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<3, T, A>; 4] {
        self.as_columns_mut()
    }
}

impl<T, A: Alignment> Affine<4, T, A>
where
    T: Scalar,
{
    /// Creates a 4D affine transform from five column vectors.
    #[inline]
    #[must_use]
    pub const fn from_columns(array: &[Vector<4, T, A>; 5]) -> Self {
        // SAFETY: `Repr5<Vector<4, T, A>>` is `Matrix<4, T, A>` (four vectors)
        // followed by `Vector<4, T, A>` followed by no padding, because
        // `Affine<4, T, Aligned>` has no padding.
        unsafe {
            transmute_generic::<Repr5<Vector<4, T, A>>, Affine<4, T, A>>(Repr5(
                array[0], array[1], array[2], array[3], array[4],
            ))
        }
    }

    /// Returns a reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns(&self) -> &[Vector<4, T, A>; 5] {
        // SAFETY: `Affine<4, T, A>` is guaranteed to begin with
        // `Matrix<4, T, A>` (four vectors) then `Vector<4, T, A>`, which is 5
        // vectors in total.
        unsafe { transmute_ref::<Affine<4, T, A>, [Vector<4, T, A>; 5]>(self) }
    }

    /// Returns a mutable reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_columns_mut(&mut self) -> &mut [Vector<4, T, A>; 5] {
        // SAFETY: `Affine<4, T, A>` is guaranteed to begin with
        // `Matrix<4, T, A>` (four vectors) then `Vector<4, T, A>`, which is 5
        // vectors in total.
        unsafe { transmute_mut::<Affine<4, T, A>, [Vector<4, T, A>; 5]>(self) }
    }

    /// Creates a 4D affine transform from five column vectors.
    ///
    /// This function has been replaced by [`from_columns`] and will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "replaced by `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_cols(
        x_axis: Vector<4, T, A>,
        y_axis: Vector<4, T, A>,
        z_axis: Vector<4, T, A>,
        w_axis: Vector<4, T, A>,
        translation: Vector<4, T, A>,
    ) -> Self {
        Self::from_columns(&[x_axis, y_axis, z_axis, w_axis, translation])
    }

    /// Creates a 4D affine transform from five column vectors.
    ///
    /// This function has been renamed to [`from_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`from_columns`]: Self::from_columns
    #[deprecated(since = "0.16.3", note = "renamed to `from_columns`")]
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<4, T, A>; 5]) -> Self {
        Self::from_columns(array)
    }

    /// Converts the affine transform to five column vectors.
    ///
    /// This function has been replaced by [`as_columns`] and will be removed in
    /// a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "replaced by `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<4, T, A>; 5] {
        *self.as_columns()
    }

    /// Returns a reference to the affine transform's columns.
    ///
    /// This function has been renamed to [`as_columns`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns`]: Self::as_columns
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<4, T, A>; 5] {
        self.as_columns()
    }

    /// Returns a mutable reference to the affine transform's columns.
    ///
    /// This function has been renamed to [`as_columns_mut`]. This name will be
    /// removed in a future version.
    ///
    /// [`as_columns_mut`]: Self::as_columns_mut
    #[deprecated(since = "0.16.3", note = "renamed to `as_columns_mut`")]
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<4, T, A>; 5] {
        self.as_columns_mut()
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

#[doc(hidden)]
#[repr(C)]
pub struct AffineDeref<const N: usize, T, A: Alignment>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// The part representing rotation, scaling and shear.
    pub matrix: Matrix<N, T, A>,
    /// The part representing translation.
    pub translation: Vector<N, T, A>,
}

impl<const N: usize, T, A: Alignment> Deref for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Target = AffineDeref<N, T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Affine<N, T, A>` is guaranteed to begin with
        // `Matrix<N, T, A>` followed by `Vector<N, T, A>`, and so begin with
        // `AffineDeref<N, T, A>`.
        unsafe { transmute_ref::<Affine<N, T, A>, AffineDeref<N, T, A>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> DerefMut for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Affine<N, T, A>` is guaranteed to begin with
        // `Matrix<N, T, A>` followed by `Vector<N, T, A>`, and so begin with
        // `AffineDeref<N, T, A>`.
        unsafe { transmute_mut::<Affine<N, T, A>, AffineDeref<N, T, A>>(self) }
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
                self.matrix.column(0),
                self.matrix.column(1),
                self.translation
            ),
            3 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}]",
                self.matrix.column(0),
                self.matrix.column(1),
                self.matrix.column(2),
                self.translation
            ),
            4 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}, {:?}]",
                self.matrix.column(0),
                self.matrix.column(1),
                self.matrix.column(2),
                self.matrix.column(3),
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
                self.matrix.column(0),
                self.matrix.column(1),
                self.translation
            ),
            3 => write!(
                f,
                "[{}, {}, {}, {}]",
                self.matrix.column(0),
                self.matrix.column(1),
                self.matrix.column(2),
                self.translation
            ),
            4 => write!(
                f,
                "[{}, {}, {}, {}, {}]",
                self.matrix.column(0),
                self.matrix.column(1),
                self.matrix.column(2),
                self.matrix.column(3),
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
        specialize!(<T as ScalarBackend<N, A>>::affine_eq(self, other))
    }

    #[expect(clippy::partialeq_ne_impl)]
    #[inline]
    fn ne(&self, other: &Self) -> bool {
        specialize!(<T as ScalarBackend<N, A>>::affine_ne(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Eq,
{
}

// SAFETY: Affines are equivalent to values of `T` mixed with padding.
// Because `T` is `Send` and padding is `Send`, the affine is too.
unsafe impl<const N: usize, T, A: Alignment> Send for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Send,
{
}

// SAFETY: Affines are equivalent to values of `T` mixed with padding.
// Because `T` is `Sync` and padding is `Sync`, the affine is too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + RefUnwindSafe,
{
}
