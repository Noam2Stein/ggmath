use std::mem::{transmute, transmute_copy};

use super::*;

pub enum ResolvedRectangle<const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    Cornered(Rectangle<N, T, A, RectCornered>),
    Centered(Rectangle<N, T, A, RectCentered>),
    MinMaxed(Rectangle<N, T, A, RectMinMaxed>),
}

pub enum ResolvedRectangleRef<'a, const N: usize, T: RectScalar, A: VecAlignment>
where
    MaybeVecLen<N>: VecLen,
{
    Cornered(&'a Rectangle<N, T, A, RectCornered>),
    Centered(&'a Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a Rectangle<N, T, A, RectMinMaxed>),
}

pub enum ResolvedRectangleMut<'a, const N: usize, T: RectScalar, A: VecAlignment>
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
    pub const fn resolve(self) -> ResolvedRectangle<N, T, A> {
        unsafe {
            if R::IS_CORNERED {
                ResolvedRectangle::Cornered(transmute_copy::<
                    Rectangle<N, T, A, R>,
                    Rectangle<N, T, A, RectCornered>,
                >(&self))
            } else if R::IS_CENTERED {
                ResolvedRectangle::Centered(transmute_copy::<
                    Rectangle<N, T, A, R>,
                    Rectangle<N, T, A, RectCentered>,
                >(&self))
            } else {
                ResolvedRectangle::MinMaxed(transmute_copy::<
                    Rectangle<N, T, A, R>,
                    Rectangle<N, T, A, RectMinMaxed>,
                >(&self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedRectangleRef<N, T, A> {
        unsafe {
            if R::IS_CORNERED {
                ResolvedRectangleRef::Cornered(transmute::<
                    &Rectangle<N, T, A, R>,
                    &Rectangle<N, T, A, RectCornered>,
                >(self))
            } else if R::IS_CENTERED {
                ResolvedRectangleRef::Centered(transmute::<
                    &Rectangle<N, T, A, R>,
                    &Rectangle<N, T, A, RectCentered>,
                >(self))
            } else {
                ResolvedRectangleRef::MinMaxed(transmute::<
                    &Rectangle<N, T, A, R>,
                    &Rectangle<N, T, A, RectMinMaxed>,
                >(self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_repr_mut(&mut self) -> ResolvedRectangleMut<N, T, A> {
        unsafe {
            if R::IS_CORNERED {
                ResolvedRectangleMut::Cornered(transmute::<
                    &mut Rectangle<N, T, A, R>,
                    &mut Rectangle<N, T, A, RectCornered>,
                >(self))
            } else if R::IS_CENTERED {
                ResolvedRectangleMut::Centered(transmute::<
                    &mut Rectangle<N, T, A, R>,
                    &mut Rectangle<N, T, A, RectCentered>,
                >(self))
            } else {
                ResolvedRectangleMut::MinMaxed(transmute::<
                    &mut Rectangle<N, T, A, R>,
                    &mut Rectangle<N, T, A, RectMinMaxed>,
                >(self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolved(
        cornered: Rectangle<N, T, A, RectCornered>,
        centered: Rectangle<N, T, A, RectCentered>,
        min_maxed: Rectangle<N, T, A, RectMinMaxed>,
    ) -> Self {
        if R::IS_CORNERED {
            unsafe {
                transmute_copy::<Rectangle<N, T, A, RectCornered>, Rectangle<N, T, A, R>>(&cornered)
            }
        } else if R::IS_CENTERED {
            unsafe {
                transmute_copy::<Rectangle<N, T, A, RectCentered>, Rectangle<N, T, A, R>>(&centered)
            }
        } else {
            unsafe {
                transmute_copy::<Rectangle<N, T, A, RectMinMaxed>, Rectangle<N, T, A, R>>(
                    &min_maxed,
                )
            }
        }
    }
}
