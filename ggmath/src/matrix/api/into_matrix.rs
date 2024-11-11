use super::*;

pub trait IntoMatrix<const C: usize, const R: usize, T: Scalar>: Sized
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, T, A, M>;
}
impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis>
    IntoMatrix<C, R, T> for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn into_matrix<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, T, A, M> {
        self
    }
}
