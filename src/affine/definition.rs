use core::{
    fmt::{Debug, Display},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Matrix, Scalar, ScalarBackend, ScalarRepr, SupportedLength,
    Unaligned, Vector,
    affine::deref::AffineDeref,
    constants::{One, Zero},
    utils::{Repr3, Repr4, Repr5, specialize, transmute_generic, transmute_mut, transmute_ref},
};

/// An affine transform, which can represent translation, rotation, scaling and
/// shear.
///
/// `Affine` is the generic form of:
///
/// - [`Affine2<T>`](crate::Affine2)
/// - [`Affine3<T>`](crate::Affine3)
/// - [`Affine2U<T>`](crate::Affine2U)
/// - [`Affine3U<T>`](crate::Affine3U)
///
/// `Affine` is generic over:
///
/// - `N`: Length (2, 3, or 4)
/// - `T`: Scalar type (see [`Scalar`])
/// - `A`: Alignment (see [`Alignment`])
///
/// # Guarantees
///
/// `Affine<N, T, A>` represents `Matrix<N, T, A>` followed by `Vector<N, T, A>`
/// followed by optional padding due to alignment.
///
/// Padding bytes are initialized and accept any bit-pattern. It is **sound** to
/// store any bit-pattern in padding, and it is **unsound** to assume that
/// padding contains valid values of `T` unless `T` accepts all bit-patterns.
///
/// - `Affine<N, T, Unaligned>`: has no padding, has no additional alignment.
///
/// - `Affine<2, T, Aligned>`: may have one padding vector, may have additional
///   alignment.
///
/// - `Affine<3, T, Aligned>`: may have four padding elements if
///   both `Matrix<3, T, Aligned>` and `Vector<3, T, Aligned>` don't have
///   padding, may have additional alignment.
///
/// - `Affine<4, T, Aligned>`: has no padding, has the alignment of
///   `Matrix<4, T, Aligned>`.
///
/// Affines of scalar types with the same [`Scalar::Repr`] are guaranteed to
/// have compatible memory layouts, unless `Repr = ()`. They are guaranteed to
/// have the same size and element positions, but their alignment may differ.
///
/// Types containing compatible `Affine` types are **not guaranteed** to have
/// the same memory layout. For example, even though `Affine2<f32>` and
/// `Affine2<i32>` have the same memory layout, `Option<Affine2<f32>>` and
/// `Option<Affine2<i32>>` may not.
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
    T: Scalar,
{
    /// Creates an affine transform from a matrix, expressing scale, rotation
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

    /// Creates an affine transform from a translation vector and a matrix,
    /// expressing scale, rotation and shear.
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

    /// Converts the affine to the specified alignment.
    ///
    /// See [`Alignment`] for more information.
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

    /// Converts the affine to [`Aligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn align(&self) -> Affine<N, T, Aligned> {
        self.to_alignment()
    }

    /// Converts the affine to [`Unaligned`] alignment.
    ///
    /// See [`Alignment`] for more information.
    #[inline]
    #[must_use]
    pub const fn unalign(&self) -> Affine<N, T, Unaligned> {
        self.to_alignment()
    }
}

impl<T, A: Alignment> Affine<2, T, A>
where
    T: Scalar,
{
    /// Creates a 2D affine transform from three column vectors.
    #[inline]
    #[must_use]
    pub const fn from_cols(
        x_axis: Vector<2, T, A>,
        y_axis: Vector<2, T, A>,
        z_axis: Vector<2, T, A>,
    ) -> Self {
        match size_of::<Affine<2, T, A>>() / size_of::<Vector<2, T, A>>() {
            // SAFETY: `Repr3<Vector<2, T, A>>` is `Matrix<2, T, A>` (two vectors)
            // followed by `Vector<2, T, A>` followed by no padding, because
            // the size has been checked to have no padding.
            3 => unsafe {
                transmute_generic::<Repr3<Vector<2, T, A>>, Affine<2, T, A>>(Repr3(
                    x_axis, y_axis, z_axis,
                ))
            },

            // SAFETY: `Repr4<Vector<2, T, A>>` is `Matrix<2, T, A>` (two vectors)
            // followed by `Vector<2, T, A>` followed by one padding vector,
            // because the size has been checked to have exactly one padding
            // vector.
            4 => unsafe {
                transmute_generic::<Repr4<Vector<2, T, A>>, Affine<2, T, A>>(Repr4(
                    x_axis, y_axis, z_axis, z_axis,
                ))
            },

            _ => unreachable!(),
        }
    }

    /// Creates a 2D affine transform from three column vectors.
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<2, T, A>; 3]) -> Self {
        Self::from_cols(array[0], array[1], array[2])
    }

    /// Converts the affine transform to three column vectors.
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<2, T, A>; 3] {
        *self.as_col_array_ref()
    }

    /// Returns a reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_ref::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }

    /// Returns a mutable reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<2, T, A>; 3] {
        // SAFETY: `Affine<2, T, A>` is guaranteed to begin with
        // `Matrix<2, T, A>` (two vectors) then `Vector<2, T, A>`, which is 3
        // vectors in total.
        unsafe { transmute_mut::<Affine<2, T, A>, [Vector<2, T, A>; 3]>(self) }
    }
}

impl<T, A: Alignment> Affine<3, T, A>
where
    T: Scalar,
{
    /// Creates a 3D affine transform from four column vectors.
    #[inline]
    #[must_use]
    pub const fn from_cols(
        x_axis: Vector<3, T, A>,
        y_axis: Vector<3, T, A>,
        z_axis: Vector<3, T, A>,
        w_axis: Vector<3, T, A>,
    ) -> Self {
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
                    x_axis, y_axis, z_axis, w_axis,
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
                    x_axis,
                    y_axis,
                    z_axis,
                    w_axis,
                    padding: Repr4(
                        x_axis.as_array_ref()[0],
                        x_axis.as_array_ref()[1],
                        x_axis.as_array_ref()[2],
                        x_axis.as_array_ref()[2],
                    ),
                })
            },

            _ => unreachable!(),
        }
    }

    /// Creates a 3D affine transform from four column vectors.
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<3, T, A>; 4]) -> Self {
        Self::from_cols(array[0], array[1], array[2], array[3])
    }

    /// Converts the affine transform to four column vectors.
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<3, T, A>; 4] {
        *self.as_col_array_ref()
    }

    /// Returns a reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_ref::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }

    /// Returns a mutable reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<3, T, A>; 4] {
        // SAFETY: `Affine<3, T, A>` is guaranteed to begin with
        // `Matrix<3, T, A>` (three vectors) then `Vector<3, T, A>`, which is 4
        // vectors in total.
        unsafe { transmute_mut::<Affine<3, T, A>, [Vector<3, T, A>; 4]>(self) }
    }
}

impl<T, A: Alignment> Affine<4, T, A>
where
    T: Scalar,
{
    /// Creates a 4D affine transform from five column vectors.
    #[inline]
    #[must_use]
    pub const fn from_cols(
        x_axis: Vector<4, T, A>,
        y_axis: Vector<4, T, A>,
        z_axis: Vector<4, T, A>,
        w_axis: Vector<4, T, A>,
        fifth_axis: Vector<4, T, A>,
    ) -> Self {
        // SAFETY: `Repr5<Vector<4, T, A>>` is `Matrix<4, T, A>` (four vectors)
        // followed by `Vector<4, T, A>` followed by no padding, because
        // `Affine<4, T, Aligned>` has no padding.
        unsafe {
            transmute_generic::<Repr5<Vector<4, T, A>>, Affine<4, T, A>>(Repr5(
                x_axis, y_axis, z_axis, w_axis, fifth_axis,
            ))
        }
    }

    /// Creates a 4D affine transform from five column vectors.
    #[inline]
    #[must_use]
    pub const fn from_col_array(array: &[Vector<4, T, A>; 5]) -> Self {
        Self::from_cols(array[0], array[1], array[2], array[3], array[4])
    }

    /// Converts the affine transform to five column vectors.
    #[inline]
    #[must_use]
    pub const fn to_col_array(&self) -> [Vector<4, T, A>; 5] {
        *self.as_col_array_ref()
    }

    /// Returns a reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_ref(&self) -> &[Vector<4, T, A>; 5] {
        // SAFETY: `Affine<4, T, A>` is guaranteed to begin with
        // `Matrix<4, T, A>` (four vectors) then `Vector<4, T, A>`, which is 5
        // vectors in total.
        unsafe { transmute_ref::<Affine<4, T, A>, [Vector<4, T, A>; 5]>(self) }
    }

    /// Returns a mutable reference to the affine transform's columns.
    #[inline]
    #[must_use]
    pub const fn as_col_array_mut(&mut self) -> &mut [Vector<4, T, A>; 5] {
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
                self.matrix.col(0),
                self.matrix.col(1),
                self.translation
            ),
            3 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}]",
                self.matrix.col(0),
                self.matrix.col(1),
                self.matrix.col(2),
                self.translation
            ),
            4 => write!(
                f,
                "[{:?}, {:?}, {:?}, {:?}, {:?}]",
                self.matrix.col(0),
                self.matrix.col(1),
                self.matrix.col(2),
                self.matrix.col(3),
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
                self.matrix.col(0),
                self.matrix.col(1),
                self.translation
            ),
            3 => write!(
                f,
                "[{}, {}, {}, {}]",
                self.matrix.col(0),
                self.matrix.col(1),
                self.matrix.col(2),
                self.translation
            ),
            4 => write!(
                f,
                "[{}, {}, {}, {}, {}]",
                self.matrix.col(0),
                self.matrix.col(1),
                self.matrix.col(2),
                self.matrix.col(3),
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
