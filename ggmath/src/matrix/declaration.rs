use super::*;

/// The matrix type.
/// Statically-sized matrix.
///
/// In most cases you can use this type's type aliases instead.
/// See in [`crate::matrix`].
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
    pub(super) inner: M::InnerMatrix<C, R, T, A>,
}

/// Type alias to [```Matrix<2, 2, T, VecAligned, ColumnMajor>```].
pub type Mat2C<T> = Matrix<2, 2, T, VecAligned, ColMajor>;
/// Type alias to [```Matrix<2, 3, T, VecAligned, ColumnMajor>```].
pub type Mat2x3C<T> = Matrix<2, 3, T, VecAligned, ColMajor>;
/// Type alias to [```Matrix<2, 4, T, VecAligned, ColumnMajor>```].
pub type Mat2x4C<T> = Matrix<2, 4, T, VecAligned, ColMajor>;

/// Type alias to [```Matrix<3, 2, T, VecAligned, ColumnMajor>```].
pub type Mat3x2C<T> = Matrix<3, 2, T, VecAligned, ColMajor>;
/// Type alias to [```Matrix<3, 3, T, VecAligned, ColumnMajor>```].
pub type Mat3C<T> = Matrix<3, 3, T, VecAligned, ColMajor>;
/// Type alias to [```Matrix<3, 4, T, VecAligned, ColumnMajor>```].
pub type Mat3x4C<T> = Matrix<3, 4, T, VecAligned, ColMajor>;

/// Type alias to [```Matrix<4, 2, T, VecAligned, ColumnMajor>```].
pub type Mat4x2C<T> = Matrix<4, 2, T, VecAligned, ColMajor>;
/// Type alias to [```Matrix<4, 3, T, VecAligned, ColumnMajor>```].
pub type Mat4x3C<T> = Matrix<4, 3, T, VecAligned, ColMajor>;
/// Type alias to [```Matrix<4, 4, T, VecAligned, ColumnMajor>```].
pub type Mat4C<T> = Matrix<4, 4, T, VecAligned, ColMajor>;

/// Type alias to [```Matrix<2, 2, T, VecAligned, RowMajor>```].
pub type Mat2R<T> = Matrix<2, 2, T, VecAligned, RowMajor>;
/// Type alias to [```Matrix<2, 3, T, VecAligned, RowMajor>```].
pub type Mat2x3R<T> = Matrix<2, 3, T, VecAligned, RowMajor>;
/// Type alias to [```Matrix<2, 4, T, VecAligned, RowMajor>```].
pub type Mat2x4R<T> = Matrix<2, 4, T, VecAligned, RowMajor>;

/// Type alias to [```Matrix<3, 2, T, VecAligned, RowMajor>```].
pub type Mat3x2R<T> = Matrix<3, 2, T, VecAligned, RowMajor>;
/// Type alias to [```Matrix<3, 3, T, VecAligned, RowMajor>```].
pub type Mat3R<T> = Matrix<3, 3, T, VecAligned, RowMajor>;
/// Type alias to [```Matrix<3, 4, T, VecAligned, RowMajor>```].
pub type Mat3x4R<T> = Matrix<3, 4, T, VecAligned, RowMajor>;

/// Type alias to [```Matrix<4, 2, T, VecAligned, RowMajor>```].
pub type Mat4x2R<T> = Matrix<4, 2, T, VecAligned, RowMajor>;
/// Type alias to [```Matrix<4, 3, T, VecAligned, RowMajor>```].
pub type Mat4x3R<T> = Matrix<4, 3, T, VecAligned, RowMajor>;
/// Type alias to [```Matrix<4, 4, T, VecAligned, RowMajor>```].
pub type Mat4R<T> = Matrix<4, 4, T, VecAligned, RowMajor>;

/// Type alias to [```Matrix<2, 2, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat2C```].
pub type Mat2CP<T> = Matrix<2, 2, T, VecPacked, ColMajor>;
/// Type alias to [```Matrix<2, 3, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat2x3C```].
pub type Mat2x3CP<T> = Matrix<2, 3, T, VecPacked, ColMajor>;
/// Type alias to [```Matrix<2, 4, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat2x4C```].
pub type Mat2x4CP<T> = Matrix<2, 4, T, VecPacked, ColMajor>;

/// Type alias to [```Matrix<3, 2, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat3x2C```].
pub type Mat3x2CP<T> = Matrix<3, 2, T, VecPacked, ColMajor>;
/// Type alias to [```Matrix<3, 3, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat3C```].
pub type Mat3CP<T> = Matrix<3, 3, T, VecPacked, ColMajor>;
/// Type alias to [```Matrix<3, 4, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat3x4C```].
pub type Mat3x4CP<T> = Matrix<3, 4, T, VecPacked, ColMajor>;

/// Type alias to [```Matrix<4, 2, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat4x2C```].
pub type Mat4x2CP<T> = Matrix<4, 2, T, VecPacked, ColMajor>;
/// Type alias to [```Matrix<4, 3, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat4x3C```].
pub type Mat4x3CP<T> = Matrix<4, 3, T, VecPacked, ColMajor>;
/// Type alias to [```Matrix<4, 4, T, VecPacked, ColumnMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat4C```].
pub type Mat4CP<T> = Matrix<4, 4, T, VecPacked, ColMajor>;

/// Type alias to [```Matrix<2, 2, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat2R```].
pub type Mat2RP<T> = Matrix<2, 2, T, VecPacked, RowMajor>;
/// Type alias to [```Matrix<2, 3, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat2x3R```].
pub type Mat2x3RP<T> = Matrix<2, 3, T, VecPacked, RowMajor>;
/// Type alias to [```Matrix<2, 4, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat2x4R```].
pub type Mat2x4RP<T> = Matrix<2, 4, T, VecPacked, RowMajor>;

/// Type alias to [```Matrix<3, 2, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat3x2R```].
pub type Mat3x2RP<T> = Matrix<3, 2, T, VecPacked, RowMajor>;
/// Type alias to [```Matrix<3, 3, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat3R```].
pub type Mat3RP<T> = Matrix<3, 3, T, VecPacked, RowMajor>;
/// Type alias to [```Matrix<3, 4, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat3x4R```].
pub type Mat3x4RP<T> = Matrix<3, 4, T, VecPacked, RowMajor>;

/// Type alias to [```Matrix<4, 2, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat4x2R```].
pub type Mat4x2RP<T> = Matrix<4, 2, T, VecPacked, RowMajor>;
/// Type alias to [```Matrix<4, 3, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat4x3R```].
pub type Mat4x3RP<T> = Matrix<4, 3, T, VecPacked, RowMajor>;
/// Type alias to [```Matrix<4, 4, T, VecPacked, RowMajor>```].
/// If you don't know the difference between ```VecAligned``` and ```VecPacked```, use [```Mat4R```].
pub type Mat4RP<T> = Matrix<4, 4, T, VecPacked, RowMajor>;
