use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const C: usize, const R: usize, T: Scalar + Debug, A: VecAlignment, M: MatrixMajorAxis> Debug
    for Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({})",
            self.into_rows().map(|c| format!("{c:?}")).join(", ")
        )
    }
}

impl<const C: usize, const R: usize, T: Scalar + Display, A: VecAlignment, M: MatrixMajorAxis>
    Display for Matrix<C, R, T, A, M>
where
    MaybeVecLen<C>: VecLen,
    MaybeVecLen<R>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "({})",
            self.into_rows().map(|c| c.to_string()).join(", ")
        )
    }
}
