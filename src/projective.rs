use crate::{Aligned, Alignment, Length, Matrix, Scalar, Unaligned, length::TwoOrThree};

/// An `N`-dimensional projective transform represented by a homogeneous matrix.
///
/// TODO
#[repr(transparent)]
#[expect(private_bounds)]
pub struct Projective<const N: usize, T, A: Alignment>(
    <Length<N> as TwoOrThree>::Select<Matrix<3, T, A>, Matrix<4, T, A>>,
)
where
    Length<N>: TwoOrThree,
    T: Scalar;

/// A 2D projective transform represented by a homogeneous 3x3 matrix.
///
/// TODO
pub type Proj2<T> = Projective<2, T, Unaligned>;

/// A 3D projective transform represented by a homogeneous 4x4 matrix.
///
/// TODO
pub type Proj3<T> = Projective<3, T, Unaligned>;

/// A 2D projective transform represented by a homogeneous 3x3 matrix.
///
/// TODO
pub type Proj2A<T> = Projective<2, T, Aligned>;

/// A 3D projective transform represented by a homogeneous 4x4 matrix.
///
/// TODO
pub type Proj3A<T> = Projective<3, T, Aligned>;

impl<T, A: Alignment> Projective<2, T, A>
where
    T: Scalar,
{
    /// Reinterprets a homogeneous 3x3 matrix as a 2D projective transform.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn from_homogeneous(homogeneous: &Matrix<3, T, A>) -> Self {
        Self(*homogeneous)
    }

    /// Reinterprets `self` as a homogeneous 3x3 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn to_homogeneous(&self) -> Matrix<3, T, A> {
        self.0
    }

    /// Reinterprets `self` as a reference to a homogeneous 3x3 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_homogeneous(&self) -> &Matrix<3, T, A> {
        &self.0
    }

    /// Reinterprets `self` as a mutable reference to a homogeneous 3x3 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_mut_homogeneous(&mut self) -> &mut Matrix<3, T, A> {
        &mut self.0
    }
}

impl<T, A: Alignment> Projective<3, T, A>
where
    T: Scalar,
{
    /// Reinterprets a homogeneous 4x4 matrix as a 3D projective transform.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn from_homogeneous(homogeneous: &Matrix<4, T, A>) -> Self {
        Self(*homogeneous)
    }

    /// Reinterprets `self` as a homogeneous 4x4 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn to_homogeneous(&self) -> Matrix<4, T, A> {
        self.0
    }

    /// Reinterprets `self` as a reference to a homogeneous 4x4 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_homogeneous(&self) -> &Matrix<4, T, A> {
        &self.0
    }

    /// Reinterprets `self` as a mutable reference to a homogeneous 4x4 matrix.
    ///
    /// This is a no-op, because the representation is the same.
    #[inline]
    #[must_use]
    pub const fn as_mut_homogeneous(&mut self) -> &mut Matrix<4, T, A> {
        &mut self.0
    }
}

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
