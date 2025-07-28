use derive_where::derive_where;

use super::*;

#[allow(private_bounds)]
pub unsafe trait RectRepr: RectReprPriv + Sized + 'static {}

pub struct RectCornered;
pub struct RectCentered;
pub struct RectMinMaxed;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub fn into_cornered(self) -> Rectangle<N, T, A, RectCornered> {
        self.into_repr()
    }

    #[inline(always)]
    pub fn into_centered(self) -> Rectangle<N, T, A, RectCentered> {
        self.into_repr()
    }

    #[inline(always)]
    pub fn into_min_maxed(self) -> Rectangle<N, T, A, RectMinMaxed> {
        self.into_repr()
    }

    #[inline(always)]
    pub fn into_repr<ROutput: RectRepr>(self) -> Rectangle<N, T, A, ROutput> {
        match self.resolve() {
            ResolvedRectangle::Centered(rect) => {
                Rectangle::from_center_extents(rect.inner.center, rect.inner.extents)
            }
            ResolvedRectangle::Cornered(rect) => {
                Rectangle::from_min_size(rect.inner.min, rect.inner.size)
            }
            ResolvedRectangle::MinMaxed(rect) => {
                Rectangle::from_min_max(rect.inner.min, rect.inner.max)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::rectangle) enum RectReprEnum {
    Cornered,
    Centered,
    MinMaxed,
}

pub(in crate::rectangle) trait RectReprPriv {
    const ENUM: RectReprEnum;

    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>: Construct + PartialEq
    where
        Usize<N>: VecLen;
}

unsafe impl RectRepr for RectCornered {}
unsafe impl RectRepr for RectCentered {}
unsafe impl RectRepr for RectMinMaxed {}

impl RectReprPriv for RectCornered {
    const ENUM: RectReprEnum = RectReprEnum::Cornered;

    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>
        = InnerCorneredRect<N, T, A>
    where
        Usize<N>: VecLen;
}
impl RectReprPriv for RectCentered {
    const ENUM: RectReprEnum = RectReprEnum::Centered;

    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>
        = InnerCenteredRect<N, T, A>
    where
        Usize<N>: VecLen;
}
impl RectReprPriv for RectMinMaxed {
    const ENUM: RectReprEnum = RectReprEnum::MinMaxed;

    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>
        = InnerMinMaxedRect<N, T, A>
    where
        Usize<N>: VecLen;
}

#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(in crate::rectangle) struct InnerCorneredRect<const N: usize, T: RectScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub size: Vector<N, T, A>,
}

#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(in crate::rectangle) struct InnerMinMaxedRect<const N: usize, T: RectScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub max: Vector<N, T, A>,
}

#[derive_where(Clone, Copy, PartialEq)]
#[derive_where(Eq; T)]
pub(in crate::rectangle) struct InnerCenteredRect<const N: usize, T: RectScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}
