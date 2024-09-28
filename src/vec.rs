use std::fmt::{self, Display, Formatter};

use crate::element::*;

pub trait VecN: fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display {
    type T: Element;
    const N: usize;
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec2<T: Element> {
    inner: T::Vec2Inner,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec3<T: Element> {
    inner: T::Vec3Inner,
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Vec4<T: Element> {
    inner: T::Vec4Inner,
}

impl<T: Element> Display for Vec2<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl<T: Element> Display for Vec3<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
impl<T: Element> Display for Vec4<T> {
    fn fmt(&self, _f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
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
