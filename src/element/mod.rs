use std::fmt::{self, Display};

pub mod default_impl;
mod impl_;

pub use ggmath_proc_macros::impl_element_inner_vecs;

pub trait Element:
    fmt::Debug
    + Copy
    + PartialEq
    + PartialOrd
    + Default
    + Display
    + ElementInnerVecs
    + ElementDefault
    + ElementVecsFromSplits
{
}
pub unsafe trait ElementInnerVecs {
    type InnerVec2: fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec3: fmt::Debug + Copy + PartialEq + PartialOrd;
    type InnerVec4: fmt::Debug + Copy + PartialEq + PartialOrd;
}
pub trait ElementDefault: ElementInnerVecs {
    fn default_vec2() -> Self::InnerVec2;
    fn default_vec3() -> Self::InnerVec3;
    fn default_vec4() -> Self::InnerVec4;
}
pub trait ElementVecsFromSplits: Sized + ElementInnerVecs {
    fn from_x_y(value: (Self, Self)) -> Self::InnerVec2;
    fn from_x_y_z(value: (Self, Self, Self)) -> Self::InnerVec3;
    fn from_xy_z(value: (Self::InnerVec2, Self)) -> Self::InnerVec3;
    fn from_x_yz(value: (Self, Self::InnerVec2)) -> Self::InnerVec3;
    fn from_x_y_z_w(value: (Self, Self, Self, Self)) -> Self::InnerVec4;
    fn from_xy_z_w(value: (Self::InnerVec2, Self, Self)) -> Self::InnerVec4;
    fn from_x_yz_w(value: (Self, Self::InnerVec2, Self)) -> Self::InnerVec4;
    fn from_x_y_zw(value: (Self, Self, Self::InnerVec2)) -> Self::InnerVec4;
    fn from_xy_zw(value: (Self::InnerVec2, Self::InnerVec2)) -> Self::InnerVec4;
    fn from_xyz_w(value: (Self::InnerVec3, Self)) -> Self::InnerVec4;
    fn from_x_yzw(value: (Self, Self::InnerVec3)) -> Self::InnerVec4;
}
