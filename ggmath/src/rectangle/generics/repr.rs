use std::{
    any::{type_name, TypeId},
    mem::{transmute, transmute_copy},
};

use super::*;

#[allow(private_bounds)]
pub trait RectRepr: Seal + Sized + 'static {
    type InnerRectangle<const N: usize, T: ScalarRect, A: VecAlignment>: Construct
    where
        ScalarCount<N>: VecLen;
}

pub struct RectCornered;
pub struct RectCentered;
pub struct RectMinMaxed;

pub enum ReprResolvedRectangle<const N: usize, T: ScalarRect, A: VecAlignment>
where
    ScalarCount<N>: VecLen,
{
    Cornered(Rectangle<N, T, A, RectCornered>),
    Centered(Rectangle<N, T, A, RectCentered>),
    MinMaxed(Rectangle<N, T, A, RectMinMaxed>),
}
pub enum ReprResolvedRectangleRef<'a, const N: usize, T: ScalarRect, A: VecAlignment>
where
    ScalarCount<N>: VecLen,
{
    Cornered(&'a Rectangle<N, T, A, RectCornered>),
    Centered(&'a Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a Rectangle<N, T, A, RectMinMaxed>),
}
pub enum ReprResolvedRectangleMut<'a, const N: usize, T: ScalarRect, A: VecAlignment>
where
    ScalarCount<N>: VecLen,
{
    Cornered(&'a mut Rectangle<N, T, A, RectCornered>),
    Centered(&'a mut Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a mut Rectangle<N, T, A, RectMinMaxed>),
}

impl<const N: usize, T: ScalarRect, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    ScalarCount<N>: VecLen,
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
                Rectangle::from_center_extents(rect.inner[0], rect.inner[1])
            }
            ReprResolvedRectangle::Cornered(rect) => {
                Rectangle::from_min_size(rect.inner[0], rect.inner[1])
            }
            ReprResolvedRectangle::MinMaxed(rect) => {
                Rectangle::from_min_max(rect.inner[0], rect.inner[1])
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

impl RectRepr for RectCornered {
    type InnerRectangle<const N: usize, T: ScalarRect, A: VecAlignment> =
    [Vector<N, T, A>; 2] where
        ScalarCount<N>: VecLen;
}

impl RectRepr for RectCentered {
    type InnerRectangle<const N: usize, T: ScalarRect, A: VecAlignment> =
    [Vector<N, T, A>; 2] where
        ScalarCount<N>: VecLen;
}

impl RectRepr for RectMinMaxed {
    type InnerRectangle<const N: usize, T: ScalarRect, A: VecAlignment> =
        [Vector<N, T, A>; 2] where
        ScalarCount<N>: VecLen;
}

trait Seal {}
impl Seal for RectCornered {}
impl Seal for RectCentered {}
impl Seal for RectMinMaxed {}
