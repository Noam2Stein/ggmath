use std::ops::Index;

use crate::element::*;

mod base;
pub use base::*;

pub mod num;
pub mod ops;

trait VecNumSeal {}
pub trait VecNum: VecNumSeal {
    const LEN: usize;
    type Array<T: Element>: std::fmt::Debug
        + Copy
        + PartialEq
        + PartialOrd
        + IntoIterator<Item = T>
        + Index<usize>;
    type InnerVec<T: Element>: std::fmt::Debug + Copy + PartialEq + PartialOrd;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VecN<N: VecNum, T: Element = f32> {
    inner: N::InnerVec<T>,
}
pub type Vec2<T> = VecN<USize<2>, T>;
pub type Vec3<T> = VecN<USize<3>, T>;
pub type Vec4<T> = VecN<USize<4>, T>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct USize<const VALUE: usize>;

impl VecNumSeal for USize<2> {}
impl VecNum for USize<2> {
    const LEN: usize = 2;
    type Array<T: Element> = [T; 2];
    type InnerVec<T: Element> = T::InnerVec2;
}

impl VecNumSeal for USize<3> {}
impl VecNum for USize<3> {
    const LEN: usize = 3;
    type Array<T: Element> = [T; 3];
    type InnerVec<T: Element> = T::InnerVec3;
}

impl VecNumSeal for USize<4> {}
impl VecNum for USize<4> {
    const LEN: usize = 4;
    type Array<T: Element> = [T; 4];
    type InnerVec<T: Element> = T::InnerVec4;
}
