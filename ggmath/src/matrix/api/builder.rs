use crate::builder::*;

use super::*;

impl<
        const C: usize,
        const R: usize,
        T: Scalar,
        AInput: VecAlignment,
        MInput: MatrixMajorAxis,
        A: VecAlignment,
        M: MatrixMajorAxis,
    > Builder<Matrix<C, R, T, A, M>> for Matrix<C, R, T, AInput, MInput>
where
    ScalarCount<C>: VecLen,
    ScalarCount<R>: VecLen,
{
    #[inline(always)]
    fn build(self) -> Matrix<C, R, T, A, M> {
        self.into_storage()
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 2, T, A, M>>
    for (Vector<C, T, A>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 2, T, A, M> {
        Matrix::<C, 2, T, A, M>::from_rows([self.0.into_alignment(), self.1.into_alignment()])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 3, T, A, M>>
    for (Vector<C, T, A>, Vector<C, T, A>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 3, T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 3, T, A, M>>
    for (Vector<C, T, A>, Matrix<C, 2, T, A, M>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 3, T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.get_row(0).unwrap().into_alignment(),
            self.1.get_row(1).unwrap().into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 3, T, A, M>>
    for (Matrix<C, 2, T, A, M>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 3, T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.get_row(0).unwrap().into_alignment(),
            self.0.get_row(1).unwrap().into_alignment(),
            self.1.into_alignment(),
        ])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (
        Vector<C, T, A>,
        Vector<C, T, A>,
        Vector<C, T, A>,
        Vector<C, T, A>,
    )
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
            self.3.into_alignment(),
        ])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Vector<C, T, A>, Vector<C, T, A>, Matrix<C, 2, T, A, M>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.get_row(0).unwrap().into_alignment(),
            self.2.get_row(1).unwrap().into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Vector<C, T, A>, Matrix<C, 2, T, A, M>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.get_row(0).unwrap().into_alignment(),
            self.1.get_row(1).unwrap().into_alignment(),
            self.2.into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Matrix<C, 2, T, A, M>, Vector<C, T, A>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.get_row(0).unwrap().into_alignment(),
            self.0.get_row(1).unwrap().into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Vector<C, T, A>, Matrix<C, 3, T, A, M>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.get_row(0).unwrap().into_alignment(),
            self.1.get_row(1).unwrap().into_alignment(),
            self.1.get_row(2).unwrap().into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Matrix<C, 3, T, A, M>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.get_row(0).unwrap().into_alignment(),
            self.0.get_row(1).unwrap().into_alignment(),
            self.0.get_row(2).unwrap().into_alignment(),
            self.1.into_alignment(),
        ])
    }
}
