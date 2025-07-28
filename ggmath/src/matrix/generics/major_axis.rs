use super::*;

#[allow(private_bounds)]
#[allow(private_interfaces)]
pub unsafe trait MatrixMajorAxis {
    const ENUM: MatrixMajorAxisEnum;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>: Construct
    where
        Usize<C>: VecLen,
        Usize<R>: VecLen;
}

pub struct ColumnMajor;
pub struct RowMajor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::matrix) enum MatrixMajorAxisEnum {
    ColumnMajor,
    RowMajor,
}

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    pub const fn to_c_major(self) -> Matrix<C, R, T, A, ColumnMajor> {
        Matrix::from_columns(self.columns())
    }

    #[inline(always)]
    pub const fn to_r_major(self) -> Matrix<C, R, T, A, RowMajor> {
        Matrix::from_rows(self.rows())
    }
}

#[allow(private_interfaces)]
unsafe impl MatrixMajorAxis for ColumnMajor {
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
