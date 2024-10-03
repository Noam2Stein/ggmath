use std::mem::{transmute, transmute_copy};

use super::*;

pub use gomath_proc_macros::impl_element_vec_inner;

pub unsafe trait VecNInner: Sized {
    type Inner: std::fmt::Debug + Copy + PartialEq + PartialOrd;

    fn from_inner(inner: Self::Inner) -> Self {
        unsafe { transmute_copy(&inner) }
    }
    fn from_inner_ref(inner: &Self::Inner) -> &Self {
        unsafe { transmute(inner) }
    }
    fn from_inner_mut(inner: &mut Self::Inner) -> &mut Self {
        unsafe { transmute(inner) }
    }
    fn inner(&self) -> &Self::Inner {
        unsafe { transmute(self) }
    }
    fn inner_mut(&mut self) -> &mut Self::Inner {
        unsafe { transmute(self) }
    }
    fn into_inner(self) -> Self::Inner {
        unsafe { transmute_copy(&self) }
    }
}

pub unsafe trait ElementVecInner {
    type InnerVec2: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec3: std::fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec4: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}

unsafe impl<T: Element> VecNInner for Vec2<T> {
    type Inner = T::InnerVec2;
}
unsafe impl<T: Element> VecNInner for Vec3<T> {
    type Inner = T::InnerVec3;
}
unsafe impl<T: Element> VecNInner for Vec4<T> {
    type Inner = T::InnerVec4;
}
