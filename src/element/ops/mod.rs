use std::ops::*;

use gomath_proc_macros::rhs_ops;

use super::*;

macro_rules! rhs_op {
    ($element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
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
