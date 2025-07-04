use derive_where::derive_where;

use super::*;

#[allow(private_bounds)]
pub unsafe trait RectRepr: InnerRectRepr + Sized + 'static {
    const IS_CORNERED: bool;
    const IS_CENTERED: bool;
}

pub struct RectCornered;
pub struct RectCentered;
pub struct RectMinMaxed;

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    MaybeVecLen<N>: VecLen,
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

pub(in crate::rectangle) trait InnerRectRepr {
    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>: Construct
    where
        MaybeVecLen<N>: VecLen;
}

unsafe impl RectRepr for RectCornered {
    const IS_CORNERED: bool = true;
    const IS_CENTERED: bool = false;
}
unsafe impl RectRepr for RectCentered {
    const IS_CORNERED: bool = false;
    const IS_CENTERED: bool = true;
}
unsafe impl RectRepr for RectMinMaxed {
    const IS_CORNERED: bool = false;
    const IS_CENTERED: bool = false;
}

impl InnerRectRepr for RectCornered {
    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>
        = InnerCorneredRect<N, T, A>
    where
        MaybeVecLen<N>: VecLen;
}
impl InnerRectRepr for RectCentered {
    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>
        = InnerCenteredRect<N, T, A>
    where
        MaybeVecLen<N>: VecLen;
}
impl InnerRectRepr for RectMinMaxed {
    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>
        = InnerMinMaxedRect<N, T, A>
    where
        MaybeVecLen<N>: VecLen;
}

#[derive_where(Clone, Copy)]
#[derive_where(PartialEq, Eq; T)]
pub(in crate::rectangle) struct InnerCorneredRect<const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub size: Vector<N, T, A>,
}

#[derive_where(Clone, Copy)]
#[derive_where(PartialEq, Eq; T)]
pub(in crate::rectangle) struct InnerMinMaxedRect<const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    pub min: Vector<N, T, A>,
    pub max: Vector<N, T, A>,
}

#[derive_where(Clone, Copy)]
#[derive_where(PartialEq, Eq; T)]
pub(in crate::rectangle) struct InnerCenteredRect<const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    pub center: Vector<N, T, A>,
    pub extents: Vector<N, T, A>,
}
