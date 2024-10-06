use super::*;

pub trait ElementVecWith<const N: usize>: ElementVecInner
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn vec_with(
        vec: InnerVec<N, Self>,
        index: usize,
        value: Self,
    ) -> Result<InnerVec<N, Self>, &'static str>;
    fn vec_with2(
        vec: InnerVec<N, Self>,
        indicies: [usize; 2],
        value: Self::InnerVec2,
    ) -> Result<InnerVec<N, Self>, &'static str>;
    fn vec_with3(
        vec: InnerVec<N, Self>,
        indicies: [usize; 3],
        value: Self::InnerVec3,
    ) -> Result<InnerVec<N, Self>, &'static str>;
    fn vec_with4(
        vec: InnerVec<N, Self>,
        indicies: [usize; 4],
        value: Self::InnerVec4,
    ) -> Result<InnerVec<N, Self>, &'static str>;

    unsafe fn vec_with_unchecked(
        vec: InnerVec<N, Self>,
        index: usize,
        value: Self,
    ) -> InnerVec<N, Self>;
    unsafe fn vec_with2_unchecked(
        vec: InnerVec<N, Self>,
        indicies: [usize; 2],
        value: Self::InnerVec2,
    ) -> InnerVec<N, Self>;
    unsafe fn vec_with3_unchecked(
        vec: InnerVec<N, Self>,
        indicies: [usize; 3],
        value: Self::InnerVec3,
    ) -> InnerVec<N, Self>;
    unsafe fn vec_with4_unchecked(
        vec: InnerVec<N, Self>,
        indicies: [usize; 4],
        value: Self::InnerVec4,
    ) -> InnerVec<N, Self>;
}

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    pub fn with(self, index: usize, value: T) -> Result<Self, &'static str> {}
    pub fn with2(self, indicies: [usize; 2], value: Vec2<T>) -> Result<Self, &'static str> {}
    pub fn with3(self, indicies: [usize; 3], value: Vec3<T>) -> Result<Self, &'static str> {}
    pub fn with4(self, indicies: [usize; 4], value: Vec4<T>) -> Result<Self, &'static str> {}

    pub unsafe fn with_unchecked(self, index: usize, value: T) -> Self {}
    pub unsafe fn with2_unchecked(self, indicies: [usize; 2], value: Vec2<T>) -> Self {}
    pub unsafe fn with3_unchecked(self, indicies: [usize; 3], value: Vec3<T>) -> Self {}
    pub unsafe fn with4_unchecked(self, indicies: [usize; 4], value: Vec4<T>) -> Self {}
}

pub(super) trait VecNumWith<const N: usize>
where
    MaybeVecNum<N>: VecNum<N>,
{
    fn with<T: Element>(
        vec: VecN<N, T>,
        index: usize,
        value: T,
    ) -> Result<VecN<N, T>, &'static str>;
    fn with2<T: Element>(
        vec: VecN<N, T>,
        indicies: [usize; 2],
        value: Vec2<T>,
    ) -> Result<VecN<N, T>, &'static str>;
    fn with3<T: Element>(
        vec: VecN<N, T>,
        indicies: [usize; 3],
        value: Vec3<T>,
    ) -> Result<VecN<N, T>, &'static str>;
    fn with4<T: Element>(
        vec: VecN<N, T>,
        indicies: [usize; 4],
        value: Vec4<T>,
    ) -> Result<VecN<N, T>, &'static str>;

    unsafe fn with_unchecked<T: Element>(vec: VecN<N, T>, index: usize, value: T) -> Self;
    unsafe fn with2_unchecked<T: Element>(
        vec: VecN<N, T>,
        indicies: [usize; 2],
        value: Vec2<T>,
    ) -> VecN<N, T>;
    unsafe fn with3_unchecked<T: Element>(
        vec: VecN<N, T>,
        indicies: [usize; 3],
        value: Vec3<T>,
    ) -> VecN<N, T>;
    unsafe fn with4_unchecked<T: Element>(
        vec: VecN<N, T>,
        indicies: [usize; 4],
        value: Vec4<T>,
    ) -> VecN<N, T>;
}
