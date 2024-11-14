use std::fmt::{Debug, Formatter, Result};

use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Debug for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.inner.fmt(f)
    }
}
