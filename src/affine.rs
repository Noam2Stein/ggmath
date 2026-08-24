use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, Deref, DerefMut, Index, IndexMut, Mul, MulAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, Matrix, One, Scalar, SupportedLength, Unaligned, Vector, Zero,
    backend::AffineBackend,
    utils::{transmute_mut, transmute_ref},
};

// These submodules have empty lines between them so that rustfmt does not
// incorrectly reorder them. The order is important since it impacts the order
// of `impl` blocks in rustdoc's output.
//
// The contents of the `generic` submodule *would* be simply put in this root
// module, but due to a rustdoc bug, that would cause functionality generic over
// `T` to be shown after all submodule functionality.

mod generic;

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
///
/// # Fields
///
/// - `matrix: Matrix<N, T, A>` (linear transformation matrix)
/// - `translation: Vector<N, T, A>` (translation vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Affine<N, T, A>`] contains a value of [`Matrix<N, T, A>`], followed by a
/// value of [`Vector<N, T, A>`], followed by optional padding.
///
/// The alignment of [`Affine<N, T, A>`] is always the alignment of
/// [`Matrix<N, T, A>`].
///
/// For `N = 3, 4` there is no padding. For `N = 2` there may be padding because
/// [`Mat2A`] is represented as [`Vec4A`]. Padding is fully initialized and
/// accepts all bit patterns. Unless `T` accepts all bit patterns, it is not
/// sound to assume padding contains valid values of `T`.
///
/// [`Mat2A`]: crate::Mat2A
/// [`Vec4A`]: crate::Vec4A
#[repr(C)]
pub struct Affine<const N: usize, T, A: Alignment>(
    #[expect(clippy::type_complexity)]
    pub(crate)  <A as Alignment>::Select<
        <Length<N> as SupportedLength>::Select<
            <T as AffineBackend<2, Aligned>>::Inner,
            <T as AffineBackend<3, Aligned>>::Inner,
            <T as AffineBackend<4, Aligned>>::Inner,
        >,
        <Length<N> as SupportedLength>::Select<
            <T as AffineBackend<2, Unaligned>>::Inner,
            <T as AffineBackend<3, Unaligned>>::Inner,
            <T as AffineBackend<4, Unaligned>>::Inner,
        >,
    >,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/// A 2D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Contains a 2x2 matrix and a 2D translation vector.
///
/// # No SIMD alignment
///
/// [`Affine2<T>`] does not have SIMD alignment, for that use [`Affine2A<T>`].
///
/// # Fields
///
/// - `matrix: Mat2<T>` (linear transformation matrix)
/// - `translation: Vec2<T>` (translation vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Affine2<T> = Affine<2, T, Unaligned>;

/// A 3D affine transform which can represent translation, rotation, scaling and
/// shear.
///
/// Contains a 3x3 matrix and a 3D translation vector.
///
/// # No SIMD alignment
///
/// [`Affine3<T>`] does not have SIMD alignment, for that use [`Affine3A<T>`].
///
/// # Fields
///
/// - `matrix: Mat3<T>` (linear transformation matrix)
/// - `translation: Vec3<T>` (translation vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
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
///
/// # Fields
///
/// - `matrix: Mat2A<T>` (linear transformation matrix)
/// - `translation: Vec2A<T>` (translation vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
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
///
/// # Fields
///
/// - `matrix: Mat3A<T>` (linear transformation matrix)
/// - `translation: Vec3A<T>` (translation vector)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
pub type Affine3A<T> = Affine<3, T, Aligned>;

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

#[doc(hidden)]
#[repr(C)]
pub struct AffineFields<const N: usize, T, A: Alignment>
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
    type Target = AffineFields<N, T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Affine` is guaranteed to begin with the corresponding matrix
        // and vector
        unsafe { transmute_ref::<Affine<N, T, A>, AffineFields<N, T, A>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> DerefMut for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Affine` is guaranteed to begin with the corresponding matrix
        // and vector
        unsafe { transmute_mut::<Affine<N, T, A>, AffineFields<N, T, A>>(self) }
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
                    &(self.matrix * rhs.matrix),
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

// SAFETY: The matrix and vector are `Send`, and so is the padding.
unsafe impl<const N: usize, T, A: Alignment> Send for Affine<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Send,
{
}

// SAFETY: The matrix and vector are `Sync`, and so is the padding.
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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Affine, Affine2, Affine2A, Affine3, Affine3A, Aligned, Mask, Mat2A, Matrix, Unaligned,
        Vec3A, Vec4A, Vector,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(size_of::<Affine2<T>>(), size_of::<T>() * 6);
            assert_eq!(align_of::<Affine2<T>>(), align_of::<T>());

            assert_eq!(size_of::<Affine3<T>>(), size_of::<T>() * 12);
            assert_eq!(align_of::<Affine3<T>>(), align_of::<T>());

            assert_eq!(size_of::<Affine<4, T, Unaligned>>(), size_of::<T>() * 20);
            assert_eq!(align_of::<Affine<4, T, Unaligned>>(), align_of::<T>());

            if align_of::<Mat2A<T>>() == size_of::<Mat2A<T>>() {
                assert_eq!(size_of::<Affine2A<T>>(), size_of::<T>() * 8);
            } else {
                assert_eq!(size_of::<Affine2A<T>>(), size_of::<T>() * 6);
            }
            assert_eq!(align_of::<Affine2A<T>>(), align_of::<Mat2A<T>>());

            assert_eq!(size_of::<Affine3A<T>>(), size_of::<Vec3A<T>>() * 4);
            assert_eq!(align_of::<Affine3A<T>>(), align_of::<Vec3A<T>>());

            assert_eq!(size_of::<Affine<4, T, Aligned>>(), size_of::<T>() * 20);
            assert_eq!(align_of::<Affine<4, T, Aligned>>(), align_of::<Vec4A<T>>());
        });
    }

    #[test]
    fn test_zero() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(
                Affine::<N, T, A>::ZERO,
                Affine::from_matrix_translation(&Matrix::ZERO, Vector::ZERO)
            );
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(
                Affine::<N, T, A>::IDENTITY,
                Affine::from_matrix_translation(&Matrix::IDENTITY, Vector::ZERO)
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
                Affine::from_matrix(&Matrix::from_scale(scale))
            );
        });
    }

    #[test]
    fn test_from_translation() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let translation = Vector::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Affine::<N, T, A>::from_translation(translation),
                Affine::from_matrix_translation(&Matrix::IDENTITY, translation)
            );
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix = Matrix::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                Affine::<N, T, A>::from_matrix(&matrix),
                Affine::from_matrix_translation(&matrix, Vector::ZERO)
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
                    &affine.matrix.align(),
                    affine.translation.align()
                )
            );
            assert_eq!(
                affine.to_alignment(),
                Affine::<N, T, Unaligned>::from_matrix_translation(
                    &affine.matrix.unalign(),
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
                    &affine.matrix.align(),
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
                    &affine.matrix.unalign(),
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
                    &Matrix::from_rows(&[rows[0], rows[1]]),
                    rows[2]
                )
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                Affine::<3, T, A>::from_rows(&rows),
                Affine::<3, T, A>::from_matrix_translation(
                    &Matrix::from_rows(&[rows[0], rows[1], rows[2]]),
                    rows[3]
                )
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(
                Affine::<4, T, A>::from_rows(&rows),
                Affine::<4, T, A>::from_matrix_translation(
                    &Matrix::from_rows(&[rows[0], rows[1], rows[2], rows[3]]),
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
            let [x, y, z, w, a, b, c, d, e, f, g, h] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Affine::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w),
                    Vector::<2, T, A>::new(a, b)
                ])
                .to_homogeneous(),
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, y, T::ZERO),
                    Vector::<3, T, A>::new(z, w, T::ZERO),
                    Vector::<3, T, A>::new(a, b, T::ONE)
                ])
            );
            assert_eq!(
                Affine::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e),
                    Vector::<3, T, A>::new(f, g, h)
                ])
                .to_homogeneous(),
                Matrix::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, T::ZERO),
                    Vector::<4, T, A>::new(w, a, b, T::ZERO),
                    Vector::<4, T, A>::new(c, d, e, T::ZERO),
                    Vector::<4, T, A>::new(f, g, h, T::ONE)
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
    fn test_mul_assign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Affine<N, T, A>; 2]>() {
                let mut result = left;
                result *= right;

                assert_test_eq!(result, left * right);
            }
        });
    }
}
