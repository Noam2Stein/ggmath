use std::mem::{transmute, transmute_copy};

use super::*;

pub enum ResolvedRectangle<const N: usize, T: RectScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(Rectangle<N, T, A, RectCornered>),
    Centered(Rectangle<N, T, A, RectCentered>),
    MinMaxed(Rectangle<N, T, A, RectMinMaxed>),
}

pub enum ResolvedRectangleRef<'a, const N: usize, T: RectScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(&'a Rectangle<N, T, A, RectCornered>),
    Centered(&'a Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a Rectangle<N, T, A, RectMinMaxed>),
}

pub enum ResolvedRectangleMut<'a, const N: usize, T: RectScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(&'a mut Rectangle<N, T, A, RectCornered>),
    Centered(&'a mut Rectangle<N, T, A, RectCentered>),
    MinMaxed(&'a mut Rectangle<N, T, A, RectMinMaxed>),
}

impl<const N: usize, T: RectScalar, A: VecAlignment, R: RectRepr> Rectangle<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedRectangle<N, T, A> {
        unsafe {
            match R::ENUM {
                RectReprEnum::Cornered => ResolvedRectangle::Cornered(transmute_copy::<
                    Rectangle<N, T, A, R>,
                    Rectangle<N, T, A, RectCornered>,
                >(&self)),

                RectReprEnum::Centered => ResolvedRectangle::Centered(transmute_copy::<
                    Rectangle<N, T, A, R>,
                    Rectangle<N, T, A, RectCentered>,
                >(&self)),

                RectReprEnum::MinMaxed => ResolvedRectangle::MinMaxed(transmute_copy::<
                    Rectangle<N, T, A, R>,
                    Rectangle<N, T, A, RectMinMaxed>,
                >(&self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedRectangleRef<N, T, A> {
        unsafe {
            match R::ENUM {
                RectReprEnum::Cornered => ResolvedRectangleRef::Cornered(transmute::<
                    &Rectangle<N, T, A, R>,
                    &Rectangle<N, T, A, RectCornered>,
                >(self)),

                RectReprEnum::Centered => ResolvedRectangleRef::Centered(transmute::<
                    &Rectangle<N, T, A, R>,
                    &Rectangle<N, T, A, RectCentered>,
                >(self)),

                RectReprEnum::MinMaxed => ResolvedRectangleRef::MinMaxed(transmute::<
                    &Rectangle<N, T, A, R>,
                    &Rectangle<N, T, A, RectMinMaxed>,
                >(self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_repr_mut(&mut self) -> ResolvedRectangleMut<N, T, A> {
        unsafe {
            match R::ENUM {
                RectReprEnum::Cornered => ResolvedRectangleMut::Cornered(transmute::<
                    &mut Rectangle<N, T, A, R>,
                    &mut Rectangle<N, T, A, RectCornered>,
                >(self)),

                RectReprEnum::Centered => ResolvedRectangleMut::Centered(transmute::<
                    &mut Rectangle<N, T, A, R>,
                    &mut Rectangle<N, T, A, RectCentered>,
                >(self)),

                RectReprEnum::MinMaxed => ResolvedRectangleMut::MinMaxed(transmute::<
                    &mut Rectangle<N, T, A, R>,
                    &mut Rectangle<N, T, A, RectMinMaxed>,
                >(self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolved(
        cornered: Rectangle<N, T, A, RectCornered>,
        centered: Rectangle<N, T, A, RectCentered>,
        min_maxed: Rectangle<N, T, A, RectMinMaxed>,
    ) -> Self {
        match R::ENUM {
            RectReprEnum::Cornered => unsafe {
                transmute_copy::<Rectangle<N, T, A, RectCornered>, Rectangle<N, T, A, R>>(&cornered)
            },

            RectReprEnum::Centered => unsafe {
                transmute_copy::<Rectangle<N, T, A, RectCentered>, Rectangle<N, T, A, R>>(&centered)
            },

            RectReprEnum::MinMaxed => unsafe {
                transmute_copy::<Rectangle<N, T, A, RectMinMaxed>, Rectangle<N, T, A, R>>(
                    &min_maxed,
                )
            },
        }
    }
}
