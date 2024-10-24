use std::fmt::{Debug, Formatter, Result};

use super::*;

impl<const N: usize, T: Scalar, S: VecAlignment> Debug for Vector<N, T, S>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.inner.fmt(f)
    }
}
