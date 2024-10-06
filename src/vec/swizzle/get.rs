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

vecnum_trait!(
    pub(super) trait VecNumGet {
        fn get<T: Element>(vec: InnerVec<N, T>, index: usize) -> Result<T, &'static str>;
        fn get2<T: Element>(
            vec: InnerVec<N, T>,
            indicies: [usize; 2],
        ) -> Result<Vec2<T>, &'static str>;
        fn get3<T: Element>(
            vec: InnerVec<N, T>,
            indicies: [usize; 3],
        ) -> Result<Vec3<T>, &'static str>;
        fn get4<T: Element>(
            vec: InnerVec<N, T>,
            indicies: [usize; 4],
        ) -> Result<InnerVec<T>, &'static str>;

        unsafe fn get_unchecked<T: Element>(vec: InnerVec<N, T>, index: usize) -> T;
        unsafe fn get2_unchecked<T: Element>(
            vec: InnerVec<N, T>,
            indicies: [usize; 2],
        ) -> InnerVec<2, T>;
        unsafe fn get3_unchecked<T: Element>(
            vec: InnerVec<N, T>,
            indicies: [usize; 3],
        ) -> InnerVec<3, T>;
        unsafe fn get4_unchecked<T: Element>(
            vec: InnerVec<N, T>,
            indicies: [usize; 4],
        ) -> InnerVec<4, T>;
    }
);
