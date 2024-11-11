use std::fmt::{Debug, Formatter, Result};

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Debug
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.inner.fmt(f)
    }
}
