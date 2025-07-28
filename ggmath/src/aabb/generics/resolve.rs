use std::mem::{transmute, transmute_copy};

use super::*;

pub enum ResolvedRectangle<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(Aabb<N, T, A, AabbCornered>),
    Centered(Aabb<N, T, A, AabbCentered>),
    MinMaxed(Aabb<N, T, A, AabbMinMaxed>),
}

pub enum ResolvedRectangleRef<'a, const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(&'a Aabb<N, T, A, AabbCornered>),
    Centered(&'a Aabb<N, T, A, AabbCentered>),
    MinMaxed(&'a Aabb<N, T, A, AabbMinMaxed>),
}

pub enum ResolvedRectangleMut<'a, const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(&'a mut Aabb<N, T, A, AabbCornered>),
    Centered(&'a mut Aabb<N, T, A, AabbCentered>),
    MinMaxed(&'a mut Aabb<N, T, A, AabbMinMaxed>),
}

impl<const N: usize, T: AabbScalar, A: VecAlignment, R: AabbRepr> Aabb<N, T, A, R>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedRectangle<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedRectangle::Cornered(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbCornered>,
                >(&self)),

                AabbReprEnum::Centered => ResolvedRectangle::Centered(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbCentered>,
                >(&self)),

                AabbReprEnum::MinMaxed => ResolvedRectangle::MinMaxed(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbMinMaxed>,
                >(&self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedRectangleRef<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedRectangleRef::Cornered(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbCornered>,
                >(self)),

                AabbReprEnum::Centered => ResolvedRectangleRef::Centered(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbCentered>,
                >(self)),

                AabbReprEnum::MinMaxed => ResolvedRectangleRef::MinMaxed(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbMinMaxed>,
                >(self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_repr_mut(&mut self) -> ResolvedRectangleMut<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedRectangleMut::Cornered(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbCornered>,
                >(self)),

                AabbReprEnum::Centered => ResolvedRectangleMut::Centered(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbCentered>,
                >(self)),

                AabbReprEnum::MinMaxed => ResolvedRectangleMut::MinMaxed(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbMinMaxed>,
                >(self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolved(
        cornered: Aabb<N, T, A, AabbCornered>,
        centered: Aabb<N, T, A, AabbCentered>,
        min_maxed: Aabb<N, T, A, AabbMinMaxed>,
    ) -> Self {
        match R::ENUM {
            AabbReprEnum::Cornered => unsafe {
                transmute_copy::<Aabb<N, T, A, AabbCornered>, Aabb<N, T, A, R>>(&cornered)
            },

            AabbReprEnum::Centered => unsafe {
                transmute_copy::<Aabb<N, T, A, AabbCentered>, Aabb<N, T, A, R>>(&centered)
            },

            AabbReprEnum::MinMaxed => unsafe {
                transmute_copy::<Aabb<N, T, A, AabbMinMaxed>, Aabb<N, T, A, R>>(&min_maxed)
            },
        }
    }
}
