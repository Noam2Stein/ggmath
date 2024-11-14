use std::fmt::{Debug, Formatter, Result};

use super::*;

impl<const N: usize, T: Scalar, A: VecAlignment> Debug for Vector<N, T, A>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.inner.fmt(f)
    }
}
