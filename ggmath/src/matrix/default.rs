use super::*;

impl<const C: usize, const R: usize, T: Scalar + Default, A: VecAlignment, M: MatMajorAxis> Default
    for Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    fn default() -> Self {
        Matrix::splat(T::default())
    }
}
