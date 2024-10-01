use crate::element::*;

mod array;
mod default;
mod display;
mod from_split;
mod get;
mod inner;
pub use array::*;
pub use default::*;
pub use from_split::*;
pub use get::*;
pub use inner::*;

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

pub trait ElementVec: ElementInnerVecs + ElementVecDefault + ElementVecGet {}

impl<T: Element> VecN<T, 2> for Vec2<T> {}
impl<T: Element> VecN<T, 3> for Vec3<T> {}
impl<T: Element> VecN<T, 4> for Vec4<T> {}

impl<T: Element> Seal for Vec2<T> {}
impl<T: Element> Seal for Vec3<T> {}
impl<T: Element> Seal for Vec4<T> {}
