use crate::builder::*;

use super::*;

pub use ggmath_proc_macros::{
    mat2, mat2c, mat2cp, mat2p, mat2x3, mat2x3c, mat2x3cp, mat2x3p, mat2x4, mat2x4c, mat2x4cp,
    mat2x4p, mat3, mat3c, mat3cp, mat3p, mat3x2, mat3x2c, mat3x2cp, mat3x2p, mat3x4, mat3x4c,
    mat3x4cp, mat3x4p, mat4, mat4c, mat4cp, mat4p, mat4x2, mat4x2c, mat4x2cp, mat4x2p, mat4x3,
    mat4x3c, mat4x3cp, mat4x3p,
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
            self.1.get_row(0).unwrap().into_alignment(),
            self.1.get_row(1).unwrap().into_alignment(),
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
            self.2.get_row(0).unwrap().into_alignment(),
            self.2.get_row(1).unwrap().into_alignment(),
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
            self.1.get_row(0).unwrap().into_alignment(),
            self.1.get_row(1).unwrap().into_alignment(),
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
    ScalarCount<C>: VecLen<C>,
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
    ScalarCount<C>: VecLen<C>,
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
