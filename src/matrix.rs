use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{
        Add, AddAssign, Deref, DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub,
        SubAssign,
    },
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Aligned, Alignment, Length, One, Scalar, SupportedLength, Unaligned, Vector, Zero,
    utils::{Repr3, Repr4, transmute_mut, transmute_ref},
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

/// An `N`x`N` row-major matrix of type `T`.
///
/// `A` controls SIMD alignment and is either [`Unaligned`] or [`Aligned`]. See
/// [`Alignment`] for more details.
///
/// This represents an `N`-dimensional linear transformation, applied using
/// `vector_n * self`.
///
/// If you need translation, use [`Affine`]. If you need projections, use
/// [`Projective`].
///
/// # Type aliases
///
/// - [`Mat2<T>`] for [`Matrix<2, T, Unaligned>`].
/// - [`Mat3<T>`] for [`Matrix<3, T, Unaligned>`].
/// - [`Mat4<T>`] for [`Matrix<4, T, Unaligned>`].
/// - [`Mat2A<T>`] for [`Matrix<2, T, Aligned>`].
/// - [`Mat3A<T>`] for [`Matrix<3, T, Aligned>`].
/// - [`Mat4A<T>`] for [`Matrix<4, T, Aligned>`].
///
/// # Fields
///
/// - `x_axis: Vector<N, T, N>` (the first row of the matrix, represents the
///   result of `+X * matrix`, exists for lengths `2`, `3`, `4`)
///
/// - `y_axis: Vector<N, T, N>` (the second row of the matrix, represents the
///   result of `+Y * matrix`, exists for lengths `2`, `3`, `4`)
///
/// - `z_axis: Vector<N, T, N>` (the third row of the matrix, represents the
///   result of `+Z * matrix`, exists for lengths `3`, `4`)
///
/// - `w_axis: Vector<N, T, N>` (the fourth row of the matrix, represents the
///   result of `+W * matrix`, exists for length `4`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Matrix<N, T, A>`] contains `N` consecutive values of [`Vector<N, T, A>`]
/// with no additional padding.
///
/// For `N = 2` this type has the size and alignment of [`Vector<4, T, A>`].
///
/// For `N = 3` and `N = 4` this type has the size and alignment of
/// `[Vector<N, T, A>; N]`.
///
/// [`Affine`]: crate::Affine
/// [`Projective`]: crate::Projective
#[repr(transparent)]
pub struct Matrix<const N: usize, T, A: Alignment>(
    #[expect(clippy::type_complexity)]
    <Length<N> as SupportedLength>::Select<
        Vector<4, T, A>,
        Repr3<Vector<3, T, A>>,
        Repr4<Vector<4, T, A>>,
    >,
)
where
    Length<N>: SupportedLength,
    T: Scalar;

/// A 2x2 row-major matrix.
///
/// This represents a 2D linear transformation, applied using `vec2 * self`.
///
/// If you need translation, use [`Affine2`]. If you need 2D projections, use
/// [`Proj2`].
///
/// # No SIMD alignment
///
/// [`Mat2<T>`] does not have SIMD alignment, for that use [`Mat2A<T>`].
///
/// # Fields
///
/// - `x_axis: Vec2<T>` (the first row of the matrix, represents the result of
///   `(1, 0) * self`)
///
/// - `y_axis: Vec2<T>` (the second row of the matrix, represents the result of
///   `(0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine2`]: crate::Affine2
/// [`Proj2`]: crate::Proj2
pub type Mat2<T> = Matrix<2, T, Unaligned>;

/// A 3x3 row-major matrix.
///
/// This represents a 3D linear transformation, applied using `vec3 * self`.
///
/// If you need translation, use [`Affine3`]. If you need projections, use
/// [`Proj3`].
///
/// Unlike many other libraries, here [`Mat3`] is not used for 2D affine and
/// projective transformations. For that use the [`Affine2`] and [`Proj2`]
/// types.
///
/// # No SIMD alignment
///
/// [`Mat3<T>`] does not have SIMD alignment, for that use [`Mat3A<T>`].
///
/// # Fields
///
/// - `x_axis: Vec3<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0) * self`)
///
/// - `y_axis: Vec3<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0) * self`)
///
/// - `z_axis: Vec3<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine3`]: crate::Affine3
/// [`Proj3`]: crate::Proj3
/// [`Affine2`]: crate::Affine2
/// [`Proj2`]: crate::Proj2
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat3<T> = Matrix<3, T, Unaligned>;

/// A 4x4 row-major matrix.
///
/// Unlike many other libraries, here [`Mat4`] is not used for 3D affine and
/// projective transformations. For that use the [`Affine3`] and [`Proj3`]
/// types.
///
/// This represents a 4D linear transformation, applied using `vec4 * self`.
/// Even though this type does not have many use cases, it is still useful for
/// raw matrix operations and interop with other libraries.
///
/// # No SIMD alignment
///
/// [`Mat4<T>`] does not have SIMD alignment, for that use [`Mat4A<T>`].
///
/// # Fields
///
/// - `x_axis: Vec4<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0, 0) * self`)
///
/// - `y_axis: Vec4<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0, 0) * self`)
///
/// - `z_axis: Vec4<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1, 0) * self`)
///
/// - `w_axis: Vec4<T>` (the fourth row of the matrix, represents the result of
///   `(0, 0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine3`]: crate::Affine3
/// [`Proj3`]: crate::Proj3
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat4<T> = Matrix<4, T, Unaligned>;

/// A 2x2 row-major matrix.
///
/// This represents a 2D linear transformation, applied using `vec2 * self`.
///
/// If you need translation, use [`Affine2A`]. If you need 2D projections, use
/// [`Proj2A`].
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Mat2A<T>`] has SIMD alignment. For no SIMD use
/// [`Mat2<T>`].
///
/// # Fields
///
/// - `x_axis: Vec2A<T>` (the first row of the matrix, represents the result of
///   `(1, 0) * self`)
///
/// - `y_axis: Vec2A<T>` (the second row of the matrix, represents the result of
///   `(0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine2A`]: crate::Affine2A
/// [`Proj2A`]: crate::Proj2A
pub type Mat2A<T> = Matrix<2, T, Aligned>;

/// A 3x3 row-major matrix.
///
/// This represents a 3D linear transformation, applied using `vec3 * self`.
///
/// If you need translation, use [`Affine3A`]. If you need projections, use
/// [`Proj3A`].
///
/// Unlike many other libraries, here [`Mat3A`] is not used for 2D affine and
/// projective transformations. For that use the [`Affine2A`] and [`Proj2A`]
/// types.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Mat3A<T>`] has SIMD alignment. For no SIMD use
/// [`Mat3<T>`].
///
/// # Fields
///
/// - `x_axis: Vec3A<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0) * self`)
///
/// - `y_axis: Vec3A<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0) * self`)
///
/// - `z_axis: Vec3A<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine3A`]: crate::Affine3A
/// [`Proj3A`]: crate::Proj3A
/// [`Affine2A`]: crate::Affine2A
/// [`Proj2A`]: crate::Proj2A
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat3A<T> = Matrix<3, T, Aligned>;

/// A 4x4 row-major matrix.
///
/// Unlike many other libraries, here [`Mat4A`] is not used for 3D affine and
/// projective transformations. For that use the [`Affine3A`] and [`Proj3A`]
/// types.
///
/// This represents a 4D linear transformation, applied using `vec4 * self`.
/// Even though this type does not have many use cases, it is still useful for
/// raw matrix operations and interop with other libraries.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Mat4A<T>`] has SIMD alignment. For no SIMD use
/// [`Mat4<T>`].
///
/// # Fields
///
/// - `x_axis: Vec4A<T>` (the first row of the matrix, represents the result of
///   `(1, 0, 0, 0) * self`)
///
/// - `y_axis: Vec4A<T>` (the second row of the matrix, represents the result of
///   `(0, 1, 0, 0) * self`)
///
/// - `z_axis: Vec4A<T>` (the third row of the matrix, represents the result of
///   `(0, 0, 1, 0) * self`)
///
/// - `w_axis: Vec4A<T>` (the fourth row of the matrix, represents the result of
///   `(0, 0, 0, 1) * self`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// [`Affine3A`]: crate::Affine3A
/// [`Proj3A`]: crate::Proj3A
/// [benchmark results]: https://github.com/Noam2Stein/ggmath/blob/main/BENCH_RESULTS.md
pub type Mat4A<T> = Matrix<4, T, Aligned>;

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

impl<const N: usize, T, A: Alignment> Index<usize> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    type Output = Vector<N, T, A>;

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to the dimension of the matrix.
    #[inline]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_rows()[index]
    }
}

impl<const N: usize, T, A: Alignment> IndexMut<usize> for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar,
{
    /// Returns a mutable reference to the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to the dimension of the matrix.
    #[inline]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_rows()[index]
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat2Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first row of the matrix.
    ///
    /// This represents the result of multiplying `(1, 0)` by the matrix.
    pub x_axis: Vector<2, T, A>,
    /// The second row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 1)` by the matrix.
    pub y_axis: Vector<2, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<2, T, A>
where
    T: Scalar,
{
    type Target = Mat2Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `Vector<2, T, A>`, and so begin with `Xy<T, A>`.
        unsafe { transmute_ref::<Matrix<2, T, A>, Mat2Fields<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<2, T, A>` is guaranteed to begin with 2 consecutive
        // values of `Vector<2, T, A>`, and so begin with `Xy<T, A>`.
        unsafe { transmute_mut::<Matrix<2, T, A>, Mat2Fields<T, A>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat3Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first row of the matrix.
    ///
    /// This represents the result of multiplying `(1, 0, 0)` by the matrix.
    pub x_axis: Vector<3, T, A>,
    /// The second row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 1, 0)` by the matrix.
    pub y_axis: Vector<3, T, A>,
    /// The third row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 0, 1)` by the matrix.
    pub z_axis: Vector<3, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<3, T, A>
where
    T: Scalar,
{
    type Target = Mat3Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`, and so begin with `Xyz<T, A>`.
        unsafe { transmute_ref::<Matrix<3, T, A>, Mat3Fields<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`, and so begin with `Xyz<T, A>`.
        unsafe { transmute_mut::<Matrix<3, T, A>, Mat3Fields<T, A>>(self) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Mat4Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first row of the matrix.
    ///
    /// This represents the result of multiplying `(1, 0, 0, 0)` by the matrix.
    pub x_axis: Vector<4, T, A>,
    /// The second row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 1, 0, 0)` by the matrix.
    pub y_axis: Vector<4, T, A>,
    /// The third row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 0, 1, 0)` by the matrix.
    pub z_axis: Vector<4, T, A>,
    /// The fourth row of the matrix.
    ///
    /// This represents the result of multiplying `(0, 0, 0, 1)` by the matrix.
    pub w_axis: Vector<4, T, A>,
}

impl<T, A: Alignment> Deref for Matrix<4, T, A>
where
    T: Scalar,
{
    type Target = Mat4Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`, and so begin with `Xyzw<T, A>`.
        unsafe { transmute_ref::<Matrix<4, T, A>, Mat4Fields<T, A>>(self) }
    }
}

impl<T, A: Alignment> DerefMut for Matrix<4, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`, and so begin with `Xyzw<T, A>`.
        unsafe { transmute_mut::<Matrix<4, T, A>, Mat4Fields<T, A>>(self) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:?}", self.as_rows())
    }
}

impl<const N: usize, T, A: Alignment> Display for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match N {
            2 => write!(f, "[{}, {}]", self[0], self[1]),
            3 => write!(f, "[{}, {}, {}]", self[0], self[1], self[2]),
            4 => write!(f, "[{}, {}, {}, {}]", self[0], self[1], self[2], self[3]),
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
        (0..N).all(|i| self[i] == other[i])
    }
}

impl<const N: usize, T, A: Alignment> Eq for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
    T: Scalar + Hash,
{
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.as_rows().hash(state);
    }
}

impl<const N: usize, T, A: Alignment> Default for Matrix<N, T, A>
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

macro_rules! impl_neg {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Neg for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                Matrix::from_row_fn(|i| -self[i])
            }
        }
    };
}
impl_neg!(
    /// Performs the unary `-` operation for each element.
    ///
    /// Equivalent to `[-self.x_axis, -self.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including integer panics.
);

macro_rules! impl_add {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Add for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                Matrix::from_row_fn(|i| self[i] + rhs[i])
            }
        }
    };
}
impl_add!(
    /// Performs the `+` operation for each element.
    ///
    /// Equivalent to
    /// `[self.x_axis + rhs.x_axis, self.y_axis + rhs.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_add_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> AddAssign for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = &*self + rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: &Self) {
                *self = &*self + rhs;
            }
        }
    };
}
impl_add_assign!(
    /// Performs the `+=` operation for each element.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis += rhs.x_axis;
    /// self.y_axis += rhs.y_axis;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix + matrix`.
);

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
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

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                Matrix::from_row_fn(|i| self[i] - rhs[i])
            }
        }
    };
}
impl_sub!(
    /// Performs the `-` operation for each element.
    ///
    /// Equivalent to
    /// `[self.x_axis - rhs.x_axis, self.y_axis - rhs.y_axis, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_sub_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> SubAssign for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = &*self - rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: &Self) {
                *self = &*self - rhs;
            }
        }
    };
}
impl_sub_assign!(
    /// Performs the `-=` operation for each element.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis -= rhs.x_axis;
    /// self.y_axis -= rhs.y_axis;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix - matrix`.
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                &self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                Matrix::from_row_fn(|i| self[i] * rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                self * *rhs
            }
        }
    };
}
impl_mul_scalar!(
    /// Matrix-scalar multiplication.
    ///
    /// Equivalent to `[self.x_axis * rhs, self.y_axis * rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Matrix<N, T, A>
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

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Matrix<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<N, T, A>) -> Self::Output {
                self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                Matrix::from_row_fn(|i| self[i] * rhs)
            }
        }
    };
}
impl_mul!(
    /// Matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, matrix multiplication first
    /// applies the left-hand side matrix, then the right-hand side matrix.
    ///
    /// Equivalent to `[self.x_axis * rhs, self.y_axis * rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<N, T, A>) -> Self::Output {
                self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                match N {
                    2 => rhs[0] * self[0] + rhs[1] * self[1],
                    3 => rhs[0] * self[0] + rhs[1] * self[1] + rhs[2] * self[2],
                    4 => rhs[0] * self[0] + rhs[1] * self[1] + rhs[2] * self[2] + rhs[3] * self[3],
                    _ => unreachable!(),
                }
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Matrix<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Matrix<N, T, A>) -> Self::Output {
                *self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Matrix<N, T, A>> for &Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Matrix<N, T, A>) -> Self::Output {
                *self * rhs
            }
        }
    };
}
impl_vector_mul!(
    /// Vector-matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, they always go on the
    /// left-hand side.
    ///
    /// Equivalent to `self.x * rhs.x_axis + self.y * rhs.y_axis + ...`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul_assign_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = &*self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = &*self * *rhs
            }
        }
    };
}
impl_mul_assign_scalar!(
    /// Matrix-scalar multiplication.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis *= rhs;
    /// self.y_axis *= rhs;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix * scalar`.
);

macro_rules! impl_mul_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = &*self * &rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Matrix<N, T, A>> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Matrix<N, T, A>) {
                *self = &*self * rhs;
            }
        }
    };
}
impl_mul_assign!(
    /// Matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, matrix multiplication first
    /// applies the left-hand side matrix, then the right-hand side matrix.
    ///
    /// Equivalent to `self = [self.x_axis * rhs, self.y_axis * rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix * matrix`.
);

macro_rules! impl_vector_mul_assign {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> MulAssign<Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Matrix<N, T, A>) {
                *self = *self * &rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Matrix<N, T, A>> for Vector<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Matrix<N, T, A>) {
                *self = *self * rhs;
            }
        }
    };
}
impl_vector_mul_assign!(
    /// Vector-matrix multiplication.
    ///
    /// Because vectors are treated as row matrices, they always go on the
    /// left-hand side.
    ///
    /// Equivalent to `self.x * rhs.x_axis + self.y * rhs.y_axis + ...`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_div_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Div<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                &self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                &self / *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Div<T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: T) -> Self::Output {
                Matrix::from_row_fn(|i| self[i] / rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Div<&T> for &Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            type Output = Matrix<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div(self, rhs: &T) -> Self::Output {
                self / *rhs
            }
        }
    };
}
impl_div_scalar!(
    /// Matrix-scalar division.
    ///
    /// Equivalent to `[self.x_axis / rhs, self.y_axis / rhs, ...]`.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
);

macro_rules! impl_div_assign_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> DivAssign<T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: T) {
                *self = &*self / rhs
            }
        }

        impl<const N: usize, T, A: Alignment> DivAssign<&T> for Matrix<N, T, A>
        where
            Length<N>: SupportedLength,
            T: Scalar + Div<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn div_assign(&mut self, rhs: &T) {
                *self = &*self / *rhs
            }
        }
    };
}
impl_div_assign_scalar!(
    /// Matrix-scalar division.
    ///
    /// Equivalent to:
    ///
    /// ```ignore
    /// self.x_axis /= rhs;
    /// self.y_axis /= rhs;
    /// ...
    /// ```
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is fully consistent with the scalar
    /// operation, including floating-point precision and integer panics.
    ///
    /// This operation is fully consistent with `matrix / scalar`.
);

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

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Aligned, Mask, Mat2A, Mat3A, Mat4A, Matrix, Unaligned, Vec3A, Vec4A, Vector,
        test_utils::{assert_panic, assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_layout() {
        for_types!(|N, T: PrimitiveNumber| {
            assert_eq!(size_of::<Matrix<N, T, Unaligned>>(), size_of::<T>() * N * N);
            assert_eq!(align_of::<Matrix<N, T, Unaligned>>(), align_of::<T>());

            assert_eq!(
                size_of::<Matrix<N, T, Aligned>>(),
                size_of::<Vector<N, T, Aligned>>() * N
            );
        });
        for_types!(|T: PrimitiveNumber| {
            assert_eq!(align_of::<Mat2A<T>>(), align_of::<Vec4A<T>>());
            assert_eq!(align_of::<Mat3A<T>>(), align_of::<Vec3A<T>>());
            assert_eq!(align_of::<Mat4A<T>>(), align_of::<Vec4A<T>>());
        });
    }

    #[test]
    fn test_zero() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(
                Matrix::<N, T, A>::ZERO,
                Matrix::from_rows(&[Vector::ZERO; N])
            );
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                Matrix::<2, T, A>::IDENTITY,
                Matrix::from_rows(&[
                    Vector::<2, T, A>::new(T::as_from(1), T::as_from(0)),
                    Vector::<2, T, A>::new(T::as_from(0), T::as_from(1))
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::IDENTITY,
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(T::as_from(1), T::as_from(0), T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), T::as_from(1), T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(1))
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::IDENTITY,
                Matrix::from_rows(&[
                    Vector::<4, T, A>::new(
                        T::as_from(1),
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(0)
                    ),
                    Vector::<4, T, A>::new(
                        T::as_from(0),
                        T::as_from(1),
                        T::as_from(0),
                        T::as_from(0)
                    ),
                    Vector::<4, T, A>::new(
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(1),
                        T::as_from(0)
                    ),
                    Vector::<4, T, A>::new(
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(0),
                        T::as_from(1)
                    )
                ])
            );
        });
    }

    #[test]
    fn test_from_row_fn() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                Matrix::<N, T, A>::from_row_fn(|i| rows[i]),
                Matrix::from_rows(&rows)
            );
        });
    }

    #[test]
    fn test_from_diagonal() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Matrix::<2, T, A>::from_diagonal(Vector::<2, T, A>::new(x, y)),
                Matrix::from_rows(&[
                    Vector::<2, T, A>::new(x, T::as_from(0)),
                    Vector::<2, T, A>::new(T::as_from(0), y)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_diagonal(Vector::<3, T, A>::new(x, y, z)),
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, T::as_from(0), T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), y, T::as_from(0)),
                    Vector::<3, T, A>::new(T::as_from(0), T::as_from(0), z)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_diagonal(Vector::<4, T, A>::new(x, y, z, w)),
                Matrix::from_rows(&[
                    Vector::<4, T, A>::new(x, T::as_from(0), T::as_from(0), T::as_from(0)),
                    Vector::<4, T, A>::new(T::as_from(0), y, T::as_from(0), T::as_from(0)),
                    Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), z, T::as_from(0)),
                    Vector::<4, T, A>::new(T::as_from(0), T::as_from(0), T::as_from(0), w)
                ])
            );
        });
    }

    #[test]
    fn test_from_scale() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let scale = Vector::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Matrix::<N, T, A>::from_scale(scale),
                Matrix::<N, T, A>::from_diagonal(scale)
            );
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                matrix.to_alignment(),
                Matrix::<N, T, Aligned>::from_rows(&matrix.as_rows().map(Vector::align))
            );
            assert_eq!(
                matrix.to_alignment(),
                Matrix::<N, T, Unaligned>::from_rows(&matrix.as_rows().map(Vector::unalign))
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                matrix.align(),
                Matrix::<N, T, Aligned>::from_rows(&matrix.as_rows().map(Vector::align))
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                matrix.unalign(),
                Matrix::<N, T, Unaligned>::from_rows(&matrix.as_rows().map(Vector::unalign))
            );
        });
    }

    #[test]
    fn test_as_rows() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(Matrix::<N, T, A>::from_rows(&rows).as_rows(), &rows);
        });
    }

    #[test]
    fn test_as_mut_rows() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let mut rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(Matrix::<N, T, A>::from_rows(&rows).as_mut_rows(), &mut rows);
        });
    }

    #[test]
    fn test_column() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            let matrix = Matrix::<2, T, A>::from_rows(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(matrix.column(0), Vector::<2, T, A>::new(x, z));
            assert_eq!(matrix.column(1), Vector::<2, T, A>::new(y, w));
            assert_panic!(matrix.column(2));

            let matrix = Matrix::<3, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(matrix.column(0), Vector::<3, T, A>::new(x, w, c));
            assert_eq!(matrix.column(1), Vector::<3, T, A>::new(y, a, d));
            assert_eq!(matrix.column(2), Vector::<3, T, A>::new(z, b, e));
            assert_panic!(matrix.column(3));

            let matrix = Matrix::<4, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(matrix.column(0), Vector::<4, T, A>::new(x, a, e, i));
            assert_eq!(matrix.column(1), Vector::<4, T, A>::new(y, b, f, j));
            assert_eq!(matrix.column(2), Vector::<4, T, A>::new(z, c, g, k));
            assert_eq!(matrix.column(3), Vector::<4, T, A>::new(w, d, h, l));
            assert_panic!(matrix.column(4));
        });
    }

    #[test]
    fn test_set_column() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            let mut matrix = Matrix::<2, T, A>::from_rows(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            matrix.set_column(0, Vector::<2, T, A>::new(a, b));
            assert_eq!(
                matrix,
                Matrix::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(a, y),
                    Vector::<2, T, A>::new(b, w)
                ])
            );
            matrix.set_column(1, Vector::<2, T, A>::new(c, d));
            assert_eq!(
                matrix,
                Matrix::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(a, c),
                    Vector::<2, T, A>::new(b, d)
                ])
            );
            assert_panic!(matrix.clone().set_column(2, Vector::ZERO));

            let mut matrix = Matrix::<3, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            matrix.set_column(0, Vector::<3, T, A>::new(a, b, d));
            assert_eq!(
                matrix,
                Matrix::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(a, y, z),
                    Vector::<3, T, A>::new(b, a, b),
                    Vector::<3, T, A>::new(d, d, e)
                ])
            );
            matrix.set_column(1, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(
                matrix,
                Matrix::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(a, x, z),
                    Vector::<3, T, A>::new(b, y, b),
                    Vector::<3, T, A>::new(d, z, e)
                ])
            );
            matrix.set_column(2, Vector::<3, T, A>::new(e, f, g));
            assert_eq!(
                matrix,
                Matrix::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(a, x, e),
                    Vector::<3, T, A>::new(b, y, f),
                    Vector::<3, T, A>::new(d, z, g)
                ])
            );
            assert_panic!(matrix.clone().set_column(3, Vector::ZERO));

            let mut matrix = Matrix::<4, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            matrix.set_column(0, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(a, y, z, w),
                    Vector::<4, T, A>::new(b, b, c, d),
                    Vector::<4, T, A>::new(c, f, g, h),
                    Vector::<4, T, A>::new(d, j, k, l)
                ])
            );
            matrix.set_column(1, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(a, x, z, w),
                    Vector::<4, T, A>::new(b, y, c, d),
                    Vector::<4, T, A>::new(c, z, g, h),
                    Vector::<4, T, A>::new(d, w, k, l)
                ])
            );
            matrix.set_column(2, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(a, x, a, w),
                    Vector::<4, T, A>::new(b, y, b, d),
                    Vector::<4, T, A>::new(c, z, c, h),
                    Vector::<4, T, A>::new(d, w, d, l)
                ])
            );
            matrix.set_column(3, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(
                matrix,
                Matrix::<4, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(a, x, a, e),
                    Vector::<4, T, A>::new(b, y, b, f),
                    Vector::<4, T, A>::new(c, z, c, g),
                    Vector::<4, T, A>::new(d, w, d, h)
                ])
            );
            assert_panic!(matrix.clone().set_column(4, Vector::ZERO));
        });
    }

    #[test]
    fn test_transpose() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(
                matrix.transpose(),
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(c * N + r)))
            );
        });
    }

    #[test]
    fn test_transpose_mul_vector() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (matrix, vector) in random_iter::<(Matrix<N, T, A>, Vector<N, T, A>)>() {
                assert_test_eq!(
                    matrix.transpose_mul_vector(vector),
                    vector * matrix.transpose(),
                    abs <= (vector * matrix.transpose()).abs() * 1e-6 + 1e-5
                );
            }
        });
    }

    #[test]
    fn test_diagonal() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            assert_eq!(matrix.diagonal(), Vector::from_fn(|i| matrix[i][i]));
        });
    }

    #[test]
    fn test_prepend_scale() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (scale, matrix) in random_iter::<(Vector<N, T, A>, Matrix<N, T, A>)>() {
                if !scale.is_finite() || !matrix.is_finite() {
                    continue;
                }

                assert_test_eq!(
                    matrix.prepend_scale(scale),
                    Matrix::from_scale(scale) * matrix,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_determinant() {
        for_types!(|T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<2, T, A>>() {
                assert_test_eq!(
                    matrix.determinant(),
                    matrix[0][0] * matrix[1][1] - matrix[0][1] * matrix[1][0]
                );
            }

            for matrix in random_iter::<Matrix<3, T, A>>() {
                if !matrix.is_finite() || matrix.as_rows().iter().flatten().any(|x| x.abs() > 1e10)
                {
                    continue;
                }

                assert_test_eq!(
                    matrix.determinant(),
                    matrix[0][0] * matrix.remove(0, 0).determinant()
                        - matrix[0][1] * matrix.remove(0, 1).determinant()
                        + matrix[0][2] * matrix.remove(0, 2).determinant(),
                    abs <= (matrix.as_rows().iter().map(|v| v.length()).product::<T>() * 1e-6)
                        .max(1e-6),
                    0.0 = -0.0
                );
            }

            for matrix in random_iter::<Matrix<4, T, A>>() {
                assert_test_eq!(
                    matrix.determinant(),
                    matrix[0][0] * matrix.remove(0, 0).determinant()
                        - matrix[0][1] * matrix.remove(0, 1).determinant()
                        + matrix[0][2] * matrix.remove(0, 2).determinant()
                        - matrix[0][3] * matrix.remove(0, 3).determinant(),
                    abs <= (matrix.as_rows().iter().map(|v| v.length()).product::<T>() * 1e-6)
                        .max(1e-6),
                    0.0 = -0.0,
                    INFINITY = NAN
                );
            }
        });
    }

    #[test]
    fn test_from_row_array() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Matrix::<2, T, A>::from_row_array(&[x, y, z, w]),
                Matrix::from_rows(&[Vector::<2, T, A>::new(x, y), Vector::<2, T, A>::new(z, w)])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_row_array(&[x, y, z, w, a, b, c, d, e]),
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
            );
            assert_eq!(
                Matrix::<4, T, A>::from_row_array(&[
                    x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l
                ]),
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
    fn test_to_homogeneous() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e] = std::array::from_fn(|i| T::as_from(i + 1));

            assert_eq!(
                Matrix::<2, T, A>::from_rows(&[
                    Vector::<2, T, A>::new(x, y),
                    Vector::<2, T, A>::new(z, w)
                ])
                .to_homogeneous(),
                Matrix::from_rows(&[
                    Vector::<3, T, A>::new(x, y, T::ZERO),
                    Vector::<3, T, A>::new(z, w, T::ZERO),
                    Vector::<3, T, A>::new(T::ZERO, T::ZERO, T::ONE)
                ])
            );
            assert_eq!(
                Matrix::<3, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(x, y, z),
                    Vector::<3, T, A>::new(w, a, b),
                    Vector::<3, T, A>::new(c, d, e)
                ])
                .to_homogeneous(),
                Matrix::from_rows(&[
                    Vector::<4, T, A>::new(x, y, z, T::ZERO),
                    Vector::<4, T, A>::new(w, a, b, T::ZERO),
                    Vector::<4, T, A>::new(c, d, e, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
                ])
            );
        });
    }

    #[test]
    fn test_remove() {
        let matrix = Mat3A::from_rows(&[
            Vec3A::new(1, 2, 3),
            Vec3A::new(4, 5, 6),
            Vec3A::new(7, 8, 9),
        ]);

        assert_panic!(matrix.remove(1, 3));
        assert_panic!(matrix.remove(3, 1));

        for row in 0..3 {
            for column in 0..3 {
                let rows = match row {
                    0 => [matrix[1], matrix[2]],
                    1 => [matrix[0], matrix[2]],
                    2 => [matrix[0], matrix[1]],
                    _ => unreachable!(),
                };
                let rows = match column {
                    0 => rows.map(|c| c.yz()),
                    1 => rows.map(|c| c.xz()),
                    2 => rows.map(|c| c.xy()),
                    _ => unreachable!(),
                };

                assert_eq!(matrix.remove(row, column), Mat2A::from_rows(&rows));
            }
        }

        let matrix = Mat4A::from_rows(&[
            Vec4A::new(1, 2, 3, 4),
            Vec4A::new(5, 6, 7, 8),
            Vec4A::new(9, 10, 11, 12),
            Vec4A::new(13, 14, 15, 16),
        ]);

        assert_panic!(matrix.remove(1, 4));
        assert_panic!(matrix.remove(4, 1));

        for row in 0..4 {
            for column in 0..4 {
                let rows = match row {
                    0 => [matrix[1], matrix[2], matrix[3]],
                    1 => [matrix[0], matrix[2], matrix[3]],
                    2 => [matrix[0], matrix[1], matrix[3]],
                    3 => [matrix[0], matrix[1], matrix[2]],
                    _ => unreachable!(),
                };
                let rows = match column {
                    0 => rows.map(|c| c.yzw()),
                    1 => rows.map(|c| c.xzw()),
                    2 => rows.map(|c| c.xyw()),
                    3 => rows.map(|c| c.xyz()),
                    _ => unreachable!(),
                };

                assert_eq!(matrix.remove(row, column), Mat3A::from_rows(&rows));
            }
        }
    }

    #[test]
    fn test_index() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            for i in 0..N {
                assert_eq!(matrix[i], matrix.as_rows()[i]);
            }
            assert_panic!(matrix[N]);
            assert_panic!(matrix[N + 1]);
        });
    }

    #[test]
    #[expect(clippy::clone_on_copy)]
    fn test_index_mut() {
        for_types!(|N, T: PrimitiveNumber, A| {
            let mut matrix =
                Matrix::<N, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * N + c)));

            for i in 0..N {
                assert_eq!(&mut matrix.clone()[i], &mut matrix.as_mut_rows()[i]);
            }
            assert_panic!(matrix[N]);
            assert_panic!(matrix[N + 1]);
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            let matrix = Matrix::<2, T, A>::from_rows(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(matrix.x_axis, Vector::<2, T, A>::new(x, y));
            assert_eq!(matrix.y_axis, Vector::<2, T, A>::new(z, w));

            let matrix = Matrix::<3, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(matrix.x_axis, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(matrix.y_axis, Vector::<3, T, A>::new(w, a, b));
            assert_eq!(matrix.z_axis, Vector::<3, T, A>::new(c, d, e));

            let matrix = Matrix::<4, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(matrix.x_axis, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(matrix.y_axis, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(matrix.z_axis, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(matrix.w_axis, Vector::<4, T, A>::new(i, j, k, l));
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            let mut matrix = Matrix::<2, T, A>::from_rows(&[
                Vector::<2, T, A>::new(x, y),
                Vector::<2, T, A>::new(z, w),
            ]);
            assert_eq!(&mut matrix.x_axis, &mut Vector::<2, T, A>::new(x, y));
            assert_eq!(&mut matrix.y_axis, &mut Vector::<2, T, A>::new(z, w));

            let mut matrix = Matrix::<3, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(&mut matrix.x_axis, &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(&mut matrix.y_axis, &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(&mut matrix.z_axis, &mut Vector::<3, T, A>::new(c, d, e));

            let mut matrix = Matrix::<4, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(&mut matrix.x_axis, &mut Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(&mut matrix.y_axis, &mut Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(&mut matrix.z_axis, &mut Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(&mut matrix.w_axis, &mut Vector::<4, T, A>::new(i, j, k, l));
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            let [x_axis, y_axis] = rows;
            assert_eq!(
                format!("{:?}", Matrix::<2, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            let [x_axis, y_axis, z_axis] = rows;
            assert_eq!(
                format!("{:?}", Matrix::<3, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {z_axis:?}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            let [x_axis, y_axis, z_axis, w_axis] = rows;
            assert_eq!(
                format!("{:?}", Matrix::<4, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {z_axis:?}, {w_axis:?}]")
            );
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            let [x_axis, y_axis] = rows;
            assert_eq!(
                format!("{}", Matrix::<2, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            let [x_axis, y_axis, z_axis] = rows;
            assert_eq!(
                format!("{}", Matrix::<3, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {z_axis}]")
            );

            let rows = std::array::from_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            let [x_axis, y_axis, z_axis, w_axis] = rows;
            assert_eq!(
                format!("{}", Matrix::<4, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {z_axis}, {w_axis}]")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for ([matrix, other], mask) in
                random_iter::<([Matrix<N, T, A>; 2], [Mask<N, T, A>; N])>()
            {
                let other = Matrix::from_row_fn(|r| mask[r].select(matrix[r], other[r]));

                assert_eq!(matrix == other, matrix.as_rows() == other.as_rows());
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|N, T: PrimitiveNumber, A| {
            for ([matrix, other], mask) in
                random_iter::<([Matrix<N, T, A>; 2], [Mask<N, T, A>; N])>()
            {
                let other = Matrix::from_row_fn(|r| mask[r].select(matrix[r], other[r]));

                assert_eq!(matrix != other, matrix.as_rows() != other.as_rows());
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N, T: PrimitiveNumber, A| {
            assert_eq!(Matrix::<N, T, A>::default(), Matrix::IDENTITY);
        });
    }

    #[test]
    fn test_neg() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for matrix in random_iter::<Matrix<N, T, A>>() {
                assert_test_eq!(-matrix, Matrix::from_rows(&matrix.as_rows().map(|v| -v)));
            }
        });
    }

    #[test]
    fn test_add() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Matrix<N, T, A>; 2]>() {
                assert_test_eq!(left + right, Matrix::from_row_fn(|r| left[r] + right[r]));
            }
        });
    }

    #[test]
    fn test_add_assign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Matrix<N, T, A>; 2]>() {
                let mut result = left;
                result += right;

                assert_test_eq!(result, left + right);
            }
        });
    }

    #[test]
    fn test_sub() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Matrix<N, T, A>; 2]>() {
                assert_test_eq!(left - right, Matrix::from_row_fn(|r| left[r] - right[r]));
            }
        });
    }

    #[test]
    fn test_sub_assign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Matrix<N, T, A>; 2]>() {
                let mut result = left;
                result -= right;

                assert_test_eq!(result, left - right);
            }
        });
    }

    #[test]
    fn test_mul_scalar() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (matrix, scalar) in random_iter::<(Matrix<N, T, A>, T)>() {
                assert_test_eq!(matrix * scalar, Matrix::from_row_fn(|r| matrix[r] * scalar));
            }
        });
    }

    #[test]
    fn test_mul() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (vector, [matrix_1, matrix_2]) in
                random_iter::<(Vector<N, T, A>, [Matrix<N, T, A>; 2])>()
            {
                if !vector.is_finite()
                    || !matrix_1.is_finite()
                    || !matrix_2.is_finite()
                    || vector.iter().any(|x| x.abs() > 1e10)
                    || matrix_1.as_rows().iter().flatten().any(|x| x.abs() > 1e10)
                    || matrix_2.as_rows().iter().flatten().any(|x| x.abs() > 1e10)
                {
                    continue;
                }

                assert_test_eq!(
                    vector * (matrix_1 * matrix_2),
                    vector * matrix_1 * matrix_2,
                    abs <= (vector * matrix_1 * matrix_2).abs() * 1e-3 + 1e-3,
                    0.0 = -0.0
                );
            }
        });
    }

    #[test]
    fn test_vector_mul() {
        for_types!(|T: PrimitiveFloat, A| {
            for (vector, matrix) in random_iter::<(Vector<2, T, A>, Matrix<2, T, A>)>() {
                assert_test_eq!(
                    vector * matrix,
                    matrix.x_axis * vector.x + matrix.y_axis * vector.y,
                );
            }

            for (vector, matrix) in random_iter::<(Vector<3, T, A>, Matrix<3, T, A>)>() {
                assert_test_eq!(
                    vector * matrix,
                    matrix.x_axis * vector.x + matrix.y_axis * vector.y + matrix.z_axis * vector.z,
                );
            }

            for (vector, matrix) in random_iter::<(Vector<4, T, A>, Matrix<4, T, A>)>() {
                assert_test_eq!(
                    vector * matrix,
                    matrix.x_axis * vector.x
                        + matrix.y_axis * vector.y
                        + matrix.z_axis * vector.z
                        + matrix.w_axis * vector.w,
                );
            }
        });
    }

    #[test]
    fn test_mul_assign_scalar() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (matrix, scalar) in random_iter::<(Matrix<N, T, A>, T)>() {
                let mut result = matrix;
                result *= scalar;

                assert_test_eq!(result, matrix * scalar);
            }
        });
    }

    #[test]
    fn test_mul_assign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for [left, right] in random_iter::<[Matrix<N, T, A>; 2]>() {
                let mut result = left;
                result *= right;

                assert_test_eq!(result, left * right);
            }
        });
    }

    #[test]
    fn test_vector_mul_assign() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (vector, matrix) in random_iter::<(Vector<N, T, A>, Matrix<N, T, A>)>() {
                let mut result = vector;
                result *= matrix;

                assert_test_eq!(result, vector * matrix);
            }
        });
    }

    #[test]
    fn test_div_scalar() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (matrix, scalar) in random_iter::<(Matrix<N, T, A>, T)>() {
                assert_test_eq!(matrix / scalar, Matrix::from_row_fn(|r| matrix[r] / scalar));
            }
        });
    }

    #[test]
    fn test_div_assign_scalar() {
        for_types!(|N, T: PrimitiveFloat, A| {
            for (matrix, scalar) in random_iter::<(Matrix<N, T, A>, T)>() {
                let mut result = matrix;
                result /= scalar;

                assert_test_eq!(result, matrix / scalar);
            }
        });
    }
}
