use std::fmt::{self, Display, Formatter};

use super::*;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Display for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        R::display_fmt(*self, f)
    }
}
