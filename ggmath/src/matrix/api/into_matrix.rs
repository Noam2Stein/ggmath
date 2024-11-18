use into_vec::IntoVector;

use super::*;

pub trait IntoMatrix<const C: usize, const R: usize, T: Scalar>: Sized
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, T, A, M>;
}

impl<const C: usize, const R: usize, T: Scalar, A2: VecAlignment, M2: MatrixMajorAxis>
    IntoMatrix<C, R, T> for Matrix<C, R, T, A2, M2>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, T, A, M> {
        self.into_repr()
    }
}

impl<T: Scalar, R0: IntoVector<2, T>, R1: IntoVector<2, T>> IntoMatrix<2, 2, T> for (R0, R1) {
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<2, 2, T, A, M> {
        Matrix::from_rows([self.0.into_vec(), self.1.into_vec()])
    }
}
impl<T: Scalar, R0: IntoVector<3, T>, R1: IntoVector<3, T>> IntoMatrix<3, 2, T> for (R0, R1) {
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<3, 2, T, A, M> {
        Matrix::from_rows([self.0.into_vec(), self.1.into_vec()])
    }
}
impl<T: Scalar, R0: IntoVector<4, T>, R1: IntoVector<4, T>> IntoMatrix<4, 2, T> for (R0, R1) {
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, 2, T, A, M> {
        Matrix::from_rows([self.0.into_vec(), self.1.into_vec()])
    }
}

impl<T: Scalar, R0: IntoVector<2, T>, R1: IntoVector<2, T>, R2: IntoVector<2, T>>
    IntoMatrix<2, 3, T> for (R0, R1, R2)
{
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<2, 3, T, A, M> {
        Matrix::from_rows([self.0.into_vec(), self.1.into_vec(), self.2.into_vec()])
    }
}
impl<T: Scalar, R01A: VecAlignment, R01M: MatrixMajorAxis, R2: IntoVector<2, T>> IntoMatrix<2, 3, T>
    for (Matrix<2, 2, T, R01A, R01M>, R2)
{
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<2, 3, T, A, M> {
        Matrix::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.1.into_vec(),
        ])
    }
}
impl<T: Scalar, R2: IntoVector<2, T>, R12A: VecAlignment, R12M: MatrixMajorAxis> IntoMatrix<2, 3, T>
    for (R2, Matrix<2, 2, T, R12A, R12M>)
{
    #[inline(always)]
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<2, 3, T, A, M> {
        Matrix::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.1.into_vec(),
        ])
    }
}
