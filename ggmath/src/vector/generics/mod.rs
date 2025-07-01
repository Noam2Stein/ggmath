use std::{
    any::TypeId,
    mem::{transmute, transmute_copy},
};

use super::*;

mod alignment;
mod length;
pub use alignment::*;
pub use length::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedVector<T: Scalar> {
    Vec2(Vec2<T>),
    Vec3(Vec3<T>),
    Vec4(Vec4<T>),
    Vec2P(Vec2P<T>),
    Vec3P(Vec3P<T>),
    Vec4P(Vec4P<T>),
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolvedVectorRef<'a, T: Scalar> {
    Vec2(&'a Vec2<T>),
    Vec3(&'a Vec3<T>),
    Vec4(&'a Vec4<T>),
    Vec2P(&'a Vec2P<T>),
    Vec3P(&'a Vec3P<T>),
    Vec4P(&'a Vec4P<T>),
}
#[derive(Debug, PartialEq, Eq)]
pub enum ResolvedVectorMut<'a, T: Scalar> {
    Vec2(&'a mut Vec2<T>),
    Vec3(&'a mut Vec3<T>),
    Vec4(&'a mut Vec4<T>),
    Vec2P(&'a mut Vec2P<T>),
    Vec3P(&'a mut Vec3P<T>),
    Vec4P(&'a mut Vec4P<T>),
}

impl<const N: usize, T: Scalar, A: VecAlignment> Vector<N, T, A>
where
    MaybeVecLen<N>: VecLen,
{
    /// Converts the math-type into the specified storage generics.
    /// In the case of a vector only ```A: VecAlignment```.
    ///
    /// This function is present for every math-type.
    ///
    /// Because a vector's storage generics are only ```A```,
    /// and ```into_alignment``` does the same thing as this function,
    /// use this function when the math-type may change,
    /// For example into a matrix.
    #[inline(always)]
    pub fn into_storage<AOutput: VecAlignment>(self) -> Vector<N, T, AOutput> {
        self.into_alignment()
    }

    #[inline(always)]
    pub fn resolve(self) -> ResolvedVector<T> {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                match N {
                    2 => ResolvedVector::Vec2(transmute_copy(&self)),
                    3 => ResolvedVector::Vec3(transmute_copy(&self)),
                    4 => ResolvedVector::Vec4(transmute_copy(&self)),
                    _ => unreachable!(),
                }
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                match N {
                    2 => ResolvedVector::Vec2P(transmute_copy(&self)),
                    3 => ResolvedVector::Vec3P(transmute_copy(&self)),
                    4 => ResolvedVector::Vec4P(transmute_copy(&self)),
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }
    }
    #[inline(always)]
    pub fn resolve_ref(&self) -> ResolvedVectorRef<T> {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                match N {
                    2 => ResolvedVectorRef::Vec2(transmute(self)),
                    3 => ResolvedVectorRef::Vec3(transmute(self)),
                    4 => ResolvedVectorRef::Vec4(transmute(self)),
                    _ => unreachable!(),
                }
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                match N {
                    2 => ResolvedVectorRef::Vec2P(transmute(self)),
                    3 => ResolvedVectorRef::Vec3P(transmute(self)),
                    4 => ResolvedVectorRef::Vec4P(transmute(self)),
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }
    }
    #[inline(always)]
    pub fn resolve_mut(&mut self) -> ResolvedVectorMut<T> {
        unsafe {
            if TypeId::of::<A>() == TypeId::of::<VecAligned>() {
                match N {
                    2 => ResolvedVectorMut::Vec2(transmute(self)),
                    3 => ResolvedVectorMut::Vec3(transmute(self)),
                    4 => ResolvedVectorMut::Vec4(transmute(self)),
                    _ => unreachable!(),
                }
            } else if TypeId::of::<A>() == TypeId::of::<VecPacked>() {
                match N {
                    2 => ResolvedVectorMut::Vec2P(transmute(self)),
                    3 => ResolvedVectorMut::Vec3P(transmute(self)),
                    4 => ResolvedVectorMut::Vec4P(transmute(self)),
                    _ => unreachable!(),
                }
            } else {
                unreachable!()
            }
        }
    }
}
