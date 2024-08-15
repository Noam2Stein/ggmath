use gmath_macros::*;

use crate::element::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec2<T: Element> {
    pub(crate) inner: T::Vec2Inner
}

pub fn vec2<T: Element>(value: impl Into<Vec2<T>>) -> Vec2<T> {
    value.into()
}

impl<T: Element> Vec2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        wrap(T::new_vec2(x, y))
    }
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        wrap(T::splat_vec2(value))
    }

    vec2_get_swizzle!(
        T {
            ##[inline(always)]
            pub fn #ident(self) -> #value_ty {
                #value_ty {
                    inner: T::#inner_ident(self.inner)
                }
            }
        }
    );
}

#[inline(always)]
fn wrap<T: Element>(inner: T::Vec2Inner) -> Vec2<T> {
    Vec2 {
        inner
    }
}