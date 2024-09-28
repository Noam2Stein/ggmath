use std::fmt::{self, Display};

pub mod default_impl;
mod impl_;

pub unsafe trait Element:
    fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display
{
    type Vec2Inner;
    type Vec3Inner;
    type Vec4Inner;
}
#[allow(unused_macros)]
#[macro_export(local_inner_macros)]
macro_rules! impl_element {
    {
        $vec2inner:ty,
        $vec3inner:ty,
        $vec4inner:ty,
        impl Element for $self:ty {
            $($tt:tt)*
        }
    } => {
        unsafe impl Element for $self {
            type Vec2Inner = $vec2inner;
            type Vec3Inner = $vec3inner;
            type Vec4Inner = $vec4inner;
            $($tt)*
        }

        const _: () = {
            unsafe fn validate_vec2(value: $vec2inner) -> [$self; 2] { std::mem::transmute(value) }
            unsafe fn validate_vec3(value: $vec3inner) -> [$self; 4] { std::mem::transmute(value) }
            unsafe fn validate_vec4(value: $vec4inner) -> [$self; 4] { std::mem::transmute(value) }
        };
    };
}
