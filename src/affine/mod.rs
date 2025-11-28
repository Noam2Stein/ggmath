#![expect(missing_docs)]

use crate::{Alignment, Length, Matrix, Scalar, SupportedLength, Vector};

#[repr(C)]
pub struct Affine<const N: usize, T: Scalar, A: Alignment>
where
    Length<N>: SupportedLength,
{
    matrix: Matrix<N, T, A>,
    translation: Vector<N, T, A>,
}

impl<const N: usize, T: Scalar, A: Alignment> Clone for Affine<N, T, A>
where
    Length<N>: SupportedLength,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

impl<const N: usize, T: Scalar, A: Alignment> Copy for Affine<N, T, A> where
    Length<N>: SupportedLength
{
}
