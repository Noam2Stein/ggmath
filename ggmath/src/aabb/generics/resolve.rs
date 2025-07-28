use std::mem::{transmute, transmute_copy};

use super::*;

pub enum ResolvedAabb<const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(Aabb<N, T, A, AabbCornered>),
    Centered(Aabb<N, T, A, AabbCentered>),
    MinMaxed(Aabb<N, T, A, AabbMinMaxed>),
}

pub enum ResolvedAabbRef<'a, const N: usize, T: AabbScalar, A: VecAlignment>
where
    Usize<N>: VecLen,
{
    Cornered(&'a Aabb<N, T, A, AabbCornered>),
    Centered(&'a Aabb<N, T, A, AabbCentered>),
    MinMaxed(&'a Aabb<N, T, A, AabbMinMaxed>),
}

pub enum ResolvedAabbMut<'a, const N: usize, T: AabbScalar, A: VecAlignment>
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
    pub const fn resolve(self) -> ResolvedAabb<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedAabb::Cornered(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbCornered>,
                >(&self)),

                AabbReprEnum::Centered => ResolvedAabb::Centered(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbCentered>,
                >(&self)),

                AabbReprEnum::MinMaxed => ResolvedAabb::MinMaxed(transmute_copy::<
                    Aabb<N, T, A, R>,
                    Aabb<N, T, A, AabbMinMaxed>,
                >(&self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedAabbRef<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedAabbRef::Cornered(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbCornered>,
                >(self)),

                AabbReprEnum::Centered => ResolvedAabbRef::Centered(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbCentered>,
                >(self)),

                AabbReprEnum::MinMaxed => ResolvedAabbRef::MinMaxed(transmute::<
                    &Aabb<N, T, A, R>,
                    &Aabb<N, T, A, AabbMinMaxed>,
                >(self)),
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_repr_mut(&mut self) -> ResolvedAabbMut<N, T, A> {
        unsafe {
            match R::ENUM {
                AabbReprEnum::Cornered => ResolvedAabbMut::Cornered(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbCornered>,
                >(self)),

                AabbReprEnum::Centered => ResolvedAabbMut::Centered(transmute::<
                    &mut Aabb<N, T, A, R>,
                    &mut Aabb<N, T, A, AabbCentered>,
                >(self)),

                AabbReprEnum::MinMaxed => ResolvedAabbMut::MinMaxed(transmute::<
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
