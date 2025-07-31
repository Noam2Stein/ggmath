use super::*;

/// Marker trait for matrix major axis.
///
/// Is implemented for `ColMajor` and `RowMajor`.
pub unsafe trait MatrixMajorAxis {
    /// Enum that is used to specialize function implementations based on the major axis.
    const ENUM: MatrixMajorAxisEnum;

    /// The inner array type that is inside a matrix.
    /// This has to be controlled by the major axis.
    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>: Construct
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}

/// Marker type for column major matrices.
pub struct ColMajor;

/// Marker type for row major matrices.
pub struct RowMajor;

/// Enum that mirrors the `MatrixMajorAxis` trait and its impls.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MatrixMajorAxisEnum {
    /// Column major matrix.
    ColumnMajor,
    /// Row major matrix.
    RowMajor,
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    /// Converts the matrix to a column major matrix.
    #[inline(always)]
    pub const fn to_c_major(self) -> Matrix<C, R, T, A, ColMajor> {
        Matrix::from_columns(self.columns())
    }

    /// Converts the matrix to a row major matrix.
    #[inline(always)]
    pub const fn to_r_major(self) -> Matrix<C, R, T, A, RowMajor> {
        Matrix::from_rows(self.rows())
    }
}

#[allow(private_interfaces)]
unsafe impl MatrixMajorAxis for ColMajor {
    const ENUM: MatrixMajorAxisEnum = MatrixMajorAxisEnum::ColumnMajor;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
        = [Vector<R, T, A>; C]
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}

#[allow(private_interfaces)]
unsafe impl MatrixMajorAxis for RowMajor {
    const ENUM: MatrixMajorAxisEnum = MatrixMajorAxisEnum::RowMajor;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
        = [Vector<C, T, A>; R]
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}
