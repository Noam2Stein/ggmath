use std::array;

use crate::construct::InnerConstruct;

use super::*;

mod centered;
mod cornered;
mod min_maxed;
pub use centered::*;
pub use cornered::*;
pub use min_maxed::*;

pub trait RectRepr: Sized {
    type InnerRectangle<const N: usize, T: ScalarNum, A: VecAlignment>: InnerConstruct + PartialEq
    where
        ScalarCount<N>: VecLen<N>;

    fn from_min_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_max_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_center_size<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        size: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_min_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_max_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        max: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_center_extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        extents: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_min_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_min_center<const N: usize, T: ScalarNum, A: VecAlignment>(
        min: Vector<N, T, A>,
        center: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;
    fn from_center_max<const N: usize, T: ScalarNum, A: VecAlignment>(
        center: Vector<N, T, A>,
        max: Vector<N, T, A>,
    ) -> Rectangle<N, T, A, Self>
    where
        ScalarCount<N>: VecLen<N>;

    fn min<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn max<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn center<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn size<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;
    fn extents<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
    ) -> Vector<N, T, A>
    where
        ScalarCount<N>: VecLen<N>;

    fn intersects<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> bool
    where
        ScalarCount<N>: VecLen<N>;
    fn intersection<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> Option<Rectangle<N, T, A, Self>>
    where
        ScalarCount<N>: VecLen<N>;

    fn display_fmt<const N: usize, T: ScalarNum, A: VecAlignment>(
        rect: Rectangle<N, T, A, Self>,
        f: &mut std::fmt::Formatter,
    ) -> std::fmt::Result
    where
        ScalarCount<N>: VecLen<N>;
}
