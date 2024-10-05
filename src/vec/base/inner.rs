use std::mem::transmute;

use super::*;

pub use gomath_proc_macros::impl_element_vec_inner;

pub unsafe trait ElementVecInner {
    type InnerVec2: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec3: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec4: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}

impl<N: VecNum, T: Element> VecN<N, T> {
    #[inline(always)]
    pub fn from_inner(inner: N::InnerVec<T>) -> Self {
        Self { inner }
    }
    #[inline(always)]
    pub fn from_inner_ref(inner: &N::InnerVec<T>) -> &Self {
        unsafe { transmute(inner) }
    }
    #[inline(always)]
    pub fn from_inner_mut(inner: &mut N::InnerVec<T>) -> &mut Self {
        unsafe { transmute(inner) }
    }
    #[inline(always)]
    pub fn inner(&self) -> &N::InnerVec<T> {
        &self.inner
    }
    #[inline(always)]
    pub fn inner_mut(&mut self) -> &mut N::InnerVec<T> {
        &mut self.inner
    }
    #[inline(always)]
    pub fn into_inner(self) -> N::InnerVec<T> {
        self.inner
    }
}
