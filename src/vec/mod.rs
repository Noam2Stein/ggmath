use crate::element::*;

mod array;
mod default;
mod display;
mod get;
mod split;
pub use array::*;
pub use split::*;

trait Seal {}

#[allow(private_bounds)]
pub trait VecN<T: Element, const N: usize>:
    Seal
    + std::fmt::Debug
    + Copy
    + PartialEq
    + PartialOrd
    + Default
    + std::fmt::Display
    + VecNArray<T, N>
{
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec2<T: Element> {
    inner: T::InnerVec2,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec3<T: Element> {
    inner: T::InnerVec3,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Vec4<T: Element> {
    inner: T::InnerVec4,
}

impl<T: Element> VecN<T, 2> for Vec2<T> {}
impl<T: Element> VecN<T, 3> for Vec3<T> {}
impl<T: Element> VecN<T, 4> for Vec4<T> {}

impl<T: Element> Seal for Vec2<T> {}
impl<T: Element> Seal for Vec3<T> {}
impl<T: Element> Seal for Vec4<T> {}

type Vec1<T> = T;
