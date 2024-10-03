use crate::element::*;

mod array;
mod const_swizzle;
mod from_split;
mod inner;
mod ops;
mod std_impl;
mod swizzle;
pub use array::*;
pub use const_swizzle::*;
pub use from_split::*;
pub use inner::*;
#[allow(unused_imports)]
pub use ops::*;
pub use std_impl::*;
pub use swizzle::*;

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T: Element = f32> {
    inner: T::InnerVec2,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T: Element = f32> {
    inner: T::InnerVec3,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec4<T: Element = f32> {
    inner: T::InnerVec4,
}

trait Seal {}
impl<T: Element> Seal for Vec2<T> {}
impl<T: Element> Seal for Vec3<T> {}
impl<T: Element> Seal for Vec4<T> {}

#[allow(private_bounds)]
pub trait VecN<T: Element, const N: usize>:
    Seal
    + std::fmt::Debug
    + Copy
    + PartialEq
    + PartialOrd
    + Default
    + std::fmt::Display
    + VecNInner
    + VecNArray<T, N>
    + VecNConstGet<T, N>
    + VecNConstGetMut<T, N>
    + VecNConstWith<T, N>
    + VecNConstSet<T, N>
{
}
impl<T: Element> VecN<T, 2> for Vec2<T> {}
impl<T: Element> VecN<T, 3> for Vec3<T> {}
impl<T: Element> VecN<T, 4> for Vec4<T> {}

pub trait ElementVec:
    ElementVecInner + ElementVecDefault + ElementVecConstSwizzle + ElementVecSwizzle
{
}
