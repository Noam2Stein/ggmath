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

swizzle_macro!(
    get_fns {
        #[inline(always)]
        pub fn $ident(self) -> $value_ty {
            $wrap_value(T::$inner_ident($unwrap_self(self)))
        }
    }
);
swizzle_macro!(
    mut_fns {
        #[inline(always)]
        pub fn $ident(&mut self) -> &mut $value_ty {
            $wrap_value_mut(T::$inner_ident($unwrap_self_mut(self)))
        }
    }
);
swizzle_macro!(
    set_fns {
        #[inline(always)]
        pub fn $ident(&mut self, value: $value_ty) {
            T::$inner_ident($unwrap_self_mut(self), $unwrap_value(value))
        }
    }
);
swizzle_macro!(
    with_fns {
        #[inline(always)]
        pub fn $ident(self, value: $value_ty) -> $self_ty {
            $wrap_self(T::$inner_ident($unwrap_self(self), $unwrap_value(value)))
        }
    }
);
impl<T: Element> Vec2<T> {
    #[inline(always)]
    pub fn new(x: T, y: T) -> Self {
        T::wrap_vec2(T::new_vec2(x, y))
    }
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        T::wrap_vec2(T::splat_vec2(value))
    }

    vec2_get_swizzle!(
        T {
            get_fns!();
        }
    );
    vec2_mut_swizzle!(
        T {
            mut_fns!();
        }
    );
    vec2_set_swizzle!(
        T {
            set_fns!();
        }
    );
    vec2_with_swizzle!(
        T {
            with_fns!();
        }
    );
}