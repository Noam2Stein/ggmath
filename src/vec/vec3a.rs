use crate::element::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec3A<T: Element> {
    inner: T::Vec3AInner
}