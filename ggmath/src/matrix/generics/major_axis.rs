use super::*;

#[allow(private_bounds)]
pub trait MatrixMajorAxis: Seal + Sized + 'static {
    const IS_COLUMN_MAJOR: bool;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>: Construct
    where
        MaybeVecLen<C>: VecLen,
        MaybeVecLen<R>: VecLen;
}

pub struct ColumnMajor;
pub struct RowMajor;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
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

impl MatrixMajorAxis for ColumnMajor {
    const IS_COLUMN_MAJOR: bool = true;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
        = [Vector<R, T, A>; C]
    where
        MaybeVecLen<C>: VecLen,
        MaybeVecLen<R>: VecLen;
}

impl MatrixMajorAxis for RowMajor {
    const IS_COLUMN_MAJOR: bool = false;

    type InnerMatrix<const C: usize, const R: usize, T: Scalar, A: VecAlignment>
        = [Vector<C, T, A>; R]
    where
        MaybeVecLen<C>: VecLen,
        MaybeVecLen<R>: VecLen;
}

trait Seal {}
impl Seal for ColumnMajor {}
impl Seal for RowMajor {}
