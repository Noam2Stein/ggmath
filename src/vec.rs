use crate::element::*;

pub trait VecN {
    type T: Element;
    const N: usize;
}

#[repr(transparent)]
pub struct Vec2<T: Element> {
    inner: T::Vec2Inner,
}
#[repr(transparent)]
pub struct Vec3<T: Element> {
    inner: T::Vec3Inner,
}
#[repr(transparent)]
pub struct Vec4<T: Element> {
    inner: T::Vec4Inner,
}
impl<T: Element> VecN for Vec2<T> {
    type T = T;
    const N: usize = 2;
}
impl<T: Element> VecN for Vec3<T> {
    type T = T;
    const N: usize = 3;
}
impl<T: Element> VecN for Vec4<T> {
    type T = T;
    const N: usize = 4;
}
