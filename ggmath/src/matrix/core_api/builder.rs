use super::*;

pub use ggmath_proc_macros::{
    mat2, mat2c, mat2cp, mat2p, mat2x3, mat2x3c, mat2x3cp, mat2x3p, mat2x4, mat2x4c, mat2x4cp,
    mat2x4p, mat3, mat3c, mat3cp, mat3p, mat3x2, mat3x2c, mat3x2cp, mat3x2p, mat3x4, mat3x4c,
    mat3x4cp, mat3x4p, mat4, mat4c, mat4cp, mat4p, mat4x2, mat4x2c, mat4x2cp, mat4x2p, mat4x3,
    mat4x3c, mat4x3cp, mat4x3p,
};

pub trait MatrixBuilder<const C: usize, const R: usize>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    type T: Scalar;

    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, Self::T, A, M>;
}

pub trait ColumnsMatrixBuilder<const C: usize, const R: usize>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    type T: Scalar;

    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, Self::T, A, M>;
}

//
//
// Rows
//
//

impl<const C: usize, const R: usize, T: Scalar, AInput: VecAlignment, MInput: MatrixMajorAxis>
    MatrixBuilder<C, R> for Matrix<C, R, T, AInput, MInput>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, Self::T, A, M> {
        self.into_storage()
    }
}

// R = 2

impl<const C: usize, T: Scalar, AInput0: VecAlignment, AInput1: VecAlignment> MatrixBuilder<C, 2>
    for (Vector<C, T, AInput0>, Vector<C, T, AInput1>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 2, Self::T, A, M> {
        Matrix::<C, 2, T, A, M>::from_rows([self.0.into_alignment(), self.1.into_alignment()])
    }
}

// R = 3

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
    > MatrixBuilder<C, 3>
    for (
        Vector<C, T, AInput0>,
        Vector<C, T, AInput1>,
        Vector<C, T, AInput2>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 3, Self::T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 3> for (Vector<C, T, AInput0>, Matrix<C, 2, T, AInput1, MInput>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 3, Self::T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.row0().into_alignment(),
            self.1.row1().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 3> for (Matrix<C, 2, T, AInput0, MInput>, Vector<C, T, AInput1>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 3, Self::T, A, M> {
        Matrix::<C, 3, T, A, M>::from_rows([
            self.0.row0().into_alignment(),
            self.0.row1().into_alignment(),
            self.1.into_alignment(),
        ])
    }
}

// R = 4

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        AInput3: VecAlignment,
    > MatrixBuilder<C, 4>
    for (
        Vector<C, T, AInput0>,
        Vector<C, T, AInput1>,
        Vector<C, T, AInput2>,
        Vector<C, T, AInput3>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
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
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 4>
    for (
        Vector<C, T, AInput0>,
        Vector<C, T, AInput1>,
        Matrix<C, 2, T, AInput2, MInput>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.row0().into_alignment(),
            self.2.row1().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 4>
    for (
        Vector<C, T, AInput0>,
        Matrix<C, 2, T, AInput1, MInput>,
        Vector<C, T, AInput2>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.row0().into_alignment(),
            self.1.row1().into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 4>
    for (
        Matrix<C, 2, T, AInput0, MInput>,
        Vector<C, T, AInput1>,
        Vector<C, T, AInput2>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.row0().into_alignment(),
            self.0.row1().into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput0: MatrixMajorAxis,
        MInput1: MatrixMajorAxis,
    > MatrixBuilder<C, 4>
    for (
        Matrix<C, 2, T, AInput0, MInput0>,
        Matrix<C, 2, T, AInput1, MInput1>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.row0().into_alignment(),
            self.0.row1().into_alignment(),
            self.1.row0().into_alignment(),
            self.1.row1().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 4> for (Vector<C, T, AInput0>, Matrix<C, 3, T, AInput1, MInput>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.into_alignment(),
            self.1.row0().into_alignment(),
            self.1.row1().into_alignment(),
            self.1.row2().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > MatrixBuilder<C, 4> for (Matrix<C, 3, T, AInput0, MInput>, Vector<C, T, AInput1>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, 4, Self::T, A, M> {
        Matrix::<C, 4, T, A, M>::from_rows([
            self.0.row0().into_alignment(),
            self.0.row1().into_alignment(),
            self.0.row2().into_alignment(),
            self.1.into_alignment(),
        ])
    }
}

//
//
// Columns
//
//

impl<const C: usize, const R: usize, T: Scalar, AInput: VecAlignment, MInput: MatrixMajorAxis>
    ColumnsMatrixBuilder<C, R> for Matrix<C, R, T, AInput, MInput>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<C, R, Self::T, A, M> {
        self.into_storage()
    }
}

// C = 2

impl<const R: usize, T: Scalar, AInput0: VecAlignment, AInput1: VecAlignment>
    ColumnsMatrixBuilder<2, R> for (Vector<R, T, AInput0>, Vector<R, T, AInput1>)
where
    MaybeVecLen<R>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<2, R, Self::T, A, M> {
        Matrix::<2, R, T, A, M>::from_columns([self.0.into_alignment(), self.1.into_alignment()])
    }
}

// R = 3

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
    > ColumnsMatrixBuilder<3, C>
    for (
        Vector<C, T, AInput0>,
        Vector<C, T, AInput1>,
        Vector<C, T, AInput2>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    #[inline(always)]
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<3, C, Self::T, A, M> {
        Matrix::<3, C, T, A, M>::from_columns([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<3, C> for (Vector<C, T, AInput0>, Matrix<2, C, T, AInput1, MInput>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<3, C, Self::T, A, M> {
        Matrix::<3, C, T, A, M>::from_columns([
            self.0.into_alignment(),
            self.1.column0().into_alignment(),
            self.1.column1().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<3, C> for (Matrix<2, C, T, AInput0, MInput>, Vector<C, T, AInput1>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<3, C, Self::T, A, M> {
        Matrix::<3, C, T, A, M>::from_columns([
            self.0.column0().into_alignment(),
            self.0.column1().into_alignment(),
            self.1.into_alignment(),
        ])
    }
}

// R = 4

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        AInput3: VecAlignment,
    > ColumnsMatrixBuilder<4, C>
    for (
        Vector<C, T, AInput0>,
        Vector<C, T, AInput1>,
        Vector<C, T, AInput2>,
        Vector<C, T, AInput3>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
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
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<4, C>
    for (
        Vector<C, T, AInput0>,
        Vector<C, T, AInput1>,
        Matrix<2, C, T, AInput2, MInput>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
            self.0.into_alignment(),
            self.1.into_alignment(),
            self.2.column0().into_alignment(),
            self.2.column1().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<4, C>
    for (
        Vector<C, T, AInput0>,
        Matrix<2, C, T, AInput1, MInput>,
        Vector<C, T, AInput2>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
            self.0.into_alignment(),
            self.1.column0().into_alignment(),
            self.1.column1().into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        AInput2: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<4, C>
    for (
        Matrix<2, C, T, AInput0, MInput>,
        Vector<C, T, AInput1>,
        Vector<C, T, AInput2>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
            self.0.column0().into_alignment(),
            self.0.column1().into_alignment(),
            self.1.into_alignment(),
            self.2.into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput0: MatrixMajorAxis,
        MInput1: MatrixMajorAxis,
    > ColumnsMatrixBuilder<4, C>
    for (
        Matrix<2, C, T, AInput0, MInput0>,
        Matrix<2, C, T, AInput1, MInput1>,
    )
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
            self.0.column0().into_alignment(),
            self.0.column1().into_alignment(),
            self.1.column0().into_alignment(),
            self.1.column1().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<4, C> for (Vector<C, T, AInput0>, Matrix<3, C, T, AInput1, MInput>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
            self.0.into_alignment(),
            self.1.column0().into_alignment(),
            self.1.column1().into_alignment(),
            self.1.column2().into_alignment(),
        ])
    }
}

impl<
        const C: usize,
        T: Scalar,
        AInput0: VecAlignment,
        AInput1: VecAlignment,
        MInput: MatrixMajorAxis,
    > ColumnsMatrixBuilder<4, C> for (Matrix<3, C, T, AInput0, MInput>, Vector<C, T, AInput1>)
where
    MaybeVecLen<C>: VecLen,
{
    type T = T;
    fn build<A: VecAlignment, M: MatrixMajorAxis>(self) -> Matrix<4, C, Self::T, A, M> {
        Matrix::<4, C, T, A, M>::from_columns([
            self.0.column0().into_alignment(),
            self.0.column1().into_alignment(),
            self.0.column2().into_alignment(),
            self.1.into_alignment(),
        ])
    }
}
