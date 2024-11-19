use crate::{
    construct::*,
    scalar::*,
    vector::{alignment::*, length::*, *},
};

pub mod repr;
use repr::*;

mod impl_std;
#[allow(unused_imports)]
pub use impl_std::*;

pub struct Rectangle<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr>
where
    ScalarCount<N>: VecLen<N>,
{
    inner: R::InnerRectangle<N, T, A>,
}

pub type Rect2<T> = Rectangle<2, T, VecAligned, RectCornered>;
pub type Rect3<T> = Rectangle<3, T, VecAligned, RectCornered>;
pub type Rect4<T> = Rectangle<4, T, VecAligned, RectCornered>;
pub type Rect2P<T> = Rectangle<2, T, VecPacked, RectCornered>;
pub type Rect3P<T> = Rectangle<3, T, VecPacked, RectCornered>;
pub type Rect4P<T> = Rectangle<4, T, VecPacked, RectCornered>;

pub type Rect2C<T> = Rectangle<2, T, VecAligned, RectCentered>;
pub type Rect3C<T> = Rectangle<3, T, VecAligned, RectCentered>;
pub type Rect4C<T> = Rectangle<4, T, VecAligned, RectCentered>;
pub type Rect2CP<T> = Rectangle<2, T, VecPacked, RectCentered>;
pub type Rect3CP<T> = Rectangle<3, T, VecPacked, RectCentered>;
pub type Rect4CP<T> = Rectangle<4, T, VecPacked, RectCentered>;

pub type Rect2M<T> = Rectangle<2, T, VecAligned, RectMinMaxed>;
pub type Rect3M<T> = Rectangle<3, T, VecAligned, RectMinMaxed>;
pub type Rect4M<T> = Rectangle<4, T, VecAligned, RectMinMaxed>;
pub type Rect2MP<T> = Rectangle<2, T, VecPacked, RectMinMaxed>;
pub type Rect3MP<T> = Rectangle<3, T, VecPacked, RectMinMaxed>;
pub type Rect4MP<T> = Rectangle<4, T, VecPacked, RectMinMaxed>;

impl<const N: usize, T: ScalarNum, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen<N>,
{
    #[inline(always)]
    pub fn from_min_size(min: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        R::from_min_size(min, size)
    }
    #[inline(always)]
    pub fn from_max_size(max: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        R::from_max_size(max, size)
    }
    #[inline(always)]
    pub fn from_center_size(center: Vector<N, T, A>, size: Vector<N, T, A>) -> Self {
        R::from_center_size(center, size)
    }
    #[inline(always)]
    pub fn from_min_extents(min: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        R::from_min_extents(min, extents)
    }
    #[inline(always)]
    pub fn from_max_extents(max: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        R::from_max_extents(max, extents)
    }
    #[inline(always)]
    pub fn from_center_extents(center: Vector<N, T, A>, extents: Vector<N, T, A>) -> Self {
        R::from_center_extents(center, extents)
    }
    #[inline(always)]
    pub fn from_min_max(min: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        R::from_min_max(min, max)
    }
    #[inline(always)]
    pub fn from_min_center(min: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        R::from_min_max(min, max)
    }
    #[inline(always)]
    pub fn from_center_max(center: Vector<N, T, A>, max: Vector<N, T, A>) -> Self {
        R::from_center_max(center, max)
    }

    #[inline(always)]
    pub fn min(self) -> Vector<N, T, A> {
        R::min(self)
    }
    #[inline(always)]
    pub fn max(self) -> Vector<N, T, A> {
        R::max(self)
    }
    #[inline(always)]
    pub fn center(self) -> Vector<N, T, A> {
        R::center(self)
    }
    #[inline(always)]
    pub fn size(self) -> Vector<N, T, A> {
        R::size(self)
    }
    #[inline(always)]
    pub fn extents(self) -> Vector<N, T, A> {
        R::extents(self)
    }

    #[inline(always)]
    pub fn intersects(self, other: Rectangle<N, T, impl VecAlignment, impl RectRepr>) -> bool {
        R::intersects(self, other)
    }
    #[inline(always)]
    pub fn intersection(
        self,
        other: Rectangle<N, T, impl VecAlignment, impl RectRepr>,
    ) -> Option<Self> {
        R::intersection(self, other)
    }
}
