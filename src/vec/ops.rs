use std::ops::*;

use gomath_proc_macros::rhs_ops;

use super::*;
use crate::element::ops::*;

macro_rules! rhs_op {
    ($element_trait:ident($vec2_fn:ident, $vec3_fn:ident, $vec4_fn:ident): $std_trait:ident($std_fn:ident)) => {
        impl<Rhs: Element, T: $element_trait<Rhs>> $std_trait<Vec2<Rhs>> for Vec2<T> {
            type Output = Vec2<T::Output>;
            #[inline(always)]
            fn $std_fn(self, rhs: Vec2<Rhs>) -> Self::Output {
                T::$vec2_fn(self, rhs)
            }
        }
    };
}
rhs_ops!(rhs_op);
