use std::fmt::{self, Display, Formatter};

use gmath_macros::*;

use crate::element::*;

macro_rules! wrap {
    (Vec2: $expr:expr) => {
        Vec2::wrap($expr)
    };
    (Vec3: $expr:expr) => {
        Vec3::wrap($expr)
    };
    (Vec3A: $expr:expr) => {
        Vec3A::wrap($expr)
    };
    (Vec4: $expr:expr) => {
        Vec4::wrap($expr)
    };
    ($element:ident: $expr:expr) => {
        $expr
    };
}
macro_rules! unwrap {
    (Vec2: $expr:expr) => {
        Vec2::unwrap($expr)
    };
    (Vec3: $expr:expr) => {
        Vec3::unwrap($expr)
    };
    (Vec3A: $expr:expr) => {
        Vec3A::unwrap($expr)
    };
    (Vec4: $expr:expr) => {
        Vec4::unwrap($expr)
    };
    ($element:ident: $expr:expr) => {
        $expr
    };
}

swizzle_macro!(
    get_fns {
        #[inline(always)]
        pub fn $ident(self) -> $value_ty {
            wrap!($value_ident: T::$inner_ident(unwrap!($self_ident: self)))
        }
    }
);
mut_swizzle_macro!(
    mut_fns {
        #[inline(always)]
        #[allow(unused_parens)]
        pub fn $ident(&mut self) -> ($(&mut $value_ty), +) {
            unsafe {
                (
                    $(
                        &mut *((self as *mut _ as *mut T).add($self_component) as *mut $value_ty)
                    ), +
                )
            }
        }
    }
);
swizzle_macro!(
    set_fns {
        #[inline(always)]
        pub fn $ident(&mut self, value: $value_ty) {
            unsafe {
                $(
                    *((self as *mut _ as *mut T).add($self_component) as *mut [T; $len]) = *((&value as *const _ as *const T).add($value_component) as *mut [T; $len]);
                )+
            }
        }
    }
);
swizzle_macro!(
    with_fns {
        #[inline(always)]
        pub fn $ident(self, value: $value_ty) -> $self_ty {
            wrap!($self_ident: T::$inner_ident(unwrap!($self_ident: self), unwrap!($value_ident: value)))
        }
    }
);
macro_rules! vec_ty {
    (
        $ident:ident, $inner:ident, $ident_lower:ident,
        $new:ident, $splat:ident,
        $get_swizzle:ident, $mut_swizzle:ident, $set_swizzle:ident, $with_swizzle:ident,
        $add:ident, $sub:ident, $mul:ident, $div:ident, $rem:ident,
        $add_assign:ident, $sub_assign:ident, $mul_assign:ident, $div_assign:ident, $rem_assign:ident,
        $neg:ident,
        $display_literal:literal, $($component:ident, $component_index:literal), +
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $ident<T: Element> {
            pub(crate) inner: T::$inner
        }

        #[inline(always)]
        pub fn $ident_lower<T: Element>(value: impl Into<$ident<T>>) -> $ident<T> {
            value.into()
        }

        impl<T: Element> $ident<T> {
            #[inline(always)]
            pub fn new($($component: T), +) -> Self {
                Self::wrap(T::$new($($component), +))
            }
            #[inline(always)]
            pub fn splat(value: T) -> Self {
                Self::wrap(T::$splat(value))
            }

            #[inline(always)]
            pub fn wrap(inner: T::$inner) -> Self {
                $ident {
                    inner
                }
            }
            #[inline(always)]
            pub fn unwrap(self) -> T::$inner {
                self.inner
            }
            #[inline(always)]
            pub fn wrap_ref<'a>(inner: &'a T::$inner) -> &'a Self {
                unsafe {
                    & *(inner as *const _ as *const Self)
                }
            }
            #[inline(always)]
            pub fn unwrap_ref<'a>(&'a self) -> &'a T::$inner {
                &self.inner
            }
            #[inline(always)]
            pub fn wrap_mut<'a>(inner: &'a mut T::$inner) -> &'a mut Self {
                unsafe {
                    &mut *(inner as *mut _ as *mut Self)
                }
            }
            #[inline(always)]
            pub fn unwrap_mut<'a>(&'a mut self) -> &'a mut T::$inner {
                &mut self.inner
            }

            #[inline(always)]
            pub fn map<F: FnMut(T) -> T>(self, mut f: F) -> Self {
                Self::new(
                    $(
                        f(self.$component())
                    ), +
                )
            }
        
            $get_swizzle!(
                T {
                    get_fns!();
                }
            );
            $mut_swizzle!(
                T {
                    mut_fns!();
                }
            );
            $set_swizzle!(
                T {
                    set_fns!();
                }
            );
            $with_swizzle!(
                T {
                    with_fns!();
                }
            );
        }
        impl<T: Element> Display for $ident<T> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                write!(f, $display_literal, $(self.$component()), +)
            }
        }
    };
}

vec_ty!(
    Vec2, Vec2Inner, vec2,
    new_vec2, splat_vec2,
    vec2_get_swizzle, vec2_mut_swizzle, vec2_set_swizzle, vec2_with_swizzle,
    add_vec2, sub_vec2, mul_vec2, div_vec2, rem_vec2,
    add_assign_vec2, sub_assign_vec2, mul_assign_vec2, div_assign_vec2, rem_assign_vec2,
    neg_vec2,
    "({}, {})", x, 0, y, 1
);
vec_ty!(
    Vec3, Vec3Inner, vec3,
    new_vec3, splat_vec3,
    vec3_get_swizzle, vec3_mut_swizzle, vec3_set_swizzle, vec3_with_swizzle,
    add_vec3, sub_vec3, mul_vec3, div_vec3, rem_vec3,
    add_assign_vec3, sub_assign_vec3, mul_assign_vec3, div_assign_vec3, rem_assign_vec3,
    neg_vec3,
    "({}, {}, {})", x, 0, y, 1, z, 2
);
vec_ty!(
    Vec3A, Vec3AInner, vec3a,
    new_vec3a, splat_vec3a,
    vec3a_get_swizzle, vec3a_mut_swizzle, vec3a_set_swizzle, vec3a_with_swizzle,
    add_vec3a, sub_vec3a, mul_vec3a, div_vec3a, rem_vec3a,
    add_assign_vec3a, sub_assign_vec3a, mul_assign_vec3a, div_assign_vec3a, rem_assign_vec3a,
    neg_vec3a,
    "({}, {}, {})", x, 0, y, 1, z, 2
);
vec_ty!(
    Vec4, Vec4Inner, vec4,
    new_vec4, splat_vec4,
    vec4_get_swizzle, vec4_mut_swizzle, vec4_set_swizzle, vec4_with_swizzle,
    add_vec4, sub_vec4, mul_vec4, div_vec4, rem_vec4,
    add_assign_vec4, sub_assign_vec4, mul_assign_vec4, div_assign_vec4, rem_assign_vec4,
    neg_vec4,
    "({}, {}, {}, {})", x, 0, y, 1, z, 2, w, 3
);