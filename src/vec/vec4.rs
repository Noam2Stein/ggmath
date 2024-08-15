use crate::element::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec4<T: Element> {
    inner: T::Vec4Inner
}