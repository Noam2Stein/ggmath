use std::fmt::{self, Formatter};

use super::*;

// Ensure Rectangle is Construct

const _: () = {
    fn ensure_rectangle_is_construct<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr>()
    where
        ScalarCount<N>: VecLen,
    {
        fn wreck_it_ralph<RogerCraigSmith: Construct>() {}

        wreck_it_ralph::<Rectangle<N, T, A, R>>();
    }
};

// Clone + Copy

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Clone for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn clone(&self) -> Self {
        Self { inner: self.inner }
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Copy for Rectangle<N, T, A, R> where
    ScalarCount<N>: VecLen
{
}

// PartialEq + Eq

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> PartialEq for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.inner.eq(&other.inner)
    }
}

impl<const N: usize, T: ScalarNum + Eq, A: VecAlignment, R: RectRepr> Eq for Rectangle<N, T, A, R> where
    ScalarCount<N>: VecLen
{
}

// Debug + Display

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> fmt::Debug
    for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        R::debug_fmt(*self, f)
    }
}

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> fmt::Display
    for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        R::display_fmt(*self, f)
    }
}
