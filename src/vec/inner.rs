use std::mem::transmute;

use super::*;

pub use gomath_proc_macros::impl_element_vec_inner;

pub unsafe trait ElementVecInner: Sized {
    type InnerVec2: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec3: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec4: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}

pub type InnerVec<const N: usize, T> = <MaybeVecNum<N> as VecNumInner>::InnerVec<T>;

impl<const N: usize, T: Element> VecN<N, T>
where
    MaybeVecNum<N>: VecNum<N>,
{
    #[inline(always)]
    pub fn from_inner(inner: InnerVec<N, T>) -> Self {
        Self { inner }
    }
    #[inline(always)]
    pub fn from_inner_ref(inner: &InnerVec<N, T>) -> &Self {
        unsafe { transmute(inner) }
    }
    #[inline(always)]
    pub fn from_inner_mut(inner: &mut InnerVec<N, T>) -> &mut Self {
        unsafe { transmute(inner) }
    }
    #[inline(always)]
    pub fn inner(&self) -> &InnerVec<N, T> {
        &self.inner
    }
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut InnerVec<N, T> {
        &mut self.inner
    }
    #[inline(always)]
    pub fn into_inner(self) -> InnerVec<N, T> {
        self.inner
    }
}

pub(super) trait VecNumInner {
    type InnerVec<T: ElementVecInner>: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}
impl VecNumInner for MaybeVecNum<2> {
    type InnerVec<T: ElementVecInner> = T::InnerVec2;
}
impl VecNumInner for MaybeVecNum<3> {
    type InnerVec<T: ElementVecInner> = T::InnerVec3;
}
impl VecNumInner for MaybeVecNum<4> {
    type InnerVec<T: ElementVecInner> = T::InnerVec4;
}
