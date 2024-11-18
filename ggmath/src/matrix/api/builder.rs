use crate::builder::*;

use super::*;

impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 2, T, AOutput, MOutput>> for (Vector<C, T, A0>, Vector<C, T, A1>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 2, T, AOutput, MOutput> {
        Matrix::<C, 2, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        A2: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 3, T, AOutput, MOutput>>
    for (Vector<C, T, A0>, Vector<C, T, A1>, Vector<C, T, A2>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 3, T, AOutput, MOutput> {
        Matrix::<C, 3, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        M1: MatrixMajorAxis,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 3, T, AOutput, MOutput>> for (Vector<C, T, A0>, Matrix<C, 2, T, A1, M1>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 3, T, AOutput, MOutput> {
        Matrix::<C, 3, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.index_row(0).into_alignment(),
            self.1.index_row(1).into_alignment(),
        ])
    }
}
impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        M0: MatrixMajorAxis,
        A1: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 3, T, AOutput, MOutput>> for (Matrix<C, 2, T, A0, M0>, Vector<C, T, A1>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 3, T, AOutput, MOutput> {
        Matrix::<C, 3, T, AOutput, MOutput>::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.1.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        A2: VecAlignment,
        A3: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 4, T, AOutput, MOutput>>
    for (
        Vector<C, T, A0>,
        Vector<C, T, A1>,
        Vector<C, T, A2>,
        Vector<C, T, A3>,
    )
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, AOutput, MOutput> {
        Matrix::<C, 4, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
            self.3.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        A2: VecAlignment,
        M2: MatrixMajorAxis,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 4, T, AOutput, MOutput>>
    for (Vector<C, T, A0>, Vector<C, T, A1>, Matrix<C, 2, T, A2, M2>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, AOutput, MOutput> {
        Matrix::<C, 4, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.index_row(0).into_alignment(),
            self.2.index_row(1).into_alignment(),
        ])
    }
}
impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        M1: MatrixMajorAxis,
        A2: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 4, T, AOutput, MOutput>>
    for (Vector<C, T, A0>, Matrix<C, 2, T, A1, M1>, Vector<C, T, A2>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, AOutput, MOutput> {
        Matrix::<C, 4, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.index_row(0).into_alignment(),
            self.1.index_row(1).into_alignment(),
            self.2.into_alignment(),
        ])
    }
}
impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        M0: MatrixMajorAxis,
        A1: VecAlignment,
        A2: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 4, T, AOutput, MOutput>>
    for (Matrix<C, 2, T, A0, M0>, Vector<C, T, A1>, Vector<C, T, A2>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, AOutput, MOutput> {
        Matrix::<C, 4, T, AOutput, MOutput>::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        A1: VecAlignment,
        M1: MatrixMajorAxis,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 4, T, AOutput, MOutput>> for (Vector<C, T, A0>, Matrix<C, 3, T, A1, M1>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, AOutput, MOutput> {
        Matrix::<C, 4, T, AOutput, MOutput>::from_rows([
            self.0.into_alignment(),
            self.1.index_row(0).into_alignment(),
            self.1.index_row(1).into_alignment(),
            self.1.index_row(2).into_alignment(),
        ])
    }
}
impl<
        const C: usize,
        T: Scalar,
        A0: VecAlignment,
        M0: MatrixMajorAxis,
        A1: VecAlignment,
        AOutput: VecAlignment,
        MOutput: MatrixMajorAxis,
    > Builder<Matrix<C, 4, T, AOutput, MOutput>> for (Matrix<C, 3, T, A0, M0>, Vector<C, T, A1>)
where
    ScalarCount<C>: VecLen<C>,
{
    fn build(self) -> Matrix<C, 4, T, AOutput, MOutput> {
        Matrix::<C, 4, T, AOutput, MOutput>::from_rows([
            self.0.index_row(0).into_alignment(),
            self.0.index_row(1).into_alignment(),
            self.0.index_row(2).into_alignment(),
            self.1.into_alignment(),
        ])
    }
}
