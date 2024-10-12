use std::fmt::{Debug, Formatter, Result};

use super::*;

impl<const N: usize, S: VecStorage, T: Scalar> Debug for Vector<N, S, T>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.inner.fmt(f)
    }
}
