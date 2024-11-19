use crate::builder::*;

use super::*;

pub use ggmath_proc_macros::{
    mat2, mat2a, mat2x3, mat2x3a, mat2x4, mat2x4a, mat3, mat3a, mat3x2, mat3x2a, mat3x4, mat3x4a,
    mat4, mat4a, mat4x2, mat4x2a, mat4x3, mat4x3a,
};

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
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    #[inline(always)]
    fn build(self) -> Matrix<C, R, T, A, M> {
        self.into_repr()
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 2, T, A, M>>
    for (Vector<C, T, A>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 2, T, A, M> {
        Matrix::<C, 2, T, A, M>::from_rows([self.0.into_alignment(), self.1.into_alignment()])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 3, T, A, M>>
    for (Vector<C, T, A>, Vector<C, T, A>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen<C>,
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
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 3, T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.index_row(0).into_alignment(),
            self.1.index_row(1).into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 3, T, A, M>>
    for (Matrix<C, 2, T, A, M>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 3, T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
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
    ScalarCount<C>: VecLen<C>,
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
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.index_row(0).into_alignment(),
            self.2.index_row(1).into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Vector<C, T, A>, Matrix<C, 2, T, A, M>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.index_row(0).into_alignment(),
            self.1.index_row(1).into_alignment(),
            self.2.into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Matrix<C, 2, T, A, M>, Vector<C, T, A>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Vector<C, T, A>, Matrix<C, 3, T, A, M>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.index_row(0).into_alignment(),
            self.1.index_row(1).into_alignment(),
            self.1.index_row(2).into_alignment(),
        ])
    }
}
impl<const C: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Builder<Matrix<C, 4, T, A, M>>
    for (Matrix<C, 3, T, A, M>, Vector<C, T, A>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.0.index_row(2).into_alignment(),
            self.1.into_alignment(),
        ])
    }
}
