use std::any::type_name;

use crate::{
    rectangle::{RectRepr, Rectangle, ScalarRect},
    scalar::Scalar,
    vector::{ScalarCount, VecAlignment, VecLen},
};

use super::{TestEq, TestFnDesc};

pub use ggmath_proc_macros::rect_test_assert;

impl TestFnDesc {
    pub fn rectangle<const N: usize, T: Scalar, A: VecAlignment, R>(fn_ident: &'static str) -> Self
    where
        ScalarCount<N>: VecLen,
    {
        Self(format!(
            "Rectangle::<{N}, {}, {}, {}>::{fn_ident}",
            type_name::<T>().split("::").last().unwrap_or(""),
            type_name::<A>().split("::").last().unwrap_or(""),
            type_name::<R>().split("::").last().unwrap_or(""),
        ))
    }
}

impl<
        const N: usize,
        T: ScalarRect + TestEq<TRhs>,
        A: VecAlignment,
        R: RectRepr,
        TRhs: ScalarRect,
        ARhs: VecAlignment,
        RRhs: RectRepr,
    > TestEq<Rectangle<N, TRhs, ARhs, RRhs>> for Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
{
    #[inline(always)]
    fn test_eq(&self, other: &Rectangle<N, TRhs, ARhs, RRhs>) -> bool {
        self.min().test_eq(&other.min()) && self.max().test_eq(&other.max())
    }
}
