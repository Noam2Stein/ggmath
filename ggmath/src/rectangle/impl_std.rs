use std::fmt::{self, Formatter};

use crate::construct::Construct;

use super::*;

const _: () = {
    fn ensure_rectangle_is_construct<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr>()
    where
        ScalarCount<N>: VecLen<N>,
    {
        fn wreck_it_ralph<RogerCraigSmith: Construct>() {}

        wreck_it_ralph::<Rectangle<N, T, A, R>>();
    }
};

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Clone for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Copy for Rectangle<N, T, A, R> where
    ScalarCount<N>: VecLen<N>
{
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> fmt::Debug
    for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> fmt::Display
    for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        R::display_fmt(*self, f)
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> PartialEq for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}
