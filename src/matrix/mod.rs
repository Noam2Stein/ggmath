//! Module for the matrix type.

use super::*;

mod convert;
mod index;
mod construct;
mod splat;
mod ops;
mod swizzle;
pub use construct::*;
pub use splat::*;

/// The matrix type.
///
/// This type has alot of generics, but in most cases you can use its type aliases instead.
/// For example, [`Mat2C<T>`].
///
/// This type is generic over columns, rows, scalar type, alignment and major axis.
/// The first 4 match the generics of [`Vector`].
///
/// The major axis is a marker type like `VecAlignment` types, which is either `ColMajor` or `RowMajor`.
/// This only affects the memory representation of the matrix,
/// and does not affect the outer API.
///
/// `ggmath` matrix size specification is ordered columns then rows.
/// `2x3` means 2 columns and 3 rows.
#[derive_where(Clone, Copy)]
pub struct Matrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatMajorAxis>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    inner: M::InnerMatrix<C, R, T, A>,
}

/// Type alias to [`Matrix<2, 2, T, VecAligned, ColMajor>`].
pub type Mat2x2C<T> = Matrix<2, 2, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<2, 2, T, VecAligned, RowMajor>`].
pub type Mat2x2R<T> = Matrix<2, 2, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<2, 2, T, VecPacked, ColMajor>`].
pub type Mat2x2CP<T> = Matrix<2, 2, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<2, 2, T, VecPacked, RowMajor>`].
pub type Mat2x2RP<T> = Matrix<2, 2, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<2, 3, T, VecAligned, ColMajor>`].
pub type Mat2x3C<T> = Matrix<2, 3, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<2, 3, T, VecAligned, RowMajor>`].
pub type Mat2x3R<T> = Matrix<2, 3, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<2, 3, T, VecPacked, ColMajor>`].
pub type Mat2x3CP<T> = Matrix<2, 3, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<2, 3, T, VecPacked, RowMajor>`].
pub type Mat2x3RP<T> = Matrix<2, 3, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<2, 4, T, VecAligned, ColMajor>`].
pub type Mat2x4C<T> = Matrix<2, 4, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<2, 4, T, VecAligned, RowMajor>`].
pub type Mat2x4R<T> = Matrix<2, 4, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<2, 4, T, VecPacked, ColMajor>`].
pub type Mat2x4CP<T> = Matrix<2, 4, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<2, 4, T, VecPacked, RowMajor>`].
pub type Mat2x4RP<T> = Matrix<2, 4, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<3, 2, T, VecAligned, ColMajor>`].
pub type Mat3x2C<T> = Matrix<3, 2, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<3, 2, T, VecAligned, RowMajor>`].
pub type Mat3x2R<T> = Matrix<3, 2, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<3, 2, T, VecPacked, ColMajor>`].
pub type Mat3x2CP<T> = Matrix<3, 2, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<3, 2, T, VecPacked, RowMajor>`].
pub type Mat3x2RP<T> = Matrix<3, 2, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<3, 3, T, VecAligned, ColMajor>`].
pub type Mat3x3C<T> = Matrix<3, 3, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<3, 3, T, VecAligned, RowMajor>`].
pub type Mat3x3R<T> = Matrix<3, 3, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<3, 3, T, VecPacked, ColMajor>`].
pub type Mat3x3CP<T> = Matrix<3, 3, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<3, 3, T, VecPacked, RowMajor>`].
pub type Mat3x3RP<T> = Matrix<3, 3, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<3, 4, T, VecAligned, ColMajor>`].
pub type Mat3x4C<T> = Matrix<3, 4, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<3, 4, T, VecAligned, RowMajor>`].
pub type Mat3x4R<T> = Matrix<3, 4, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<3, 4, T, VecPacked, ColMajor>`].
pub type Mat3x4CP<T> = Matrix<3, 4, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<3, 4, T, VecPacked, RowMajor>`].
pub type Mat3x4RP<T> = Matrix<3, 4, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<4, 2, T, VecAligned, ColMajor>`].
pub type Mat4x2C<T> = Matrix<4, 2, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<4, 2, T, VecAligned, RowMajor>`].
pub type Mat4x2R<T> = Matrix<4, 2, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<4, 2, T, VecPacked, ColMajor>`].
pub type Mat4x2CP<T> = Matrix<4, 2, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<4, 2, T, VecPacked, RowMajor>`].
pub type Mat4x2RP<T> = Matrix<4, 2, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<4, 3, T, VecAligned, ColMajor>`].
pub type Mat4x3C<T> = Matrix<4, 3, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<4, 3, T, VecAligned, RowMajor>`].
pub type Mat4x3R<T> = Matrix<4, 3, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<4, 3, T, VecPacked, ColMajor>`].
pub type Mat4x3CP<T> = Matrix<4, 3, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<4, 3, T, VecPacked, RowMajor>`].
pub type Mat4x3RP<T> = Matrix<4, 3, T, VecPacked, RowMajor>;

/// Type alias to [`Matrix<4, 4, T, VecAligned, ColMajor>`].
pub type Mat4x4C<T> = Matrix<4, 4, T, VecAligned, ColMajor>;

/// Type alias to [`Matrix<4, 4, T, VecAligned, RowMajor>`].
pub type Mat4x4R<T> = Matrix<4, 4, T, VecAligned, RowMajor>;

/// Type alias to [`Matrix<4, 4, T, VecPacked, ColMajor>`].
pub type Mat4x4CP<T> = Matrix<4, 4, T, VecPacked, ColMajor>;

/// Type alias to [`Matrix<4, 4, T, VecPacked, RowMajor>`].
pub type Mat4x4RP<T> = Matrix<4, 4, T, VecPacked, RowMajor>;

