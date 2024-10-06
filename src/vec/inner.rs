use std::mem::transmute;

use super::*;

pub use gomath_proc_macros::impl_element_vec_inner;

pub unsafe trait ElementVecInner: Sized {
    type InnerVec2: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec3: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec4: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}

pub type InnerVec<const N: usize, T> = <MaybeVecNum<N> as VecNumInner<N>>::InnerVec<T>;
pub type InnerVec2<T> = InnerVec<2, T>;
pub type InnerVec3<T> = InnerVec<3, T>;
pub type InnerVec4<T> = InnerVec<4, T>;

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

vecnum_trait!(
    pub(super) trait VecNumInner {
        type InnerVec<T: ElementVecInner>: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    }
);
