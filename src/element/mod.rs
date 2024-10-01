use std::fmt::{self, Display};

pub mod default_impl;
mod impl_;

pub use gomath_proc_macros::impl_element_inner_vecs;

use crate::vec::*;

pub trait Element:
    'static + fmt::Debug + Copy + PartialEq + PartialOrd + Default + Display + ElementVec
{
}
