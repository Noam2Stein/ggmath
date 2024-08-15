use crate::element::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec3<T: Element> {
    inner: T::Vec3Inner
}