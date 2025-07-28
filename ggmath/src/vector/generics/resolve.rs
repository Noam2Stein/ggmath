use std::mem::{transmute, transmute_copy};

use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedVector<const N: usize, T: Scalar>
where
    Usize<N>: VecLen,
{
    Aligned(Vector<N, T, VecAligned>),
    Packed(Vector<N, T, VecPacked>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedVectorRef<'a, const N: usize, T: Scalar>
where
    Usize<N>: VecLen,
{
    Aligned(&'a Vector<N, T, VecAligned>),
    Packed(&'a Vector<N, T, VecPacked>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedVectorMut<'a, const N: usize, T: Scalar>
where
    Usize<N>: VecLen,
{
    Aligned(&'a mut Vector<N, T, VecAligned>),
    Packed(&'a mut Vector<N, T, VecPacked>),
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    Usize<N>: VecLen,
{
    #[inline(always)]
    pub const fn resolve(self) -> ResolvedVector<N, T> {
        unsafe {
            if A::IS_ALIGNED {
                ResolvedVector::Aligned(
                    transmute_copy::<Vector<N, T, A>, Vector<N, T, VecAligned>>(&self),
                )
            } else {
                ResolvedVector::Packed(transmute_copy::<Vector<N, T, A>, Vector<N, T, VecPacked>>(
                    &self,
                ))
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_ref(&self) -> ResolvedVectorRef<N, T> {
        unsafe {
            if A::IS_ALIGNED {
                ResolvedVectorRef::Aligned(
                    transmute::<&Vector<N, T, A>, &Vector<N, T, VecAligned>>(self),
                )
            } else {
                ResolvedVectorRef::Packed(transmute::<&Vector<N, T, A>, &Vector<N, T, VecPacked>>(
                    self,
                ))
            }
        }
    }

    #[inline(always)]
    pub const fn resolve_mut(&mut self) -> ResolvedVectorMut<N, T> {
        unsafe {
            if A::IS_ALIGNED {
                ResolvedVectorMut::Aligned(transmute::<
                    &mut Vector<N, T, A>,
                    &mut Vector<N, T, VecAligned>,
                >(self))
            } else {
                ResolvedVectorMut::Packed(transmute::<
                    &mut Vector<N, T, A>,
                    &mut Vector<N, T, VecPacked>,
                >(self))
            }
        }
    }

    #[inline(always)]
    pub const fn resolved(
        aligned: Vector<N, T, VecAligned>,
        packed: Vector<N, T, VecPacked>,
    ) -> Self {
        if A::IS_ALIGNED {
            unsafe { transmute_copy::<Vector<N, T, VecAligned>, Vector<N, T, A>>(&aligned) }
        } else {
            unsafe { transmute_copy::<Vector<N, T, VecPacked>, Vector<N, T, A>>(&packed) }
        }
    }
}
