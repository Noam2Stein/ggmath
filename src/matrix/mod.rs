#![expect(missing_docs)]

use crate::{Alignment, Length, Scalar, SupportedLength, Vector};

#[repr(transparent)]
pub struct Matrix<const N: usize, T: Scalar, A: Alignment>(MatrixRepr<N, T, A>)
where
    Length<N>: SupportedLength;

type MatrixRepr<const N: usize, T, A> = <Length<N> as SupportedLength>::Select<
    Vector<4, T, A>,
    [Vector<3, T, A>; 3],
    [Vector<4, T, A>; 4],
>;

impl<const N: usize, T: Scalar, A: Alignment> Clone for Matrix<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Copy for Matrix<N, T, A> where
    Length<N>: SupportedLength
{
}
