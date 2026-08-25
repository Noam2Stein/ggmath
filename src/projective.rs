use core::{
    fmt::{Debug, Display},
    hash::Hash,
    ops::{Add, AddAssign, Deref, DerefMut, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    panic::{RefUnwindSafe, UnwindSafe},
};

use crate::{
    Affine, Aligned, Alignment, Length, Matrix, One, Scalar, Unaligned, Vector, Zero,
    length::TwoOrThree,
    utils::{specialize_23, transmute_mut, transmute_ref},
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

/// An `N`-dimensional projective transform represented by a homogeneous matrix.
///
/// `A` controls SIMD alignment and is either [`Unaligned`] or [`Aligned`]. See
/// [`Alignment`] for more details.
///
/// This can represent translation, rotation, scaling, shear and projections. To
/// apply this assuming no projection, use [`transform_point`] and
/// [`transform_vector`]. To apply this with perspective divide, use
/// [`project_point`]. To transform a homogeneous vector, use
/// `vector_np1 * self`.
///
/// # Type aliases
///
/// - [`Proj2<T>`] for [`Projective<2, T, Unaligned>`].
/// - [`Proj3<T>`] for [`Projective<3, T, Unaligned>`].
/// - [`Proj2A<T>`] for [`Projective<2, T, Aligned>`].
/// - [`Proj3A<T>`] for [`Projective<3, T, Aligned>`].
///
/// # Fields
///
/// - `x_axis: Vector<N + 1, T, N>` (first row of inner matrix, exists for
///   dimensions `2`, `3`)
///
/// - `y_axis: Vector<N + 1, T, N>` (second row of inner matrix, exists for
///   dimensions `2`, `3`)
///
/// - `z_axis: Vector<N + 1, T, N>` (third row of inner matrix, exists for
///   dimensions `2`, `3`)
///
/// - `w_axis: Vector<N + 1, T, N>` (fourth row of inner matrix, exists for
///   dimension `3`)
///
/// Note that these fields are only exposed by implementing [`Deref`] and
/// [`DerefMut`].
///
/// # Memory layout
///
/// [`Projective<N, T, A>`] is a transparent wrapper over
/// [`Matrix<N + 1, T, A>`]. The types can be transmuted both ways.
///
/// [`transform_point`]: Projective::transform_point
/// [`transform_vector`]: Projective::transform_vector
/// [`project_point`]: Projective::project_point
#[repr(transparent)]
#[expect(private_bounds)]
pub struct Projective<const N: usize, T, A: Alignment>(
    // This type always corresponds to `Matrix<N - 1, T, A>`, which cannot be
    // written directly due to type system limitations. Many functions here use
    // the [`specialize_23`] macro to circumvent this limitation.
    pub(crate) <Length<N> as TwoOrThree>::Select<Matrix<3, T, A>, Matrix<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D projective transform represented by a homogeneous 3x3 matrix.
///
/// This can represent 2D translation, rotation, scaling, shear and projections.
/// To apply this assuming no projection, use [`transform_point`] and
/// [`transform_vector`]. To apply this with perspective divide, use
/// [`project_point`]. To transform a homogeneous 3D vector, use `vec3 * self`.
///
/// # No SIMD alignment
///
/// [`Proj2<T>`] does not have SIMD alignment, for that use [`Proj2A<T>`].
///
/// [`transform_point`]: Projective::transform_point
/// [`transform_vector`]: Projective::transform_vector
/// [`project_point`]: Projective::project_point
pub type Proj2<T> = Projective<2, T, Unaligned>;

/// A 3D projective transform represented by a homogeneous 4x4 matrix.
///
/// This can represent 3D translation, rotation, scaling, shear and projections.
/// To apply this assuming no projection, use [`transform_point`] and
/// [`transform_vector`]. To apply this with perspective divide, use
/// [`project_point`]. To transform a homogeneous 4D vector, use `vec4 * self`.
///
/// # No SIMD alignment
///
/// [`Proj3<T>`] does not have SIMD alignment, for that use [`Proj3A<T>`].
///
/// [`transform_point`]: Projective::transform_point
/// [`transform_vector`]: Projective::transform_vector
/// [`project_point`]: Projective::project_point
pub type Proj3<T> = Projective<3, T, Unaligned>;

/// A 2D projective transform represented by a homogeneous 3x3 matrix.
///
/// This can represent 2D translation, rotation, scaling, shear and projections.
/// To apply this assuming no projection, use [`transform_point`] and
/// [`transform_vector`]. To apply this with perspective divide, use
/// [`project_point`]. To transform a homogeneous 3D vector, use `vec3 * self`.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Proj2A<T>`] has SIMD alignment. For no SIMD use
/// [`Proj2<T>`].
///
/// [`transform_point`]: Projective::transform_point
/// [`transform_vector`]: Projective::transform_vector
/// [`project_point`]: Projective::project_point
pub type Proj2A<T> = Projective<2, T, Aligned>;

/// A 3D projective transform represented by a homogeneous 4x4 matrix.
///
/// This can represent 3D translation, rotation, scaling, shear and projections.
/// To apply this assuming no projection, use [`transform_point`] and
/// [`transform_vector`]. To apply this with perspective divide, use
/// [`project_point`]. To transform a homogeneous 4D vector, use `vec4 * self`.
///
/// # SIMD alignment
///
/// For appropriate `T` types, [`Proj3A<T>`] has SIMD alignment. For no SIMD use
/// [`Proj3<T>`].
///
/// [`transform_point`]: Projective::transform_point
/// [`transform_vector`]: Projective::transform_vector
/// [`project_point`]: Projective::project_point
pub type Proj3A<T> = Projective<3, T, Aligned>;

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

impl<T, A: Alignment> Index<usize> for Projective<2, T, A>
where
    T: Scalar,
{
    type Output = Vector<3, T, A>;

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `N + 1`.
    #[inline]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_rows()[index]
    }
}

impl<T, A: Alignment> Index<usize> for Projective<3, T, A>
where
    T: Scalar,
{
    type Output = Vector<4, T, A>;

    /// Returns the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `N + 1`.
    #[inline]
    #[track_caller]
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_rows()[index]
    }
}

impl<T, A: Alignment> IndexMut<usize> for Projective<2, T, A>
where
    T: Scalar,
{
    /// Returns a mutable reference to the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `N + 1`.
    #[inline]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_rows()[index]
    }
}

impl<T, A: Alignment> IndexMut<usize> for Projective<3, T, A>
where
    T: Scalar,
{
    /// Returns a mutable reference to the row at the given index.
    ///
    /// # Panics
    ///
    /// Panics if index is greater than or equal to `N + 1`.
    #[inline]
    #[track_caller]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.as_mut_rows()[index]
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Proj2Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first row of the projective transform.
    ///
    /// This is a vector3, since projective transforms are represented by
    /// homogeneous matrices.
    pub x_axis: Vector<3, T, A>,
    /// The second row of the projective transform.
    ///
    /// This is a vector3, since projective transforms are represented by
    /// homogeneous matrices.
    pub y_axis: Vector<3, T, A>,
    /// The third row of the projective transform.
    ///
    /// This is a vector3, since projective transforms are represented by
    /// homogeneous matrices.
    pub z_axis: Vector<3, T, A>,
}

impl<T, A: Alignment> Deref for Projective<2, T, A>
where
    T: Scalar,
{
    type Target = Proj2Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`.
        unsafe { transmute_ref::<Matrix<3, T, A>, Proj2Fields<T, A>>(&self.0) }
    }
}

impl<T, A: Alignment> DerefMut for Projective<2, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<3, T, A>` is guaranteed to begin with 3 consecutive
        // values of `Vector<3, T, A>`.
        unsafe { transmute_mut::<Matrix<3, T, A>, Proj2Fields<T, A>>(&mut self.0) }
    }
}

#[doc(hidden)]
#[repr(C)]
pub struct Proj3Fields<T, A: Alignment>
where
    T: Scalar,
{
    /// The first row of the projective transform.
    ///
    /// This is a vector4, since projective transforms are represented by
    /// homogeneous matrices.
    pub x_axis: Vector<4, T, A>,
    /// The second row of the projective transform.
    ///
    /// This is a vector4, since projective transforms are represented by
    /// homogeneous matrices.
    pub y_axis: Vector<4, T, A>,
    /// The third row of the projective transform.
    ///
    /// This is a vector4, since projective transforms are represented by
    /// homogeneous matrices.
    pub z_axis: Vector<4, T, A>,
    /// The fourth row of the projective transform.
    ///
    /// This is a vector4, since projective transforms are represented by
    /// homogeneous matrices.
    pub w_axis: Vector<4, T, A>,
}

impl<T, A: Alignment> Deref for Projective<3, T, A>
where
    T: Scalar,
{
    type Target = Proj3Fields<T, A>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`.
        unsafe { transmute_ref::<Matrix<4, T, A>, Proj3Fields<T, A>>(&self.0) }
    }
}

impl<T, A: Alignment> DerefMut for Projective<3, T, A>
where
    T: Scalar,
{
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: `Matrix<4, T, A>` is guaranteed to begin with 4 consecutive
        // values of `Vector<4, T, A>`.
        unsafe { transmute_mut::<Matrix<4, T, A>, Proj3Fields<T, A>>(&mut self.0) }
    }
}

impl<const N: usize, T, A: Alignment> Debug for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Debug,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        specialize_23!(Projective::<N, T, A>::debug_backend(self, f))
    }
}

impl<const N: usize, T, A: Alignment> Display for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Display,
{
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        specialize_23!(Projective::<N, T, A>::display_backend(self, f))
    }
}

impl<const N: usize, T, A: Alignment> PartialEq for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + PartialEq,
{
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        specialize_23!(Projective::<N, T, A>::eq_backend(self, other))
    }
}

impl<const N: usize, T, A: Alignment> Eq for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Eq,
{
}

impl<const N: usize, T, A: Alignment> Hash for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Hash,
{
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        specialize_23!(Projective::<N, T, A>::hash_backend(self, (state,)))
    }
}

impl<const N: usize, T, A: Alignment> Default for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
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
        impl<const N: usize, T, A: Alignment> Neg for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

        impl<const N: usize, T, A: Alignment> Neg for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Neg<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn neg(self) -> Self::Output {
                specialize_23!(Projective::<N, T, A>::neg_backend(self))
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
        impl<const N: usize, T, A: Alignment> Add for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

        impl<const N: usize, T, A: Alignment> Add<&Projective<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

        impl<const N: usize, T, A: Alignment> Add<Projective<N, T, A>> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Projective<N, T, A>) -> Self::Output {
                self + (&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Add for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add(self, rhs: Self) -> Self::Output {
                specialize_23!(Projective::<N, T, A>::add_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn add_assign(&mut self, rhs: Self) {
                *self = &*self + rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> AddAssign<&Projective<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

macro_rules! impl_sub {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Sub for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

        impl<const N: usize, T, A: Alignment> Sub<&Projective<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

        impl<const N: usize, T, A: Alignment> Sub<Projective<N, T, A>> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Projective<N, T, A>) -> Self::Output {
                self - (&rhs)
            }
        }

        impl<const N: usize, T, A: Alignment> Sub for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub(self, rhs: Self) -> Self::Output {
                specialize_23!(Projective::<N, T, A>::sub_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Sub<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn sub_assign(&mut self, rhs: Self) {
                *self = &*self - rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> SubAssign<&Projective<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

macro_rules! impl_vector_mul {
    ($(#[$doc:meta])*) => {
        impl<T, A: Alignment> Mul<Projective<2, T, A>> for Vector<3, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<2, T, A>) -> Self::Output {
                self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<Projective<3, T, A>> for Vector<4, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<3, T, A>) -> Self::Output {
                self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<&Projective<2, T, A>> for Vector<3, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<2, T, A>) -> Self::Output {
                self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<&Projective<3, T, A>> for Vector<4, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<3, T, A>) -> Self::Output {
                self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<Projective<2, T, A>> for &Vector<3, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<3, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<2, T, A>) -> Self::Output {
                *self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<Projective<3, T, A>> for &Vector<4, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<4, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<3, T, A>) -> Self::Output {
                *self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<&Projective<2, T, A>> for &Vector<3, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<3, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<2, T, A>) -> Self::Output {
                *self * &rhs.0
            }
        }

        impl<T, A: Alignment> Mul<&Projective<3, T, A>> for &Vector<4, T, A>
        where
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Vector<4, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<3, T, A>) -> Self::Output {
                *self * &rhs.0
            }
        }
    };
}
impl_vector_mul!(
    /// Transforms a homogeneous vector by a projective transform.
    ///
    /// The vector has one element more than the dimension of the transform,
    /// because projective transforms are represented as homogeneous matrices.
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

macro_rules! impl_mul {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
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

        impl<const N: usize, T, A: Alignment> Mul<&Projective<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<N, T, A>) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<Projective<N, T, A>> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: Projective<N, T, A>) -> Self::Output {
                self * &rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&Projective<N, T, A>> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &Projective<N, T, A>) -> Self::Output {
                specialize_23!(Projective::<N, T, A>::mul_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: Self) {
                *self = &*self * &rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&Projective<N, T, A>> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &Projective<N, T, A>) {
                *self = &*self * rhs;
            }
        }
    };
}
impl_mul!(
    /// Multiplies two projective transforms.
    ///
    /// The resulting transform is equivalent to first applying the left
    /// transform, then the right transform.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_mul_scalar {
    ($(#[$doc:meta])*) => {
        impl<const N: usize, T, A: Alignment> Mul<T> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                &self * rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Self;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                &self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<T> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: T) -> Self::Output {
                specialize_23!(Projective::<N, T, A>::mul_scalar_backend(self, rhs))
            }
        }

        impl<const N: usize, T, A: Alignment> Mul<&T> for &Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            type Output = Projective<N, T, A>;

            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul(self, rhs: &T) -> Self::Output {
                self * *rhs
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<T> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: T) {
                *self = &*self * rhs;
            }
        }

        impl<const N: usize, T, A: Alignment> MulAssign<&T> for Projective<N, T, A>
        where
            Length<N>: TwoOrThree,
            T: Scalar + Add<Output = T> + Mul<Output = T>,
        {
            $(#[$doc])*
            #[inline]
            #[track_caller]
            fn mul_assign(&mut self, rhs: &T) {
                *self = &*self * *rhs;
            }
        }
    };
}
impl_mul_scalar!(
    /// Multiplies every element by a uniform scalar.
    ///
    /// # Consistency
    ///
    /// For primitive types this operation is cross-platform deterministic and
    /// fully consistent with scalar addition and multiplication, including
    /// floating-point precision and integer panics.
);

macro_rules! impl_affine_mul {
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
impl_affine_mul!(
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

macro_rules! impl_mul_affine {
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
impl_mul_affine!(
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

// SAFETY: Projective is equivalent to values of `T` mixed with padding.
// Because `T` is `Send` and padding is `Send`, projective is too.
unsafe impl<const N: usize, T, A: Alignment> Send for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Send,
{
}

// SAFETY: Projective is equivalent to values of `T` mixed with padding.
// Because `T` is `Sync` and padding is `Sync`, the projective is too.
unsafe impl<const N: usize, T, A: Alignment> Sync for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Sync,
{
}

impl<const N: usize, T, A: Alignment> Unpin for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + Unpin,
{
}

impl<const N: usize, T, A: Alignment> UnwindSafe for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + UnwindSafe,
{
}

impl<const N: usize, T, A: Alignment> RefUnwindSafe for Projective<N, T, A>
where
    Length<N>: TwoOrThree,
    T: Scalar + RefUnwindSafe,
{
}

#[cfg(test)]
mod tests {
    extern crate std;

    use std::format;

    use crate::{
        Affine, Aligned, Mask, Matrix, Projective, Unaligned, Vector,
        test_utils::{assert_test_eq, for_types, random_iter},
    };

    #[test]
    fn test_layout() {
        for_types!(|T: PrimitiveNumber, A| {
            assert_eq!(
                size_of::<Projective<2, T, A>>(),
                size_of::<Matrix<3, T, A>>()
            );
            assert_eq!(
                size_of::<Projective<3, T, A>>(),
                size_of::<Matrix<4, T, A>>()
            );

            assert_eq!(
                align_of::<Projective<2, T, A>>(),
                align_of::<Matrix<3, T, A>>()
            );
            assert_eq!(
                align_of::<Projective<3, T, A>>(),
                align_of::<Matrix<4, T, A>>()
            );
        });
    }

    #[test]
    fn test_zero() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_eq!(Projective::<2, T, A>::ZERO, Projective(Matrix::ZERO));
            assert_eq!(Projective::<3, T, A>::ZERO, Projective(Matrix::ZERO));
        });
    }

    #[test]
    fn test_identity() {
        for_types!(|T: PrimitiveFloat, A| {
            assert_eq!(
                Projective::<2, T, A>::IDENTITY,
                Projective(Matrix::IDENTITY)
            );
            assert_eq!(
                Projective::<3, T, A>::IDENTITY,
                Projective(Matrix::IDENTITY)
            );
        });
    }

    #[test]
    fn test_from_scale() {
        for_types!(|T: PrimitiveNumber, A| {
            let scale = Vector::from_fn(|i| T::as_from(i + 1));
            assert_eq!(
                Projective::<2, T, A>::from_scale(scale),
                Projective(Matrix::from_scale(scale.extend(T::ONE)))
            );

            let scale = Vector::from_fn(|i| T::as_from(i + 1));
            assert_eq!(
                Projective::<3, T, A>::from_scale(scale),
                Projective(Matrix::from_scale(scale.extend(T::ONE)))
            );
        });
    }

    #[test]
    fn test_from_translation() {
        for_types!(|T: PrimitiveNumber, A| {
            let translation = Vector::from_fn(|i| T::as_from(i + 1));
            assert_eq!(
                Projective::<2, T, A>::from_translation(translation),
                Projective::<2, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(T::ONE, T::ZERO, T::ZERO),
                    Vector::<3, T, A>::new(T::ZERO, T::ONE, T::ZERO),
                    translation.extend(T::ONE)
                ])
            );

            let translation = Vector::from_fn(|i| T::as_from(i + 1));
            assert_eq!(
                Projective::<3, T, A>::from_translation(translation),
                Projective::<3, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(T::ONE, T::ZERO, T::ZERO, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ONE, T::ZERO, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ONE, T::ZERO),
                    translation.extend(T::ONE)
                ])
            );
        });
    }

    #[test]
    fn test_from_scale_translation() {
        for_types!(|T: PrimitiveNumber, A| {
            let scale = Vector::<2, T, A>::from_fn(|i| T::as_from(i + 1));
            let translation = Vector::<2, T, A>::from_fn(|i| T::as_from(i + 3));
            assert_eq!(
                Projective::<2, T, A>::from_scale_translation(scale, translation),
                Projective::<2, T, A>::from_rows(&[
                    Vector::<3, T, A>::new(scale.x, T::ZERO, T::ZERO),
                    Vector::<3, T, A>::new(T::ZERO, scale.y, T::ZERO),
                    translation.extend(T::ONE)
                ])
            );

            let scale = Vector::<3, T, A>::from_fn(|i| T::as_from(i + 1));
            let translation = Vector::<3, T, A>::from_fn(|i| T::as_from(i + 3));
            assert_eq!(
                Projective::<3, T, A>::from_scale_translation(scale, translation),
                Projective::<3, T, A>::from_rows(&[
                    Vector::<4, T, A>::new(scale.x, T::ZERO, T::ZERO, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, scale.y, T::ZERO, T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, scale.z, T::ZERO),
                    translation.extend(T::ONE)
                ])
            );
        });
    }

    #[test]
    fn test_from_matrix() {
        for_types!(|T: PrimitiveNumber, A| {
            let matrix = Matrix::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(
                Projective::<2, T, A>::from_matrix(&matrix),
                Projective::<2, T, A>::from_rows(&[
                    matrix.x_axis.extend(T::ZERO),
                    matrix.y_axis.extend(T::ZERO),
                    Vector::<3, T, A>::new(T::ZERO, T::ZERO, T::ONE)
                ])
            );

            let matrix = Matrix::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                Projective::<3, T, A>::from_matrix(&matrix),
                Projective::<3, T, A>::from_rows(&[
                    matrix.x_axis.extend(T::ZERO),
                    matrix.y_axis.extend(T::ZERO),
                    matrix.z_axis.extend(T::ZERO),
                    Vector::<4, T, A>::new(T::ZERO, T::ZERO, T::ZERO, T::ONE)
                ])
            );
        });
    }

    #[test]
    fn test_from_matrix_translation() {
        for_types!(|T: PrimitiveNumber, A| {
            let matrix = Matrix::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            let translation = Vector::from_fn(|i| T::as_from(i + 3));
            assert_eq!(
                Projective::<2, T, A>::from_matrix_translation(&matrix, translation),
                Projective::<2, T, A>::from_rows(&[
                    matrix.x_axis.extend(T::ZERO),
                    matrix.y_axis.extend(T::ZERO),
                    translation.extend(T::ONE)
                ])
            );

            let matrix = Matrix::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            let translation = Vector::from_fn(|i| T::as_from(i + 3));
            assert_eq!(
                Projective::<3, T, A>::from_matrix_translation(&matrix, translation),
                Projective::<3, T, A>::from_rows(&[
                    matrix.x_axis.extend(T::ZERO),
                    matrix.y_axis.extend(T::ZERO),
                    matrix.z_axis.extend(T::ZERO),
                    translation.extend(T::ONE)
                ])
            );
        });
    }

    #[test]
    fn test_from_affine() {
        for_types!(|T: PrimitiveNumber, A| {
            let affine = Affine::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 2 + c)));
            assert_eq!(
                Projective::<2, T, A>::from_affine(&affine),
                Projective::<2, T, A>::from_rows(&[
                    affine.matrix.x_axis.extend(T::ZERO),
                    affine.matrix.y_axis.extend(T::ZERO),
                    affine.translation.extend(T::ONE)
                ])
            );

            let affine = Affine::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                Projective::<3, T, A>::from_affine(&affine),
                Projective::<3, T, A>::from_rows(&[
                    affine.matrix.x_axis.extend(T::ZERO),
                    affine.matrix.y_axis.extend(T::ZERO),
                    affine.matrix.z_axis.extend(T::ZERO),
                    affine.translation.extend(T::ONE)
                ])
            );
        });
    }

    #[test]
    fn test_translation() {
        for_types!(|T: PrimitiveNumber, A| {
            let projective =
                Projective::<2, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(projective.translation(), projective.z_axis.truncate());

            let projective =
                Projective::<3, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(projective.translation(), projective.w_axis.truncate());
        });
    }

    #[test]
    fn test_to_alignment() {
        for_types!(|T: PrimitiveNumber, A| {
            let projective =
                Projective::<2, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                projective.to_alignment(),
                Projective::<2, T, Aligned>::from_rows(&projective.as_rows().map(Vector::align))
            );
            assert_eq!(
                projective.to_alignment(),
                Projective::<2, T, Unaligned>::from_rows(
                    &projective.as_rows().map(Vector::unalign)
                )
            );

            let projective =
                Projective::<3, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(
                projective.to_alignment(),
                Projective::<3, T, Aligned>::from_rows(&projective.as_rows().map(Vector::align))
            );
            assert_eq!(
                projective.to_alignment(),
                Projective::<3, T, Unaligned>::from_rows(
                    &projective.as_rows().map(Vector::unalign)
                )
            );
        });
    }

    #[test]
    fn test_align() {
        for_types!(|T: PrimitiveNumber, A| {
            let projective =
                Projective::<2, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                projective.align(),
                Projective::<2, T, Aligned>::from_rows(&projective.as_rows().map(Vector::align))
            );

            let projective =
                Projective::<3, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(
                projective.align(),
                Projective::<3, T, Aligned>::from_rows(&projective.as_rows().map(Vector::align))
            );
        });
    }

    #[test]
    fn test_unalign() {
        for_types!(|T: PrimitiveNumber, A| {
            let projective =
                Projective::<2, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 3 + c)));
            assert_eq!(
                projective.unalign(),
                Projective::<2, T, Unaligned>::from_rows(
                    &projective.as_rows().map(Vector::unalign)
                )
            );

            let projective =
                Projective::<3, T, A>::from_row_fn(|r| Vector::from_fn(|c| T::as_from(r * 4 + c)));
            assert_eq!(
                projective.unalign(),
                Projective::<3, T, Unaligned>::from_rows(
                    &projective.as_rows().map(Vector::unalign)
                )
            );
        });
    }

    #[test]
    fn test_deref() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            let projective = Projective::<2, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(projective.x_axis, Vector::<3, T, A>::new(x, y, z));
            assert_eq!(projective.y_axis, Vector::<3, T, A>::new(w, a, b));
            assert_eq!(projective.z_axis, Vector::<3, T, A>::new(c, d, e));

            let projective = Projective::<3, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(projective.x_axis, Vector::<4, T, A>::new(x, y, z, w));
            assert_eq!(projective.y_axis, Vector::<4, T, A>::new(a, b, c, d));
            assert_eq!(projective.z_axis, Vector::<4, T, A>::new(e, f, g, h));
            assert_eq!(projective.w_axis, Vector::<4, T, A>::new(i, j, k, l));
        });
    }

    #[test]
    fn test_deref_mut() {
        for_types!(|T: PrimitiveNumber, A| {
            let [x, y, z, w, a, b, c, d, e, f, g, h, i, j, k, l] =
                std::array::from_fn(|i| T::as_from(i + 1));

            let mut projective = Projective::<2, T, A>::from_rows(&[
                Vector::<3, T, A>::new(x, y, z),
                Vector::<3, T, A>::new(w, a, b),
                Vector::<3, T, A>::new(c, d, e),
            ]);
            assert_eq!(&mut projective.x_axis, &mut Vector::<3, T, A>::new(x, y, z));
            assert_eq!(&mut projective.y_axis, &mut Vector::<3, T, A>::new(w, a, b));
            assert_eq!(&mut projective.z_axis, &mut Vector::<3, T, A>::new(c, d, e));

            let mut projective = Projective::<3, T, A>::from_rows(&[
                Vector::<4, T, A>::new(x, y, z, w),
                Vector::<4, T, A>::new(a, b, c, d),
                Vector::<4, T, A>::new(e, f, g, h),
                Vector::<4, T, A>::new(i, j, k, l),
            ]);
            assert_eq!(
                &mut projective.x_axis,
                &mut Vector::<4, T, A>::new(x, y, z, w)
            );
            assert_eq!(
                &mut projective.y_axis,
                &mut Vector::<4, T, A>::new(a, b, c, d)
            );
            assert_eq!(
                &mut projective.z_axis,
                &mut Vector::<4, T, A>::new(e, f, g, h)
            );
            assert_eq!(
                &mut projective.w_axis,
                &mut Vector::<4, T, A>::new(i, j, k, l)
            );
        });
    }

    #[test]
    fn test_debug() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows =
                std::array::from_fn(|r| Vector::<3, T, A>::from_fn(|c| T::as_from(r * 3 + c)));
            let [x_axis, y_axis, z_axis] = rows;
            assert_eq!(
                format!("{:?}", Projective::<2, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {z_axis:?}]")
            );

            let rows =
                std::array::from_fn(|r| Vector::<4, T, A>::from_fn(|c| T::as_from(r * 4 + c)));
            let [x_axis, y_axis, z_axis, w_axis] = rows;
            assert_eq!(
                format!("{:?}", Projective::<3, T, A>::from_rows(&rows)),
                format!("[{x_axis:?}, {y_axis:?}, {z_axis:?}, {w_axis:?}]")
            );
        });
    }

    #[test]
    fn test_display() {
        for_types!(|T: PrimitiveNumber, A| {
            let rows =
                std::array::from_fn(|r| Vector::<3, T, A>::from_fn(|c| T::as_from(r * 3 + c)));
            let [x_axis, y_axis, z_axis] = rows;
            assert_eq!(
                format!("{}", Projective::<2, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {z_axis}]")
            );

            let rows =
                std::array::from_fn(|r| Vector::<4, T, A>::from_fn(|c| T::as_from(r * 4 + c)));
            let [x_axis, y_axis, z_axis, w_axis] = rows;
            assert_eq!(
                format!("{}", Projective::<3, T, A>::from_rows(&rows)),
                format!("[{x_axis}, {y_axis}, {z_axis}, {w_axis}]")
            );
        });
    }

    #[test]
    fn test_eq() {
        for_types!(|T: PrimitiveNumber, A| {
            for ([projective, other], mask) in
                random_iter::<([Projective<2, T, A>; 2], [Mask<3, T, A>; 3])>()
            {
                let other =
                    Projective::<2, T, A>::from_row_fn(|r| mask[r].select(projective[r], other[r]));

                assert_eq!(projective == other, projective.as_rows() == other.as_rows());
            }

            for ([projective, other], mask) in
                random_iter::<([Projective<3, T, A>; 2], [Mask<4, T, A>; 4])>()
            {
                let other =
                    Projective::<3, T, A>::from_row_fn(|r| mask[r].select(projective[r], other[r]));

                assert_eq!(projective == other, projective.as_rows() == other.as_rows());
            }
        });
    }

    #[test]
    fn test_ne() {
        for_types!(|T: PrimitiveNumber, A| {
            for ([projective, other], mask) in
                random_iter::<([Projective<2, T, A>; 2], [Mask<3, T, A>; 3])>()
            {
                let other =
                    Projective::<2, T, A>::from_row_fn(|r| mask[r].select(projective[r], other[r]));

                assert_eq!(projective == other, projective.as_rows() == other.as_rows());
            }

            for ([projective, other], mask) in
                random_iter::<([Projective<3, T, A>; 2], [Mask<4, T, A>; 4])>()
            {
                let other =
                    Projective::<3, T, A>::from_row_fn(|r| mask[r].select(projective[r], other[r]));

                assert_eq!(projective != other, projective.as_rows() != other.as_rows());
            }
        });
    }

    #[test]
    fn test_default() {
        for_types!(|N: TwoOrThree, T: PrimitiveNumber, A| {
            assert_eq!(Projective::<N, T, A>::default(), Projective::IDENTITY);
        });
    }

    #[test]
    fn test_mul_scalar() {
        for_types!(|T: PrimitiveFloat, A| {
            for (projective, scalar) in random_iter::<(Projective<2, T, A>, T)>() {
                assert_test_eq!(projective * scalar, Projective(projective.0 * scalar));
            }
            for (projective, scalar) in random_iter::<(Projective<3, T, A>, T)>() {
                assert_test_eq!(projective * scalar, Projective(projective.0 * scalar));
            }
        });
    }

    #[test]
    fn test_affine_mul() {
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
    fn test_mul_affine() {
        for_types!(|N: TwoOrThree, T: PrimitiveFloat, A| {
            for (projective, affine) in random_iter::<(Projective<N, T, A>, Affine<N, T, A>)>() {
                assert_test_eq!(
                    projective * affine,
                    projective * Projective::from_affine(&affine)
                );
            }
        });
    }
}
