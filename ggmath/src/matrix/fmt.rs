use std::fmt::{Debug, Display, Formatter, Result};

use super::*;

impl<const C: usize, const R: usize, T: Scalar + Debug, A: VecAlignment, M: MatMajorAxis> Debug
    for Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.rows().map(|c| format!("{c:?}")).join(", "))
    }
}

impl<const C: usize, const R: usize, T: Scalar + Display, A: VecAlignment, M: MatMajorAxis>
    Display for Matrix<C, R, T, A, M>
where
    Usize<C>: VecLen,
    Usize<R>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "({})", self.rows().map(|c| c.to_string()).join(", "))
    }
}
