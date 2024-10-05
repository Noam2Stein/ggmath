use std::ops::*;

use gomath_proc_macros::{assign_ops, rhs_ops, self_ops};

use super::*;

macro_rules! self_op {
    ($vecn_trait:ident: $element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        pub trait $element_trait: Element + $std_trait<Output: Element> {
            fn $vec2_fn(vec: Vec2<Self>) -> Vec2<<Self as $std_trait>::Output>;
            fn $vec3_fn(vec: Vec3<Self>) -> Vec3<<Self as $std_trait>::Output>;
            fn $vec4_fn(vec: Vec4<Self>) -> Vec4<<Self as $std_trait>::Output>;
        }
    };
}
self_ops!(self_op);

macro_rules! rhs_op {
    ($vecn_trait:ident: $element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        pub trait $element_trait<Rhs: Element = Self>:
            Element + $std_trait<Rhs, Output: Element>
        {
            fn $vec2_fn(vec: Vec2<Self>, rhs: Vec2<Rhs>)
                -> Vec2<<Self as $std_trait<Rhs>>::Output>;
            fn $vec3_fn(vec: Vec3<Self>, rhs: Vec3<Rhs>)
                -> Vec3<<Self as $std_trait<Rhs>>::Output>;
            fn $vec4_fn(vec: Vec4<Self>, rhs: Vec4<Rhs>)
                -> Vec4<<Self as $std_trait<Rhs>>::Output>;
        }
    };
}
rhs_ops!(rhs_op);

macro_rules! assign_op {
    ($vecn_trait:ident: $element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        pub trait $element_trait<Rhs: Element = Self>: Element + $std_trait<Rhs> {
            fn $vec2_fn(vec: &mut Vec2<Self>, rhs: Vec2<Rhs>);
            fn $vec3_fn(vec: &mut Vec3<Self>, rhs: Vec3<Rhs>);
            fn $vec4_fn(vec: &mut Vec4<Self>, rhs: Vec4<Rhs>);
        }
    };
}
assign_ops!(assign_op);
