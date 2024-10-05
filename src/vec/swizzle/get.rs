use super::*;

pub trait ElementVecGet<const N: usize>: Sized + ElementVecInner
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn vec_get(vec: InnerVec<N, Self>, index: usize) -> Result<Self, &'static str>;
    fn vec_get2(
        vec: InnerVec<N, Self>,
        indicies: [usize; 2],
    ) -> Result<Self::InnerVec2, &'static str>;
    fn vec_get3(
        vec: InnerVec<N, Self>,
        indicies: [usize; 3],
    ) -> Result<Self::InnerVec3, &'static str>;
    fn vec_get4(
        vec: InnerVec<N, Self>,
        indicies: [usize; 4],
    ) -> Result<Self::InnerVec4, &'static str>;

    unsafe fn vec_get_unchecked(vec: InnerVec<N, Self>, index: usize) -> Self;
    unsafe fn vec_get2_unchecked(vec: InnerVec<N, Self>, indicies: [usize; 2]) -> Self::InnerVec2;
    unsafe fn vec_get3_unchecked(vec: InnerVec<N, Self>, indicies: [usize; 3]) -> Self::InnerVec3;
    unsafe fn vec_get4_unchecked(vec: InnerVec<N, Self>, indicies: [usize; 4]) -> Self::InnerVec4;
}

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    pub fn get(self, index: usize) -> Result<T, &'static str> {
        MaybeVecNum::<N>::get(self, index)
    }
    pub fn get2(self, indicies: [usize; 2]) -> Result<Vec2<T>, &'static str> {
        MaybeVecNum::<N>::get2(self, indicies)
    }
    pub fn get3(self, indicies: [usize; 3]) -> Result<Vec3<T>, &'static str> {
        MaybeVecNum::<N>::get3(self, indicies)
    }
    pub fn get4(self, indicies: [usize; 4]) -> Result<Vec4<T>, &'static str> {
        MaybeVecNum::<N>::get4(self, indicies)
    }

    pub unsafe fn get_unchecked(self, index: usize) -> T {
        MaybeVecNum::<N>::get_unchecked(self, index)
    }
    pub unsafe fn get2_unchecked(self, indicies: [usize; 2]) -> Vec2<T> {
        MaybeVecNum::<N>::get2_unchecked(self, indicies)
    }
    pub unsafe fn get3_unchecked(self, indicies: [usize; 3]) -> Vec3<T> {
        MaybeVecNum::<N>::get3_unchecked(self, indicies)
    }
    pub unsafe fn get4_unchecked(self, indicies: [usize; 4]) -> Vec4<T> {
        MaybeVecNum::<N>::get4_unchecked(self, indicies)
    }
}

pub(super) trait VecNumGet<const N: usize>
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn get<T: Element>(vec: VecN<N, T>, index: usize) -> Result<T, &'static str>;
    fn get2<T: Element>(vec: VecN<N, T>, indicies: [usize; 2]) -> Result<Vec2<T>, &'static str>;
    fn get3<T: Element>(vec: VecN<N, T>, indicies: [usize; 3]) -> Result<Vec3<T>, &'static str>;
    fn get4<T: Element>(vec: VecN<N, T>, indicies: [usize; 4]) -> Result<Vec4<T>, &'static str>;

    unsafe fn get_unchecked<T: Element>(vec: VecN<N, T>, index: usize) -> T;
    unsafe fn get2_unchecked<T: Element>(vec: VecN<N, T>, indicies: [usize; 2]) -> Vec2<T>;
    unsafe fn get3_unchecked<T: Element>(vec: VecN<N, T>, indicies: [usize; 3]) -> Vec3<T>;
    unsafe fn get4_unchecked<T: Element>(vec: VecN<N, T>, indicies: [usize; 4]) -> Vec4<T>;
}
impl VecNumGet<2> for MaybeVecNum<2> {
    fn get<T: Element>(vec: VecN<2, T>, index: usize) -> Result<T, &'static str> {
        T::vec_get(vec, index)
    }
    fn get2<T: Element>(vec: VecN<2, T>, indicies: [usize; 2]) -> Result<Vec2<T>, &'static str> {
        T::vec_get2(vec, indicies).map(|inner| VecN::from_inner(inner))
    }
    fn get3<T: Element>(vec: VecN<2, T>, indicies: [usize; 3]) -> Result<Vec3<T>, &'static str> {
        T::vec_get3(vec, indicies).map(|inner| VecN::from_inner(inner))
    }
    fn get4<T: Element>(vec: VecN<2, T>, indicies: [usize; 4]) -> Result<Vec4<T>, &'static str> {
        T::vec_get4(vec, indicies).map(|inner| VecN::from_inner(inner))
    }

    unsafe fn get_unchecked<T: Element>(vec: VecN<2, T>, index: usize) -> T {
        T::vec_get_unchecked(vec, index)
    }
    unsafe fn get2_unchecked<T: Element>(vec: VecN<2, T>, indicies: [usize; 2]) -> Vec2<T> {
        VecN::from_inner(T::vec_get2_unchecked(vec, indicies))
    }
    unsafe fn get3_unchecked<T: Element>(vec: VecN<2, T>, indicies: [usize; 3]) -> Vec3<T> {
        VecN::from_inner(T::vec_get3_unchecked(vec, indicies))
    }
    unsafe fn get4_unchecked<T: Element>(vec: VecN<2, T>, indicies: [usize; 4]) -> Vec4<T> {
        VecN::from_inner(T::vec_get4_unchecked(vec, indicies))
    }
}
impl VecNumGet<3> for MaybeVecNum<3> {
    fn get<T: Element>(vec: VecN<3, T>, index: usize) -> Result<T, &'static str> {
        T::vec_get(vec, index)
    }
    fn get2<T: Element>(vec: VecN<3, T>, indicies: [usize; 2]) -> Result<Vec2<T>, &'static str> {
        T::vec_get2(vec, indicies).map(|inner| VecN::from_inner(inner))
    }
    fn get3<T: Element>(vec: VecN<3, T>, indicies: [usize; 3]) -> Result<Vec3<T>, &'static str> {
        T::vec_get3(vec, indicies).map(|inner| VecN::from_inner(inner))
    }
    fn get4<T: Element>(vec: VecN<3, T>, indicies: [usize; 4]) -> Result<Vec4<T>, &'static str> {
        T::vec_get4(vec, indicies).map(|inner| VecN::from_inner(inner))
    }

    unsafe fn get_unchecked<T: Element>(vec: VecN<3, T>, index: usize) -> T {
        T::vec_get_unchecked(vec, index)
    }
    unsafe fn get2_unchecked<T: Element>(vec: VecN<3, T>, indicies: [usize; 2]) -> Vec2<T> {
        VecN::from_inner(T::vec_get2_unchecked(vec, indicies))
    }
    unsafe fn get3_unchecked<T: Element>(vec: VecN<3, T>, indicies: [usize; 3]) -> Vec3<T> {
        VecN::from_inner(T::vec_get3_unchecked(vec, indicies))
    }
    unsafe fn get4_unchecked<T: Element>(vec: VecN<3, T>, indicies: [usize; 4]) -> Vec4<T> {
        VecN::from_inner(T::vec_get4_unchecked(vec, indicies))
    }
}
impl VecNumGet<4> for MaybeVecNum<4> {
    fn get<T: Element>(vec: VecN<4, T>, index: usize) -> Result<T, &'static str> {
        T::vec_get(vec, index)
    }
    fn get2<T: Element>(vec: VecN<4, T>, indicies: [usize; 2]) -> Result<Vec2<T>, &'static str> {
        T::vec_get2(vec, indicies).map(|inner| VecN::from_inner(inner))
    }
    fn get3<T: Element>(vec: VecN<4, T>, indicies: [usize; 3]) -> Result<Vec3<T>, &'static str> {
        T::vec_get3(vec, indicies).map(|inner| VecN::from_inner(inner))
    }
    fn get4<T: Element>(vec: VecN<4, T>, indicies: [usize; 4]) -> Result<Vec4<T>, &'static str> {
        T::vec_get4(vec, indicies).map(|inner| VecN::from_inner(inner))
    }

    unsafe fn get_unchecked<T: Element>(vec: VecN<4, T>, index: usize) -> T {
        T::vec_get_unchecked(vec, index)
    }
    unsafe fn get2_unchecked<T: Element>(vec: VecN<4, T>, indicies: [usize; 2]) -> Vec2<T> {
        VecN::from_inner(T::vec_get2_unchecked(vec, indicies))
    }
    unsafe fn get3_unchecked<T: Element>(vec: VecN<4, T>, indicies: [usize; 3]) -> Vec3<T> {
        VecN::from_inner(T::vec_get3_unchecked(vec, indicies))
    }
    unsafe fn get4_unchecked<T: Element>(vec: VecN<4, T>, indicies: [usize; 4]) -> Vec4<T> {
        VecN::from_inner(T::vec_get4_unchecked(vec, indicies))
    }
}
