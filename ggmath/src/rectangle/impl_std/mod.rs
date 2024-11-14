use crate::construct::Construct;

use super::*;

mod copy;
mod debug;
mod display;
mod partial_eq;

const _: () = {
    fn ensure_rectangle_is_construct<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr>()
    where
        ScalarCount<N>: VecLen<N>,
    {
        fn wreck_it_ralph<RogerCraigSmith: Construct>() {}

        wreck_it_ralph::<Rectangle<N, T, A, R>>();
    }
};
