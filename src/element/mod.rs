use std::fmt::{self, Display};

use crate::vec::inner::*;

pub mod default_impl;
mod impl_;

pub use ggmath_proc_macros::impl_element;

pub unsafe trait Element:
    fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display
{
    type InnerVec2: InnerVec2<T = Self>;
    type InnerVec3: InnerVec3<T = Self>;
    type InnerVec4: InnerVec4<T = Self>;
}

pub trait ElementContainer: fmt::Debug + Copy + PartialEq + PartialOrd {
    type T: Element;
}