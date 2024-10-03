use std::ops::*;

use gomath_proc_macros::{rhs_ops, self_ops};

use super::*;
use crate::element::ops::*;

macro_rules! self_op {
    ($element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        impl<T: $element_trait> $std_trait for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline(always)]
            fn $std_fn(self) -> Self::Output {
                T::$vec2_fn(self)
            }
        }
        impl<T: $element_trait> $std_trait for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline(always)]
            fn $std_fn(self) -> Self::Output {
                T::$vec3_fn(self)
            }
        }
        impl<T: $element_trait> $std_trait for Vec4<T> {
            type Output = Vec4<T::Output>;
            #[inline(always)]
            fn $std_fn(self) -> Self::Output {
                T::$vec4_fn(self)
            }
        }
    };
}
self_ops!(self_op);

macro_rules! rhs_op {
    ($element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec2<Rhs>> for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline(always)]
            fn $std_fn(self, rhs: Vec2<Rhs>) -> Self::Output {
                T::$vec2_fn(self, rhs)
            }
        }
        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec3<Rhs>> for Vec3<T> {
            type Output = Vec3<T::Output>;
            #[inline(always)]
            fn $std_fn(self, rhs: Vec3<Rhs>) -> Self::Output {
                T::$vec3_fn(self, rhs)
            }
        }
        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec4<Rhs>> for Vec4<T> {
            type Output = Vec4<T::Output>;
            #[inline(always)]
            fn $std_fn(self, rhs: Vec4<Rhs>) -> Self::Output {
                T::$vec4_fn(self, rhs)
            }
        }
    };
}
rhs_ops!(rhs_op);
