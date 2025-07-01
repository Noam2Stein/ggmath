use std::{
    any::{TypeId, type_name},
    mem::{transmute, transmute_copy},
};

use derive_where::derive_where;

use super::*;

#[allow(private_bounds)]
pub trait RectRepr: InnerRectRepr + Sized + 'static {}

pub struct RectCornered;
pub struct RectCentered;
pub struct RectMinMaxed;

pub enum ReprResolvedRectangle<const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    Cornered(Rectangle<N, T, A, RectCornered>),
    Centered(Rectangle<N, T, A, RectCentered>),
    MinMaxed(Rectangle<N, T, A, RectMinMaxed>),
}
pub enum ReprResolvedRectangleRef<'a, const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    Cornered(&'a Rectangle<N, T, A, RectCornered>),
    Centered(&'a Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a Rectangle<N, T, A, RectMinMaxed>),
}
pub enum ReprResolvedRectangleMut<'a, const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    Cornered(&'a mut Rectangle<N, T, A, RectCornered>),
    Centered(&'a mut Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a mut Rectangle<N, T, A, RectMinMaxed>),
}

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
        match self.resolve_repr() {
            ReprResolvedRectangle::Centered(rect) => {
                Rectangle::from_center_extents(rect.inner.center, rect.inner.extents)
            }
            ReprResolvedRectangle::Cornered(rect) => {
                Rectangle::from_min_size(rect.inner.min, rect.inner.size)
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                Rectangle::from_min_max(rect.inner.min, rect.inner.max)
            }
        }
    }

    #[inline(always)]
    pub fn resolve_repr(self) -> ReprResolvedRectangle<N, T, A> {
        unsafe {
            if TypeId::of::<R>() == TypeId::of::<RectCornered>() {
                ReprResolvedRectangle::Cornered(transmute_copy(&self))
            } else if TypeId::of::<R>() == TypeId::of::<RectCentered>() {
                ReprResolvedRectangle::Centered(transmute_copy(&self))
            } else if TypeId::of::<R>() == TypeId::of::<RectMinMaxed>() {
                ReprResolvedRectangle::MinMaxed(transmute_copy(&self))
            } else {
                panic!("invalid RectRepr: {}", type_name::<R>())
            }
        }
    }
    #[inline(always)]
    pub fn resolve_repr_ref(&self) -> ReprResolvedRectangleRef<N, T, A> {
        unsafe {
            if TypeId::of::<R>() == TypeId::of::<RectCornered>() {
                ReprResolvedRectangleRef::Cornered(transmute(self))
            } else if TypeId::of::<R>() == TypeId::of::<RectCentered>() {
                ReprResolvedRectangleRef::Centered(transmute(self))
            } else if TypeId::of::<R>() == TypeId::of::<RectMinMaxed>() {
                ReprResolvedRectangleRef::MinMaxed(transmute(self))
            } else {
                panic!("invalid RectRepr: {}", type_name::<R>())
            }
        }
    }
    #[inline(always)]
    pub fn resolve_repr_mut(&mut self) -> ReprResolvedRectangleMut<N, T, A> {
        unsafe {
            if TypeId::of::<R>() == TypeId::of::<RectCornered>() {
                ReprResolvedRectangleMut::Cornered(transmute(self))
            } else if TypeId::of::<R>() == TypeId::of::<RectCentered>() {
                ReprResolvedRectangleMut::Centered(transmute(self))
            } else if TypeId::of::<R>() == TypeId::of::<RectMinMaxed>() {
                ReprResolvedRectangleMut::MinMaxed(transmute(self))
            } else {
                panic!("invalid RectRepr: {}", type_name::<R>())
            }
        }
    }

    #[inline(always)]
    pub fn from_resolved_repr_fns(
        f_cornered: impl FnOnce() -> Rectangle<N, T, A, RectCornered>,
        f_centered: impl FnOnce() -> Rectangle<N, T, A, RectCentered>,
        f_min_maxed: impl FnOnce() -> Rectangle<N, T, A, RectMinMaxed>,
    ) -> Self {
        unsafe {
            if TypeId::of::<R>() == TypeId::of::<RectCornered>() {
                transmute_copy(&f_cornered())
            } else if TypeId::of::<R>() == TypeId::of::<RectCentered>() {
                transmute_copy(&f_centered())
            } else if TypeId::of::<R>() == TypeId::of::<RectMinMaxed>() {
                transmute_copy(&f_min_maxed())
            } else {
                panic!("invalid RectRepr: {}", type_name::<R>())
            }
        }
    }
}

pub(in crate::rectangle) trait InnerRectRepr {
    type InnerRectangle<const N: usize, T: RectScalar, A: VecAlignment>: Construct
    where
        MaybeVecLen<N>: VecLen;
}

impl RectRepr for RectCornered {}
impl RectRepr for RectCentered {}
impl RectRepr for RectMinMaxed {}

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
