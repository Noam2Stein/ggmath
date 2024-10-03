use std::ops::*;

use gomath_proc_macros::{assign_ops, rhs_ops, self_ops};

use super::*;
use crate::element::ops::*;

mod component_dot;
mod component_sum;
pub use component_dot::*;
pub use component_sum::*;

macro_rules! self_op {
    ($vecn_trait:ident: $element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        pub trait $vecn_trait<T: $element_trait, const N: usize>:
            VecN<T, N> + $std_trait<Output = Self::WithT<T::Output>>
        {
        }
        impl<T: $element_trait> $vecn_trait<T, 2> for Vec2<T> {}
        impl<T: $element_trait> $vecn_trait<T, 3> for Vec3<T> {}
        impl<T: $element_trait> $vecn_trait<T, 4> for Vec4<T> {}

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
    ($vecn_trait:ident: $element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        pub trait $vecn_trait<T: $element_trait<Rhs>, const N: usize, Rhs: Element = T>:
            VecN<T, N> + $std_trait<Self::WithT<Rhs>, Output = Self::WithT<T::Output>>
        {
        }
        impl<Rhs: Element, T: $element_trait<Rhs>> $vecn_trait<T, 2, Rhs> for Vec2<T> {}
        impl<Rhs: Element, T: $element_trait<Rhs>> $vecn_trait<T, 3, Rhs> for Vec3<T> {}
        impl<Rhs: Element, T: $element_trait<Rhs>> $vecn_trait<T, 4, Rhs> for Vec4<T> {}

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

macro_rules! assign_op {
    ($vecn_trait:ident: $element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        pub trait $vecn_trait<T: $element_trait<Rhs>, const N: usize, Rhs: Element = T>:
            VecN<T, N> + $std_trait<Self::WithT<Rhs>>
        {
        }
        impl<Rhs: Element, T: $element_trait<Rhs>> $vecn_trait<T, 2, Rhs> for Vec2<T> {}
        impl<Rhs: Element, T: $element_trait<Rhs>> $vecn_trait<T, 3, Rhs> for Vec3<T> {}
        impl<Rhs: Element, T: $element_trait<Rhs>> $vecn_trait<T, 4, Rhs> for Vec4<T> {}

        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec2<Rhs>> for Vec2<T> {
            #[inline(always)]
            fn $std_fn(&mut self, rhs: Vec2<Rhs>) {
                T::$vec2_fn(self, rhs)
            }
        }
        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec3<Rhs>> for Vec3<T> {
            #[inline(always)]
            fn $std_fn(&mut self, rhs: Vec3<Rhs>) {
                T::$vec3_fn(self, rhs)
            }
        }
        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec4<Rhs>> for Vec4<T> {
            #[inline(always)]
            fn $std_fn(&mut self, rhs: Vec4<Rhs>) {
                T::$vec4_fn(self, rhs)
            }
        }
    };
}
assign_ops!(assign_op);
