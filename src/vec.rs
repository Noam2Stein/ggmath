use std::{fmt::{self, Display, Formatter}, ops};

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
vec_macro!(
    declare_vec {
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
        pub struct $self<T: Element> {
            pub(crate) inner: T::$inner
        }

        #[inline(always)]
        pub fn $self_lower<T: Element>(value: impl Into<$self<T>>) -> $self<T> {
            value.into()
        }

        impl<T: Element> $self<T> {
            #[inline(always)]
            pub fn new($($component: T), +) -> Self {
                Self::wrap(T::$inner_new($($component), +))
            }
            #[inline(always)]
            pub fn splat(value: T) -> Self {
                Self::wrap(T::$inner_splat(value))
            }

            #[inline(always)]
            pub fn wrap(inner: T::$inner) -> Self {
                $self {
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
        impl<T: Element> Display for $self<T> {
            fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
                write!(f, $display_literal, $(self.$component()), +)
            }
        }
        $(
            impl<T: $op_trait> ops::$op_trait for $self<T> {
                type Output = $self<T::Output>;
                #[inline(always)]
                fn $op_fn(self) -> Self::Output {
                    $self::wrap(T::$inner_op_fn(self.inner))
                }
            }
        )+
        $(
            impl<Rhs: Element, T: $rhs_op_trait<Rhs>> ops::$rhs_op_trait<$self<Rhs>> for $self<T> {
                type Output = $self<T::Output>;
                #[inline(always)]
                fn $rhs_op_fn(self, rhs: $self<Rhs>) -> Self::Output {
                    $self::wrap(T::$inner_rhs_op_fn(self.inner, rhs.inner))
                }
            }
            impl<Rhs: Element, T: $assign_op_trait<Rhs>> ops::$assign_op_trait<$self<Rhs>> for $self<T> {
                #[inline(always)]
                fn $assign_op_fn(&mut self, rhs: $self<Rhs>) {
                    T::$inner_assign_op_fn(&mut self.inner, rhs.inner);
                }
            }
        )+
    }
);

vecs!(
    declare_vec!();
);