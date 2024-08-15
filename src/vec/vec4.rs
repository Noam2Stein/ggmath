use gmath_macros::*;

use crate::element::*;
use super::*;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Vec4<T: Element> {
    pub(crate) inner: T::Vec4Inner
}

pub fn vec4<T: Element>(value: impl Into<Vec4<T>>) -> Vec4<T> {
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
impl<T: Element> Vec4<T> {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T, w: T) -> Self {
        T::wrap_vec4(T::new_vec4(x, y, z, w))
    }
    #[inline(always)]
    pub fn splat(value: T) -> Self {
        T::wrap_vec4(T::splat_vec4(value))
    }

    vec4_get_swizzle!(
        T {
            get_fns!();
        }
    );
    vec4_mut_swizzle!(
        T {
            mut_fns!();
        }
    );
    vec4_set_swizzle!(
        T {
            set_fns!();
        }
    );
    vec4_with_swizzle!(
        T {
            with_fns!();
        }
    );
}