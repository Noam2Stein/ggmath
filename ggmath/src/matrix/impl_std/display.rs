use std::fmt::{self, Display, Formatter};

use super::*;

impl<const C: usize, const R: usize, T: Scalar, A: VecAlignment, M: MatrixMajorAxis> Display
    for Matrix<C, R, T, A, M>
where
    ScalarCount<C>: VecLen<C>,
    ScalarCount<R>: VecLen<R>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({})",
            self.into_rows().map(|c| c.to_string()).join(", ")
        )
    }
}
