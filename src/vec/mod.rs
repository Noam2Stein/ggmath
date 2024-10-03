use crate::element::*;

mod vecn;
pub use vecn::*;

mod ops;

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
